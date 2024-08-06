// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # reconnecting-jsonrpsee-ws-client
//!

mod platform;
#[cfg(test)]
mod tests;
mod utils;

use std::{
    pin::Pin,
    sync::Arc,
    task::{self, Poll},
    time::Duration,
};

use super::{RawRpcFuture, RawRpcSubscription, RpcClientT};
use crate::error::RpcError as SubxtRpcError;

use finito::Retry;
use futures::{Future, FutureExt, Stream, StreamExt, TryStreamExt};
use jsonrpsee::core::{
    client::{
        Client as WsClient, ClientT, Subscription as RpcSubscription, SubscriptionClientT,
        SubscriptionKind,
    },
    traits::ToRpcParams,
};
use serde_json::value::RawValue;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot, Notify,
};
use utils::display_close_reason;
use utils::{reconnect_channel, ReconnectRx, ReconnectTx};

// re-exports
pub use finito::{ExponentialBackoff, FibonacciBackoff, FixedInterval};
pub use jsonrpsee::core::client::IdKind;
pub use jsonrpsee::{core::client::error::Error as RpcError, rpc_params, types::SubscriptionId};

#[cfg(feature = "native")]
pub use jsonrpsee::ws_client::{HeaderMap, PingConfig};

const LOG_TARGET: &str = "subxt-reconnecting-rpc-client";

/// Method result.
pub type MethodResult = Result<Box<RawValue>, Error>;
/// Subscription result.
pub type SubscriptionResult = Result<Box<RawValue>, DisconnectedWillReconnect>;

/// The connection was closed, reconnect initiated and the subscription was dropped.
#[derive(Debug, thiserror::Error)]
#[error("The connection was closed because of `{0:?}` and reconnect initiated")]
pub struct DisconnectedWillReconnect(String);

/// Serialized JSON-RPC params.
#[derive(Debug, Clone)]
pub struct RpcParams(Option<Box<RawValue>>);

impl RpcParams {
    /// Create new [`RpcParams`] from JSON.
    pub fn new(json: Option<Box<RawValue>>) -> Self {
        Self(json)
    }
}

impl ToRpcParams for RpcParams {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
        Ok(self.0)
    }
}

#[derive(Debug)]
enum Op {
    Call {
        method: String,
        params: RpcParams,
        send_back: oneshot::Sender<MethodResult>,
    },
    Subscription {
        subscribe_method: String,
        params: RpcParams,
        unsubscribe_method: String,
        send_back: oneshot::Sender<Result<Subscription, Error>>,
    },
}

/// Error that can occur when for a RPC call or subscription.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The client was dropped by the user.
    #[error("The client was dropped")]
    Dropped,
    /// The connection was closed and reconnect initiated.
    #[error(transparent)]
    DisconnectedWillReconnect(#[from] DisconnectedWillReconnect),
    /// Other rpc error.
    #[error("{0}")]
    RpcError(RpcError),
}

/// Represent a single subscription.
pub struct Subscription {
    id: SubscriptionId<'static>,
    stream: mpsc::UnboundedReceiver<SubscriptionResult>,
}

impl Subscription {
    /// Returns the next notification from the stream.
    /// This may return `None` if the subscription has been terminated,
    /// which may happen if the channel becomes full or is dropped.
    ///
    /// **Note:** This has an identical signature to the [`StreamExt::next`]
    /// method (and delegates to that). Import [`StreamExt`] if you'd like
    /// access to other stream combinator methods.
    #[allow(clippy::should_implement_trait)]
    pub async fn next(&mut self) -> Option<SubscriptionResult> {
        StreamExt::next(self).await
    }

    /// Get the subscription ID.
    pub fn id(&self) -> SubscriptionId<'static> {
        self.id.clone()
    }
}

impl Stream for Subscription {
    type Item = SubscriptionResult;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        match self.stream.poll_recv(cx) {
            Poll::Ready(Some(msg)) => Poll::Ready(Some(msg)),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl std::fmt::Debug for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Subscription {:?}", self.id))
    }
}

/// JSON-RPC client that reconnects automatically and may loose
/// subscription notifications when it reconnects.
#[derive(Clone, Debug)]
pub struct Client {
    tx: mpsc::UnboundedSender<Op>,
    reconnect: ReconnectRx,
}

