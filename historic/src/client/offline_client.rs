
use crate::config::Config;
use crate::error::OfflineClientAtBlockError;
use crate::utils::RangeMap;
use std::collections::HashMap;
use frame_metadata::RuntimeMetadata;
use scale_info_legacy::TypeRegistrySet;
use super::ClientAtBlock;

/// A client which exposes the means to decode historic data on a chain offline.
pub struct OfflineClient<T: Config> {
    /// The configuration for this client.
    config: T,
    /// An explicit mapping from block range to spec version.
    spec_version_for_block_number: RangeMap<u64, u32>,
    /// An explicit mapping from spec version to metadata.
    metadata_for_spec_version: HashMap<u32, RuntimeMetadata>,
}

impl <T: Config> OfflineClient<T> {
    /// Construct an offline-only client at a specific block.
    pub fn at<'this>(&'this self, block_number: u64) -> Result<ClientAtBlock<OfflineClientAtBlock<'this, T>, T>, OfflineClientAtBlockError> {
        let config = &self.config;
        let spec_version = *self
            .spec_version_for_block_number
            .get(block_number)
            .ok_or_else(|| OfflineClientAtBlockError::SpecVersionNotFound { block_number })?;

        let legacy_types = self.config.legacy_types_for_spec_version(spec_version);
        let metadata = self.metadata_for_spec_version
            .get(&spec_version)
            .ok_or_else(|| OfflineClientAtBlockError::MetadataNotFound { spec_version })?;

        Ok(ClientAtBlock::new(OfflineClientAtBlock {
            config,
            legacy_types,
            metadata,
        }))
    }
}

/// This represents an offline-only client at a specific block.
#[doc(hidden)]
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
#[doc(hidden)]
pub struct OfflineClientAtBlock<'client, T: Config + 'client> {
    /// The configuration for thie chain.
    config: &'client T,
    /// Historic types to use at this block number.
    legacy_types: TypeRegistrySet<'client>,
    /// Metadata to use at this block number.
    metadata: &'client RuntimeMetadata,
}

impl <'client, T: Config + 'client> OfflineClientAtBlockT<'client, T> for OfflineClientAtBlock<'client, T> {
    fn config(&self) -> &'client T {
        self.config
    }
    fn legacy_types(&self) -> &TypeRegistrySet<'client> {
        &self.legacy_types
    }
    fn metadata(&self) -> &'client RuntimeMetadata {
        self.metadata
    }
}