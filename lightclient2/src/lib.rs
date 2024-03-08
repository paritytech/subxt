#![allow(missing_docs)]

mod platform;
mod shared_client;
// mod receiver;
mod background;
mod rpc;

use std::sync::{ Arc, Mutex };
use std::future::Future;
use serde_json::value::RawValue;
use shared_client::SharedClient;
use tokio::sync::mpsc;
use platform::DefaultPlatform;
use background::{ BackgroundTask, BackgroundTaskHandle };

struct Chain {
    id: smoldot_light::ChainId,
    responses: smoldot_light::JsonRpcResponses
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error encountered while adding the chain to the light-client.
    #[error("Failed to add the chain to the light client: {0}.")]
    AddChainError(String),
    /// Error originated while trying to make an RPC request to Smoldot.
    #[error("Error making RPC request: {0}.")]
    RpcRequest(String),
    /// The background task went away.
    #[error("Smoldot unexpectedly closed the connection.")]
    SmoldotClosedUnexpectedly
}

struct LightClient<TPlat: smoldot_light::platform::PlatformRef> {
    client: SharedClient<TPlat>,
    relay_chain_id: smoldot_light::ChainId,
}

impl LightClient<DefaultPlatform> {
    pub fn relay_chain(chain_spec: &str) -> Result<(Self, LightClientRpc), Error> {
        let mut client = smoldot_light::Client::new(platform::build_platform());

        let config = smoldot_light::AddChainConfig {
            specification: chain_spec,
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: u32::MAX.try_into().unwrap(),
                max_subscriptions: u32::MAX,
            },
            database_content: "",
            potential_relay_chains: std::iter::empty(),
            user_data: (),
        };

        let added_chain = client
            .add_chain(config)
            .map_err(|err| Error::AddChainError(err.to_string()))?;

        let relay_chain_id = added_chain.chain_id;
        let rpc_responses = added_chain.json_rpc_responses.expect("Light client RPC configured; qed");
        let shared_client: SharedClient<_> = client.into();

        let (background_task, background_handle) = BackgroundTask::new(
            shared_client,
            relay_chain_id,
            rpc_responses
        );

        // For now we spawn the background task internally, but later we can expose
        // methods to give this back to the user so that they can control backpressure.
        spawn(async move { background_task.run().await });

        let light_client = Self {
            client: shared_client.clone(),
            relay_chain_id
        };

        let light_client_rpc = LightClientRpc {
            handle: background_handle
        };

        Ok((light_client, light_client_rpc))
    }

    pub fn parachain(&self, chain_spec: &str) -> Result<LightClientRpc, Error> {
        let config = smoldot_light::AddChainConfig {
            specification: chain_spec,
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: u32::MAX.try_into().unwrap(),
                max_subscriptions: u32::MAX,
            },
            database_content: "",
            potential_relay_chains: std::iter::once(self.relay_chain_id),
            user_data: (),
        };

        let added_chain = self.client
            .add_chain(config)
            .map_err(|err| Error::AddChainError(err.to_string()))?;

        let chain_id = added_chain.chain_id;
        let rpc_responses = added_chain.json_rpc_responses.expect("Light client RPC configured; qed");

        let (background_task, background_handle) = BackgroundTask::new(
            self.client.clone(),
            chain_id,
            rpc_responses
        );

        // For now we spawn the background task internally, but later we can expose
        // methods to give this back to the user so that they can control backpressure.
        spawn(async move { background_task.run().await });

        Ok(LightClientRpc {
            handle: background_handle
        })
    }
}

struct LightClientRpc {
    handle: BackgroundTaskHandle
}

impl LightClientRpc {
    pub fn request_raw(&self, method: String, params: Option<Box<RawValue>>) -> impl Future<Output = Result<Box<RawValue>, Error>> {
        self.handle.request(method, params)
    }
    pub fn subscribe_raw(&self, method: String, params: Option<Box<RawValue>>, unsub: String) -> impl Future<Output = Result<Subscription, RpcError>> {
        self.handle.subscribe(method, params, unsub)
    }
}

pub struct Subscription {
    inner: mpsc::Receiver<Box<RawValue>>
}

pub enum RpcError {

}

fn spawn<F: Future + Send + 'static>(future: F) {
    #[cfg(feature = "native")]
    tokio::spawn(async move { future.await; });
    #[cfg(feature = "web")]
    wasm_bindgen_futures::spawn_local(async move { future.await; });
}