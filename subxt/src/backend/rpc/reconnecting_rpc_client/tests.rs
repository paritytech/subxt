// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::*;
use futures::{future::Either, FutureExt};

use jsonrpsee::core::BoxError;
use jsonrpsee::server::{
    http, stop_channel, ws, ConnectionGuard, ConnectionState, HttpRequest, HttpResponse, RpcModule,
    RpcServiceBuilder, ServerConfig, SubscriptionMessage,
};

#[tokio::test]
async fn call_works() {
    tracing_subscriber::fmt::init();
    let (_handle, addr) = run_server().await.unwrap();
    let client = RpcClient::builder().build(addr).await.unwrap();
    assert!(client.request("say_hello".to_string(), None).await.is_ok(),)
}

#[tokio::test]
async fn sub_works() {
    tracing_subscriber::fmt::init();
    let (_handle, addr) = run_server().await.unwrap();

    let client = RpcClient::builder()
        .retry_policy(ExponentialBackoff::from_millis(50))
        .build(addr)
        .await
        .unwrap();

    let mut sub = client
        .subscribe(
            "subscribe_lo".to_string(),
            None,
            "unsubscribe_lo".to_string(),
        )
        .await
        .unwrap();

    assert!(sub.next().await.is_some());
}

#[tokio::test]
async fn sub_with_reconnect() {
    tracing_subscriber::fmt::init();
    let (handle, addr) = run_server().await.unwrap();
    let client = RpcClient::builder().build(addr.clone()).await.unwrap();

    let mut sub = client
        .subscribe(
            "subscribe_lo".to_string(),
            None,
            "unsubscribe_lo".to_string(),
        )
        .await
        .unwrap();

    let _ = handle.send(());

    // Hack to wait for the server to restart.
    tokio::time::sleep(Duration::from_millis(100)).await;

    assert!(matches!(sub.next().await, Some(Ok(_))));
    assert!(matches!(
        sub.next().await,
        Some(Err(DisconnectedWillReconnect(_)))
    ));

    // Restart the server.
    let (_handle, _) = run_server_with_settings(Some(&addr), false).await.unwrap();

    // Hack to wait for the server to restart.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Subscription should work after reconnect.
    let mut sub = client
        .subscribe(
            "subscribe_lo".to_string(),
            None,
            "unsubscribe_lo".to_string(),
        )
        .await
        .unwrap();

    assert!(matches!(sub.next().await, Some(Ok(_))));
}

#[tokio::test]
async fn call_with_reconnect() {
    tracing_subscriber::fmt::init();
    let (handle, addr) = run_server_with_settings(None, true).await.unwrap();

    let client = Arc::new(RpcClient::builder().build(addr.clone()).await.unwrap());

    let req_fut = client.request("say_hello".to_string(), None).boxed();
    let timeout_fut = tokio::time::sleep(Duration::from_secs(5));

    // If the call isn't replied in 5 secs then it's regarded as it's still pending.
    let req_fut = match futures::future::select(Box::pin(timeout_fut), req_fut).await {
        Either::Left((_, f)) => f,
        Either::Right(_) => panic!("RPC call finished"),
    };

    // Close the connection with a pending call.
    let _ = handle.send(());

    // Restart the server
    let (_handle, _) = run_server_with_settings(Some(&addr), false).await.unwrap();

    // Hack to wait for the server to restart.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // This call should fail because reconnect.
    assert!(req_fut.await.is_err());
    // Future call should work after reconnect.
    assert!(client.request("say_hello".to_string(), None).await.is_ok());
}

async fn run_server() -> Result<(tokio::sync::broadcast::Sender<()>, String), BoxError> {
    run_server_with_settings(None, false).await
}

