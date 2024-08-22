#[subxt::subxt(wasm_file_path = "../../../../artifacts/westend_runtime.wasm")]
mod runtime {}

#[subxt::subxt(wasm_file_path = "../../../../artifacts/westend_runtime.compact.compressed.wasm")]
mod runtime_compressed {}

// Deadlocks due to calling `cargo build`
// #[subxt::subxt(
//     runtime_crate_name = "westend-runtime",
//     wasm_file_path = "../../../../target/release/wbuild/westend_runtime.wasm"
// )]
// mod compiled {}

#[test]
fn v() {
    use runtime;
    use runtime_compressed;

    let _ = runtime::system::events::CodeUpdated;
    let _ = runtime_compressed::system::events::CodeUpdated;
    // let _ = compiled::system::events::CodeUpdated;
}
