// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A wrapper around [`smoldot_light`] which provides an light client capable of connecting
//! to Substrate based chains.

#![deny(missing_docs)]

mod platform;
mod shared_client;
// mod receiver;
mod background;
mod chain_config;
mod rpc;

use background::{BackgroundTask, BackgroundTaskHandle};
use futures::Stream;
use platform::DefaultPlatform;
use serde_json::value::RawValue;
use shared_client::SharedClient;
use std::future::Future;
use tokio::sync::mpsc;

pub use chain_config::{ChainConfig, ChainConfigError};

/// Things that can go wrong when constructing the [`LightClient`].
#[derive(Debug, thiserror::Error)]
pub enum LightClientError {
    /// Error encountered while adding the chain to the light-client.
    #[error("Failed to add the chain to the light client: {0}.")]
    AddChainError(String),
}

/// Things that can go wrong calling methods of [`LightClientRpc`].
#[derive(Debug, thiserror::Error)]
pub enum LightClientRpcError {
    /// Error response from the JSON-RPC server.
    #[error("{0}")]
    JsonRpcError(JsonRpcError),
    /// Smoldot could not handle the RPC call.
    #[error("Smoldot could not handle the RPC call: {0}.")]
    SmoldotError(String),
    /// Background task dropped.
    #[error("The background task was dropped.")]
    BackgroundTaskDropped,
}

/// An error response from the JSON-RPC server (ie smoldot) in response to
/// a method call or as a subscription notification.
#[derive(Debug, thiserror::Error)]
#[error("RPC Error: {0}.")]
pub struct JsonRpcError(Box<RawValue>);

/// This represents a single light client connection to the network. Instantiate
/// it with [`LightClient::relay_chain()`] to communicate with a relay chain, and
/// then call [`LightClient::parachain()`] to establish connections to parachains.
#[derive(Clone)]
pub struct LightClient {
    client: SharedClient<DefaultPlatform>,
    relay_chain_id: smoldot_light::ChainId,
}

impl LightClient {
    /// Given a chain spec, establish a connection to a relay chain. Any subsequent calls to
    /// [`LightClient::parachain()`] will set this as the relay chain.
    ///
    /// # Panics
    ///
    /// The panic behaviour depends on the feature flag being used:
    ///
    /// ## Native
    ///
    /// Panics when called outside of a `tokio` runtime context.
    ///
    /// ## Web
    ///
    /// If smoldot panics, then the promise created will be leaked. For more details, see
    /// https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/fn.future_to_promise.html.
    pub fn relay_chain<'a>(
        chain_config: impl Into<ChainConfig<'a>>,
    ) -> Result<(Self, LightClientRpc), LightClientError> {
        let mut client = smoldot_light::Client::new(platform::build_platform());
        let chain_config = chain_config.into();
        let chain_spec = chain_config.as_chain_spec();

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
            .map_err(|err| LightClientError::AddChainError(err.to_string()))?;

        let relay_chain_id = added_chain.chain_id;
        let rpc_responses = added_chain
            .json_rpc_responses
            .expect("Light client RPC configured; qed");
        let shared_client: SharedClient<_> = client.into();

        let light_client_rpc =
            LightClientRpc::new_raw(shared_client.clone(), relay_chain_id, rpc_responses);
        let light_client = Self {
            client: shared_client,
            relay_chain_id,
        };

        Ok((light_client, light_client_rpc))
    }

    /// Given a chain spec, establish a connection to a parachain.
    ///
    /// # Panics
    ///
    /// The panic behaviour depends on the feature flag being used:
    ///
    /// ## Native
    ///
    /// Panics when called outside of a `tokio` runtime context.
    ///
    /// ## Web
    ///
    /// If smoldot panics, then the promise created will be leaked. For more details, see
    /// https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/fn.future_to_promise.html.
    pub fn parachain<'a>(
        &self,
        chain_config: impl Into<ChainConfig<'a>>,
    ) -> Result<LightClientRpc, LightClientError> {
        let chain_config = chain_config.into();
        let chain_spec = chain_config.as_chain_spec();

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

        let added_chain = self
            .client
            .add_chain(config)
            .map_err(|err| LightClientError::AddChainError(err.to_string()))?;

        let chain_id = added_chain.chain_id;
        let rpc_responses = added_chain
            .json_rpc_responses
            .expect("Light client RPC configured; qed");

        Ok(LightClientRpc::new_raw(
            self.client.clone(),
            chain_id,
            rpc_responses,
        ))
    }
}

/// This represents a single RPC connection to a specific chain, and is constructed by calling
/// one of the methods on [`LightClient`]. Using this, you can make RPC requests to the chain.
#[derive(Clone, Debug)]
pub struct LightClientRpc {
    handle: BackgroundTaskHandle,
}

impl LightClientRpc {
    // Dev note: this would provide a "low leveL" interface if one is needed.
    // Do we actually need to provide this, or can we entirely hide Smoldot?
    pub(crate) fn new_raw<TPlat, TChain>(
        client: impl Into<SharedClient<TPlat, TChain>>,
        chain_id: smoldot_light::ChainId,
        rpc_responses: smoldot_light::JsonRpcResponses,
    ) -> Self
    where
        TPlat: smoldot_light::platform::PlatformRef + Send + 'static,
        TChain: Send + 'static,
    {
        let (background_task, background_handle) =
            BackgroundTask::new(client.into(), chain_id, rpc_responses);

        // For now we spawn the background task internally, but later we can expose
        // methods to give this back to the user so that they can exert backpressure.
        spawn(async move { background_task.run().await });

        LightClientRpc {
            handle: background_handle,
        }
    }

    /// Make an RPC request to a chain, getting back a result.
    pub async fn request(
        &self,
        method: String,
        params: Option<Box<RawValue>>,
    ) -> Result<Box<RawValue>, LightClientRpcError> {
        self.handle.request(method, params).await
    }

    /// Subscribe to some RPC method, getting back a stream of notifications.
    pub async fn subscribe(
        &self,
        method: String,
        params: Option<Box<RawValue>>,
        unsub: String,
    ) -> Result<LightClientRpcSubscription, LightClientRpcError> {
        let (id, notifications) = self.handle.subscribe(method, params, unsub).await?;
        Ok(LightClientRpcSubscription { id, notifications })
    }
}

/// A stream of notifications handed back when [`LightClientRpc::subscribe`] is called.
pub struct LightClientRpcSubscription {
    notifications: mpsc::UnboundedReceiver<Result<Box<RawValue>, JsonRpcError>>,
    id: String,
}

impl LightClientRpcSubscription {
    /// Return the subscription ID
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Stream for LightClientRpcSubscription {
    type Item = Result<Box<RawValue>, JsonRpcError>;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.notifications.poll_recv(cx)
    }
}

/// A quick helper to spawn a task that works for WASM.
fn spawn<F: Future + Send + 'static>(future: F) {
    #[cfg(feature = "native")]
    tokio::spawn(async move {
        future.await;
    });
    #[cfg(feature = "web")]
    wasm_bindgen_futures::spawn_local(async move {
        future.await;
    });
}
