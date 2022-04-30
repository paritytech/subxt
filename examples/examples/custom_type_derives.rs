// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

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
