// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{
    Codec,
    Encode,
    EncodeLike,
};
use core::fmt::Debug;
use sp_runtime::traits::{
    AtLeast32Bit,
    Extrinsic,
    Hash,
    Header,
    MaybeSerializeDeserialize,
    Member,
    Verify,
};

/// Runtime types.
// Note: the 'static bound isn't strictly required, but currently deriving TypeInfo
// automatically applies a 'static bound to all generic types (including this one),
// and so until that is resolved, we'll keep the (easy to satisfy) constraint here.
pub trait Config: Debug + 'static {
    /// Account index (aka nonce) type. This stores the number of previous
    /// transactions associated with a sender account.
    type Index: Parameter
        + Member
        + serde::de::DeserializeOwned
        + Default
        + AtLeast32Bit
        + Copy
        + scale_info::TypeInfo
        + Into<u64>;

    /// The block number type used by the runtime.
    type BlockNumber: Parameter
        + Member
        + Default
        + Copy
        + core::hash::Hash
        + core::str::FromStr
        + Into<u64>;

    /// The output of the `Hashing` function.
    type Hash: Parameter
        + Member
        + MaybeSerializeDeserialize
        + Ord
        + Default
        + Copy
        + std::hash::Hash
        + AsRef<[u8]>
        + AsMut<[u8]>
        + scale_info::TypeInfo;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type Hashing: Hash<Output = Self::Hash>;

    /// The user account identifier type for the runtime.
    type AccountId: Parameter + Member + serde::Serialize;

    /// The address type. This instead of `<frame_system::Trait::Lookup as StaticLookup>::Source`.
    type Address: Codec + Clone + PartialEq;

    /// The block header.
    type Header: Parameter
        + Header<Number = Self::BlockNumber, Hash = Self::Hash>
        + serde::de::DeserializeOwned;

    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;

    /// Extrinsic type within blocks.
    type Extrinsic: Parameter + Extrinsic + Debug + MaybeSerializeDeserialize;
}

/// Parameter trait copied from `substrate::frame_support`
pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + Debug {}

/// Default set of commonly used types by Substrate runtimes.
// Note: We only use this at the type level, so it should be impossible to
// create an instance of it.
#[derive(Debug)]
pub enum DefaultConfig {}

impl Config for DefaultConfig {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = sp_runtime::AccountId32;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header =
        sp_runtime::generic::Header<Self::BlockNumber, sp_runtime::traits::BlakeTwo256>;
    type Signature = sp_runtime::MultiSignature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
