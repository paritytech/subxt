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

use codec::Encode;
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

/// Application specific crypto types
///
/// # Note
///
/// These are redefined here to avoid dependencies on the substrate creates where they are defined.
/// They must be identical to the definitions in the target substrate version.
pub mod app {
    use application_crypto::{
        app_crypto,
        ed25519,
        key_types,
        sr25519,
    };

    /// Authority discovery app crypto types
    pub mod authority_discovery {
        use super::*;
        app_crypto!(sr25519, key_types::AUTHORITY_DISCOVERY);
    }
    /// Babe app crypto types
    pub mod babe {
        use super::*;
        app_crypto!(sr25519, key_types::BABE);
    }
    /// Im online discovery app crypto types
    pub mod im_online {
        use super::*;
        app_crypto!(ed25519, key_types::IM_ONLINE);
    }
    /// Grandpa app crypto types
    pub mod grandpa {
        use super::*;
        app_crypto!(ed25519, key_types::GRANDPA);
    }
    /// Validator app crypto types
    pub mod validator {
        use super::*;
        app_crypto!(ed25519, sp_core::crypto::KeyTypeId(*b"para"));
    }
}

impl sp_runtime::BoundToRuntimeAppPublic for Babe {
    type Public = app::babe::Public;
}

/// ImOnline marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImOnline;
impl sp_runtime::BoundToRuntimeAppPublic for ImOnline {
    type Public = app::im_online::Public;
}

/// GRANDPA marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Grandpa;
impl sp_runtime::BoundToRuntimeAppPublic for Grandpa {
    type Public = app::grandpa::Public;
}

/// Parachain marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Parachains;

impl sp_runtime::BoundToRuntimeAppPublic for Parachains {
    type Public = app::validator::Public;
}

/// Authority discovery marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthorityDiscovery;
impl sp_runtime::BoundToRuntimeAppPublic for AuthorityDiscovery {
    type Public = app::authority_discovery::Public;
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
            BalancesEventTypeRegistry,
        },
        contracts::{
            Contracts,
            ContractsEventTypeRegistry,
        },
        session::{
            Session,
            SessionEventTypeRegistry,
        },
        staking::{
            Staking,
            StakingEventTypeRegistry,
        },
        sudo::{
            Sudo,
            SudoEventTypeRegistry,
        },
        system::{
            System,
            SystemEventTypeRegistry,
        },
    },
    EventTypeRegistry,
};

/// Runtime trait.
pub trait Runtime: System + Sized + Send + Sync + 'static {
    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;
    /// Transaction extras.
    type Extra: SignedExtra<Self> + Send + Sync + 'static;

    /// Register type sizes for this runtime
    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>);
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

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_session();
        event_type_registry.with_contracts();
        event_type_registry.with_sudo();
        register_default_type_sizes(event_type_registry);
    }
}

impl System for DefaultNodeRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
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

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_session();
        event_type_registry.with_sudo();
        register_default_type_sizes(event_type_registry);
    }
}

impl System for NodeTemplateRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
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

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_contracts();
        event_type_registry.with_sudo();
        register_default_type_sizes(event_type_registry);
    }
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

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_session();
        event_type_registry.with_staking();
        register_default_type_sizes(event_type_registry);
    }
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

/// Identity of a Grandpa authority.
pub type AuthorityId = crate::runtimes::app::grandpa::Public;
/// The weight of an authority.
pub type AuthorityWeight = u64;
/// A list of Grandpa authorities with associated weights.
pub type AuthorityList = Vec<(AuthorityId, AuthorityWeight)>;

/// Register default common runtime type sizes
pub fn register_default_type_sizes<T: Runtime>(
    event_type_registry: &mut EventTypeRegistry<T>,
) {
    // primitives
    event_type_registry.register_type_size::<bool>("bool");
    event_type_registry.register_type_size::<u8>("u8");
    event_type_registry.register_type_size::<u32>("u32");
    event_type_registry.register_type_size::<u64>("u64");
    event_type_registry.register_type_size::<u128>("u128");

    event_type_registry.register_type_size::<()>("PhantomData");

    // frame_support types
    event_type_registry
        .register_type_size::<frame_support::dispatch::DispatchInfo>("DispatchInfo");
    event_type_registry
        .register_type_size::<frame_support::dispatch::DispatchResult>("DispatchResult");
    event_type_registry
        .register_type_size::<frame_support::dispatch::DispatchError>("DispatchError");
    event_type_registry
        .register_type_size::<frame_support::traits::BalanceStatus>("Status");

    // aliases etc.
    event_type_registry.register_type_size::<u32>("ReferendumIndex");
    event_type_registry.register_type_size::<[u8; 16]>("Kind");

    event_type_registry.register_type_size::<u32>("AccountIndex");
    event_type_registry.register_type_size::<u32>("PropIndex");
    event_type_registry.register_type_size::<u32>("ProposalIndex");
    event_type_registry.register_type_size::<u32>("AuthorityIndex");
    event_type_registry.register_type_size::<u32>("MemberCount");

    event_type_registry.register_type_size::<u8>("VoteThreshold");
    event_type_registry
        .register_type_size::<(T::BlockNumber, u32)>("TaskAddress<BlockNumber>");

    event_type_registry.register_type_size::<AuthorityId>("AuthorityId");
    event_type_registry.register_type_size::<AuthorityWeight>("AuthorityWeight");
    event_type_registry
        .register_type_size::<Vec<(AuthorityId, AuthorityWeight)>>("AuthorityList");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_register_default_runtime_type_sizes() {
        EventTypeRegistry::<DefaultNodeRuntime>::new();
    }

    #[test]
    fn can_register_node_template_runtime_type_sizes() {
        EventTypeRegistry::<NodeTemplateRuntime>::new();
    }

    #[test]
    fn can_register_contracts_template_runtime_type_sizes() {
        EventTypeRegistry::<ContractsTemplateRuntime>::new();
    }

    #[test]
    fn can_register_kusama_runtime_type_sizes() {
        EventTypeRegistry::<KusamaRuntime>::new();
    }
}
