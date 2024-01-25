// Copyright 2019-2020
//     Parity Technologies (UK) Ltd. Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![allow(internal_features)]
#![feature(lang_items, start)]
#![feature(alloc_error_handler)]
#![no_std]

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
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

// use subxt_core::utils::H256;

// Note: Panics in this function will lead to `Aborted (core dumped)` and a non-zero exit status => suitable for CI tests.
fn main() {
    let metadata_bytes: &[u8] = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");

    // let genesis_hash = {
    //     let h = "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";
    //     let bytes = hex::decode(h).unwrap();
    //     H256::from_slice(&bytes)
    // };

    // // 2. A runtime version (system_version constant on a Substrate node has these):
    // let runtime_version = subxt::backend::RuntimeVersion {
    //     spec_version: 9370,
    //     transaction_version: 20,
    // };

    // // 3. Metadata (I'll load it from the downloaded metadata, but you can use
    // //    `subxt metadata > file.scale` to download it):
    // let metadata = {
    //     let bytes: &[u8] = include_bytes!("../polkadot_metadata_small.scale");
    //     Metadata::decode(&mut &*bytes).unwrap()
    // };

    // // Create an offline client using the details obtained above:
    // let api = OfflineClient::<PolkadotConfig>::new(genesis_hash, runtime_version, metadata);
}
