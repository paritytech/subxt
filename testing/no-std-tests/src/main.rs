// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![allow(internal_features)]
#![feature(lang_items, start)]
#![feature(alloc_error_handler)]
#![no_std]

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    compile_test();
    0
}

#[lang = "eh_personality"]
#[no_mangle]
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

    // Subxt Signer compiles:
    use subxt_signer::sr25519;
    let keypair = sr25519::dev::alice();
    let message = b"Hello!";
    let _signature = keypair.sign(message);
    let _public_key = keypair.public_key();

    // Note: `ecdsa` is not compiling for the `thumbv7em-none-eabi` target.
    //
    // use subxt_signer::ecdsa;
    // let keypair = ecdsa::dev::alice();
    // let message = b"Hello!";
    // let _signature = keypair.sign(message);
    // let _public_key = keypair.public_key();
    //

    // Subxt Core compiles:
    let _era = subxt_core::utils::Era::Immortal;
    
}
