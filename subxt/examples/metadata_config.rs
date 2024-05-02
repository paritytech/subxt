#![allow(missing_docs)]
use subxt::{OnlineClient, SubstrateConfig};
use subxt_core::config::{Config, DefaultExtrinsicParams};
use subxt_signer::sr25519::dev;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_insecure_url = "ws://localhost:9999")]
pub mod polkadot {}

// Derives aren't strictly needed, they just make developer life easier.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MetadataConfig {}

impl Config for MetadataConfig {
    // Extracted from metadata directly:
    type Hash = polkadot::custom_types::Hash;
    type AccountId = polkadot::custom_types::AccountId;
    type AssetId = polkadot::custom_types::AssetId;
    type Address = polkadot::custom_types::Address;

    // Present in metadata but this PoC needs to add
    // fn specific per name of type to impl the hashing fn.
    type Hasher = <SubstrateConfig as Config>::Hasher;
    // Present in metadata but this PoC needs to impl the header
    // trait to make use of this.
    type Header = <SubstrateConfig as Config>::Header;
    // Same story, present in md but needs subxt::tx::Signer<T>.
    // type Signature = polkadot::custom_types::Signature;
    type Signature = <SubstrateConfig as Config>::Signature;

    // Not exposed in metadata, seems like heavily involved with
    // code functionality which cannot safely be expressed in the
    // metadata.
    type ExtrinsicParams = DefaultExtrinsicParams<Self>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to nodes.
    let api = OnlineClient::<MetadataConfig>::from_insecure_url("ws://localhost:9999").await?;

    // Build a balance transfer extrinsic.
    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer_allow_death(dest, 10_000);

    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Find a Transfer event and print it.
    let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }

    Ok(())
}
