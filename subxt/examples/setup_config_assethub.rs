#![allow(missing_docs)]
use subxt::config::{
    Config, DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder, PolkadotConfig, SubstrateConfig,
};
use subxt_signer::sr25519::dev;

#[subxt::subxt(
    runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
    derive_for_type(
        path = "staging_xcm::v3::multilocation::MultiLocation",
        derive = "Clone",
        recursive
    )
)]
pub mod runtime {}
use runtime::runtime_types::staging_xcm::v3::multilocation::MultiLocation;
use runtime::runtime_types::xcm::v3::junctions::Junctions;

// We don't need to construct this at runtime, so an empty enum is appropriate.
pub enum AssetHubConfig {}

impl Config for AssetHubConfig {
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = DefaultExtrinsicParams<AssetHubConfig>;
    // Here we use the MultiLocation from the metadata as a part of the config:
    // The `ChargeAssetTxPayment` signed extension that is part of the ExtrinsicParams above, now uses the type:
    type AssetId = MultiLocation;
}

#[tokio::main]
async fn main() {
    // With the config defined, we can create an extrinsic with subxt:
    let client = subxt::OnlineClient::<AssetHubConfig>::new().await.unwrap();
    let tx_payload = runtime::tx().system().remark(b"Hello".to_vec());

    // Build extrinsic params using an asset at this location as a tip:
    let location: MultiLocation = MultiLocation {
        parents: 3,
        interior: Junctions::Here,
    };
    let tx_config = DefaultExtrinsicParamsBuilder::<AssetHubConfig>::new()
        .tip_of(1234, location)
        .build();

    // And provide the extrinsic params including the tip when submitting a transaction:
    let _ = client
        .tx()
        .sign_and_submit_then_watch(&tx_payload, &dev::alice(), tx_config)
        .await;
}
