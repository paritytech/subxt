// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides two clients that can be used to work with
//! transactions, storage and events. The [`OfflineClient`] works
//! entirely offline and can be passed to any function that doesn't
//! require network access. The [`OnlineClient`] requires network
//! access.

mod offline_client;
mod online_client;

crate::macros::cfg_unstable_light_client! {
    mod light_client;

    pub use light_client::{
        LightClient, LightClientBuilder, LightClientError, RawLightClient, RawLightClientBuilder,
    };
}

use derivative::Derivative;
pub use offline_client::{OfflineClient, OfflineClientT};
pub use online_client::{
    ClientRuntimeUpdater, OnlineClient, OnlineClientT, RuntimeUpdaterStream, Update, UpgradeError,
};

use crate::{backend::RuntimeVersion, Config, Metadata};

/// The inner values, any client should contain.
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct BaseClient<T: Config> {
    genesis_hash: T::Hash,
    runtime_version: RuntimeVersion,
    metadata: Metadata,
}

impl<T: Config> BaseClient<T> {
    /// Create a new [`BaseClient`].
    pub fn new(genesis_hash: T::Hash, runtime_version: RuntimeVersion, metadata: Metadata) -> Self {
        Self {
            genesis_hash,
            runtime_version,
            metadata,
        }
    }

    /// Return the genesis hash.
    pub fn genesis_hash(&self) -> T::Hash {
        self.genesis_hash
    }

    /// Return the runtime version.
    pub fn runtime_version(&self) -> RuntimeVersion {
        self.runtime_version.clone()
    }

    /// Return the [`Metadata`] used in this client.
    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
}
