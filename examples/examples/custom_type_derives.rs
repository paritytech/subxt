// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Example verified against polkadot 0.9.18-4542a603cc-aarch64-macos.

#![allow(clippy::redundant_clone)]

#[subxt::subxt(
    runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
    // We can add (certain) custom derives to the generated types by providing
    // a comma separated list to the below attribute. Most useful for adding `Clone`.
    // The derives that we can add ultimately is limited to the traits that the base
    // types relied upon by the codegen implement.
    derive_for_all_types = "Clone, PartialEq",

    // To apply derives to specific generated types, add a `derive_for_type` per type,
    // mapping the type path to the derives which should be added for that type only.
    // Note that these derives will be in addition to those specified above in
    // `derive_for_all_types`
    derive_for_type(type = "frame_support::PalletId", derive = "Eq, Ord, PartialOrd"),
    derive_for_type(type = "sp_runtime::ModuleError", derive = "Eq, Hash"),
)]
pub mod polkadot {}

use polkadot::runtime_types::frame_support::PalletId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pallet_id = PalletId([1u8; 8]);
    let _ = pallet_id.clone();
    Ok(())
}
