use sp_core::hexdisplay::AsBytesRef;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::tx::Signer;
use subxt::utils::AccountId32;
use subxt_signer::ecdsa::dev;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a balance transfer extrinsic.
    let dest = dev::dave().public_key().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 3_210_000);

    // Submit the balance transfer extrinsic from Charlie, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = dev::charlie();
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
