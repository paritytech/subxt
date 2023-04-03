// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    blocks::BlocksClient, constants::ConstantsClient, events::EventsClient,
    rpc::types::RuntimeVersion, runtime_api::RuntimeApiClient, storage::StorageClient,
    tx::TxClient, Config, Metadata,
};
use derivative::Derivative;
use std::sync::Arc;

/// A trait representing a client that can perform
/// offline-only actions.
pub trait OfflineClientT<T: Config>: Clone + Send + Sync + 'static {
    /// Return the provided [`Metadata`].
    fn metadata(&self) -> Metadata;
    /// Return the provided genesis hash.
    fn genesis_hash(&self) -> T::Hash;
    /// Return the provided [`RuntimeVersion`].
    fn runtime_version(&self) -> RuntimeVersion;

    /// Work with transactions.
    fn tx(&self) -> TxClient<T, Self> {
        TxClient::new(self.clone())
    }

    /// Work with events.
    fn events(&self) -> EventsClient<T, Self> {
        EventsClient::new(self.clone())
    }

    /// Work with storage.
    fn storage(&self) -> StorageClient<T, Self> {
        StorageClient::new(self.clone())
    }

    /// Access constants.
    fn constants(&self) -> ConstantsClient<T, Self> {
        ConstantsClient::new(self.clone())
    }

    /// Work with blocks.
    fn blocks(&self) -> BlocksClient<T, Self> {
        BlocksClient::new(self.clone())
    }

    /// Work with runtime API.
    fn runtime_api(&self) -> RuntimeApiClient<T, Self> {
        RuntimeApiClient::new(self.clone())
    }
}

/// A client that is capable of performing offline-only operations.
/// Can be constructed as long as you can populate the required fields.
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct OfflineClient<T: Config> {
    inner: Arc<Inner<T>>,
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
struct Inner<T: Config> {
    genesis_hash: T::Hash,
    runtime_version: RuntimeVersion,
    metadata: Metadata,
}

impl<T: Config> OfflineClient<T> {
    /// Construct a new [`OfflineClient`], providing
    /// the necessary runtime and compile-time arguments.
    pub fn new(
        genesis_hash: T::Hash,
        runtime_version: RuntimeVersion,
        metadata: Metadata,
    ) -> OfflineClient<T> {
        OfflineClient {
            inner: Arc::new(Inner {
                genesis_hash,
                runtime_version,
                metadata,
            }),
        }
    }

    /// Return the genesis hash.
    pub fn genesis_hash(&self) -> T::Hash {
        self.inner.genesis_hash
    }

    /// Return the runtime version.
    pub fn runtime_version(&self) -> RuntimeVersion {
        self.inner.runtime_version.clone()
    }

    /// Return the [`Metadata`] used in this client.
    pub fn metadata(&self) -> Metadata {
        self.inner.metadata.clone()
    }

    // Just a copy of the most important trait methods so that people
    // don't need to import the trait for most things:

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
}

impl<T: Config> OfflineClientT<T> for OfflineClient<T> {
    fn genesis_hash(&self) -> T::Hash {
        self.genesis_hash()
    }
    fn runtime_version(&self) -> RuntimeVersion {
        self.runtime_version()
    }
    fn metadata(&self) -> Metadata {
        self.metadata()
    }
}

// For ergonomics; cloning a client is deliberately fairly cheap (via Arc),
// so this allows users to pass references to a client rather than explicitly
// cloning. This is partly for consistency with OnlineClient, which can be
// easily converted into an OfflineClient for ergonomics.
impl<'a, T: Config> From<&'a OfflineClient<T>> for OfflineClient<T> {
    fn from(c: &'a OfflineClient<T>) -> Self {
        c.clone()
    }
}
