// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot 0.9.18-f6d6ab005d-aarch64-macos.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use subxt::{
    rpc::Subscription,
    sp_runtime::{
        generic::Header,
        traits::BlakeTwo256,
    },
    ClientBuilder,
    DefaultConfig,
    PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = ClientBuilder::new()
        .set_url("wss://rpc.polkadot.io:443")
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    // For non-finalised blocks use `.subscribe_blocks()`
    let mut blocks: Subscription<Header<u32, BlakeTwo256>> =
        api.client.rpc().subscribe_finalized_blocks().await?;

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
