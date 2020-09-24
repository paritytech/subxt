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
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_runtime::{
    generic::Header,
    impl_opaque_keys,
    traits::{
        BlakeTwo256,
        IdentifyAccount,
        Verify,
    },
    MultiSignature,
    OpaqueExtrinsic,
};
use sp_std::prelude::*;

/// BABE marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Babe;
impl sp_runtime::BoundToRuntimeAppPublic for Babe {
    type Public = sp_consensus_babe::AuthorityId;
}

/// ImOnline marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImOnline;
impl sp_runtime::BoundToRuntimeAppPublic for ImOnline {
    type Public = ImOnlineId;
}

/// GRANDPA marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Grandpa;
impl sp_runtime::BoundToRuntimeAppPublic for Grandpa {
    type Public = sp_finality_grandpa::AuthorityId;
}

use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;

mod validator_app {
    use application_crypto::{
        app_crypto,
        sr25519,
    };
    app_crypto!(sr25519, sp_core::crypto::KeyTypeId(*b"para"));
}

/// Parachain marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Parachains;

impl sp_runtime::BoundToRuntimeAppPublic for Parachains {
    type Public = validator_app::Public;
}

/// Authority discovery marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthorityDiscovery;
impl sp_runtime::BoundToRuntimeAppPublic for AuthorityDiscovery {
    type Public = AuthorityDiscoveryId;
}

impl_opaque_keys! {
    /// Substrate base runtime keys
    pub struct BasicSessionKeys {
        /// GRANDPA session key
        pub grandpa: Grandpa,
        /// BABE session key
        pub babe: Babe,
        /// ImOnline session key
        pub im_online: ImOnline,
        /// Parachain validation session key
        pub parachains: Parachains,
        /// AuthorityDiscovery session key
        pub authority_discovery: AuthorityDiscovery,
    }
}

impl_opaque_keys! {
    /// Polkadot/Kusama runtime keys
    pub struct SessionKeys {
        /// GRANDPA session key
        pub grandpa: Grandpa,
        /// BABE session key
        pub babe: Babe,
        /// ImOnline session key
        pub im_online: ImOnline,
        /// ParachainValidator session key
        pub parachain_validator: Parachains,
        /// AuthorityDiscovery session key
        pub authority_discovery: AuthorityDiscovery,
    }
}

use crate::{
    extrinsic::{
        DefaultExtra,
        SignedExtra,
    },
    frame::{
        balances::{
            AccountData,
            Balances,
        },
        contracts::Contracts,
        session::Session,
        staking::Staking,
        sudo::Sudo,
        system::System,
    },
};

/// Runtime trait.
pub trait Runtime: System + Sized + Send + Sync + 'static {
    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;
    /// Transaction extras.
    type Extra: SignedExtra<Self> + Send + Sync + 'static;
}

/// Concrete type definitions compatible with those in the default substrate `node_runtime`
///
/// # Note
///
/// If the concrete types in the target substrate runtime differ from these, a custom Runtime
/// definition MUST be used to ensure type compatibility.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultNodeRuntime;

impl Staking for DefaultNodeRuntime {}

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

impl Session for DefaultNodeRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
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

impl Session for NodeTemplateRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
}

impl Sudo for NodeTemplateRuntime {}

/// Concrete type definitions compatible with the node template, with the
/// contracts pallet enabled.
///
/// Inherits types from [`NodeTemplateRuntime`], but adds an implementation for
/// the contracts pallet trait.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContractsTemplateRuntime;

impl Runtime for ContractsTemplateRuntime {
    type Signature = <NodeTemplateRuntime as Runtime>::Signature;
    type Extra = DefaultExtra<Self>;
}

impl System for ContractsTemplateRuntime {
    type Index = <NodeTemplateRuntime as System>::Index;
    type BlockNumber = <NodeTemplateRuntime as System>::BlockNumber;
    type Hash = <NodeTemplateRuntime as System>::Hash;
    type Hashing = <NodeTemplateRuntime as System>::Hashing;
    type AccountId = <NodeTemplateRuntime as System>::AccountId;
    type Address = <NodeTemplateRuntime as System>::Address;
    type Header = <NodeTemplateRuntime as System>::Header;
    type Extrinsic = <NodeTemplateRuntime as System>::Extrinsic;
    type AccountData = <NodeTemplateRuntime as System>::AccountData;
}

impl Balances for ContractsTemplateRuntime {
    type Balance = <NodeTemplateRuntime as Balances>::Balance;
}

impl Contracts for ContractsTemplateRuntime {}

impl Sudo for ContractsTemplateRuntime {}

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

impl Session for KusamaRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = SessionKeys;
}

impl Staking for KusamaRuntime {}

impl Balances for KusamaRuntime {
    type Balance = u128;
}
