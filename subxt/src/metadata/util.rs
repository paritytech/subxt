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

use serde::{
    Deserialize,
    Serialize,
};
use sp_runtime::{
    AccountId32,
    MultiAddress as SubstrateMultiAddress,
};

pub type MultiAddress = SubstrateMultiAddress<AccountId32, u32>;

#[derive(Serialize, Deserialize)]
#[serde(remote = "MultiAddress")]
pub enum RemoteAddress {
    /// It's an account ID (pubkey).
    Id(AccountId32),
    /// It's an account index.
    Index(u32),
    /// It's some arbitrary raw bytes.
    Raw(Vec<u8>),
    /// It's a 32 byte representation.
    Address32([u8; 32]),
    /// It's a 20 byte representation.
    Address20([u8; 20]),
}