async fn run_server_with_settings(
    url: Option<&str>,
    dont_respond_to_method_calls: bool,
) -> Result<(tokio::sync::broadcast::Sender<()>, String), BoxError> {
    use jsonrpsee::server::HttpRequest;

    let sockaddr = match url {
        Some(url) => url.strip_prefix("ws://").unwrap(),
        None => "127.0.0.1:0",
    };

    let mut i = 0;

    let listener = loop {
        if let Ok(l) = tokio::net::TcpListener::bind(sockaddr).await {
            break l;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;

        if i >= 10 {
            panic!("Addr already in use");
        }

        i += 1;
    };

    let mut module = RpcModule::new(());

    if dont_respond_to_method_calls {
        module.register_async_method("say_hello", |_, _, _| async {
            futures::future::pending::<()>().await;
            "timeout"
        })?;
    } else {
        module.register_async_method("say_hello", |_, _, _| async { "lo" })?;
    }

    module.register_subscription(
        "subscribe_lo",
        "subscribe_lo",
        "unsubscribe_lo",
        |_params, pending, _ctx, _| async move {
            let sink = pending.accept().await.unwrap();
            let i = 0;

            loop {
                if sink
                    .send(SubscriptionMessage::from_json(&i).unwrap())
                    .await
                    .is_err()
                {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(6)).await;
            }
        },
    )?;

    let (tx, mut rx) = tokio::sync::broadcast::channel(4);
    let tx2 = tx.clone();
    let (stop_handle, server_handle) = stop_channel();
    let addr = listener.local_addr().expect("Could not find local addr");

    tokio::spawn(async move {
        loop {
            let sock = tokio::select! {
                res = listener.accept() => {
                    match res {
                        Ok((stream, _remote_addr)) => stream,
                        Err(e) => {
                            tracing::error!("Failed to accept connection: {:?}", e);
                            continue;
                        }
                    }
                }
                _ = rx.recv() => {
                    break
                }
            };

            let module = module.clone();
            let rx2 = tx2.subscribe();
            let tx2 = tx2.clone();
            let stop_handle2 = stop_handle.clone();

            let svc = tower::service_fn(move |req: HttpRequest<hyper::body::Incoming>| {
                let module = module.clone();
                let tx = tx2.clone();
                let stop_handle = stop_handle2.clone();

                let conn_permit = ConnectionGuard::new(1).try_acquire().unwrap();

                if ws::is_upgrade_request(&req) {
                    let rpc_service = RpcServiceBuilder::new();
                    let conn = ConnectionState::new(stop_handle, 1, conn_permit);

                    async move {
                        let mut rx = tx.subscribe();

                        let (rp, conn_fut) =
                            ws::connect(req, ServerConfig::default(), module, conn, rpc_service)
                                .await
                                .unwrap();

                        tokio::spawn(async move {
                            tokio::select! {
                                _ = conn_fut => (),
                                _ = rx.recv() => {},
                            }
                        });

                        Ok::<_, BoxError>(rp)
                    }
                    .boxed()
                } else {
                    async { Ok(http::response::denied()) }.boxed()
                }
            });

            tokio::spawn(serve_with_graceful_shutdown(sock, svc, rx2));
        }

        drop(server_handle);
    });

    Ok((tx, format!("ws://{}", addr)))
}

async fn serve_with_graceful_shutdown<S, B, I>(
    io: I,
    service: S,
    mut rx: tokio::sync::broadcast::Receiver<()>,
) where
    S: tower::Service<HttpRequest<hyper::body::Incoming>, Response = HttpResponse<B>>
        + Clone
        + Send
        + 'static,
    S::Future: Send,
    S::Response: Send,
    S::Error: Into<BoxError>,
    B: http_body::Body<Data = hyper::body::Bytes> + Send + 'static,
    B::Error: Into<BoxError>,
    I: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send + Unpin + 'static,
{
    if let Err(e) =
        jsonrpsee::server::serve_with_graceful_shutdown(io, service, rx.recv().map(|_| ())).await
    {
        tracing::error!("Error while serving: {:?}", e);
    }
}
