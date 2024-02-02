// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![allow(internal_features)]
#![feature(lang_items, start)]
#![feature(alloc_error_handler)]
#![no_std]

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    run_tests();
    0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        libc::abort();
    }
}

use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

//////////////////////////////////////////////////////////////////////////////

extern crate alloc;

// Note: Panics in this function will lead to `Aborted (core dumped)` and a non-zero exit status => suitable for CI tests.
fn run_tests() {
    subxt_metadata_test();
    subxt_core_test();
}

/// Makes sure, subxt-metadata works in a no-std-context:
fn subxt_metadata_test() {
    use codec::Decode;
    let bytes: alloc::vec::Vec<u8> = alloc::vec![0, 1, 2, 3, 4];
    subxt_metadata::Metadata::decode(&mut &bytes[..]).expect_err("invalid byte sequence");

    const METADATA: &[u8] = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
    subxt_metadata::Metadata::decode(&mut &METADATA[..]).expect("should be valid metadata");
}

fn subxt_signer_test() {
    use subxt_signer::{ SecretUri, ecdsa::Keypair };
    use core::str::FromStr;
    let uri = SecretUri::from_str("//Alice").unwrap();
    let keypair = Keypair::from_uri(&uri).unwrap();
}

fn subxt_core_test() {
    let _ = subxt_core::utils::Era::Immortal;
}

