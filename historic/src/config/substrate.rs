use primitive_types::H256;
use scale_info_legacy::{ChainTypeRegistry, TypeRegistrySet};
use super::Config;

pub struct SubstrateConfig {
    legacy_types: ChainTypeRegistry,
}

impl Config for SubstrateConfig {
    type Hash = H256;
    type LegacyTypes<'a> = TypeRegistrySet<'a>;

    fn legacy_types_for_spec_version(&'_ self, spec_version: u32) -> Self::LegacyTypes<'_> {
        self.legacy_types.for_spec_version(spec_version as u64)
    }
}

impl subxt_rpcs::RpcConfig for SubstrateConfig {
    type Hash = <Self as Config>::Hash;
    // We don't use these types in any of the RPC methods we call,
    // so don't bother setting them up:
    type Header = ();
    type AccountId = ();
}