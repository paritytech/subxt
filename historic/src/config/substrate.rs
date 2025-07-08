use primitive_types::H256;
use scale_info_legacy::{ChainTypeRegistry, TypeRegistrySet};
use super::Config;

pub struct SubstrateConfig {
    legacy_types: ChainTypeRegistry,
}

impl Config for SubstrateConfig {
    type Hash = H256;
    type LegacyTypes<'a> = TypeRegistrySet<'a>;

    fn legacy_types_for_spec_version(&'_ self, spec_version: u64) -> Self::LegacyTypes<'_> {
        self.legacy_types.for_spec_version(spec_version)
    }
}