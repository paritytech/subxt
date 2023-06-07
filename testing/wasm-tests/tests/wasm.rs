#![cfg(target_arch = "wasm32")]

use subxt::config::PolkadotConfig;
use subxt::rpc::LightClient;
use wasm_bindgen_test::*;
use std::sync::Arc;
use serde_json::value::RawValue;
use subxt::rpc::RpcClientT;
use subxt::rpc::Subscription;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Run the tests by `$ wasm-pack test --firefox --headless`

fn init_tracing() {
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();
}

// #[wasm_bindgen_test]
// async fn wasm_ws_transport_works() {
//     let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
//         .await
//         .unwrap();

//     let chain = client.rpc().system_chain().await.unwrap();
//     assert_eq!(&chain, "Development");
// }

#[wasm_bindgen_test]
async fn light_client_transport_works() {
	init_tracing();

    tracing::warn!("Starting test");
    let light_client = LightClient::new(include_str!("../../../artifacts/dev_spec.json")).unwrap();
    tracing::warn!("RPC layer created..");

    // Note: It is impractical to construct a full subxt `OnlineClient` because the
    // light-client must sync with the tip of the chain to fetch the Runtime Version
    // needed by subxt.
    // The default wasm-bindgen test timeout is 20 seconds, tested locally the
    // client does not sync in time with WASM_BINDGEN_TEST_TIMEOUT=1000 (around 16 seconds).

    // Test raw RPC method calls.
    let chain = light_client.request_raw("system_chain", None).await.unwrap();
    let chain: String = serde_json::from_str(chain.get()).unwrap();
    assert_eq!(&chain, "Development");

    let param = RawValue::from_string("[0]".to_owned()).expect("Should be valid JSON");
    let genesis = light_client.request_raw("chain_getBlockHash", Some(param)).await.unwrap();
    let genesis: String = serde_json::from_str(genesis.get()).unwrap();
    assert!(genesis.starts_with("0x"));
    tracing::warn!("Genesis hash {:?}", genesis);


    // Ensure light-client functions with subscriptions.
    let sub = light_client.subscribe_raw("chain_subscribeAllHeads", None, "chain_unsubscribeAllHeads").await.unwrap();
    // The subscription result is actually a PolkadotConfig::Header, we are interested in iteration.
    let mut sub: Subscription<String> = Subscription::new(sub);

    let block = sub.next().await.expect("Subscription failed").expect("Subscription ended");
    tracing::warn!("Block hash {:?}", block);

    let block = sub.next().await.expect("Subscription failed").expect("Subscription ended");
    tracing::warn!("Block hash {:?}", block);
}
