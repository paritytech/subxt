use sp_keyring::AccountKeyring;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a balance transfer extrinsic.
    let dest = AccountKeyring::Bob.to_account_id().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 10_000);

    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = PairSigner::new(AccountKeyring::Alice.pair());
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
