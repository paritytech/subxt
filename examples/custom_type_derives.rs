// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

#[subxt::subxt(
    runtime_metadata_path = "examples/polkadot_metadata.scale",
    generated_type_derives = "Clone, Debug"
)]
pub mod polkadot {}

use polkadot::runtime_types::frame_support::PalletId;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pallet_id = PalletId([1u8; 8]);
    let _ = <PalletId as Clone>::clone(&pallet_id);
    Ok(())
}
