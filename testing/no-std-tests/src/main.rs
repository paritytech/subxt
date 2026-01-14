// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![allow(internal_features)]
#![feature(lang_items, alloc_error_handler)]
#![no_std]
#![no_main]

pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> isize {
    compile_test();
    0
}

#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

//////////////////////////////////////////////////////////////////////////////

extern crate alloc;

/// Including code here makes sure it is not pruned.
/// We want all code included to compile fine for the `thumbv7em-none-eabi` target.
fn compile_test() {
    // Subxt Metadata compiles:
    use codec::Decode;
    let bytes: alloc::vec::Vec<u8> = alloc::vec![0, 1, 2, 3, 4];
    subxt_metadata::Metadata::decode(&mut &bytes[..]).expect_err("invalid byte sequence");

    const METADATA: &[u8] = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
    subxt_metadata::Metadata::decode(&mut &METADATA[..]).expect("should be valid metadata");

    // Subxt signer compiles (though nothing much works on this particular nostd target...):
    // Supported targets: <https://docs.rs/getrandom/latest/getrandom/#supported-targets>
    use core::str::FromStr;
    let _ = subxt_signer::SecretUri::from_str("//Alice/bar");

    // Note: sr25519 needs randomness, but `thumbv7em-none-eabi` isn't supported by
    // `getrandom`, so we can't sign in nostd on this target.
    //
    // use subxt_signer::sr25519;
    // let keypair = sr25519::dev::alice();
    // let message = b"Hello!";
    // let _signature = keypair.sign(message);
    // let _public_key = keypair.public_key();

    // Note: `ecdsa` is also not compiling for the `thumbv7em-none-eabi` target owing to
    // an issue compiling `secp256k1-sys`.
    //
    // use subxt_signer::ecdsa;
    // let keypair = ecdsa::dev::alice();
    // let message = b"Hello!";
    // let _signature = keypair.sign(message);
    // let _public_key = keypair.public_key();
}

