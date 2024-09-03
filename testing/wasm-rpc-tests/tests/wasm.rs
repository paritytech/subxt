#![cfg(target_arch = "wasm32")]

use subxt::config::SubstrateConfig;
use subxt::backend::rpc::reconnecting_rpc_client::RpcClient as ReconnectingRpcClient;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Run the tests by calling:
//
// ```text
// wasm-pack test --firefox --headless`
// ```
//
// You'll need to have a substrate node running:
//
// ```bash
// ./substrate-node --dev --node-key 0000000000000000000000000000000000000000000000000000000000000001 --listen-addr /ip4/0.0.0.0/tcp/30333/ws
// ```
//
// Use the following to enable logs:
// ```
//  console_error_panic_hook::set_once();
//  tracing_wasm::set_as_global_default();
// ```

#[wasm_bindgen_test]
async fn wasm_ws_transport_works() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    let client = subxt::client::OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944")
        .await
        .unwrap();

    let mut stream = client.backend().stream_best_block_headers().await.unwrap();
    assert!(stream.next().await.is_some());
}

#[wasm_bindgen_test]
async fn reconnecting_rpc_client_ws_transport_works() {
    let rpc = ReconnectingRpcClient::builder().build("ws://127.0.0.1:9944".to_string()).await.unwrap();
    let client = subxt::client::OnlineClient::<SubstrateConfig>::from_rpc_client(rpc.clone()).await.unwrap();
    let mut stream = client.backend().stream_best_block_headers().await.unwrap();
    assert!(stream.next().await.is_some());
}

