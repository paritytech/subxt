// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.
use super::{
    background::{BackendMessage, BackgroundTask, MethodResponse},
    LightClientError,
};
use crate::{
    error::{Error, RpcError},
    rpc::{RpcClientT, RpcFuture, RpcSubscription},
};
use futures::{stream::StreamExt, Stream};
use serde_json::value::RawValue;
use smoldot_light::{platform::async_std::AsyncStdTcpWebSocket as Platform, ChainId};
use std::pin::Pin;
use tokio::sync::{mpsc, mpsc::error::SendError, oneshot};
use tokio_stream::wrappers::ReceiverStream;

pub const LOG_TARGET: &str = "light-client";

/// The light-client implementation that is used to connect with the chain.
#[derive(Clone)]
pub struct LightClient {
    /// Communicate with the backend task that multiplexes the responses
    /// back to the frontend.
    to_backend: mpsc::Sender<BackendMessage>,
}

impl LightClient {
    /// Constructs a new [`LightClient`], providing the chain specification.
    ///
    /// The chain specification can be downloaded from a trusted network via
    /// the `sync_state_genSyncSpec` RPC method. This parameter expects the
    /// chain spec in text format (ie not in hex-encoded scale-encoded as RPC methods
    /// will provide).
    ///
    /// ## Panics
    ///
    /// Panics if being called outside of `tokio` runtime context.
    pub fn new(
        config: smoldot_light::AddChainConfig<'_, (), impl Iterator<Item = ChainId>>,
    ) -> Result<LightClient, Error> {
        tracing::trace!(target: LOG_TARGET, "Create light client");

        let mut client = smoldot_light::Client::new(Platform::new(
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
            let mut task = BackgroundTask::new(client, chain_id);
            task.start_task(backend, rpc_responses).await;
        });

        Ok(LightClient { to_backend })
    }

    /// Submits an RPC method request to the light-client.
    ///
    /// This method sends a request to the light-client to execute an RPC method with the provided parameters.
    /// The parameters are parsed into a valid JSON object in the background.
    async fn method_request(
        &self,
        method: String,
        params: String,
    ) -> Result<oneshot::Receiver<MethodResponse>, SendError<BackendMessage>> {
        let (sender, receiver) = oneshot::channel();

        self.to_backend
            .send(BackendMessage::Request {
                method,
                params,
                sender,
            })
            .await?;

        Ok(receiver)
    }

    /// Makes an RPC subscription call to the light-client.
    ///
    /// This method sends a request to the light-client to establish an RPC subscription with the provided parameters.
    /// The parameters are parsed into a valid JSON object in the background.
    async fn subscription_request(
        &self,
        method: String,
        params: String,
    ) -> Result<
        (
            oneshot::Receiver<MethodResponse>,
            mpsc::Receiver<Box<RawValue>>,
        ),
        SendError<BackendMessage>,
    > {
        let (sub_id, sub_id_rx) = oneshot::channel();
        let (sender, receiver) = mpsc::channel(128);

        self.to_backend
            .send(BackendMessage::Subscription {
                method,
                params,
                sub_id,
                sender,
            })
            .await?;

        Ok((sub_id_rx, receiver))
    }
}

impl RpcClientT for LightClient {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RpcFuture<'a, Box<RawValue>> {
        let client = self.clone();

        Box::pin(async move {
            let params = match params {
                Some(params) => serde_json::to_string(&params).map_err(|_| {
                    RpcError::ClientError(Box::new(LightClientError::InvalidParams))
                })?,
                None => "[]".into(),
            };

            // Fails if the background is closed.
            let rx = client
                .method_request(method.to_string(), params)
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?;

            // Fails if the background is closed.
            let response = rx
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?;

            tracing::trace!(target: LOG_TARGET, "RPC response {:?}", response);

            response.map_err(|err| RpcError::ClientError(Box::new(err)))
        })
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        _unsub: &'a str,
    ) -> RpcFuture<'a, RpcSubscription> {
        let client = self.clone();

        Box::pin(async move {
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

            // Fails if the background is closed.
            let (sub_id, notif) = client
                .subscription_request(sub.to_string(), params)
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?;

            // Fails if the background is closed.
            let result = sub_id
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?
                .map_err(|err| {
                    RpcError::ClientError(Box::new(LightClientError::Request(err.to_string())))
                })?;

            let mut sub_id = result.to_string();
            sub_id.retain(|ch| ch.is_ascii_digit());
            tracing::warn!(target: LOG_TARGET, "FRONT: SUB ID parsed: {}", sub_id);

            let stream = ReceiverStream::new(notif);

            let rpc_substription_stream: Pin<
                Box<dyn Stream<Item = Result<Box<RawValue>, RpcError>> + Send + 'static>,
            > = Box::pin(stream.map(Ok));

            let rpc_subscription: RpcSubscription = RpcSubscription {
                stream: rpc_substription_stream,
                id: Some(sub_id.to_string()),
            };

            Ok(rpc_subscription)
        })
    }
}
