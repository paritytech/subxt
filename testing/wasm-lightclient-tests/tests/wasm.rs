#![cfg(target_arch = "wasm32")]

use futures_util::StreamExt;
use subxt::{
    client::{LightClient, LightClientBuilder},
    config::PolkadotConfig,
};
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Run the tests by calling:
//
// ```text
// wasm-pack test --firefox --headless
// ```
//
// Use the following to enable logs:
// ```
//  console_error_panic_hook::set_once();
//  tracing_wasm::set_as_global_default();
// ```

#[wasm_bindgen_test]
async fn light_client_works() {
    let api = connect_to_rpc_node().await;

    tracing::info!("Subscribe to latest finalized blocks: ");

    let mut blocks_sub = api
        .blocks()
        .subscribe_finalized()
        .await
        .expect("Cannot subscribe to finalized hashes")
        .take(3);

    // For each block, print information about it:
    while let Some(block) = blocks_sub.next().await {
        let block = block.expect("Block not valid");

        let block_number = block.header().number;
        let block_hash = block.hash();

        tracing::info!("Block #{block_number}:");
        tracing::info!("  Hash: {block_hash}");
    }
}

/// We connect to an RPC node because the light client can struggle to sync in
/// time to a new local node for some reason. Because this can be brittle (eg RPC nodes can
/// go down or have network issues), we try a few RPC nodes until we find one that works.
async fn connect_to_rpc_node() -> LightClient<PolkadotConfig> {
    let rpc_node_urls = [
        "wss://rpc.polkadot.io",
        "wss://1rpc.io/dot",
        "wss://polkadot-public-rpc.blockops.network/ws",
    ];

    for url in rpc_node_urls {
        let res = LightClientBuilder::new().build_from_url(url).await;

        match res {
            Ok(api) => return api,
            Err(e) => tracing::warn!("Error connecting to RPC node {url}: {e}"),
        }
    }

    panic!("Could not connect to any RPC node")
}
