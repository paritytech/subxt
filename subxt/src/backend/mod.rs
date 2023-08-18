// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a backend trait for Subxt which allows us to get and set
//! the necessary information (probably from a JSON-RPC API, but that's up to the
//! implementation).

pub mod legacy;
pub mod rpc;

use crate::error::Error;
use crate::metadata::Metadata;
use crate::Config;
use async_trait::async_trait;
use codec::{Decode, Encode};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;

/// Prevent the backend trait being implemented externally.
#[doc(hidden)]
pub(crate) mod sealed {
    pub trait Sealed {}
}

/// This trait exposes the interface that Subxt will use to communicate with
/// a backend. Its goal is to be as minimal as possible.
#[async_trait]
pub trait Backend<T: Config>: sealed::Sealed + Send + Sync + 'static {
    /// Fetch values from storage.
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: T::Hash,
    ) -> Result<StreamOfResults<StorageResponse>, Error>;

    /// Fetch keys underneath the given key from storage.
    async fn storage_fetch_descendant_keys(
        &self,
        key: Vec<u8>,
        starting_at: Option<Vec<u8>>,
        at: T::Hash,
    ) -> Result<StreamOfResults<Vec<u8>>, Error>;

    /// Fetch values underneath the given key from storage.
    async fn storage_fetch_descendant_values(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<StreamOfResults<StorageResponse>, Error>;

    /// Fetch the genesis hash
    async fn genesis_hash(&self) -> Result<T::Hash, Error>;

    /// Get a block header
    async fn block_header(&self, at: T::Hash) -> Result<Option<T::Header>, Error>;

    /// Return the extrinsics found in the block. Each extrinsic is represented
    /// by a vector of bytes which has _not_ been SCALE decoded (in other words, the
    /// first bytes in the vector will decode to the compact encoded length of the extrinsic)
    async fn block_body(&self, at: T::Hash) -> Result<Option<Vec<Vec<u8>>>, Error>;

    /// Get the most recent finalized block hash.
    /// Note: needed only in blocks client for finalized block stream; can prolly be removed.
    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<T::Hash>, Error>;

    /// Get the most recent best block hash.
    /// Note: needed only in blocks client for finalized block stream; can prolly be removed.
    async fn latest_best_block_ref(&self) -> Result<BlockRef<T::Hash>, Error>;

    /// Get information about the current runtime.
    async fn current_runtime_version(&self) -> Result<RuntimeVersion, Error>;

    /// A stream of all new runtime versions as they occur.
    async fn stream_runtime_version(&self) -> Result<StreamOfResults<RuntimeVersion>, Error>;

    /// A stream of all new block headers as they arrive.
    async fn stream_all_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error>;

    /// A stream of best block headers.
    async fn stream_best_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error>;

    /// A stream of finalized block headers.
    async fn stream_finalized_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error>;

    /// Submit a transaction. This will return a stream of events about it.
    async fn submit_transaction(
        &self,
        bytes: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<T::Hash>>, Error>;

    /// Make a call to some runtime API.
    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: T::Hash,
    ) -> Result<Vec<u8>, Error>;
}

/// helpeful utility methods derived from those provided on [`Backend`]
#[async_trait]
pub trait BackendExt<T: Config>: Backend<T> {
    /// Fetch a single value from storage.
    async fn storage_fetch_value(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<Option<Vec<u8>>, Error> {
        self.storage_fetch_values(vec![key], at)
            .await?
            .next()
            .await
            .transpose()
            .map(|o| o.map(|s| s.value))
    }

    /// The same as a [`Backend::call()`], but it will also attempt to decode the
    /// result into the given type, which is a fairly common operation.
    async fn call_decoding<D: codec::Decode>(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: T::Hash,
    ) -> Result<D, Error> {
        let bytes = self.call(method, call_parameters, at).await?;
        let res = D::decode(&mut &*bytes)?;
        Ok(res)
    }

    /// Return the metadata at some version.
    async fn metadata_at_version(&self, version: u32, at: T::Hash) -> Result<Metadata, Error> {
        let param = version.encode();

        let opaque: Option<frame_metadata::OpaqueMetadata> = self
            .call_decoding("Metadata_metadata_at_version", Some(&param), at)
            .await?;
        let Some(opaque) = opaque else {
            return Err(Error::Other("Metadata version not found".into()));
        };

        let metadata: Metadata = Decode::decode(&mut &opaque.0[..])?;
        Ok(metadata)
    }

    /// Return V14 metadata from the legacy `Metadata_metadata` call.
    async fn legacy_metadata(&self, at: T::Hash) -> Result<Metadata, Error> {
        let opaque: frame_metadata::OpaqueMetadata =
            self.call_decoding("Metadata_metadata", None, at).await?;
        let metadata: Metadata = Decode::decode(&mut &opaque.0[..])?;
        Ok(metadata)
    }
}

#[async_trait]
impl<B: Backend<T> + ?Sized, T: Config> BackendExt<T> for B {}

/// An opaque struct which, while alive, indicates that some references to a block
/// still exist. This gives the backend the opportunity to keep the corresponding block
/// details around for a while if it likes and is able to. No guarantees can be made about
/// how long the corresponding details might be available for, but if no references to a block
/// exist, then the backend is free to discard any details for it.
#[derive(Clone)]
pub struct BlockRef<H> {
    hash: H,
    // We keep this around so that when it is dropped, it has the
    // opportunity to tell the backend.
    _pointer: Option<Arc<dyn BlockRefT>>,
}

impl<H> From<H> for BlockRef<H> {
    fn from(value: H) -> Self {
        BlockRef::from_hash(value)
    }
}

impl<H: PartialEq> PartialEq for BlockRef<H> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
impl<H: Eq> Eq for BlockRef<H> {}

impl<H: PartialOrd> PartialOrd for BlockRef<H> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl<H: Ord> Ord for BlockRef<H> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}