/// Builder for [`Client`].
#[derive(Clone, Debug)]
pub struct ClientBuilder<P> {
    max_request_size: u32,
    max_response_size: u32,
    retry_policy: P,
    #[cfg(feature = "native")]
    ping_config: Option<PingConfig>,
    #[cfg(feature = "native")]
    // web doesn't support custom headers
    // https://stackoverflow.com/a/4361358/6394734
    headers: HeaderMap,
    max_redirections: u32,
    id_kind: IdKind,
    max_log_len: u32,
    max_concurrent_requests: u32,
    request_timeout: Duration,
    connection_timeout: Duration,
}

impl Default for ClientBuilder<ExponentialBackoff> {
    fn default() -> Self {
        Self {
            max_request_size: 10 * 1024 * 1024,
            max_response_size: 10 * 1024 * 1024,
            retry_policy: ExponentialBackoff::from_millis(10).max_delay(Duration::from_secs(60)),
            #[cfg(feature = "native")]
            ping_config: Some(PingConfig::new()),
            #[cfg(feature = "native")]
            headers: HeaderMap::new(),
            max_redirections: 5,
            id_kind: IdKind::Number,
            max_log_len: 1024,
            max_concurrent_requests: 1024,
            request_timeout: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(10),
        }
    }
}

impl ClientBuilder<ExponentialBackoff> {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<P> ClientBuilder<P>
where
    P: Iterator<Item = Duration> + Send + Sync + 'static + Clone,
{
    /// Configure the min response size a for websocket message.
    ///
    /// Default: 10MB
    pub fn max_request_size(mut self, max: u32) -> Self {
        self.max_request_size = max;
        self
    }

    /// Configure the max response size a for websocket message.
    ///
    /// Default: 10MB
    pub fn max_response_size(mut self, max: u32) -> Self {
        self.max_response_size = max;
        self
    }

    /// Set the max number of redirections to perform until a connection is regarded as failed.
    ///
    /// Default: 5
    pub fn max_redirections(mut self, redirect: u32) -> Self {
        self.max_redirections = redirect;
        self
    }

    /// Configure how many concurrent method calls are allowed.
    ///
    /// Default: 1024
    pub fn max_concurrent_requests(mut self, max: u32) -> Self {
        self.max_concurrent_requests = max;
        self
    }

    /// Configure how long until a method call is regarded as failed.
    ///
    /// Default: 1 minute
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Set connection timeout for the WebSocket handshake
    ///
    /// Default: 10 seconds
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// Configure the data type of the request object ID
    ///
    /// Default: number
    pub fn id_format(mut self, kind: IdKind) -> Self {
        self.id_kind = kind;
        self
    }

    /// Set maximum length for logging calls and responses.
    /// Logs bigger than this limit will be truncated.
    ///
    /// Default: 1024
    pub fn set_max_logging_length(mut self, max: u32) -> Self {
        self.max_log_len = max;
        self
    }

    #[cfg(feature = "native")]
    #[cfg_attr(docsrs, doc(cfg(feature = "native")))]
    /// Configure custom headers to use in the WebSocket handshake.
    pub fn set_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    /// Configure which retry policy to use when a connection is lost.
    ///
    /// Default: Exponential backoff 10ms
    pub fn retry_policy<T>(self, retry_policy: T) -> ClientBuilder<T> {
        ClientBuilder {
            max_request_size: self.max_request_size,
            max_response_size: self.max_response_size,
            retry_policy,
            #[cfg(feature = "native")]
            ping_config: self.ping_config,
            #[cfg(feature = "native")]
            headers: self.headers,
            max_redirections: self.max_redirections,
            max_log_len: self.max_log_len,
            id_kind: self.id_kind,
            max_concurrent_requests: self.max_concurrent_requests,
            request_timeout: self.request_timeout,
            connection_timeout: self.connection_timeout,
        }
    }

    #[cfg(feature = "native")]
    #[cfg_attr(docsrs, doc(cfg(feature = "native")))]
    /// Configure the WebSocket ping/pong interval.
    ///
    /// Default: 30 seconds.
    pub fn enable_ws_ping(mut self, ping_config: PingConfig) -> Self {
        self.ping_config = Some(ping_config);
        self
    }

    #[cfg(feature = "native")]
    #[cfg_attr(docsrs, doc(cfg(feature = "native")))]
    /// Disable WebSocket ping/pongs.
    ///
    /// Default: 30 seconds.
    pub fn disable_ws_ping(mut self) -> Self {
        self.ping_config = None;
        self
    }

    /// Build and connect to the target.
    pub async fn build(self, url: String) -> Result<Client, RpcError> {
        let (tx, rx) = mpsc::unbounded_channel();
        let client = Retry::new(self.retry_policy.clone(), || {
            platform::ws_client(url.as_ref(), &self)
        })
        .await?;
        let (reconn_tx, reconn_rx) = reconnect_channel();

        platform::spawn(background_task(client, rx, url, reconn_tx, self));

        Ok(Client {
            tx,
            reconnect: reconn_rx,
        })
    }
}

impl Client {
    /// Perform a JSON-RPC method call.
    pub async fn request(
        &self,
        method: String,
        params: Option<Box<RawValue>>,
    ) -> Result<Box<RawValue>, Error> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Op::Call {
                method,
                params: RpcParams(params),
                send_back: tx,
            })
            .map_err(|_| Error::Dropped)?;

