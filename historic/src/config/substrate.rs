use primitive_types::H256;
use scale_info_legacy::{ChainTypeRegistry, TypeRegistrySet};
use std::collections::HashMap;
use std::sync::Arc;
use crate::utils::RangeMap;
use super::Config;
use std::sync::Mutex;

/// Configuration that's suitable for standard Substrate chains (ie those
/// that have not customized the block hash type).
pub struct SubstrateConfig {
    legacy_types: ChainTypeRegistry,
    spec_version_for_block_number: RangeMap<u64, u32>,
    metadata_for_spec_version: Mutex<HashMap<u32, Arc<frame_metadata::RuntimeMetadata>>>,
}

impl SubstrateConfig {
    /// Create a new SubstrateConfig with no legacy types. 
    /// 
    /// Without any further configuration, this will only work with
    /// the [`crate::client::OnlineClient`] for blocks that were produced by Runtimes
    /// that emit metadata V14 or later.
    /// 
    /// To support working at any block with the [`crate::client::OnlineClient`], you
    /// must call [`SubstrateConfig::set_legacy_types`] with appropriate legacy type
    /// definitions.
    /// 
    /// To support working with the [`crate::client::OfflineClient`] at any block,
    /// you must also call:
    /// - [`SubstrateConfig::set_metadata_for_spec_versions`] to set the metadata to
    ///   use at each spec version we might encounter.
    /// - [`SubstrateConfig::set_spec_version_for_block_ranges`] to set the spec version
    ///   to use for each range of blocks we might encounter.
    pub fn new() -> Self {
        // TODO: Fix this horrible hack because `ChainTypeRegistry` stupidly doesn't
        // have any new/empty constructor, whoops!
        let empty_chain_types = serde_json::json!({ "global": {} });
        let legacy_types: ChainTypeRegistry = serde_json::from_value(empty_chain_types).unwrap();

        Self {
            legacy_types,
            spec_version_for_block_number: RangeMap::empty(),
            metadata_for_spec_version: Mutex::new(HashMap::new()),
        }
    }

    /// Set the legacy types to use for this configuration. This enables support for
    /// blocks produced by Runtimes that emit metadata older than V14.
    pub fn set_legacy_types(mut self, legacy_types: ChainTypeRegistry) -> Self {
        self.legacy_types = legacy_types;
        self
    }

    /// Set the metadata to be used for decoding blocks at the given spec versions.
    pub fn set_metadata_for_spec_versions(self, ranges: impl Iterator<Item = (u32, frame_metadata::RuntimeMetadata)>) -> Self {
        let mut map = self.metadata_for_spec_version.lock().unwrap();
        for (spec_version, metadata) in ranges {
            map.insert(spec_version, Arc::new(metadata));
        }
        drop(map);
        self
    }

    /// Given an iterator of block ranges to spec version of the form `(start, end, spec_version)`, add them
    /// to this configuration.
    pub fn set_spec_version_for_block_ranges(mut self, ranges: impl Iterator<Item = (u64, u64, u32)>) -> Self {
        let mut m = RangeMap::builder();
        for (start, end, spec_version) in ranges {
            m = m.add_range(start, end, spec_version);
        }
        self.spec_version_for_block_number = m.build();
        self
    }
}

impl Config for SubstrateConfig {
    type Hash = H256;

    fn legacy_types_for_spec_version(&'_ self, spec_version: u32) -> TypeRegistrySet<'_> {
        self.legacy_types.for_spec_version(spec_version as u64)
    }

    fn spec_version_for_block_number(&self, block_number: u64) -> Option<u32> {
        self.spec_version_for_block_number.get(block_number).copied()
    }

    fn metadata_for_spec_version(&self, spec_version: u32) -> Option<Arc<frame_metadata::RuntimeMetadata>> {
        self.metadata_for_spec_version
            .lock()
            .unwrap()
            .get(&spec_version)
            .map(|metadata| metadata.clone())
    }

    fn set_metadata_for_spec_version(
        &self,
        spec_version: u32,
        metadata: frame_metadata::RuntimeMetadata,
    ) -> Arc<frame_metadata::RuntimeMetadata> {
        self.metadata_for_spec_version.lock().unwrap().insert(spec_version, Arc::new(metadata));
        self.metadata_for_spec_version(spec_version).expect("We just inserted this metadata, so it should be available")
    }

    fn hash(s: &[u8]) -> <Self as Config>::Hash {
        sp_crypto_hashing::blake2_256(s).into()
    }
}

impl subxt_rpcs::RpcConfig for SubstrateConfig {
    type Hash = <Self as Config>::Hash;
    // We don't use these types in any of the RPC methods we call,
    // so don't bother setting them up:
    type Header = ();
    type AccountId = ();
}