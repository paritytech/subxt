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

    // Subscribe to several balance related events. If we ask for more than one event,
    // we'll be given a correpsonding tuple of `Option`'s, with exactly one
    // variant populated each time.
    let mut balance_events = api.events().subscribe().await?.filter_events::<(
        polkadot::balances::events::Withdraw,
        polkadot::balances::events::Transfer,
        polkadot::balances::events::Deposit,
    )>();

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

    // Our subscription will see all of the balance events we're filtering on:
    while let Some(ev) = balance_events.next().await {
        let event_details = ev?;

        let block_hash = event_details.block_hash;
        let event = event_details.event;
        println!("Event at {:?}:", block_hash);

        if let (Some(withdraw), _, _) = &event {
            println!("  Withdraw event: {withdraw:?}");
        }
        if let (_, Some(transfer), _) = &event {
            println!("  Transfer event: {transfer:?}");
        }
        if let (_, _, Some(deposit)) = &event {
            println!("  Deposit event: {deposit:?}");
        }
    }

    Ok(())
}
