// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::iter;

use super::{
    background::{BackgroundTask, FromSubxt, MethodResponse},
    LightClientRpcError,
};
use serde_json::value::RawValue;
use tokio::sync::{mpsc, mpsc::error::SendError, oneshot};

use super::platform::build_platform;

pub const LOG_TARGET: &str = "subxt-light-client";

/// A raw light-client RPC implementation that can connect to multiple chains.
#[derive(Clone)]
pub struct RawLightClientRpc {
    /// Communicate with the backend task that multiplexes the responses
    /// back to the frontend.
    to_backend: mpsc::UnboundedSender<FromSubxt>,
}

impl RawLightClientRpc {
    /// Construct a [`LightClientRpc`] that can communicated with the provided chain.
    ///
    /// The provided chain ID is provided by the `smoldot_light::Client::add_chain` and it must
    /// match one of the `smoldot_light::JsonRpcResponses` provided in [`Self::new_from_client`].
    ///
    /// # Note
    ///
    /// This uses the same underlying instance created by [`LightClientRpc::new_from_client`].
    pub fn for_chain(&self, chain_id: smoldot_light::ChainId) -> LightClientRpc {
        LightClientRpc {
            to_backend: self.to_backend.clone(),
            chain_id,
        }
    }
}

/// The light-client RPC implementation that is used to connect with the chain.
#[derive(Clone)]
pub struct LightClientRpc {
    /// Communicate with the backend task that multiplexes the responses
    /// back to the frontend.
    to_backend: mpsc::UnboundedSender<FromSubxt>,
    /// The chain ID to target for requests.
    chain_id: smoldot_light::ChainId,
}

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
        config: smoldot_light::AddChainConfig<'_, (), impl Iterator<Item = smoldot_light::ChainId>>,
    ) -> Result<LightClientRpc, LightClientRpcError> {
        tracing::trace!(target: LOG_TARGET, "Create light client");

        let mut client = smoldot_light::Client::new(build_platform());

        let smoldot_light::AddChainSuccess {
            chain_id,
            json_rpc_responses,
        } = client
            .add_chain(config)
            .map_err(|err| LightClientRpcError::AddChainError(err.to_string()))?;

        let rpc_responses = json_rpc_responses.expect("Light client RPC configured; qed");

        let raw_client = Self::new_from_client(
            client,
            iter::once(AddedChain {
                chain_id,
                rpc_responses,
            }),
        );
        Ok(raw_client.for_chain(chain_id))
    }

    /// Constructs a new [`RawLightClientRpc`] from the raw smoldot client.
    ///
    /// Receives a list of RPC objects as a result of calling `smoldot_light::Client::add_chain`.
    /// This [`RawLightClientRpc`] can target different chains using [`RawLightClientRpc::for_chain`] method.
    ///
    /// ## Panics
    ///
    /// Panics if being called outside of `tokio` runtime context.
    pub fn new_from_client<TPlat>(
        client: smoldot_light::Client<TPlat>,
        chains: impl Iterator<Item = AddedChain>,
    ) -> RawLightClientRpc
    where
        TPlat: smoldot_light::platform::PlatformRef + Clone,
    {
        let (to_backend, backend) = mpsc::unbounded_channel();
        let chains = chains.collect();

        let future = async move {
            let mut task = BackgroundTask::new(client);
            task.start_task(backend, chains).await;
        };

        #[cfg(feature = "native")]
        tokio::spawn(future);
        #[cfg(feature = "web")]
        wasm_bindgen_futures::spawn_local(future);

        RawLightClientRpc { to_backend }
    }

    /// Returns the chain ID of the current light-client.
    pub fn chain_id(&self) -> smoldot_light::ChainId {
        self.chain_id
    }

    /// Submits an RPC method request to the light-client.
    ///
    /// This method sends a request to the light-client to execute an RPC method with the provided parameters.
    /// The parameters are parsed into a valid JSON object in the background.
    pub fn method_request(
        &self,
        method: String,
        params: String,
    ) -> Result<oneshot::Receiver<MethodResponse>, SendError<FromSubxt>> {
        let (sender, receiver) = oneshot::channel();

        self.to_backend.send(FromSubxt::Request {
            method,
            params,
            sender,
            chain_id: self.chain_id,
        })?;

        Ok(receiver)
    }

    /// Makes an RPC subscription call to the light-client.
    ///
    /// This method sends a request to the light-client to establish an RPC subscription with the provided parameters.
    /// The parameters are parsed into a valid JSON object in the background.
    pub fn subscription_request(
        &self,
        method: String,
        params: String,
    ) -> Result<
        (
            oneshot::Receiver<MethodResponse>,
            mpsc::UnboundedReceiver<Box<RawValue>>,
        ),
        SendError<FromSubxt>,
    > {
        let (sub_id, sub_id_rx) = oneshot::channel();
        let (sender, receiver) = mpsc::unbounded_channel();

        self.to_backend.send(FromSubxt::Subscription {
            method,
            params,
            sub_id,
            sender,
            chain_id: self.chain_id,
        })?;

        Ok((sub_id_rx, receiver))
    }
}

/// The added chain of the light-client.
pub struct AddedChain {
    /// The id of the chain.
    pub chain_id: smoldot_light::ChainId,
    /// Producer of RPC responses for the chain.
    pub rpc_responses: smoldot_light::JsonRpcResponses,
}
