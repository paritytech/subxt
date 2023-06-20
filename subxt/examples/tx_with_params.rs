use subxt::config::polkadot::{Era, PlainTip, PolkadotExtrinsicParamsBuilder as Params};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a balance transfer extrinsic.
    let dest = dev::bob().public_key().into();
    let tx = polkadot::tx().balances().transfer(dest, 10_000);

    // Configure the transaction parameters; for Polkadot the tip and era:
    let tx_params = Params::new()
        .tip(PlainTip::new(1_000))
        .era(Era::Immortal, api.genesis_hash());

    // submit the transaction:
    let from = dev::alice();
    let hash = api.tx().sign_and_submit(&tx, &from, tx_params).await?;
    println!("Balance transfer extrinsic submitted with hash : {hash}");

    Ok(())
}
