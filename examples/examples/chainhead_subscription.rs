// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot v0.9.28-9ffe6e9e3da.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.28/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use codec::Encode;
use futures::StreamExt;
use sp_keyring::AccountKeyring;
use std::time::Duration;
use subxt::{
    OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let genesis = api.rpc().chainhead_genesis_hash().await?;
    println!("Genesis: {:?}", genesis);

    // TODO: spec v2 no longer uses the `pipe_from_stream`, therefore we need some
    // time between two sequencial `chainHead_follow` subscription to propagate
    // the `chainHead_unfollow` from the first subscription.
    tokio::time::sleep(Duration::from_secs(5)).await;

    let mut follow_sub = api.blocks().subscribe_chainhead_finalized(true).await?;
    println!("Following subscription...");

    // Handle all subscriptions from the `chainHead_follow`.
    while let Some(block) = follow_sub.next().await {
        let block = block?;

        let body = block.body().await?;
        println!("[hash={:?}] body={:?}", block.hash(), body);

        let header = block.header().await?;
        println!("[hash={:?}] header={:?}", block.hash(), header);

        let active_era_addr = polkadot::storage().staking().active_era();
        let era = block.storage(&active_era_addr).await?.unwrap();
        println!(
            "[hash={:?}] storage index: {:?}, start: {:?}",
            block.hash(),
            era.index,
            era.start
        );

        let call_params = AccountKeyring::Alice.to_account_id().encode();
        let call = block
            .call("AccountNonceApi_account_nonce".into(), Some(&call_params))
            .await?;
        println!("[hash={:?}] call={:?}", block.hash(), call);

        println!("Events:");
        let events = block.events().await?;
        for event in events.iter() {
            let event = event?;
            let pallet = event.pallet_name();
            let variant = event.variant_name();
            println!("    {pallet}::{variant} event");
        }

        // Unpin the block as last step.
        block.unpin().await?;
    }

    Ok(())
}
