// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides support for light clients.

mod builder;
mod rpc;

use crate::{
    backend::rpc::RpcClient,
    blocks::BlocksClient,
    client::{OfflineClientT, OnlineClientT},
    config::Config,
    constants::ConstantsClient,
    custom_values::CustomValuesClient,
    events::EventsClient,
    runtime_api::RuntimeApiClient,
    storage::StorageClient,
    tx::TxClient,
    OnlineClient,
};
pub use builder::{LightClientBuilder, RawLightClientBuilder};
use derivative::Derivative;
use subxt_lightclient::LightClientRpcError;

// Re-export smoldot related objects.
pub use subxt_lightclient::smoldot;

/// Light client error.
#[derive(Debug, thiserror::Error)]
pub enum LightClientError {
    /// Error originated from the low-level RPC layer.
    #[error("Rpc error: {0}")]
    Rpc(LightClientRpcError),
    /// The background task is closed.
    #[error("Failed to communicate with the background task.")]
    BackgroundClosed,
    /// Invalid RPC parameters cannot be serialized as JSON string.
    #[error("RPC parameters cannot be serialized as JSON string.")]
    InvalidParams,
    /// The provided URL scheme is invalid.
    ///
    /// Supported versions: WS, WSS.
    #[error("The provided URL scheme is invalid.")]
    InvalidScheme,
    /// The provided URL is invalid.
    #[error("The provided URL scheme is invalid.")]
    InvalidUrl,
    /// The provided chain spec is invalid.
    #[error("The provided chain spec is not a valid JSON object.")]
    InvalidChainSpec,
    /// Handshake error while connecting to a node.
    #[error("WS handshake failed.")]
    Handshake,
}

/// The raw light-client RPC implementation that is used to connect with the chain.
#[derive(Clone)]
pub struct RawLightClient {
    raw_rpc: rpc::RawLightClientRpc,
}

impl RawLightClient {
    /// Construct a [`RawLightClient`] using its builder interface.
    ///
    /// The raw builder is utilized for constructing light-clients from a low
    /// level smoldot client.
    ///
    /// This is especially useful when you want to gain access to the smoldot client.
    /// For example, you may want to connect to multiple chains and/or parachains while reusing the
    /// same smoldot client under the hood. Or you may want to configure different values for
    /// smoldot internal buffers, number of subscriptions and relay chains.
    ///
    /// # Note
    ///
    /// If you are unsure, please use [`LightClient::builder`] instead.
    pub fn builder() -> RawLightClientBuilder {
        RawLightClientBuilder::default()
    }

    /// Target a different chain identified by the provided chain ID for requests.
    ///
    /// The provided chain ID is provided by the `smoldot_light::Client::add_chain` and it must
    /// match one of the `smoldot_light::JsonRpcResponses` provided in [`RawLightClientBuilder::add_chain`].
    ///
    /// # Note
    ///
    /// This uses the same underlying instance spawned by the builder.
    pub async fn for_chain<TChainConfig: Config>(
        &self,
        chain_id: smoldot::ChainId,
    ) -> Result<LightClient<TChainConfig>, crate::Error> {
        let raw_rpc = self.raw_rpc.for_chain(chain_id);
        let rpc_client = RpcClient::new(raw_rpc.clone());
        let client = OnlineClient::<TChainConfig>::from_rpc_client(rpc_client).await?;

        Ok(LightClient { client, chain_id })
    }
}

/// The light-client RPC implementation that is used to connect with the chain.
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct LightClient<T: Config> {
    client: OnlineClient<T>,
    chain_id: smoldot::ChainId,
}

impl<T: Config> LightClient<T> {
    /// Construct a [`LightClient`] using its builder interface.
    pub fn builder() -> LightClientBuilder<T> {
        LightClientBuilder::new()
    }

    // We add the below impls so that we don't need to
    // think about importing the OnlineClientT/OfflineClientT
    // traits to use these things:

    /// Return the [`crate::Metadata`] used in this client.
    fn metadata(&self) -> crate::Metadata {
        self.client.metadata()
    }

    /// Return the genesis hash.
    fn genesis_hash(&self) -> <T as Config>::Hash {
        self.client.genesis_hash()
    }

    /// Return the runtime version.
    fn runtime_version(&self) -> crate::client::RuntimeVersion {
        self.client.runtime_version()
    }

    /// Return the inner client metadata.
    fn client_metadata(&self) -> crate::client::ClientMetadata<T> {
        self.client.client_metadata()
    }

    /// Work with transactions.
    pub fn tx(&self) -> TxClient<T, Self> {
        <Self as OfflineClientT<T>>::tx(self)
    }

    /// Work with events.
    pub fn events(&self) -> EventsClient<T, Self> {
        <Self as OfflineClientT<T>>::events(self)
    }

    /// Work with storage.
    pub fn storage(&self) -> StorageClient<T, Self> {
        <Self as OfflineClientT<T>>::storage(self)
    }

    /// Access constants.
    pub fn constants(&self) -> ConstantsClient<T, Self> {
        <Self as OfflineClientT<T>>::constants(self)
    }

    /// Access custom types.
    pub fn custom_values(&self) -> CustomValuesClient<T, Self> {
        <Self as OfflineClientT<T>>::custom_values(self)
    }

    /// Work with blocks.
    pub fn blocks(&self) -> BlocksClient<T, Self> {
        <Self as OfflineClientT<T>>::blocks(self)
    }

    /// Work with runtime API.
    pub fn runtime_api(&self) -> RuntimeApiClient<T, Self> {
        <Self as OfflineClientT<T>>::runtime_api(self)
    }

    /// Returns the chain ID of the current light-client.
    pub fn chain_id(&self) -> smoldot::ChainId {
        self.chain_id
    }
}

impl<T: Config> OnlineClientT<T> for LightClient<T> {
    fn backend(&self) -> &dyn crate::backend::Backend<T> {
        self.client.backend()
    }
}

impl<T: Config> OfflineClientT<T> for LightClient<T> {
    fn metadata(&self) -> crate::Metadata {
        self.metadata()
    }

    fn genesis_hash(&self) -> <T as Config>::Hash {
        self.genesis_hash()
    }

    fn runtime_version(&self) -> crate::client::RuntimeVersion {
        self.runtime_version()
    }

    fn client_metadata(&self) -> crate::client::ClientMetadata<T> {
        self.client_metadata()
    }
}
