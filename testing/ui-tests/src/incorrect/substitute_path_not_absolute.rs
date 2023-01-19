#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata.scale",
    substitute_type(
        type = "sp_arithmetic::per_things::Perbill",
        with = "sp_runtime::Perbill"
    )
)]
pub mod node_runtime {}

fn main() {}
