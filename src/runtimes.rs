// Copyright 2019 Parity Technologies (UK) Ltd.
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

use crate::srml::{
    balances::Balances,
    contracts::Contracts,
    system::System,
};
use runtime_primitives::{
    generic::Header,
    traits::{
        BlakeTwo256,
        Verify,
    },
    AnySignature,
};

/// Concrete type definitions compatible with those in the default substrate `node_runtime`
///
/// # Note
///
/// If the concrete types in the target substrate runtime differ from these, a custom Runtime
/// definition MUST be used to ensure type compatibility.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultNodeRuntime;

impl System for DefaultNodeRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = substrate_primitives::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <AnySignature as Verify>::Signer;
    type Address = srml_indices::address::Address<Self::AccountId, u32>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
}

impl Balances for DefaultNodeRuntime {
    type Balance = u64;
}

impl Contracts for DefaultNodeRuntime {}
