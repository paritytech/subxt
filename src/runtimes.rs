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
#![allow(missing_docs)]

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
pub struct Babe;
impl sp_runtime::BoundToRuntimeAppPublic for Babe {
    type Public = sp_consensus_babe::AuthorityId;
}

/// ImOnline marker struct
pub struct ImOnline;
impl sp_runtime::BoundToRuntimeAppPublic for ImOnline {
    type Public = ImOnlineId;
}

/// GRANDPA marker struct
pub struct Grandpa;
impl sp_runtime::BoundToRuntimeAppPublic for Grandpa {
    type Public = sp_finality_grandpa::AuthorityId;
}

use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;

#[cfg(feature = "kusama")]
mod validator_app {
	use application_crypto::{app_crypto, sr25519};
	app_crypto!(sr25519, sp_core::crypto::KeyTypeId(*b"para"));
}

/// Parachain marker struct
#[cfg(feature = "kusama")]
pub struct Parachains;

#[cfg(feature = "kusama")]
impl sp_runtime::BoundToRuntimeAppPublic for Parachains {
    type Public = validator_app::Public;
}

/// Authority discovery marker struct
pub struct AuthorityDiscovery;
impl sp_runtime::BoundToRuntimeAppPublic for AuthorityDiscovery {
    type Public = AuthorityDiscoveryId;
}

#[cfg(feature = "kusama")]
impl_opaque_keys! {
    /// Substrate base runtime keys
    pub struct BasicSessionKeys {
        //// GRANDPA session key
        pub grandpa: Grandpa,
        //// BABE session key
        pub babe: Babe,
        //// ImOnline session key
        pub im_online: ImOnline,
        //// Parachain validation session key
        pub parachains: Parachains,
        //// AuthorityDiscovery session key
        pub authority_discovery: AuthorityDiscovery,
    }
}

#[cfg(feature = "kusama")]
impl_opaque_keys! {
    /// Polkadot/Kusama runtime keys
    pub struct SessionKeys {
        //// GRANDPA session key
        pub grandpa: Grandpa,
        //// BABE session key
        pub babe: Babe,
        //// ImOnline session key
        pub im_online: ImOnline,
		//// ParachainValidator session key
		pub parachain_validator: Parachains,
        //// AuthorityDiscovery session key
        pub authority_discovery: AuthorityDiscovery,
    }
}

use crate::frame::{
    balances::{
        AccountData,
        Balances,
    },
    contracts::Contracts,
    session::Session,
    staking::Staking,
    system::System,
};

/// Concrete type definitions compatible with those in the default substrate `node_runtime`
///
/// # Note
///
/// If the concrete types in the target substrate runtime differ from these, a custom Runtime
/// definition MUST be used to ensure type compatibility.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultNodeRuntime;

impl Staking for DefaultNodeRuntime {
    type NominatorIndex = u32;
    type ValidatorIndex = u16;
    const MAX_VALIDATORS: usize = Self::ValidatorIndex::max_value() as usize;
    const MAX_NOMINATORS: usize = Self::NominatorIndex::max_value() as usize;
    type EraIndex = u32;
    type RewardPoint = u32;
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
    type SessionIndex = u32;
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
}

impl Contracts for DefaultNodeRuntime {}

/// Concrete type definitions compatible with those for kusama, v0.7
///
/// # Note
///
/// Main difference is `type Address = AccountId`.
/// Also the contracts module is not part of the kusama runtime.
#[cfg(feature = "kusama")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KusamaRuntime;

#[cfg(feature = "kusama")]
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

#[cfg(feature = "kusama")]
impl Session for KusamaRuntime {
    type SessionIndex = u32;
    type ValidatorId = <Self as System>::AccountId;
    type Keys = SessionKeys;
}

#[cfg(feature = "kusama")]
impl Staking for KusamaRuntime {
    type NominatorIndex = u32;
    type ValidatorIndex = u16;
    const MAX_VALIDATORS: usize = Self::ValidatorIndex::max_value() as usize;
    const MAX_NOMINATORS: usize = Self::NominatorIndex::max_value() as usize;
    type EraIndex = u32;
    type RewardPoint = u32;
}

#[cfg(feature = "kusama")]
impl Balances for KusamaRuntime {
    type Balance = u128;
}
