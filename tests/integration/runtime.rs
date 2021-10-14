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

#[subxt::subxt(runtime_metadata_path = "tests/integration/node_runtime.scale")]
pub mod node_runtime {
    #[subxt(substitute_type = "sp_core::crypto::AccountId32")]
    use sp_core::crypto::AccountId32;
    #[subxt(substitute_type = "primitive_types::H256")]
    use sp_core::H256;
    #[subxt(substitute_type = "sp_runtime::multiaddress::MultiAddress")]
    use sp_runtime::MultiAddress;

    // todo: [AJ] remove the requirement for these by implementing Compact handling properly
    #[subxt(substitute_type = "sp_arithmetic::per_things::Perbill")]
    use sp_arithmetic::per_things::Perbill;
    #[subxt(substitute_type = "sp_arithmetic::per_things::Perquintill")]
    use sp_arithmetic::per_things::Perquintill;
}
