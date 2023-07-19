// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::LightClientError;
use crate::{
    error::{Error, RpcError},
    rpc::{RpcClientT, RpcFuture, RpcSubscription},
};
use futures::{Stream, StreamExt};
use serde_json::value::RawValue;
use std::pin::Pin;
use subxt_lightclient::{AddChainConfig, ChainId, LightClientRpcError};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub const LOG_TARGET: &str = "light-client";

/// The light-client RPC implementation that is used to connect with the chain.
#[derive(Clone)]
pub struct LightClientRpc(subxt_lightclient::LightClientRpc);

impl LightClientRpc {
    /// Constructs a new [`LightClientRpc`], providing the chain specification.
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
        config: AddChainConfig<'_, (), impl Iterator<Item = ChainId>>,
    ) -> Result<LightClientRpc, Error> {
        let rpc = subxt_lightclient::LightClientRpc::new(config)
            .map_err(|err| LightClientError::Rpc(err))?;

        Ok(LightClientRpc(rpc))
    }
}

impl RpcClientT for LightClientRpc {
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
                .0
                .method_request(method.to_string(), params)
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
                .0
                .subscription_request(sub.to_string(), params)
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?;

            // Fails if the background is closed.
            let result = sub_id
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?
                .map_err(|err| {
                    RpcError::ClientError(Box::new(LightClientError::Rpc(
                        LightClientRpcError::Request(err.to_string()),
                    )))
                })?;

            let sub_id = result
                .get()
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string();
            tracing::trace!(target: LOG_TARGET, "Received subscription ID: {}", sub_id);

            let stream = UnboundedReceiverStream::new(notif);

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
