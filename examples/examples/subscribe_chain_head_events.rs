// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot 0.9.29-41a9d84b152.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.29/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use std::collections::HashMap;

use subxt::{
    rpc::types::FollowEvent,
    OnlineClient,
    PolkadotConfig,
};
use tokio::time::Instant;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api =
        OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    // Start the chainHead subscription without runtime information.
    let mut chain_head = api.rpc().chainhead_unstable_follow(false).await?;

    let subscription_id = chain_head
        .subscription_id()
        .expect("A subscription ID must be provided")
        .to_string();

    let mut best_blocks = HashMap::new();

    while let Some(event) = chain_head.next().await {
        let event = event?;

        match event {
            // Drain the initialized event.
            FollowEvent::Initialized(init) => {
                let _res = api
                    .rpc()
                    .chainhead_unstable_unpin(
                        subscription_id.clone(),
                        init.finalized_block_hash,
                    )
                    .await;
            }
            FollowEvent::NewBlock(_) => continue,
            FollowEvent::BestBlockChanged(best_block) => {
                let Some(header) = api.rpc().header(Some(best_block.best_block_hash)).await? else {
                    // Expected substrate to provide a valid header.
                    continue
                };

                // Insert the best block for book keeping.
                best_blocks
                    .insert(best_block.best_block_hash, (header.number, Instant::now()));
            }
            FollowEvent::Finalized(finalized) => {
                let now = Instant::now();

                for finalized_hash in finalized.finalized_block_hashes.iter() {
                    let Some((block_number, best_time)) =
                        best_blocks.remove(finalized_hash) else {
                            continue
                    };

                    println!(
                        "Finalization lag: {:?}ms for block #{} with hash {:?}",
                        now.checked_duration_since(best_time)
                            .map(|duration| duration.as_millis()),
                        block_number,
                        finalized_hash,
                    );
                }

                for hash in finalized
                    .finalized_block_hashes
                    .iter()
                    .chain(finalized.pruned_block_hashes.iter())
                {
                    let _res = api
                        .rpc()
                        .chainhead_unstable_unpin(subscription_id.clone(), *hash)
                        .await;
                }
            }
            FollowEvent::Stop => break,
        }
    }

    Ok(())
}
