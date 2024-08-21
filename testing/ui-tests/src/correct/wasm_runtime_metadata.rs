#[subxt::subxt(wasm_file_path = "../../../../artifacts/westend_runtime.wasm")]
mod runtime {}

#[subxt::subxt(wasm_file_path = "../../../../artifacts/westend_runtime.compact.compressed.wasm")]
mod runtime_compressed {}

fn main() {
    use runtime;
    use runtime_compressed;

    let _ = runtime::system::events::CodeUpdated;
    let _ = runtime_compressed::system::events::CodeUpdated;
}
