use futures::StreamExt;
use sp_keyring::AccountKeyring;
use subxt::{tx::{TxStatus, PairSigner}, OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let dest = AccountKeyring::Bob.to_account_id().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 10_000);

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let mut balance_transfer_progress = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &signer)
        .await?;

    while let Some(status) = balance_transfer_progress.next().await {
        match status? {
            // It's finalized in a block!
            TxStatus::Finalized(in_block) => {
                println!(
                    "Transaction {:?} is finalized in block {:?}",
                    in_block.extrinsic_hash(),
                    in_block.block_hash()
                );

                // grab the events and fail if no ExtrinsicSuccess event seen:
                let events = in_block.wait_for_success().await?;
                // We can look for events (this uses the static interface; we can also iterate
                // over them and dynamically decode them):
                let transfer_event = events
                    .find_first::<polkadot::balances::events::Transfer>()?;

                if let Some(event) = transfer_event {
                    println!("Balance transfer success: {event:?}");
                } else {
                    println!("Failed to find Balances::Transfer Event");
                }
            },
            // Just log any other status we encounter:
            other => {
                println!("Status: {other:?}");
            }
        }
    }
    Ok(())
}