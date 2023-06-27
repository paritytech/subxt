// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    background::{BackgroundTask, FromSubxt, MethodResponse},
    LightClientRpcError,
};
use serde_json::value::RawValue;
use smoldot_light::ChainId;
use tokio::sync::{mpsc, mpsc::error::SendError, oneshot};

use super::platform::default::SubxtPlatform as Platform;

pub const LOG_TARGET: &str = "light-client";

/// The light-client RPC implementation that is used to connect with the chain.
#[derive(Clone)]
pub struct LightClientRpc {
    /// Communicate with the backend task that multiplexes the responses
    /// back to the frontend.
    to_backend: mpsc::UnboundedSender<FromSubxt>,
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
        config: smoldot_light::AddChainConfig<'_, (), impl Iterator<Item = ChainId>>,
    ) -> Result<LightClientRpc, LightClientRpcError> {
        tracing::trace!(target: LOG_TARGET, "Create light client");

        let mut client = smoldot_light::Client::new(Platform::new());

        let smoldot_light::AddChainSuccess {
            chain_id,
            json_rpc_responses,
        } = client
            .add_chain(config)
            .map_err(|err| LightClientRpcError::AddChainError(err.to_string()))?;

        let (to_backend, backend) = mpsc::unbounded_channel();

        // `json_rpc_responses` can only be `None` if we had passed `json_rpc: Disabled`.
        let rpc_responses = json_rpc_responses.expect("Light client RPC configured; qed");

        let future = async move {
            let mut task = BackgroundTask::new(client, chain_id);
            task.start_task(backend, rpc_responses).await;
        };

        #[cfg(feature = "native")]
        tokio::spawn(future);
        #[cfg(feature = "web")]
        wasm_bindgen_futures::spawn_local(future);

        Ok(LightClientRpc { to_backend })
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
        })?;

        Ok((sub_id_rx, receiver))
    }
}
