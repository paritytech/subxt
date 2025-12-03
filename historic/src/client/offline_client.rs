use super::ClientAtBlock;
use crate::config::Config;
use crate::error::OfflineClientAtBlockError;
use frame_metadata::RuntimeMetadata;
use scale_info_legacy::TypeRegistrySet;
use std::sync::Arc;

/// A client which exposes the means to decode historic data on a chain offline.
#[derive(Clone, Debug)]
pub struct OfflineClient<T: Config> {
    /// The configuration for this client.
    config: Arc<T>,
}

impl<T: Config> OfflineClient<T> {
    /// Create a new [`OfflineClient`] with the given configuration.
    pub fn new(config: T) -> Self {
        OfflineClient {
            config: Arc::new(config),
        }
    }

    /// Pick the block height at which to operate. This references data from the
    /// [`OfflineClient`] it's called on, and so cannot outlive it.
    pub fn at<'this>(
        &'this self,
        block_number: u64,
    ) -> Result<ClientAtBlock<OfflineClientAtBlock<'this, T>, T>, OfflineClientAtBlockError> {
        let config = &self.config;
        let spec_version = self
            .config
            .spec_version_for_block_number(block_number)
            .ok_or(OfflineClientAtBlockError::SpecVersionNotFound { block_number })?;

        let legacy_types = self.config.legacy_types_for_spec_version(spec_version);
        let metadata = self
            .config
            .metadata_for_spec_version(spec_version)
            .ok_or(OfflineClientAtBlockError::MetadataNotFound { spec_version })?;

        Ok(ClientAtBlock::new(OfflineClientAtBlock {
            config,
            legacy_types,
            metadata,
        }))
    }
}

/// This represents an offline-only client at a specific block.
pub trait OfflineClientAtBlockT<'client, T: Config + 'client> {
    /// Get the configuration for this client.
    fn config(&self) -> &'client T;
    /// Get the legacy types that work at this block.
    fn legacy_types(&'_ self) -> &TypeRegistrySet<'client>;
    /// Get the metadata appropriate for this block.
    fn metadata(&self) -> &RuntimeMetadata;
}

// Dev note: this shouldn't need to be exposed unless there is some
// need to explicitly name the ClientAAtBlock type. Rather keep it
// private to allow changes if possible.
pub struct OfflineClientAtBlock<'client, T: Config + 'client> {
    /// The configuration for this chain.
    config: &'client T,
    /// Historic types to use at this block number.
    legacy_types: TypeRegistrySet<'client>,
    /// Metadata to use at this block number.
    metadata: Arc<RuntimeMetadata>,
}

impl<'client, T: Config + 'client> OfflineClientAtBlockT<'client, T>
    for OfflineClientAtBlock<'client, T>
{
    fn config(&self) -> &'client T {
        self.config
    }
    fn legacy_types(&self) -> &TypeRegistrySet<'client> {
        &self.legacy_types
    }
    fn metadata(&self) -> &RuntimeMetadata {
        &self.metadata
    }
}
