// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.
use super::{
    background::{BackendMessage, BackgroundTask},
    LightClientError,
};
use crate::{
    error::{Error, RpcError},
    rpc::{RpcClientT, RpcFuture, RpcSubscription},
};
use futures::{lock::Mutex as AsyncMutex, stream::StreamExt, Stream};
use serde_json::value::RawValue;
use smoldot_light::{platform::async_std::AsyncStdTcpWebSocket, ChainId};
use std::{pin::Pin, sync::Arc};
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::ReceiverStream;

const LOG_TARGET: &str = "light-client";

/// Inner structure to work with light clients.
struct LightClientInner {
    /// Smoldot light client implementation that leverages the `AsyncStdTcpWebSocket`.
    ///
    /// Note: `AsyncStdTcpWebSocket` is not wasm compatible.
    client: smoldot_light::Client<AsyncStdTcpWebSocket>,
    /// The ID of the chain used to identify the chain protocol (ie. substrate).
    ///
    /// Note: A single chain is supported for a client. This aligns with the subxt's
    /// vision of the Client.
    chain_id: ChainId,
    /// Communicate with the backend task.
    to_backend: mpsc::Sender<BackendMessage>,
    /// Atomic used to generate unique IDs.
    id_provider: u64,
}

impl LightClientInner {
    /// Generate the next unique ID used to populate the Json RPC request.
    ///
    /// This is unique to identify the sender of the request.
    fn next_id(&mut self) -> String {
        let id = self.id_provider;
        self.id_provider += 1;
        id.to_string()
    }

    /// Register a RPC method request.
    ///
    /// Returns a channel that produces only one item, which is the result of the method.
    ///
    /// The result is a raw jsonrpc string similar to:
    ///
    /// ```bash
    /// {"jsonrpc":"2.0","id":"1","result":"my result object"}
    /// ```
    ///
    /// # Note
    ///
    /// Registering the request must happen before submitting the request in order
    /// for the background task to provide a response.
    async fn register_request(
        &self,
        id: String,
    ) -> Result<oneshot::Receiver<Box<RawValue>>, LightClientError> {
        let (sender, receiver) = oneshot::channel();

        self.to_backend
            .send(BackendMessage::Request { id, sender })
            .await
            .map_err(|_| LightClientError::BackgroundClosed)?;

        Ok(receiver)
    }

    /// Register a RPC subscription request.
    ///
    /// Returns a channel that produces the items of the subscription.
    ///
    /// The JsonRPC subscription is generated as follows:
    /// - Make a plain RPC method request which returns the subscription ID, in the result field:
    ///
    /// ```bash
    /// {"jsonrpc":"2.0","id":"1","result":"0"}
    /// ```
    ///
    /// - Register with the provided ID to the notifications of the subscription. Notifications look like:
    ///
    /// ```bash
    /// {"jsonrpc":"2.0","method":"author_extrinsicUpdate","params":{"subscription":"0","result":"Dropped"}}
    /// ```
    ///
    /// # Note
    ///
    /// The notification messages are buffered internally to ensure that users will receive all
    /// messages in the following case:
    ///
    /// * T0. [`Self::register_request()`].
    /// * T1. submit a plain RPC method request.
    /// * T2. the subscription produces a notification. (T2 happens before the user calls this method)
    /// * T3. user parses the subscription ID from (T1) and calls [`Self::register_subscription`].
    async fn register_subscription(
        &self,
        id: String,
    ) -> Result<mpsc::Receiver<Box<RawValue>>, LightClientError> {
        let (sender, receiver) = mpsc::channel(128);

        self.to_backend
            .send(BackendMessage::Subscription { id, sender })
            .await
            .map_err(|_| LightClientError::BackgroundClosed)?;

        Ok(receiver)
    }
}

/// The LightClient RPC offers a slightly different RPC methods than the
/// substrate based chains. This is because the light client only exposes
/// a small subset of the RPCs needed for basic functionality.
pub struct LightClient {
    // Note: Used for interior mutability as subxt's RpcClientT trait
    // passes the RPC client as immutable reference and the smoldot_light crate
    // needed a mutable reference to the smoldot_light::Client.
    inner: Arc<AsyncMutex<LightClientInner>>,
}