        rx.await.map_err(|_| Error::Dropped)?
    }

    /// Perform a JSON-RPC subscription.
    pub async fn subscribe(
        &self,
        subscribe_method: String,
        params: Option<Box<RawValue>>,
        unsubscribe_method: String,
    ) -> Result<Subscription, Error> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Op::Subscription {
                subscribe_method,
                params: RpcParams::new(params),
                unsubscribe_method,
                send_back: tx,
            })
            .map_err(|_| Error::Dropped)?;
        rx.await.map_err(|_| Error::Dropped)?
    }

    /// A future that resolves when the client has initiated a reconnection.
    /// This method returns another future that resolves when the client has reconnected.
    ///
    /// This may be called multiple times.
    pub async fn reconnect_initiated(&self) -> impl Future<Output = ()> + '_ {
        self.reconnect.reconnect_started().await;
        self.reconnect.reconnected()
    }

    /// Get how many times the client has reconnected successfully.
    pub fn reconnect_count(&self) -> usize {
        self.reconnect.count()
    }
}

impl Client {
    /// Create a builder.
    pub fn builder() -> ClientBuilder<ExponentialBackoff> {
        ClientBuilder::new()
    }
}

impl RpcClientT for Client {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        async {
            self.request(method.to_string(), params)
                .await
                .map_err(|e| SubxtRpcError::DisconnectedWillReconnect(e.to_string()))
        }
        .boxed()
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        async {
            let sub = self
                .subscribe(sub.to_string(), params, unsub.to_string())
                .await
                .map_err(|e| SubxtRpcError::ClientError(Box::new(e)))?;

            let id = match sub.id() {
                SubscriptionId::Num(n) => n.to_string(),
                SubscriptionId::Str(s) => s.to_string(),
            };
            let stream = sub
                .map_err(|e| SubxtRpcError::DisconnectedWillReconnect(e.to_string()))
                .boxed();

            Ok(RawRpcSubscription {
                stream,
                id: Some(id),
            })
        }
        .boxed()
    }
}

async fn background_task<P>(
    mut client: Arc<WsClient>,
    mut rx: UnboundedReceiver<Op>,
    url: String,
    reconn: ReconnectTx,
    client_builder: ClientBuilder<P>,
) where
    P: Iterator<Item = Duration> + Send + 'static + Clone,
{
    let disconnect = Arc::new(tokio::sync::Notify::new());

    loop {
        tokio::select! {
            // An incoming JSON-RPC call to dispatch.
            next_message = rx.recv() => {
                match next_message {
                    None => break,
                    Some(op) => {
                       tokio::spawn(dispatch_call(client.clone(), op, disconnect.clone()));
                    }
                };
            }
            // The connection was terminated and try to reconnect.
            _ = client.on_disconnect() => {
                let params = ReconnectParams {
                    url: &url,
                    reconnect: reconn.clone(),
                    client_builder: &client_builder,
                    close_reason: client.disconnect_reason().await,
                };

                client = match reconnect(params).await {
                    Ok(client) => client,
                    Err(e) => {
                        tracing::debug!(target: LOG_TARGET, "Failed to reconnect: {e}; terminating the connection");
                        break;
                    }
                };
            }
        }
    }

    disconnect.notify_waiters();
}

