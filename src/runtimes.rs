// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

use codec::Encode;
use sp_runtime::{
    generic::Header,
    traits::{
        BlakeTwo256,
        IdentifyAccount,
        Verify,
    },
    MultiSignature,
    OpaqueExtrinsic,
};

use crate::{
    extra::{
        DefaultExtra,
        SignedExtra,
    },
    frame::{
        balances::{
            AccountData,
            Balances,
        },
        contracts::Contracts,
        sudo::Sudo,
        system::System,
    },
    Encoded,
};

/// Runtime trait.
pub trait Runtime: System + Sized + Send + Sync + 'static {
    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;
    /// Transaction extras.
    type Extra: SignedExtra<Self> + Send + Sync + 'static;
}

/// Extra type.
pub type Extra<T> = <<T as Runtime>::Extra as SignedExtra<T>>::Extra;

/// UncheckedExtrinsic type.
pub type UncheckedExtrinsic<T> = sp_runtime::generic::UncheckedExtrinsic<
    <T as System>::Address,
    Encoded,
    <T as Runtime>::Signature,
    Extra<T>,
>;

/// SignedPayload type.
pub type SignedPayload<T> = sp_runtime::generic::SignedPayload<Encoded, Extra<T>>;

/// Concrete type definitions compatible with those in the default substrate `node_runtime`
///
/// # Note
///
/// If the concrete types in the target substrate runtime differ from these, a custom Runtime
/// definition MUST be used to ensure type compatibility.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultNodeRuntime;

impl Runtime for DefaultNodeRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl System for DefaultNodeRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = pallet_indices::address::Address<Self::AccountId, u32>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for DefaultNodeRuntime {
    type Balance = u128;
}

impl Contracts for DefaultNodeRuntime {}

impl Sudo for DefaultNodeRuntime {}

/// Concrete type definitions compatible with the node template.
///
/// # Note
///
/// Main difference is `type Address = AccountId`.
/// Also the contracts module is not part of the node template runtime.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NodeTemplateRuntime;

impl Runtime for NodeTemplateRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl System for NodeTemplateRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = Self::AccountId;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for NodeTemplateRuntime {
    type Balance = u128;
}

impl Sudo for NodeTemplateRuntime {}

/// Concrete type definitions compatible with those for kusama, v0.7
///
/// # Note
///
/// Main difference is `type Address = AccountId`.
/// Also the contracts module is not part of the kusama runtime.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KusamaRuntime;

impl Runtime for KusamaRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl System for KusamaRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = Self::AccountId;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for KusamaRuntime {
    type Balance = u128;
}
