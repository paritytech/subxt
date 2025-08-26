use super::Config;
use super::SubstrateConfig;
use scale_info_legacy::{ChainTypeRegistry, TypeRegistrySet};
use std::sync::Arc;

/// Configuration that's suitable for the Polkadot Relay Chain
pub struct PolkadotConfig(SubstrateConfig);

impl PolkadotConfig {
    /// Create a new PolkadotConfig.
    pub fn new() -> Self {
        let config = SubstrateConfig::new()
            .set_legacy_types(frame_decode::legacy_types::polkadot::relay_chain());

        // TODO: Set spec versions as well with known spec version changes, to speed
        // up accessing historic blocks within the known ranges. For now, we just let
        // the online client look these up on chain.

        Self(config)
    }

    /// Set the metadata to be used for decoding blocks at the given spec versions.
    pub fn set_metadata_for_spec_versions(
        mut self,
        ranges: impl Iterator<Item = (u32, frame_metadata::RuntimeMetadata)>,
    ) -> Self {
        self = Self(self.0.set_metadata_for_spec_versions(ranges));
        self
    }

    /// Given an iterator of block ranges to spec version of the form `(start, end, spec_version)`, add them
    /// to this configuration.
    pub fn set_spec_version_for_block_ranges(
        mut self,
        ranges: impl Iterator<Item = (u64, u64, u32)>,
    ) -> Self {
        self = Self(self.0.set_spec_version_for_block_ranges(ranges));
        self
    }
}

/// This hands back the legacy types for the Polkadot Relay Chain, which is what [`PolkadotConfig`] uses internally.
pub fn legacy_types() -> ChainTypeRegistry {
    frame_decode::legacy_types::polkadot::relay_chain()
}

impl Default for PolkadotConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Config for PolkadotConfig {
    type Hash = <SubstrateConfig as Config>::Hash;

    fn legacy_types_for_spec_version(&'_ self, spec_version: u32) -> TypeRegistrySet<'_> {
        self.0.legacy_types_for_spec_version(spec_version)
    }

    fn spec_version_for_block_number(&self, block_number: u64) -> Option<u32> {
        self.0.spec_version_for_block_number(block_number)
    }

    fn metadata_for_spec_version(
        &self,
        spec_version: u32,
    ) -> Option<Arc<frame_metadata::RuntimeMetadata>> {
        self.0.metadata_for_spec_version(spec_version)
    }

    fn set_metadata_for_spec_version(
        &self,
        spec_version: u32,
        metadata: Arc<frame_metadata::RuntimeMetadata>,
    ) {
        self.0.set_metadata_for_spec_version(spec_version, metadata)
    }

    fn hash(s: &[u8]) -> <Self as Config>::Hash {
        SubstrateConfig::hash(s)
    }
}

impl subxt_rpcs::RpcConfig for PolkadotConfig {
    type Hash = <SubstrateConfig as subxt_rpcs::RpcConfig>::Hash;
    type Header = <SubstrateConfig as subxt_rpcs::RpcConfig>::Header;
    type AccountId = <SubstrateConfig as subxt_rpcs::RpcConfig>::AccountId;
}
