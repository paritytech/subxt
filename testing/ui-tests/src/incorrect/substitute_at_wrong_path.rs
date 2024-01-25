#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::Event",
        with = "crate::MyEvent"
    )
)]
pub mod node_runtime {}

fn main() {}
