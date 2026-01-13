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
pub struct PolkadotConfigBuilder {
    use_historic_types: bool,
    inner: SubstrateConfigBuilder,
}

impl Default for PolkadotConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PolkadotConfigBuilder {
    /// Create a new [`PolkadotConfigBuilder`].
    pub fn new() -> Self {
        let inner = SubstrateConfigBuilder::new();
        PolkadotConfigBuilder {
            use_historic_types: true,
            inner,
        }
    }

    /// Use historic types. These are enabled by default, but can be disabled to avoid
    /// loading them in if they are not needed (ie you do not need to access any block
    /// using a runtime which has V14 metadata).
    pub fn use_historic_types(mut self, b: bool) -> Self {
        self.use_historic_types = b;
        self
    }

    /// Set the metadata to be used for decoding blocks at the given spec versions.
    pub fn set_metadata_for_spec_versions(
        mut self,
        ranges: impl IntoIterator<Item = (u32, ArcMetadata)>,
    ) -> Self {
        self.inner = self.inner.set_metadata_for_spec_versions(ranges);
        self
    }

    /// Use the current "known" spec version information for the Polkadot Relay Chain. For historic blocks in
    /// the known block range, this will avoid needing to check the spec version at each block.
    ///
    /// ## Warning
    ///
    /// If you're connecting to anything other than the **live** Polkadot Relay Chain with this configuration,
    /// enabling this will lead to obscure errors.
    pub fn use_known_spec_versions(self) -> Self {
        self.set_spec_version_for_block_ranges(polkadot_spec_versions_for_block_ranges())
    }

    /// Given an iterator of block ranges to spec version of the form `(start, end, spec_version)`, add them
    /// to this configuration.
    pub fn set_spec_version_for_block_ranges(
        mut self,
        ranges: impl IntoIterator<Item = SpecVersionForRange>,
    ) -> Self {
        self.inner = self.inner.set_spec_version_for_block_ranges(ranges);
        self
    }

    /// Construct the [`PolkadotConfig`] from this builder.
    pub fn build(mut self) -> PolkadotConfig {
        if self.use_historic_types {
            self.inner = self
                .inner
                .set_legacy_types(frame_decode::legacy_types::polkadot::relay_chain());
        }

        PolkadotConfig(self.inner.build())
    }
}

/// Configuration for the Polkadot Relay Chain. This should not be used to connect
/// to any other chain; instead use [`crate::config::SubstrateConfig`] and configure
/// that as needed to support the chain you're connecting to.
#[derive(Debug, Clone)]
pub struct PolkadotConfig(SubstrateConfig);

