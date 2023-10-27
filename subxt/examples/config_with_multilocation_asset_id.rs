//! This example just shows how to use MultiLocation as the type of the AssetId in the subxt Config. Some chains may use something like this.

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale")]
pub mod polkadot {}

use subxt::{config::DefaultExtrinsicParams, Config, SubstrateConfig};

enum MyConfig {}

impl Config for MyConfig {
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = DefaultExtrinsicParams<Self>;
    type AssetId = polkadot::runtime_types::xcm::v2::multilocation::MultiLocation;
}

pub fn main() {}
