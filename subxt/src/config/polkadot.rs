// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Polkadot specific configuration

use super::{Config, DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder};

use crate::config::substrate::{SubstrateConfig, SubstrateConfigBuilder};
use crate::metadata::ArcMetadata;
use scale_info_legacy::TypeRegistrySet;

pub use crate::config::substrate::{SpecVersionForRange, SubstrateHeader};
pub use crate::utils::{AccountId32, MultiAddress, MultiSignature};
pub use primitive_types::{H256, U256};

/// Construct a [`PolkadotConfig`] using this.
pub struct PolkadotConfigBuilder(SubstrateConfigBuilder);

impl Default for PolkadotConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PolkadotConfigBuilder {
    /// Create a new [`PolkadotConfigBuilder`].
    pub fn new() -> Self {
        let inner = SubstrateConfigBuilder::new()
            .set_legacy_types(frame_decode::legacy_types::polkadot::relay_chain());

        PolkadotConfigBuilder(inner)
    }

    /// Set the metadata to be used for decoding blocks at the given spec versions.
    pub fn set_metadata_for_spec_versions(
        mut self,
        ranges: impl IntoIterator<Item = (u32, ArcMetadata)>,
    ) -> Self {
        self = Self(self.0.set_metadata_for_spec_versions(ranges));
        self
    }

    /// Given an iterator of block ranges to spec version of the form `(start, end, spec_version)`, add them
    /// to this configuration.
    pub fn set_spec_version_for_block_ranges(
        mut self,
        ranges: impl IntoIterator<Item = SpecVersionForRange>,
    ) -> Self {
        self = Self(self.0.set_spec_version_for_block_ranges(ranges));
        self
    }

    /// Construct the [`PolkadotConfig`] from this builder.
    pub fn build(self) -> PolkadotConfig {
        PolkadotConfig(self.0.build())
    }
}

/// Configuration that's suitable for the Polkadot Relay Chain.
#[derive(Debug, Clone)]
pub struct PolkadotConfig(SubstrateConfig);

impl PolkadotConfig {
    /// Create a new, default, [`PolkadotConfig`].
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Build a new [`PolkadotConfig`].
    pub fn builder() -> PolkadotConfigBuilder {
        PolkadotConfigBuilder(SubstrateConfig::builder())
    }
}

impl Config for PolkadotConfig {
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type AssetId = <SubstrateConfig as Config>::AssetId;

    // Address on Polkadot has no account index, whereas it's u32 on
    // the default substrate dev node.
    type Address = MultiAddress<Self::AccountId, ()>;

    // These are the same as the default substrate node, but redefined
    // because we need to pass the PolkadotConfig trait as a param.
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;

    fn genesis_hash(&self) -> Option<super::HashFor<Self>> {
        self.0.genesis_hash()
    }

    fn legacy_types_for_spec_version(&'_ self, spec_version: u32) -> Option<TypeRegistrySet<'_>> {
        self.0.legacy_types_for_spec_version(spec_version)
    }

    fn spec_and_transaction_version_for_block_number(
        &self,
        block_number: u64,
    ) -> Option<(u32, u32)> {
        self.0
            .spec_and_transaction_version_for_block_number(block_number)
    }

    fn metadata_for_spec_version(&self, spec_version: u32) -> Option<ArcMetadata> {
        self.0.metadata_for_spec_version(spec_version)
    }

    fn set_metadata_for_spec_version(&self, spec_version: u32, metadata: ArcMetadata) {
        self.0.set_metadata_for_spec_version(spec_version, metadata)
    }
}

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type PolkadotExtrinsicParams<T> = DefaultExtrinsicParams<T>;

/// A builder which leads to [`PolkadotExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type PolkadotExtrinsicParamsBuilder<T> = DefaultExtrinsicParamsBuilder<T>;
