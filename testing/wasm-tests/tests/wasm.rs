#![cfg(target_arch = "wasm32")]

use subxt::config::PolkadotConfig;
use subxt::rpc::LightClient;
use wasm_bindgen_test::*;
use std::sync::Arc;

use subxt::rpc::Subscription;
use subxt::rpc::RpcClientT;
use subxt::OnlineClient;
use futures_util::StreamExt;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Run the tests by `$ wasm-pack test --firefox --headless`

fn init_tracing() {
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();
}

#[wasm_bindgen_test]
async fn wasm_ws_transport_works() {
    let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
        .await
        .unwrap();

    let chain = client.rpc().system_chain().await.unwrap();
    assert_eq!(&chain, "Development");
}

#[wasm_bindgen_test]
async fn light_client_transport_works() {
	init_tracing();

    let light_client = LightClient::new(include_str!("../../../artifacts/dev_spec.json")).unwrap();
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(Arc::new(light_client)).await.unwrap();

    // Subscribe to the latest 3 finalized blocks.
    {
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

 
    // Ensure light-client functions with subscriptions.
    // let sub = light_client
    // .subscribe_raw("chain_subscribeAllHeads", None, "chain_unsubscribeAllHeads")
    // .await
    // .unwrap();
    // The subscription result is actually a PolkadotConfig::Header, we are interested in iteration.
    // let mut sub: Subscription<String> = Subscription::new(sub);

    // let block = sub
    //     .next()
    //     .await
    //     .expect("Subscription failed")
    //     .expect("Subscription ended");
    //     tracing::warn!("Block hash {:?}", block);

    // let block = sub
    //     .next()
    //     .await
    //     .expect("Subscription failed")
    //     .expect("Subscription ended");
    //     tracing::warn!("Block hash {:?}", block);

    // let chain = client.rpc().system_chain().await.unwrap();
    // assert_eq!(&chain, "Development");
}
