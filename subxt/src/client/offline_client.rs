// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    Config,
    Metadata,
    rpc::RuntimeVersion,
};
use std::sync::Arc;
use derivative::Derivative;

/// A client that is capable of performing offline-only operations.
/// Can be constructed as long as you can populate the required fields.
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct OfflineClient<T: Config> {
    inner: Arc<Inner<T>>
}

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

    /// Return the genesis hash.
    pub fn genesis_hash(&self) -> &T::Hash {
        &self.inner.genesis_hash
    }

    /// Return the runtime version.
    pub fn runtime_version(&self) -> &RuntimeVersion {
        &self.inner.runtime_version
    }

    /// Return the [`Metadata`] used in this client.
    pub fn metadata(&self) -> &Metadata {
        &self.inner.metadata
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