use subxt::config::polkadot::PolkadotExtrinsicParamsBuilder as Params;
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
    let tx = polkadot::tx().balances().transfer_allow_death(dest, 10_000);

    let latest_block = api.blocks().at_latest().await?;

    // Configure the transaction parameters; we give a small tip and set the
    // transaction to live for 32 blocks from the `latest_block` above.
    let tx_params = Params::new()
        .tip(1_000)
        .mortal(latest_block.header(), 32)
        .build();

    // submit the transaction:
    let from = dev::alice();
    let hash = api.tx().sign_and_submit(&tx, &from, tx_params).await?;
    println!("Balance transfer extrinsic submitted with hash : {hash}");

    Ok(())
}
