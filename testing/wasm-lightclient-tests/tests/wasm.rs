#![cfg(target_arch = "wasm32")]

use futures_util::StreamExt;
use subxt::{client::OnlineClient, config::PolkadotConfig, lightclient::LightClient};
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Run the tests by calling:
//
// ```text
// RUST_LOG=info WASM_BINDGEN_TEST_TIMEOUT=300 wasm-pack test --headless --chrome
// ```
//
// Use the following to enable logs:
// ```
//  console_error_panic_hook::set_once();
//  tracing_wasm::set_as_global_default();
// ```

const POLKADOT_SPEC: &str = include_str!("../../../artifacts/demo_chain_specs/polkadot.json");

#[wasm_bindgen_test]
async fn light_client_works() {
    // Create a new LightClient based client. This uses the same chainSpec as our
    // native light client tests. If we hit any issues then we probably need to update our
    // chain spec so that the light client is uptodate enough not to need to sync lots
    // (which can lead to DisconnectedWillReconnect errors as we try subscribing to
    // blocks but Smoldot hasn't finished syncing yet).
    let api = {
        let (_lc, rpc) = LightClient::relay_chain(POLKADOT_SPEC)
            .expect("Should be able to run LightClient::relay_chain");
        OnlineClient::<PolkadotConfig>::from_rpc_client(rpc).await.unwrap()
    };

    let begin_time = web_time::Instant::now();
    let mut blocks_sub = api
        .stream_blocks()
        .await
        .expect("Cannot subscribe to finalized blocks")
        .take(3);

    // For each block, print information about it:
    while let Some(block) = blocks_sub.next().await {
        let block = block.unwrap();
        let block_number = block.header().number;
        let block_hash = block.hash();

        tracing::info!("Block #{block_number}:");
        tracing::info!("  Hash: {block_hash}");
    }
}