async fn dispatch_call(client: Arc<WsClient>, op: Op, on_disconnect: Arc<tokio::sync::Notify>) {
    match op {
        Op::Call {
            method,
            params,
            send_back,
        } => {
            match client.request::<Box<RawValue>, _>(&method, params).await {
                Ok(rp) => {
                    // Fails only if the request is dropped by the client.
                    let _ = send_back.send(Ok(rp));
                }
                Err(RpcError::RestartNeeded(e)) => {
                    // Fails only if the request is dropped by the client.
                    let _ = send_back.send(Err(DisconnectedWillReconnect(e.to_string()).into()));
                }
                Err(e) => {
                    // Fails only if the request is dropped by the client.
                    let _ = send_back.send(Err(Error::RpcError(e)));
                }
            }
        }
        Op::Subscription {
            subscribe_method,
            params,
            unsubscribe_method,
            send_back,
        } => {
            match client
                .subscribe::<Box<RawValue>, _>(
                    &subscribe_method,
                    params.clone(),
                    &unsubscribe_method,
                )
                .await
            {
                Ok(sub) => {
                    let (tx, rx) = mpsc::unbounded_channel();
                    let sub_id = match sub.kind() {
                        SubscriptionKind::Subscription(id) => id.clone().into_owned(),
                        _ => unreachable!("No method subscriptions possible in this crate; qed"),
                    };

                    platform::spawn(subscription_handler(
                        tx.clone(),
                        sub,
                        on_disconnect.clone(),
                        client.clone(),
                    ));

                    let stream = Subscription {
                        id: sub_id,
                        stream: rx,
                    };

                    // Fails only if the request is dropped by the client.
                    let _ = send_back.send(Ok(stream));
                }
                Err(RpcError::RestartNeeded(e)) => {
                    // Fails only if the request is dropped by the client.
                    let _ = send_back.send(Err(DisconnectedWillReconnect(e.to_string()).into()));
                }
                Err(e) => {
                    // Fails only if the request is dropped.
                    let _ = send_back.send(Err(Error::RpcError(e)));
                }
            }
        }
    }
}

/// Handler for each individual subscription.
async fn subscription_handler(
    sub_tx: UnboundedSender<SubscriptionResult>,
    mut rpc_sub: RpcSubscription<Box<RawValue>>,
    client_closed: Arc<Notify>,
    client: Arc<WsClient>,
) {
    loop {
        tokio::select! {
            next_msg = rpc_sub.next() => {
                let Some(notif) = next_msg else {
                    let close = client.disconnect_reason().await;
                    _ = sub_tx.send(Err(DisconnectedWillReconnect(close.to_string())));
                    break;
                };

                let msg = notif.expect("RawValue is valid JSON; qed");

                // Fails only if subscription was closed by the user.
                if sub_tx.send(Ok(msg)).is_err() {
                    break;
                }
            }
             // This channel indices whether the subscription was closed by user.
             _ = sub_tx.closed() => {
                break;
            }
            // This channel indicates whether the main task has been closed.
            // at this point no further messages are processed.
            _ = client_closed.notified() => {
                break;
            }
        }
    }
}

struct ReconnectParams<'a, P> {
    url: &'a str,
    reconnect: ReconnectTx,
    client_builder: &'a ClientBuilder<P>,
    close_reason: RpcError,
}

async fn reconnect<P>(params: ReconnectParams<'_, P>) -> Result<Arc<WsClient>, RpcError>
where
    P: Iterator<Item = Duration> + Send + 'static + Clone,
{
    let ReconnectParams {
        url,
        reconnect,
        client_builder,
        close_reason,
    } = params;

    let retry_policy = client_builder.retry_policy.clone();

    tracing::debug!(target: LOG_TARGET, "Connection to {url} was closed: `{}`; starting to reconnect", display_close_reason(&close_reason));
    reconnect.reconnect_initiated();

    let client = Retry::new(retry_policy.clone(), || {
        platform::ws_client(url, client_builder)
    })
    .await?;

    reconnect.reconnected();
    tracing::debug!(target: LOG_TARGET, "Connection to {url} was successfully re-established");

    Ok(client)
}
