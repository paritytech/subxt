// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::*;
use futures::{future::Either, FutureExt};

use jsonrpsee::core::BoxError;
use jsonrpsee::server::middleware::rpc::RpcServiceBuilder;
use jsonrpsee::server::{
    http, stop_channel, ws, ConnectionGuard, ConnectionState, HttpRequest, HttpResponse, RpcModule,
    ServerConfig,
};

#[tokio::test]
async fn call_works() {
    let (_handle, addr) = run_server().await.unwrap();
    let client = RpcClient::builder().build(addr).await.unwrap();
    assert!(client.request("say_hello".to_string(), None).await.is_ok(),)
}

#[tokio::test]
async fn sub_works() {
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
    let (handle, addr) = run_server().await.unwrap();
    let client = RpcClient::builder().build(addr.clone()).await.unwrap();

    let sub = client
        .subscribe(
            "subscribe_lo".to_string(),
            None,
            "unsubscribe_lo".to_string(),
        )
        .await
        .unwrap();

    // Tell server to shut down.
    let _ = handle.send(());

    // Drain any values from the subscription. We should end with a DisconnectedWillReconnect error,
    // so that subscriptions have the opportunity to react to the fact that we were disconnected.
    let sub_ended_with_disconnect_err = sub.fold(false, async |_, next| matches!(next, Err(DisconnectedWillReconnect(_))));
    let sub_ended_with_disconnect_err = tokio::time::timeout(tokio::time::Duration::from_secs(5), sub_ended_with_disconnect_err)
        .await
        .expect("timeout should not be hit");

    assert!(sub_ended_with_disconnect_err, "DisconnectedWillReconnect err was last message in sub");

    // Start a new server at the same address as the old one. (This will wait a bit for the addr to be free)
    let (_handle, _) = run_server_with_settings(Some(&addr), false).await.unwrap();

    // Hack to wait for the server to restart.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // We can subscribe again on the same client and it should work.
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

#[tokio::test]
async fn call_with_reconnect_after_hanging_handshake() {
    let (handle, addr) = run_server().await.unwrap();
    let client = RpcClient::builder()
        .retry_policy(FixedInterval::from_millis(10))
        .connection_timeout(Duration::from_millis(500))
        .build(addr.clone())
        .await
        .unwrap();

    // Shut down the server, the client starts to reconnect.
    let _ = handle.send(());

    // Accept the next reconnect attempt and keep the connection open without ever answering
    // the WebSocket handshake.
    let sockaddr = addr.strip_prefix("ws://").unwrap();
    let listener = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if let Ok(listener) = tokio::net::TcpListener::bind(sockaddr).await {
                break listener;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("address should be free after the server shut down");
    let (_unresponsive_conn, _) = tokio::time::timeout(Duration::from_secs(5), listener.accept())
        .await
        .expect("client should try to reconnect")
        .unwrap();
    drop(listener);

    let (_handle, _) = run_server_with_settings(Some(&addr), false).await.unwrap();

    let res = tokio::time::timeout(
        Duration::from_secs(5),
        client.request("say_hello".to_string(), None),
    )
    .await;
    assert!(
        matches!(res, Ok(Ok(_))),
        "client didn't recover after a reconnect attempt hung in the handshake: {res:?}"
    );
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

        if i >= 100 {
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
                    .send(serde_json::value::to_raw_value(&i).unwrap())
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

    Ok((tx, format!("ws://{addr}")))
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
