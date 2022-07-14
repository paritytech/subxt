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

use subxt::{
    OnlineClient,
    PolkadotConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let block_number = 1u32;

    let block_hash = api.rpc().block_hash(Some(block_number.into())).await?;

    if let Some(hash) = block_hash {
        println!("Block hash for block number {block_number}: {hash}");
    } else {
        println!("Block number {block_number} not found.");
    }

    Ok(())
}
