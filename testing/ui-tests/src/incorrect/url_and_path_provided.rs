#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_tiny.scale",
    runtime_metadata_insecure_url = "wss://rpc.polkadot.io:443"
)]
pub mod node_runtime {}

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_tiny.scale",
    runtime_metadata_insecure_url = "wss://rpc.polkadot.io:443",
    runtime_path = "../../../../artifacts/westend_runtime.wasm"
)]
pub mod node_runtime2 {}

fn main() {}
