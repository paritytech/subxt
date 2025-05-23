// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::custom_values::CustomValuesClient;
use crate::{
    Metadata,
    blocks::BlocksClient,
    config::{Config, HashFor},
    constants::ConstantsClient,
    events::EventsClient,
    runtime_api::RuntimeApiClient,
    storage::StorageClient,
    tx::TxClient,
    view_functions::ViewFunctionsClient,
};

use derive_where::derive_where;
use std::sync::Arc;
use subxt_core::client::{ClientState, RuntimeVersion};

/// A trait representing a client that can perform
/// offline-only actions.
pub trait OfflineClientT<T: Config>: Clone + Send + Sync + 'static {
    /// Return the provided [`Metadata`].
    fn metadata(&self) -> Metadata;

    /// Return the provided genesis hash.
    fn genesis_hash(&self) -> HashFor<T>;

    /// Return the provided [`RuntimeVersion`].
    fn runtime_version(&self) -> RuntimeVersion;

    /// Return the hasher used on the chain.
    fn hasher(&self) -> T::Hasher;

    /// Return the [subxt_core::client::ClientState] (metadata, runtime version and genesis hash).
    fn client_state(&self) -> ClientState<T> {
        ClientState {
            genesis_hash: self.genesis_hash(),
            runtime_version: self.runtime_version(),
            metadata: self.metadata(),
        }
    }

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

    /// Work with runtime APIs.
    fn runtime_api(&self) -> RuntimeApiClient<T, Self> {
        RuntimeApiClient::new(self.clone())
    }

    /// Work with View Functions.
    fn view_functions(&self) -> ViewFunctionsClient<T, Self> {
        ViewFunctionsClient::new(self.clone())
    }

    /// Work this custom types.
    fn custom_values(&self) -> CustomValuesClient<T, Self> {
        CustomValuesClient::new(self.clone())
    }
}

/// A client that is capable of performing offline-only operations.
/// Can be constructed as long as you can populate the required fields.
#[derive_where(Debug, Clone)]
pub struct OfflineClient<T: Config> {
    inner: Arc<ClientState<T>>,
    hasher: T::Hasher,
}

impl<T: Config> OfflineClient<T> {
    /// Construct a new [`OfflineClient`], providing
    /// the necessary runtime and compile-time arguments.
    pub fn new(
        genesis_hash: HashFor<T>,
        runtime_version: RuntimeVersion,
        metadata: impl Into<Metadata>,
    ) -> OfflineClient<T> {
        let metadata = metadata.into();
        let hasher = <T::Hasher as subxt_core::config::Hasher>::new(&metadata);

        OfflineClient {
            hasher,
            inner: Arc::new(ClientState {
                genesis_hash,
                runtime_version,
                metadata,
            }),
        }
    }

    /// Return the genesis hash.
    pub fn genesis_hash(&self) -> HashFor<T> {
        self.inner.genesis_hash
    }

    /// Return the runtime version.
    pub fn runtime_version(&self) -> RuntimeVersion {
        self.inner.runtime_version
    }

    /// Return the [`Metadata`] used in this client.
    pub fn metadata(&self) -> Metadata {
        self.inner.metadata.clone()
    }

    /// Return the hasher used for the chain.
    pub fn hasher(&self) -> T::Hasher {
        self.hasher
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

    /// Work with blocks.
    pub fn blocks(&self) -> BlocksClient<T, Self> {
        <Self as OfflineClientT<T>>::blocks(self)
    }

    /// Work with runtime APIs.
    pub fn runtime_api(&self) -> RuntimeApiClient<T, Self> {
        <Self as OfflineClientT<T>>::runtime_api(self)
    }

    /// Work with View Functions.
    pub fn view_functions(&self) -> ViewFunctionsClient<T, Self> {
        <Self as OfflineClientT<T>>::view_functions(self)
    }

    /// Access custom types
    pub fn custom_values(&self) -> CustomValuesClient<T, Self> {
        <Self as OfflineClientT<T>>::custom_values(self)
    }
}

impl<T: Config> OfflineClientT<T> for OfflineClient<T> {
    fn genesis_hash(&self) -> HashFor<T> {
        self.genesis_hash()
    }
    fn runtime_version(&self) -> RuntimeVersion {
        self.runtime_version()
    }
    fn metadata(&self) -> Metadata {
        self.metadata()
    }
    fn hasher(&self) -> T::Hasher {
        self.hasher()
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
