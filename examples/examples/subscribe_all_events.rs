// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot polkadot 0.9.25-5174e9ae75b.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.25/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use futures::StreamExt;
use sp_keyring::AccountKeyring;
use std::time::Duration;
use subxt::{
    tx::PairSigner,
    OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

/// Subscribe to all events, and then manually look through them and
/// pluck out the events that we care about.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Subscribe to any events that occur:
    let mut event_sub = api.events().subscribe().await?;

    // While this subscription is active, balance transfers are made somewhere:
    tokio::task::spawn({
        let api = api.clone();
        async move {
            let signer = PairSigner::new(AccountKeyring::Alice.pair());
            let mut transfer_amount = 1_000_000_000;

            // Make small balance transfers from Alice to Bob in a loop:
            loop {
                let transfer_tx = polkadot::tx().balances().transfer(
                    AccountKeyring::Bob.to_account_id().into(),
                    transfer_amount,
                );
                api.tx()
                    .sign_and_submit_default(&transfer_tx, &signer)
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(10)).await;
                transfer_amount += 100_000_000;
            }
        }
    });

    // Our subscription will see the events emitted as a result of this:
    while let Some(events) = event_sub.next().await {
        let events = events?;
        let block_hash = events.block_hash();

        // We can dynamically decode events:
        println!("  Dynamic event details: {block_hash:?}:");
        for event in events.iter() {
            let event = event?;
            let is_balance_transfer = event
                .as_event::<polkadot::balances::events::Transfer>()?
                .is_some();
            let pallet = event.pallet_name();
            let variant = event.variant_name();
            println!(
                "    {pallet}::{variant} (is balance transfer? {is_balance_transfer})"
            );
        }

        // Or we can find the first transfer event, ignoring any others:
        let transfer_event =
            events.find_first::<polkadot::balances::events::Transfer>()?;

        if let Some(ev) = transfer_event {
            println!("  - Balance transfer success: value: {:?}", ev.amount);
        } else {
            println!("  - No balance transfer event found in this block");
        }
    }

    Ok(())
}
