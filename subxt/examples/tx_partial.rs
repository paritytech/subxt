#![allow(missing_docs)]
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    // Spawned tasks require things held across await points to impl Send,
    // so we use one to demonstrate that this is possible with `PartialExtrinsic`
    tokio::spawn(signing_example()).await??;
    Ok(())
}

async fn signing_example() -> Result<(), BoxedError> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a balance transfer extrinsic.
    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer_allow_death(dest, 10_000);

    let alice = dev::alice();

    // Create partial tx, ready to be signed.
    let partial_tx = api
        .tx()
        .create_partial_signed(
            &balance_transfer_tx,
            &alice.public_key().to_account_id(),
            Default::default(),
        )
        .await?;

    // Simulate taking some time to get a signature back, in part to
    // show that the `PartialExtrinsic` can be held across await points.
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let signature = alice.sign(&partial_tx.signer_payload());

    // Sign the transaction.
    let tx = partial_tx
        .sign_with_address_and_signature(&alice.public_key().to_address(), &signature.into());

    // Submit it.
    tx.submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?;

    Ok(())
}
