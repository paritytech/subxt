// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides support for light clients.

mod builder;
mod rpc;

use crate::{
    blocks::BlocksClient,
    client::{OfflineClientT, OnlineClientT},
    config::Config,
    constants::ConstantsClient,
    events::EventsClient,
    runtime_api::RuntimeApiClient,
    storage::StorageClient,
    tx::TxClient,
    OnlineClient,
};
pub use builder::LightClientBuilder;
use derivative::Derivative;
use subxt_lightclient::LightClientRpcError;

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

/// The light-client RPC implementation that is used to connect with the chain.
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct LightClient<T: Config>(OnlineClient<T>);

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
        self.0.metadata()
    }

    /// Return the genesis hash.
    fn genesis_hash(&self) -> <T as Config>::Hash {
        self.0.genesis_hash()
    }

    /// Return the runtime version.
    fn runtime_version(&self) -> crate::rpc::types::RuntimeVersion {
        self.0.runtime_version()
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

    /// Work with blocks.
    pub fn blocks(&self) -> BlocksClient<T, Self> {
        <Self as OfflineClientT<T>>::blocks(self)
    }

    /// Work with runtime API.
    pub fn runtime_api(&self) -> RuntimeApiClient<T, Self> {
        <Self as OfflineClientT<T>>::runtime_api(self)
    }
}

impl<T: Config> OnlineClientT<T> for LightClient<T> {
    fn rpc(&self) -> &crate::rpc::Rpc<T> {
        self.0.rpc()
    }
}

impl<T: Config> OfflineClientT<T> for LightClient<T> {
    fn metadata(&self) -> crate::Metadata {
        self.metadata()
    }

    fn genesis_hash(&self) -> <T as Config>::Hash {
        self.genesis_hash()
    }

    fn runtime_version(&self) -> crate::rpc::types::RuntimeVersion {
        self.runtime_version()
    }
}
