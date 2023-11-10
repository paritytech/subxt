// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::LightClientError;
use crate::{
    backend::rpc::{RawRpcFuture, RawRpcSubscription, RpcClientT},
    error::{Error, RpcError},
};
use futures::StreamExt;
use serde_json::value::RawValue;
use subxt_lightclient::{AddChainConfig, ChainId, LightClientRpcError};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub const LOG_TARGET: &str = "subxt-rpc-light-client";

/// The raw light-client RPC implementation that is used to connect with the chain.
#[derive(Clone)]
pub struct RawLightClientRpc(subxt_lightclient::RawLightClientRpc);

impl RawLightClientRpc {
    /// Constructs a new [`RawLightClientRpc`] from a low level [`subxt_lightclient::RawLightClientRpc`].
    pub fn from_inner(client: subxt_lightclient::RawLightClientRpc) -> RawLightClientRpc {
        RawLightClientRpc(client)
    }

    /// Constructs a new [`LightClientRpc`] that communicates with the provided chain.
    pub fn for_chain(&self, chain_id: subxt_lightclient::ChainId) -> LightClientRpc {
        LightClientRpc(self.0.for_chain(chain_id))
    }
}

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
    /// The panic behaviour depends on the feature flag being used:
    ///
    /// ### Native
    ///
    /// Panics when called outside of a `tokio` runtime context.
    ///
    /// ### Web
    ///
    /// If smoldot panics, then the promise created will be leaked. For more details, see
    /// https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/fn.future_to_promise.html.
    pub fn new(
        config: AddChainConfig<'_, (), impl Iterator<Item = ChainId>>,
    ) -> Result<LightClientRpc, Error> {
        let rpc = subxt_lightclient::LightClientRpc::new(config)
            .map_err(|err| LightClientError::Rpc(err))?;

        Ok(LightClientRpc(rpc))
    }

    /// Returns the chain ID of the current light-client.
    pub fn chain_id(&self) -> subxt_lightclient::ChainId {
        self.0.chain_id()
    }
}

impl RpcClientT for LightClientRpc {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        let client = self.clone();
        let chain_id = self.chain_id();

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

            tracing::trace!(target: LOG_TARGET, "RPC response={:?} chain={:?}", response, chain_id);

            response.map_err(|err| RpcError::ClientError(Box::new(err)))
        })
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        _unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        let client = self.clone();
        let chain_id = self.chain_id();

        Box::pin(async move {
            tracing::trace!(
                target: LOG_TARGET,
                "Subscribe to {:?} with params {:?} chain={:?}",
                sub,
                params,
                chain_id,
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
            tracing::trace!(target: LOG_TARGET, "Received subscription={} chain={:?}", sub_id, chain_id);

            let stream = UnboundedReceiverStream::new(notif);

            let rpc_subscription = RawRpcSubscription {
                stream: Box::pin(stream.map(Ok)),
                id: Some(sub_id),
            };

            Ok(rpc_subscription)
        })
    }
}
