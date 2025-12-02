use crate::client::ClientAtBlock;
use crate::config::Config;
use crate::error::OfflineClientAtBlockError;
use std::sync::Arc;
use subxt_metadata::Metadata;

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
    ) -> Result<ClientAtBlock<OfflineClientAtBlock, T>, OfflineClientAtBlockError> {
        let block_number = block_number.into();
        let spec_version = self
            .config
            .spec_version_for_block_number(block_number)
            .ok_or(OfflineClientAtBlockError::SpecVersionNotFound { block_number })?;

        let metadata = self
            .config
            .metadata_for_spec_version(spec_version)
            .ok_or(OfflineClientAtBlockError::MetadataNotFound { spec_version })?;

        Ok(ClientAtBlock::new(OfflineClientAtBlock { metadata }))
    }
}

#[derive(Clone)]
pub struct OfflineClientAtBlock {
    metadata: Arc<Metadata>,
}

/// This represents an offline-only client at a specific block.
#[doc(hidden)]
pub trait OfflineClientAtBlockT: Clone {
    /// Get a reference to the metadata appropriate for this block.
    fn metadata_ref(&self) -> &Metadata;
    /// Get a clone of the metadata appropriate for this block.
    fn metadata(&self) -> Arc<Metadata>;
}

impl OfflineClientAtBlockT for OfflineClientAtBlock {
    fn metadata_ref(&self) -> &Metadata {
        &self.metadata
    }
    fn metadata(&self) -> Arc<Metadata> {
        self.metadata.clone()
    }
}