impl<H: std::fmt::Debug> std::fmt::Debug for BlockRef<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BlockRef").field(&self.hash).finish()
    }
}

impl<H: std::hash::Hash> std::hash::Hash for BlockRef<H> {
    fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
        self.hash.hash(state);
    }
}

impl<H> BlockRef<H> {
    /// A [`BlockRef`] that doesn't reference a given block, but does have an associated hash.
    /// This is used in the legacy backend, which has no notion of pinning blocks.
    pub fn from_hash(hash: H) -> Self {
        Self {
            hash,
            _pointer: None,
        }
    }
    /// Construct a [`BlockRef`] from an instance of the underlying trait. It's expected
    /// that the [`Backend`] implementation will call this if it wants to track which blocks
    /// are potentially in use.
    pub fn new<P: BlockRefT>(hash: H, inner: P) -> Self {
        Self {
            hash,
            _pointer: Some(Arc::new(inner)),
        }
    }

    /// Return the hash of the referenced block.
    pub fn hash(&self) -> H
    where
        H: Copy,
    {
        self.hash
    }
}

/// A trait that a [`Backend`] can implement to know when some block
/// can be unpinned: when this is dropped, there are no remaining references
/// to the block that it's associated with.
pub trait BlockRefT: Send + Sync + 'static {}

/// A stream of some item.
pub struct StreamOf<T>(Pin<Box<dyn Stream<Item = T> + Send + 'static>>);

impl<T> Stream for StreamOf<T> {
    type Item = T;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.0.poll_next_unpin(cx)
    }
}

impl<T> std::fmt::Debug for StreamOf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StreamOf").field(&"<stream>").finish()
    }
}

impl<T> StreamOf<T> {
    /// Construct a new stream.
    pub fn new(inner: Pin<Box<dyn Stream<Item = T> + Send + 'static>>) -> Self {
        StreamOf(inner)
    }

    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next_item(&mut self) -> Option<T> {
        self.next().await
    }
}

/// A stream of [`Result<Item, Error>`].
pub type StreamOfResults<T> = StreamOf<Result<T, Error>>;

/// Runtime version information needed to submit transactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
///
/// If the status is [`TransactionStatus::InFinalizedBlock`], [`TransactionStatus::Error`],
/// [`TransactionStatus::Invalid`] or [`TransactionStatus::Dropped`], then no future
/// events will be emitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionStatus<Hash> {
    /// Transaction is part of the future queue.
    Validated,
    /// The transaction has been broadcast to other nodes.
    Broadcasted {
        /// Number of peers it's been broadcast to.
        num_peers: u32,
    },
    /// Transaction has been included in block with given hash.
    InBestBlock {
        /// Block hash the transaction is in.
        hash: Hash,
    },
    /// Transaction has been finalized by a finality-gadget, e.g GRANDPA
    InFinalizedBlock {
        /// Block hash the transaction is in.
        hash: Hash,
    },
    /// Something went wrong in the node.
    Error {
        /// Human readable message; what went wrong.
        message: String,
    },
    /// Transaction is invalid (bad nonce, signature etc).
    Invalid {
        /// Human readable message; why was it invalid.
        message: String,
    },
    /// The transaction was dropped.
    Dropped {
        /// Human readable message; why was it dropped.
        message: String,
    },
}

/// A response from [`Backend::storage_fetch`].
pub struct StorageResponse {
    /// The key.
    pub key: Vec<u8>,
    /// The associated value.
    pub value: Vec<u8>,
}

// Just a test that the backend trait is object safe.
#[cfg(test)]
#[allow(dead_code)]
fn is_object_safe() {
    use crate::config::PolkadotConfig;
    let _: Box<dyn Backend<PolkadotConfig>> = unimplemented!();
}
