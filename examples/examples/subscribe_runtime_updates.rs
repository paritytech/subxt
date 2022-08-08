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

use std::time::Duration;
use subxt::{
    OnlineClient,
    PolkadotConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Start a new tokio task to perform the runtime updates while
    // utilizing the API for other use cases.
    let update_client = api.subscribe_to_updates();
    tokio::spawn(async move {
        let result = update_client.perform_runtime_updates().await;
        println!("Runtime update failed with result={:?}", result);
    });

    // If this client is kept in use a while, it'll update its metadata and such
    // as needed when the node it's pointed at updates.
    tokio::time::sleep(Duration::from_secs(10_000)).await;

    Ok(())
}