impl LightClient {
    /// Constructs a new [`LightClient`], providing the chain specification.
    ///
    /// The chain specification can be downloaded from a trusted network via
    /// the `sync_state_genSyncSpec` RPC method. This parameter expects the
    /// chain spec in text format (ie not in hex-encoded scale-encoded as RPC methods
    /// will provide).
    pub fn new<'a>(
        config: smoldot_light::AddChainConfig<'a, (), impl Iterator<Item = ChainId>>,
    ) -> Result<LightClient, Error> {
        tracing::trace!(target: LOG_TARGET, "Create light client");

        let mut client = smoldot_light::Client::new(AsyncStdTcpWebSocket::new(
            env!("CARGO_PKG_NAME").into(),
            env!("CARGO_PKG_VERSION").into(),
        ));

        let smoldot_light::AddChainSuccess {
            chain_id,
            json_rpc_responses,
        } = client
            .add_chain(config)
            .map_err(|err| LightClientError::AddChainError(err.to_string()))?;

        let (to_backend, backend) = mpsc::channel(128);

        // `json_rpc_responses` can only be `None` if we had passed `json_rpc: Disabled`.
        let rpc_responses = json_rpc_responses.expect("Light client RPC configured; qed");

        tokio::spawn(async move {
            let mut task = BackgroundTask::new();
            task.start_task(backend, rpc_responses).await;
        });

        Ok(LightClient {
            inner: Arc::new(AsyncMutex::new(LightClientInner {
                client,
                chain_id,
                to_backend,
                id_provider: 1,
            })),
        })
    }
}

impl RpcClientT for LightClient {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RpcFuture<'a, Box<RawValue>> {
        let inner = self.inner.clone();

        Box::pin(async move {
            let mut data = inner.lock().await;

            let params = match params {
                Some(params) => serde_json::to_string(&params).map_err(|_| {
                    RpcError::ClientError(Box::new(LightClientError::InvalidParams))
                })?,
                None => "[]".into(),
            };

            // Obtain an unique ID.
            let id = data.next_id();
            // Register the ID for responses.
            let rx = data
                .register_request(id.clone())
                .await
                .map_err(|err| RpcError::ClientError(Box::new(err)))?;

            // Submit the RPC request with the provided ID.
            // Note: The ID is necessary otherwise smoldot reaches an 'unreachable!()' macro.
            let request = format!(
                r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                id, method, params
            );
            tracing::trace!(target: LOG_TARGET, "Submit request {:?}", request);
            let chain_id = data.chain_id;

            data.client
                .json_rpc_request(request, chain_id)
                .map_err(|err| {
                    RpcError::ClientError(Box::new(LightClientError::Request(err.to_string())))
                })?;

            let response = rx
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?;

            tracing::trace!(target: LOG_TARGET, "RPC response {:?}", response);

            Ok(response)
        })
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        _unsub: &'a str,
    ) -> RpcFuture<'a, RpcSubscription> {
        let inner = self.inner.clone();

        Box::pin(async move {
            let mut data = inner.lock().await;

            tracing::trace!(
                target: LOG_TARGET,
                "Subscribe to {:?} with params {:?}",
                sub,
                params
            );

            let params = match params {
                Some(params) => serde_json::to_string(&params).map_err(|_| {
                    RpcError::ClientError(Box::new(LightClientError::InvalidParams))
                })?,
                None => "[]".into(),
            };

            // For subscriptions we need to make a plain RPC request to the subscription method.
            // The server will return as a result the subscription ID.
            // Then, the subscription ID is registered in the backend and will receive notifications from the chain.
            let id = data.next_id();
            let rx = data
                .register_request(id.clone())
                .await
                .map_err(|err| RpcError::ClientError(Box::new(err)))?;
            let request = format!(
                r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                id, sub, params
            );

            let chain_id = data.chain_id;
            data.client
                .json_rpc_request(request, chain_id)
                .map_err(|err| {
                    RpcError::ClientError(Box::new(LightClientError::Request(err.to_string())))
                })?;

            // The subscription ID.
            let sub_id = rx
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?;

            let sub_id_str = sub_id.get();
            // Try removing the first and last chars that are extra quotes.
            let sub_id_str = if sub_id_str.len() > 2 {
                &sub_id_str[1..sub_id_str.len() - 1]
            } else {
                sub_id_str
            };
            let sub_id = sub_id_str.to_string();
            tracing::trace!(target: LOG_TARGET, "Subscription ID {:?}", sub_id);

            let rx = data
                .register_subscription(sub_id.clone())
                .await
                .map_err(|err| RpcError::ClientError(Box::new(err)))?;
            let stream = ReceiverStream::new(rx);

            let rpc_substription_stream: Pin<
                Box<dyn Stream<Item = Result<Box<RawValue>, RpcError>> + Send + 'static>,
            > = Box::pin(stream.map(Ok));

            let rpc_subscription: RpcSubscription = RpcSubscription {
                stream: rpc_substription_stream,
                id: Some(sub_id),
            };

            Ok(rpc_subscription)
        })
    }
}
