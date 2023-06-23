#![cfg(target_arch = "wasm32")]

use subxt::{config::PolkadotConfig,
    client::{LightClient, OfflineClientT, LightClientBuilder},
};
use futures_util::StreamExt;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Run the tests by calling:
//
// ```text
// wasm-pack test --firefox --headless`
// ```
//
// You'll need to have a substrate/polkadot node running:
//
// ```bash
// # Polkadot does not accept by default WebSocket connections to the P2P network.
// # Ensure `--listen-addr` is provided with valid ws adddress endpoint.
// # The `--node-key` provides a deterministic p2p address for the node.
// ./polkadot --dev --node-key 0000000000000000000000000000000000000000000000000000000000000001 --listen-addr /ip4/0.0.0.0/tcp/30333/ws
// ```
//

#[wasm_bindgen_test]
async fn wasm_ws_transport_works() {
    let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
        .await
        .unwrap();

    let chain = client.rpc().system_chain().await.unwrap();
    assert_eq!(&chain, "Development");
}

#[wasm_bindgen_test]
async fn light_client_works() {
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();

    let api: LightClient<PolkadotConfig> = LightClientBuilder::new()
        .bootnodes(
            ["/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"]
                .into_iter(),
        )
        .build_from_url("ws://127.0.0.1:9944")
        .await
        .unwrap();

    tracing::info!("Subscribe to latest finalized blocks: ");

    let mut blocks_sub = api.blocks().subscribe_finalized().await.expect("Cannot subscribe to finalized hashes").take(3);
    // For each block, print a bunch of information about it:
    while let Some(block) = blocks_sub.next().await {
        let block = block.expect("Block not valid");

        let block_number = block.header().number;
        let block_hash = block.hash();

        tracing::info!("Block #{block_number}:");
        tracing::info!("  Hash: {block_hash}");
    }
}
