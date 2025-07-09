
use crate::config::Config;
use crate::error::OfflineClientAtBlockError;
use crate::utils::RangeMap;
use std::collections::HashMap;
use frame_metadata::RuntimeMetadata;

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
    pub fn at<'this>(&'this self, block_number: u64) -> Result<OfflineClientAtBlock<'this, T>, OfflineClientAtBlockError> {
        let config = &self.config;
        let spec_version = *self
            .spec_version_for_block_number
            .get(block_number)
            .ok_or_else(|| OfflineClientAtBlockError::SpecVersionNotFound { block_number })?;

        let legacy_types = self.config.legacy_types_for_spec_version(spec_version);
        let metadata = self.metadata_for_spec_version
            .get(&spec_version)
            .ok_or_else(|| OfflineClientAtBlockError::MetadataNotFound { spec_version })?;

        Ok(OfflineClientAtBlock {
            config,
            legacy_types,
            metadata,
        })
    }
}

/// This represents an offline-only client at a specific block.
pub trait OfflineClientAtBlockT<'client, T: Config + 'client> {
    /// Get the configuration for this client.
    fn config(&self) -> &'client T;
    /// Get the legacy types that work at this block.
    fn legacy_types(&'_ self) -> &T::LegacyTypes<'client>;
    /// Get the metadata appropriate for this block.
    fn metadata(&self) -> &RuntimeMetadata;
}

/// An offline client that's ready to work at a specific block.
pub struct OfflineClientAtBlock<'client, T: Config + 'client> {
    /// The configuration for thie chain.
    config: &'client T,
    /// Historic types to use at this block number.
    legacy_types: T::LegacyTypes<'client>,
    /// Metadata to use at this block number.
    metadata: &'client RuntimeMetadata,
}

impl <'client, T: Config + 'client> OfflineClientAtBlockT<'client, T> for OfflineClientAtBlock<'client, T> {
    fn config(&self) -> &'client T {
        self.config
    }
    fn legacy_types(&self) -> &T::LegacyTypes<'client> {
        &self.legacy_types
    }
    fn metadata(&self) -> &'client RuntimeMetadata {
        self.metadata
    }
}