impl Default for PolkadotConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PolkadotConfig {
    /// Create a new, default, [`PolkadotConfig`].
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Build a new [`PolkadotConfig`].
    pub fn builder() -> PolkadotConfigBuilder {
        PolkadotConfigBuilder::new()
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

/// The known blocks at which the spec version or transaction versions change.
/// This can be used in the [`PolkadotConfig`] to speed things up a little when
/// accessing historic blocks.
fn polkadot_spec_versions_for_block_ranges() -> impl IntoIterator<Item = SpecVersionForRange> {
    #[derive(Clone, Copy)]
    struct ChangeAt {
        block: u64,
        spec_version: u32,
        transaction_version: u32,
    }

    /// A list of entries denoting each block that the spec and transaction versions change.
    const POLKADOT_SPEC_VERSION_BLOCKS: [ChangeAt; 70] = [
        ChangeAt {
            block: 0,
            spec_version: 0,
            transaction_version: 0,
        },
        ChangeAt {
            block: 29231,
            spec_version: 1,
            transaction_version: 0,
        },
        ChangeAt {
            block: 188836,
            spec_version: 5,
            transaction_version: 0,
        },
        ChangeAt {
            block: 199405,
            spec_version: 6,
            transaction_version: 0,
        },
        ChangeAt {
            block: 214264,
            spec_version: 7,
            transaction_version: 0,
        },
        ChangeAt {
            block: 244358,
            spec_version: 8,
            transaction_version: 0,
        },
        ChangeAt {
            block: 303079,
            spec_version: 9,
            transaction_version: 0,
        },
        ChangeAt {
            block: 314201,
            spec_version: 10,
            transaction_version: 0,
        },
        ChangeAt {
            block: 342400,
            spec_version: 11,
            transaction_version: 0,
        },
        ChangeAt {
            block: 443963,
            spec_version: 12,
            transaction_version: 0,
        },
        ChangeAt {
            block: 528470,
            spec_version: 13,
            transaction_version: 2,
        },
        ChangeAt {
            block: 687751,
            spec_version: 14,
            transaction_version: 2,
        },
        ChangeAt {
            block: 746085,
            spec_version: 15,
            transaction_version: 2,
        },
        ChangeAt {
            block: 787923,
            spec_version: 16,
            transaction_version: 2,
        },
        ChangeAt {
            block: 799302,
            spec_version: 17,
            transaction_version: 3,
        },
        ChangeAt {
            block: 1205128,
            spec_version: 18,
            transaction_version: 4,
        },
        ChangeAt {
            block: 1603423,
            spec_version: 23,
            transaction_version: 5,
        },
        ChangeAt {
            block: 1733218,
            spec_version: 24,
            transaction_version: 5,
        },
        ChangeAt {
            block: 2005673,
            spec_version: 25,
            transaction_version: 5,
        },
        ChangeAt {
            block: 2436698,
            spec_version: 26,
            transaction_version: 5,
        },
        ChangeAt {
            block: 3613564,
            spec_version: 27,
            transaction_version: 5,
        },
        ChangeAt {
            block: 3899547,
            spec_version: 28,
            transaction_version: 6,
        },
        ChangeAt {
            block: 4345767,
            spec_version: 29,
            transaction_version: 6,
        },
        ChangeAt {
            block: 4876134,
            spec_version: 30,
            transaction_version: 7,
        },
        ChangeAt {
            block: 5661442,
            spec_version: 9050,
            transaction_version: 7,
        },
        ChangeAt {
            block: 6321619,
            spec_version: 9080,
            transaction_version: 7,
        },
        ChangeAt {
            block: 6713249,
            spec_version: 9090,
            transaction_version: 7,
        },
        ChangeAt {
            block: 7217907,
            spec_version: 9100,
            transaction_version: 8,
        },
        ChangeAt {
            block: 7229126,
            spec_version: 9110,
            transaction_version: 8,
        },
        ChangeAt {
            block: 7560558,
            spec_version: 9122,
            transaction_version: 8,
        },
        ChangeAt {
            block: 8115869,
            spec_version: 9140,
            transaction_version: 9,
        },
        ChangeAt {
            block: 8638103,
            spec_version: 9151,
            transaction_version: 9,
        },
        ChangeAt {
            block: 9280179,
            spec_version: 9170,
            transaction_version: 11,
        },
        ChangeAt {
            block: 9738717,
            spec_version: 9180,
            transaction_version: 12,
        },
        ChangeAt {
            block: 10156856,
            spec_version: 9190,
            transaction_version: 12,
        },
        ChangeAt {
            block: 10458576,
            spec_version: 9200,
            transaction_version: 12,
        },
        ChangeAt {
            block: 10655116,
            spec_version: 9220,
            transaction_version: 12,
        },
        ChangeAt {
            block: 10879371,
            spec_version: 9230,
            transaction_version: 12,
        },
        ChangeAt {
            block: 11328884,
            spec_version: 9250,
            transaction_version: 13,
        },
        ChangeAt {
            block: 11532856,
            spec_version: 9260,
            transaction_version: 13,
        },
        ChangeAt {
            block: 11933818,
            spec_version: 9270,
            transaction_version: 13,
        },
        ChangeAt {
            block: 12217535,
            spec_version: 9280,
            transaction_version: 13,
        },
        ChangeAt {
            block: 12245277,
            spec_version: 9281,
            transaction_version: 13,
        },
        ChangeAt {
            block: 12532644,
            spec_version: 9291,
            transaction_version: 14,
        },
        ChangeAt {
            block: 12876189,
            spec_version: 9300,
            transaction_version: 15,
        },
        ChangeAt {
            block: 13800015,
            spec_version: 9340,
            transaction_version: 18,
        },
        ChangeAt {
            block: 14188833,
            spec_version: 9360,
            transaction_version: 19,
        },
        ChangeAt {
            block: 14543918,
            spec_version: 9370,
            transaction_version: 20,
        },
        ChangeAt {
            block: 15978362,
            spec_version: 9420,
            transaction_version: 23,
        },
        ChangeAt {
            block: 16450000,
            spec_version: 9430,
            transaction_version: 24,
        },
        ChangeAt {
            block: 17840000,
            spec_version: 9431,
            transaction_version: 24,
        },
        ChangeAt {
            block: 18407475,
            spec_version: 1000001,
            transaction_version: 24,
        },
        ChangeAt {
            block: 19551000,
            spec_version: 1001002,
            transaction_version: 25,
        },
        ChangeAt {
            block: 20181758,
            spec_version: 1001003,
            transaction_version: 25,
        },
        ChangeAt {
            block: 20438530,
            spec_version: 1002000,
            transaction_version: 25,
        },
        ChangeAt {
            block: 21169168,
            spec_version: 1002004,
            transaction_version: 25,
        },
        ChangeAt {
            block: 21455374,
            spec_version: 1002005,
            transaction_version: 26,
        },
        ChangeAt {
            block: 21558004,
            spec_version: 1002006,
            transaction_version: 26,
        },
        ChangeAt {
            block: 21800141,
            spec_version: 1002007,
            transaction_version: 26,
        },
        ChangeAt {
            block: 22572435,
            spec_version: 1003000,
            transaction_version: 26,
        },
        ChangeAt {
            block: 22975676,
            spec_version: 1003003,
            transaction_version: 26,
        },
        ChangeAt {
            block: 23463101,
            spec_version: 1003004,
            transaction_version: 26,
        },
        ChangeAt {
            block: 24899777,
            spec_version: 1004000,
            transaction_version: 26,
        },
        ChangeAt {
            block: 25005483,
            spec_version: 1004001,
            transaction_version: 26,
        },
        ChangeAt {
            block: 26170985,
            spec_version: 1005001,
            transaction_version: 26,
        },
        ChangeAt {
            block: 26902698,
            spec_version: 1006001,
            transaction_version: 26,
        },
        ChangeAt {
            block: 27707460,
            spec_version: 1006002,
            transaction_version: 26,
        },
        ChangeAt {
            block: 27994522,
            spec_version: 1007001,
            transaction_version: 26,
        },
        ChangeAt {
            block: 28476903,
            spec_version: 2000000,
            transaction_version: 26,
        },
        ChangeAt {
            block: 28524511,
            spec_version: 2000001,
            transaction_version: 26,
        },
    ];

    POLKADOT_SPEC_VERSION_BLOCKS.windows(2).map(|window| {
        let last = window[0];
        let current = window[1];
        SpecVersionForRange {
            block_range: last.block..current.block,
            spec_version: last.spec_version,
            transaction_version: last.transaction_version,
        }
    })
}
