// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use sp_runtime::traits::BlakeTwo256;
use subxt::{Runtime, StorageEntry, subxt};

#[subxt(runtime_metadata_path = "node_runtime.scale")]
pub mod node_runtime {
    #[subxt(substitute_type = "sp_core::crypto::AccountId32")]
    use sp_core::crypto::AccountId32;
    #[subxt(substitute_type = "primitive_types::H256")]
    use sp_core::H256;
    #[subxt(substitute_type = "sp_runtime::multiaddress::MultiAddress")]
    use sp_runtime::MultiAddress;

    #[subxt(substitute_type = "sp_arithmetic::per_things::Perbill")]
    use sp_arithmetic::per_things::Perbill;
    #[subxt(substitute_type = "sp_arithmetic::per_things::Perquintill")]
    use sp_arithmetic::per_things::Perquintill;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestRuntime;

impl Runtime for TestRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = sp_runtime::AccountId32;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header = sp_runtime::generic::Header<Self::BlockNumber, BlakeTwo256>;
    type Extra = subxt::extrinsic::DefaultExtra<Self>;
    type Signature = sp_runtime::MultiSignature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
    type AccountData = node_runtime::system::storage::Account;
}

impl subxt::AccountData<TestRuntime> for node_runtime::system::storage::Account {
    fn new(account_id: <TestRuntime as Runtime>::AccountId) -> Self {
        Self(account_id)
    }

    fn nonce(result: &<Self as StorageEntry>::Value) -> <TestRuntime as Runtime>::Index {
        result.nonce
    }
}
