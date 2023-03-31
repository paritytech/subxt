// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot v0.9.28-9ffe6e9e3da.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.28/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use subxt::{config::Header, OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    // For non-finalised blocks use `.subscribe_blocks()`
    let mut blocks = api.rpc().subscribe_finalized_block_headers().await?;

    while let Some(Ok(block)) = blocks.next().await {
        println!(
            "block number: {} hash:{} parent:{} state root:{} extrinsics root:{}",
            block.number,
            block.hash(),
            block.parent_hash,
            block.state_root,
            block.extrinsics_root
        );
    }

    Ok(())
}
