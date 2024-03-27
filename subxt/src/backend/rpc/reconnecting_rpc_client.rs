// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub use reconnecting_jsonrpsee_ws_client::{CallRetryPolicy, RetryPolicy};

use super::{RawRpcFuture, RawRpcSubscription, RpcClientT};
use crate::error::RpcError;
use futures::{FutureExt, StreamExt, TryStreamExt};
use reconnecting_jsonrpsee_ws_client::{Client as InnerClient, SubscriptionId};
use serde_json::value::RawValue;
use std::time::Duration;

/// Reconnecting rpc client builder.
pub struct Builder {
    retry_policy: RetryPolicy,
}

impl Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            retry_policy: RetryPolicy::exponential(Duration::from_millis(10))
                .with_max_delay(Duration::from_secs(30)),
        }
    }

    /// Set retry policy when reconnecting.
    pub fn retry_policy_for_reconnect(self, retry_policy: RetryPolicy) -> Self {
        Self { retry_policy }
    }

    /// Build a new rpc client i.e, connect.
    pub async fn build(self, url: String) -> Result<Client, RpcError> {
        let client = InnerClient::builder()
            .retry_policy(self.retry_policy)
            .build(url)
            .await
            .map_err(|e| RpcError::ClientError(Box::new(e)))?;

        Ok(Client { inner: client })
    }
}

/// Reconnecting rpc client.
pub struct Client {
    inner: InnerClient,
}

impl Client {
    /// Create a builder.
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Future that returns when the reconnection has started.
    pub async fn on_reconnect(&self) {
        self.inner.on_reconnect().await
    }

    /// Counter to determine how many times the client has reconnected.
    pub fn reconnect_count(&self) -> usize {
        self.inner.reconnect_count()
    }
}

impl RpcClientT for Client {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        async {
            self.inner
                .request_raw_with_policy(method.to_string(), params, CallRetryPolicy::Drop)
                .await
                .map_err(|e| RpcError::ClientError(Box::new(e)))
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
                .inner
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
