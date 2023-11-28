// cargo build --package wasm32 --target wasm32-unknown-unknown
// native
// subxt = { workspace = true, default-features = false, features = ["native"]}
// 
// error: the wasm*-unknown-unknown targets are not supported by default, you may need to enable the "js" feature. For more information see: https://docs.rs/getrandom/#webassembly-support
// --> /home/dz/.cargo/registry/src/index.crates.io-6f17d22bba15001f/getrandom-0.2.11/src/lib.rs:290:9
// |
// 290 | /         compile_error!("the wasm*-unknown-unknown targets are not supported by \
// 291 | |                         default, you may need to enable the \"js\" feature. \
// 292 | |                         For more information see: \
// 293 | |                         https://docs.rs/getrandom/#webassembly-support");
// | |________________________________________________________________________^

// error[E0433]: failed to resolve: use of undeclared crate or module `imp`
// --> /home/dz/.cargo/registry/src/index.crates.io-6f17d22bba15001f/getrandom-0.2.11/src/lib.rs:346:9
// |
// 346 |         imp::getrandom_inner(dest)?;
// |         ^^^ use of undeclared crate or module `imp`
// nix run github:informalsystems/cosmos.nix#cosmwasm-check ../../target/wasm32-unknown-unknown/debug/wasm32.wasm
// subxt = { workspace = true, default-features = false, features = ["web"]}
// bash-5.1$ nix run github:informalsystems/cosmos.nix#cosmwasm-check ../../target/wasm32-unknown-unknown/debug/wasm32.wasm 
// Available capabilities: {"iterator", "cosmwasm_1_1", "staking", "stargate", "cosmwasm_1_2"}

// ../../target/wasm32-unknown-unknown/debug/wasm32.wasm: failure
// Error during static Wasm validation: Wasm contract requires unsupported import: "__wbindgen_placeholder__.__wbindgen_describe". Required imports: {"__wbindgen_externref_xform__.__wbindgen_externref_table_grow", "__wbindgen_externref_xform__.__wbindgen_externref_table_set_null", "__wbindgen_placeholder__.__wbindgen_describe", ... 1 more}. Available imports: ["env.abort", "env.db_read", "env.db_write", "env.db_remove", "env.addr_validate", "env.addr_canonicalize", "env.addr_humanize", "env.secp256k1_verify", "env.secp256k1_recover_pubkey", "env.ed25519_verify", "env.ed25519_batch_verify", "env.debug", "env.query_chain", "env.db_scan", "env.db_next"].

// Passes: 0, failures: 1
// bash-5.1$ 


#[subxt::subxt(runtime_metadata_path = "../../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[no_mangle]
extern "C" fn interface_version_8() -> () {}

#[no_mangle]
extern "C" fn allocate(size: usize) -> u32 {
    42
}

#[no_mangle]
extern "C" fn deallocate(pointer: u32) {
}

#[no_mangle]
extern "C" fn instantiate(pointer: u32) {
}

mod test {
    
    fn dep() {
        //let bob = Keypair::from_uri(&SecretUri::from_str("//Bob").unwrap()).unwrap();
        //let dest = bob.public_key().into();
        let dest =
            subxt::utils::MultiAddress::<subxt::utils::AccountId32, ()>::Address32([0u8; 32]);
        let _ = crate::polkadot::tx()
            .balances()
            .transfer_allow_death(dest, 10_000);
    }
}
