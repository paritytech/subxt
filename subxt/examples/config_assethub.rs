//! Configuring Subxt to talk to AssetHub.
use subxt::Error;
use subxt::config::{
    Config, DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder, PolkadotConfig, SubstrateConfig,
};
use subxt_signer::sr25519::dev;

#[subxt::subxt(
    runtime_metadata_path = "../artifacts/polkadot_assethub_metadata_small.scale",
    derive_for_type(
        path = "staging_xcm::v5::location::Location",
        derive = "Clone, codec::Encode",
        recursive
    )
)]
pub mod assethub {}
use assethub::runtime_types::staging_xcm::v5::junctions::Junctions;
use assethub::runtime_types::staging_xcm::v5::location::Location;

/// Our AssetHub configuration wraps SubstrateConfig and
/// tweaks it in a couple of places.
#[derive(Debug, Clone, Default)]
pub struct AssetHubConfig(SubstrateConfig);

impl Config for AssetHubConfig {
    // AssetHub, like Polkadot, has no account index on its address type:
    type Address = <PolkadotConfig as Config>::Address;

    // Configure the asset location to be our generated Location type.
    type AssetId = Location;

    // Just copy the default SubstrateConfig for these:
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = DefaultExtrinsicParams<AssetHubConfig>;

    // Forward these methods to the default SubstrateConfig:
    fn genesis_hash(&self) -> Option<subxt::config::HashFor<Self>> {
        self.0.genesis_hash()
    }
    fn legacy_types_for_spec_version<'this>(
        &'this self,
        spec_version: u32,
    ) -> Option<scale_info_legacy::TypeRegistrySet<'this>> {
        self.0.legacy_types_for_spec_version(spec_version)
    }
    fn metadata_for_spec_version(&self, spec_version: u32) -> Option<subxt::ArcMetadata> {
        self.0.metadata_for_spec_version(spec_version)
    }
    fn set_metadata_for_spec_version(&self, spec_version: u32, metadata: subxt::ArcMetadata) {
        self.0.set_metadata_for_spec_version(spec_version, metadata);
    }
    fn spec_and_transaction_version_for_block_number(
        &self,
        block_number: u64,
    ) -> Option<(u32, u32)> {
        self.0
            .spec_and_transaction_version_for_block_number(block_number)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // With the config defined, we can create a Subxt client using it:
    let client = subxt::OnlineClient::<AssetHubConfig>::new().await?;

    // Build some extrinsic to submit:
    let tx_payload = assethub::tx().system().remark(b"Hello".to_vec());

    // Build extrinsic params using an asset at some location as a tip:
    let location = Location {
        parents: 3,
        interior: Junctions::Here,
    };
    let tx_config = DefaultExtrinsicParamsBuilder::<AssetHubConfig>::new()
        .tip_of(1234, location)
        .build();

    // And provide the extrinsic params including the tip when submitting a transaction:
    let _ = client
        .tx()
        .await?
        .sign_and_submit_then_watch(&tx_payload, &dev::alice(), tx_config)
        .await;

    Ok(())
}
