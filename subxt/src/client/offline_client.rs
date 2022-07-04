// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    Config,
    Metadata,
    rpc::RuntimeVersion,
    extrinsic::TxClient,
    events::EventsClient,
};
use std::sync::Arc;
use derivative::Derivative;

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
}

/// A client that is capable of performing offline-only operations.
/// Can be constructed as long as you can populate the required fields.
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct OfflineClient<T: Config> {
    inner: Arc<Inner<T>>
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
struct Inner<T: Config> {
    genesis_hash: T::Hash,
    runtime_version: RuntimeVersion,
    metadata: Metadata,
}

impl <T: Config> OfflineClient<T> {
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
            })
        }
    }
}

impl <T: Config> OfflineClientT<T> for OfflineClient<T> {
        /// Return the genesis hash.
        fn genesis_hash(&self) -> T::Hash {
            self.inner.genesis_hash
        }

        /// Return the runtime version.
        fn runtime_version(&self) -> RuntimeVersion {
            self.inner.runtime_version.clone()
        }

        /// Return the [`Metadata`] used in this client.
        fn metadata(&self) -> Metadata {
            self.inner.metadata.clone()
        }
}

// For ergonomics; cloning a client is deliberately fairly cheap (via Arc),
// so this allows users to pass references to a client rather than explicitly
// cloning. This is partly for consistency with OnlineClient, which can be
// easily converted into an OfflineClient for ergonomics.
impl <'a, T: Config> From<&'a OfflineClient<T>> for OfflineClient<T> {
    fn from(c: &'a OfflineClient<T>) -> Self {
        c.clone()
    }
}