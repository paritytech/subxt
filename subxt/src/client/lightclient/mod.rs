// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides support for light clients.

mod background;
mod builder;
mod rpc;

use derivative::Derivative;

use crate::{
    client::{OfflineClientT, OnlineClientT},
    config::Config,
    OnlineClient,
};

pub use builder::LightClientBuilder;

/// Light client error.
#[derive(Debug, thiserror::Error)]
pub enum LightClientError {
    /// Error encountered while adding the chain to the light-client.
    #[error("Failed to add the chain to the light client: {0}.")]
    AddChainError(String),
    /// The background task is closed.
    #[error("Failed to communicate with the background task.")]
    BackgroundClosed,
    /// Invalid RPC parameters cannot be serialized as JSON string.
    #[error("RPC parameters cannot be serialized as JSON string.")]
    InvalidParams,
    /// Error originated while trying to submit a RPC request.
    #[error("RPC request cannot be sent: {0}.")]
    Request(String),
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

impl<T: Config> OnlineClientT<T> for LightClient<T> {
    fn rpc(&self) -> &crate::rpc::Rpc<T> {
        self.0.rpc()
    }
}

impl<T: Config> OfflineClientT<T> for LightClient<T> {
    fn metadata(&self) -> crate::Metadata {
        self.0.metadata()
    }

    fn genesis_hash(&self) -> <T as Config>::Hash {
        self.0.genesis_hash()
    }

    fn runtime_version(&self) -> crate::rpc::types::RuntimeVersion {
        self.0.runtime_version()
    }
}
