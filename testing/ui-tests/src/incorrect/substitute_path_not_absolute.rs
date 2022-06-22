#[subxt::subxt(runtime_metadata_path = "../../../artifacts/polkadot_metadata.scale")]
pub mod node_runtime {
    #[subxt::subxt(substitute_type = "sp_arithmetic::per_things::Perbill")]
    use sp_runtime::Perbill;
}

fn main() {}
