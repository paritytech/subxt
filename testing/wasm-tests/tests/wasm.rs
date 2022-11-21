#![cfg(target_arch = "wasm32")]

use subxt::config::PolkadotConfig;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Run the tests by `$ wasm-pack test --firefox --headless`

#[wasm_bindgen_test]
async fn wasm_ws_transport_works() {
    let client =
        subxt::client::OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
            .await
            .unwrap();

    let chain = client.rpc().system_chain().await.unwrap();
    assert_eq!(&chain, "Development");
}
