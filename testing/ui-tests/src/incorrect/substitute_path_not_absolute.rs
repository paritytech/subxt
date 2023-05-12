#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_tiny.scale",
    substitute_type(
        path = "sp_arithmetic::per_things::Perbill",
        with = "sp_runtime::Perbill"
    )
)]
pub mod node_runtime {}

fn main() {}
