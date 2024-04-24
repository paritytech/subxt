// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{RawRpcFuture, RawRpcSubscription, RpcClientT};
use crate::error::RpcError;
use futures::{FutureExt, StreamExt, TryStreamExt};
use reconnecting_jsonrpsee_ws_client::{CallRetryPolicy, Client as InnerClient, SubscriptionId};
use serde_json::value::RawValue;
use std::time::Duration;

pub use reconnecting_jsonrpsee_ws_client::{
    ExponentialBackoff, FibonacciBackoff, FixedInterval, IdKind,
};

#[cfg(feature = "native")]
use reconnecting_jsonrpsee_ws_client::{HeaderMap, PingConfig};

/// Builder for [`Client`].
#[derive(Debug, Clone)]
pub struct Builder<P> {
    max_request_size: u32,
    max_response_size: u32,
    retry_policy: P,
    max_redirections: u32,
    id_kind: IdKind,
    max_log_len: u32,
    max_concurrent_requests: u32,
    request_timeout: Duration,
    connection_timeout: Duration,
    #[cfg(feature = "native")]
    ping_config: Option<PingConfig>,
    #[cfg(feature = "native")]
    headers: HeaderMap,
}

impl Default for Builder<ExponentialBackoff> {
    fn default() -> Self {
        Self {
            max_request_size: 10 * 1024 * 1024,
            max_response_size: 10 * 1024 * 1024,
            retry_policy: ExponentialBackoff::from_millis(10).max_delay(Duration::from_secs(60)),
            max_redirections: 5,
            id_kind: IdKind::Number,
            max_log_len: 1024,
            max_concurrent_requests: 1024,
            request_timeout: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(10),
            #[cfg(feature = "native")]
            ping_config: Some(PingConfig::new()),
            #[cfg(feature = "native")]
            headers: HeaderMap::new(),
        }
    }
}

impl Builder<ExponentialBackoff> {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<P> Builder<P>
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

    /// Configure which retry policy to use.
    ///
    /// Default: Exponential backoff 10ms
    pub fn retry_policy<T>(self, retry_policy: T) -> Builder<T> {
        Builder {
            max_request_size: self.max_request_size,
            max_response_size: self.max_response_size,
            retry_policy,
            max_redirections: self.max_redirections,
            max_log_len: self.max_log_len,
            id_kind: self.id_kind,
            max_concurrent_requests: self.max_concurrent_requests,
            request_timeout: self.request_timeout,
            connection_timeout: self.connection_timeout,
            #[cfg(feature = "native")]
            ping_config: self.ping_config,
            #[cfg(feature = "native")]
            headers: self.headers,
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

    #[cfg(feature = "native")]
    #[cfg_attr(docsrs, doc(cfg(native)))]
    /// Configure custom headers to use in the WebSocket handshake.
    pub fn set_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    /// Build and connect to the target.
    pub async fn build(self, url: String) -> Result<Client, RpcError> {
        let client = InnerClient::builder()
            .retry_policy(self.retry_policy)
            .build(url)
            .await
            .map_err(|e| RpcError::ClientError(Box::new(e)))?;

        Ok(Client(client))
    }
}

/// Reconnecting rpc client.
#[derive(Debug, Clone)]
pub struct Client(InnerClient);

impl Client {
    /// Create a builder.
    pub fn builder() -> Builder<ExponentialBackoff> {
        Builder::new()
    }

    /// A future that returns once the client has started to reconnect.
    ///
    /// This may be called multiple times.
    pub async fn reconnect_started(&self) {
        self.0.reconnect_started().await
    }

    /// A future that return once the client has reconnected.
    ///
    /// This may be called multiple times.
    pub async fn reconnected(&self) {
        self.0.reconnected().await
    }

    /// Get how many times the client has reconnected successfully.
    pub fn reconnect_count(&self) -> usize {
        self.0.reconnect_count()
    }
}

impl RpcClientT for Client {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        async {
            self.0
                .request_raw_with_policy(method.to_string(), params, CallRetryPolicy::Drop)
                .await
                .map_err(|e| RpcError::DisconnectedWillReconnect(e.to_string()))
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
                .0
                .subscribe_raw_with_policy(
                    sub.to_string(),
                    params,
                    unsub.to_string(),
                    CallRetryPolicy::Drop,
                )
                .await
                .map_err(|e| RpcError::ClientError(Box::new(e)))?;

            let id = match sub.id() {
                SubscriptionId::Num(n) => n.to_string(),
                SubscriptionId::Str(s) => s.to_string(),
            };
            let stream = sub
                .map_err(|e| RpcError::DisconnectedWillReconnect(e.to_string()))
                .boxed();

            Ok(RawRpcSubscription {
                stream,
                id: Some(id),
            })
        }
        .boxed()
    }
}
