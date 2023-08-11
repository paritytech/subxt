// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a backend trait for Subxt which allows us to get and set
//! the necessary information (probably from a JSON-RPC API, but that's up to the
//! implementation).

pub mod legacy;
pub mod rpc;

use crate::Config;
use crate::error::Error;
use async_trait::async_trait;
use std::pin::Pin;
use futures::Stream;
use std::sync::Arc;

/// This trait exposes the interface that Subxt will use to communicate with
/// a backend. Its goal is to be as minimal as possible.
#[async_trait]
pub trait Backend<T: Config> {
    /// Fetch the raw bytes for a given storage key
    async fn storage_fetch_value(
        &self,
        key: &[u8],
        at: Option<T::Hash>,
    ) -> Result<Option<StorageData>, Error>;

    /// Returns the keys with prefix with pagination support.
    /// Up to `count` keys will be returned.
    /// If `start_key` is passed, return next keys in storage in lexicographic order.
    async fn storage_fetch_keys(
        &self,
        key: &[u8],
        count: u32,
        start_key: Option<&[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<StorageKey>, Error>;

    /// Query historical storage entries
    async fn query_storage_at(
        &self,
        keys: &dyn Iterator<Item = &[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<T::Hash>>, Error>;

    /// Fetch the genesis hash
    async fn genesis_hash(&self) -> Result<T::Hash, Error>;

    /// Get a block header
    async fn block_header(&self, at: Option<T::Hash>) -> Result<Option<T::Header>, Error>;

    /// Return the extrinsics found in the block. Each extrinsic is represented
    /// by a vector of bytes which has _not_ been SCALE decoded (in other words, the
    /// first bytes in the vector will decode to the compact encoded length of the extrinsic)
    async fn block_body(&self, at: Option<T::Hash>) -> Result<Option<Vec<Vec<u8>>>, Error>;

    /// Get the most recent finalized block hash.
    /// Note: needed only in blocks client for finalized block stream; can prolly be removed.
    async fn latest_finalized_block_hash(&self) -> Result<Option<WithBlockRef<T::Hash>>, Error>;

    /// Get the most recent best block hash.
    /// Note: needed only in blocks client for finalized block stream; can prolly be removed.
    async fn latest_best_block_hash(&self) -> Result<Option<WithBlockRef<T::Hash>>, Error>;

    /// Get information about the current runtime.
    async fn current_runtime_version(&self) -> Result<RuntimeVersion, Error>;

    /// A stream of all new runtime versions as they occur.
    async fn stream_runtime_version(&self) -> Result<StreamOf<RuntimeVersion>, Error>;

    /// A stream of all new block headers as they arrive.
    async fn stream_all_block_headers(&self) -> Result<StreamOf<WithBlockRef<T::Header>>, Error>;

    /// A stream of best block headers.
    async fn stream_best_block_headers(&self) -> Result<StreamOf<WithBlockRef<T::Header>>, Error>;

    /// A stream of finalized block headers.
    async fn stream_finalized_block_headers(&self) -> Result<StreamOf<WithBlockRef<T::Header>>, Error>;

    /// Submit a transaction. This will return a stream of events about it.
    async fn submit_transaction(&self) -> Result<StreamOf<TransactionStatus<T::Hash>>, Error>;

    // Dev note: no dry_run function exists, but see this for how to call it via runtime API:
    // https://github.com/paritytech/json-rpc-interface-spec/issues/55

    /// Make a call to some runtime API.
    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<u8>, Error>;
}

/// This returns some details about a block with the corresponding [`BlockRef`].
pub struct WithBlockRef<T> {
    /// The result.
    pub result: T,
    /// When this [`BlockRef`] is dropped, the backend no longer has
    /// any obligation to hold on to the associated block details.
    pub block_ref: BlockRef
}

/// An opaque struct which, while alive, indicates that some references to a block
/// still exist. This gives the backend the opportunity to keep the corresponding block
/// details around for a while if it likes and is able to. No guarantees can be made about
/// how long the corresponding details might be available for, but if no references to a block
/// exist, then the backend is free to discard any details for it.
#[derive(Clone)]
pub struct BlockRef(Option<Arc<dyn BlockRefT>>);

impl BlockRef {
    /// An empty [`BlockRef`] that will do nothing.
    pub fn empty() -> Self {
        Self(None)
    }
    /// Construct a [`BlockRef`] from an instance of the underlying trait. It's expected
    /// that the [`Backend`] implementation will call this if it wants to track which blocks
    /// are potentially in use.
    pub fn new<P: BlockRefT + 'static>(inner: P) -> Self {
        Self(Some(Arc::new(inner)))
    }
}

/// A trait that a [`Backend`] can implement to know when some block
/// can be unpinned: when this is dropped, there are no remaining references
/// to the block that it's associated with.
pub trait BlockRefT {}

/// A stream of some item.
type StreamOf<T> = Pin<Box<dyn Stream<Item = T> + Send + 'static>>;

/// The raw bytes for a storage request.
type StorageData = Vec<u8>;

/// A storage key.
type StorageKey = Vec<u8>;

/// Storage keys that changed at some block hash
pub struct StorageChangeSet<Hash> {
    /// Block hash
    pub block: Hash,
    /// A list of changes
    pub changes: Vec<(StorageKey, Option<StorageData>)>,
}

/// Runtime version information needed to submit transactions.
pub struct RuntimeVersion {
    /// Version of the runtime specification. A full-node will not attempt to use its native
    /// runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    /// `spec_version` and `authoring_version` are the same between Wasm and native.
    pub spec_version: u32,

    /// All existing dispatches are fully compatible when this number doesn't change. If this
    /// number changes, then `spec_version` must change, also.
    ///
    /// This number must change when an existing dispatchable (module ID, dispatch ID) is changed,
    /// either through an alteration in its user-level semantics, a parameter
    /// added/removed/changed, a dispatchable being removed, a module being removed, or a
    /// dispatchable/module changing its index.
    ///
    /// It need *not* change when a new module is added or when a dispatchable is added.
    pub transaction_version: u32,
}

/// The status of the transaction.
pub enum TransactionStatus<Hash> {
    /// Transaction is part of the future queue.
    Future,
    /// Transaction is part of the ready queue.
    Ready,
    /// The transaction has been broadcast to the given peers.
    Broadcast(Vec<String>),
    /// Transaction has been included in block with given hash.
    InBlock(Hash),
    /// The block this transaction was included in has been retracted.
    Retracted(Hash),
    /// Maximum number of finality watchers has been reached,
    /// old watchers are being removed.
    FinalityTimeout(Hash),
    /// Transaction has been finalized by a finality-gadget, e.g GRANDPA
    Finalized(Hash),
    /// Transaction has been replaced in the pool, by another transaction
    /// that provides the same tags. (e.g. same (sender, nonce)).
    Usurped(Hash),
    /// Transaction has been dropped from the pool because of the limit.
    Dropped,
    /// Transaction is no longer valid in the current state.
    Invalid,
}

// Just a test that the trait is object safe.
#[cfg(test)]
#[allow(dead_code)]
fn is_object_safe() {
    use crate::config::PolkadotConfig;
    let _: Box<dyn Backend<PolkadotConfig>> = unimplemented!();
}