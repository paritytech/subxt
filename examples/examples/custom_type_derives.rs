// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Example verified against polkadot polkadot 0.9.25-5174e9ae75b.
#![allow(clippy::redundant_clone)]

#[subxt::subxt(
    runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
    // We can add (certain) custom derives to the generated types by providing
    // a comma separated list to the below attribute. Most useful for adding `Clone`.
    // The derives that we can add ultimately is limited to the traits that the base
    // types relied upon by the codegen implement.
    derive_for_all_types = "Clone, PartialEq, Eq",

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
