#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_tiny.scale",
    runtime_metadata_insecure_url = "wss://rpc.polkadot.io:443"
)]
pub mod node_runtime {}

fn main() {}
