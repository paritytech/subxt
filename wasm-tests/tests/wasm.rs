#![cfg(target_arch = "wasm32")]

use subxt::config::PolkadotConfig;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Run the tests by `$ wasm-pack test --firefox --headless`

fn init_tracing() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

#[wasm_bindgen_test]
async fn wasm_ws_transport_works() {
    init_tracing();

    let client = subxt::client::OnlineClient<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
        .await
        .unwrap();

    assert_eq!(1, 2);
}
