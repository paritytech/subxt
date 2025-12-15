use crate::client::ClientAtBlock;
use crate::config::{Config, HashFor, Hasher};
use crate::error::OfflineClientAtBlockError;
use crate::metadata::{ArcMetadata, Metadata};

/// A client which does not require a connection to a chain, and can perform certain
/// actions which don't require a network connection.
#[derive(Clone, Debug)]
pub struct OfflineClient<T: Config> {
    /// The configuration for this client.
    config: T,
}

impl<T: Config> OfflineClient<T> {
    /// Create a new [`OfflineClient`] with the given configuration.
    pub fn new(config: T) -> Self {
        OfflineClient { config }
    }

    /// Pick the block height at which to operate. This references data from the
    /// [`OfflineClient`] it's called on, and so cannot outlive it.
    pub fn at_block(
        &self,
        block_number: impl Into<u64>,
    ) -> Result<ClientAtBlock<T, OfflineClientAtBlockImpl<T>>, OfflineClientAtBlockError> {
        let block_number = block_number.into();
        let (spec_version, transaction_version) = self
            .config
            .spec_and_transaction_version_for_block_number(block_number)
            .ok_or(OfflineClientAtBlockError::SpecVersionNotFound { block_number })?;

        let metadata = self
            .config
            .metadata_for_spec_version(spec_version)
            .ok_or(OfflineClientAtBlockError::MetadataNotFound { spec_version })?;

        let genesis_hash = self.config.genesis_hash();

        let hasher = <T::Hasher as Hasher>::new(&metadata);

        let offline_client_at_block = OfflineClientAtBlockImpl {
            metadata,
            block_number,
            genesis_hash,
            spec_version,
            hasher,
            transaction_version,
        };

        Ok(ClientAtBlock::new(offline_client_at_block))
    }
}

/// An implementation of the [`OfflineClientAtBlockT`] trait, which is used in conjunction
/// with [`crate::client::ClientAtBlock`] to provide a working client. You won't tend to need this
/// type and instead should prefer to refer to [`crate::client::OfflineClientAtBlock`].
#[derive(Clone)]
pub struct OfflineClientAtBlockImpl<T: Config> {
    metadata: ArcMetadata,
    block_number: u64,
    genesis_hash: Option<HashFor<T>>,
    spec_version: u32,
    hasher: T::Hasher,
    transaction_version: u32,
}

/// This represents an offline-only client at a specific block.
#[doc(hidden)]
pub trait OfflineClientAtBlockT<T: Config>: Clone {
    /// Get a reference to the metadata appropriate for this block.
    fn metadata_ref(&self) -> &Metadata;
    /// Get a clone of the metadata appropriate for this block.
    fn metadata(&self) -> ArcMetadata;
    /// The block number we're operating at.
    fn block_number(&self) -> u64;
    /// Return the genesis hash for the chain if it is known.
    fn genesis_hash(&self) -> Option<HashFor<T>>;
    /// The spec version at the current block.
    fn spec_version(&self) -> u32;
    /// Return a hasher that works at the current block.
    fn hasher(&self) -> &T::Hasher;
    /// The transaction version at the current block.
    ///
    /// Note: This is _not_ the same as the transaction version that
    /// is encoded at the beginning of transactions (ie 4 or 5).
    fn transaction_version(&self) -> u32;
}

impl<T: Config> OfflineClientAtBlockT<T> for OfflineClientAtBlockImpl<T> {
    fn metadata_ref(&self) -> &Metadata {
        &self.metadata
    }
    fn metadata(&self) -> ArcMetadata {
        self.metadata.clone()
    }
    fn block_number(&self) -> u64 {
        self.block_number
    }
    fn genesis_hash(&self) -> Option<HashFor<T>> {
        self.genesis_hash
    }
    fn spec_version(&self) -> u32 {
        self.spec_version
    }
    fn transaction_version(&self) -> u32 {
        self.transaction_version
    }
    fn hasher(&self) -> &T::Hasher {
        &self.hasher
    }
}
