// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides a [`Config`] type, which is used to define various
//! types that are important in order to speak to a particular chain.
//! [`SubstrateConfig`] provides a default set of these types suitable for the
//! default Substrate node implementation, and [`PolkadotConfig`] for a
//! Polkadot node.

pub mod extrinsic_params;
pub mod polkadot;
pub mod substrate;

use codec::{
    Codec,
    Encode,
    EncodeLike,
};
use core::fmt::Debug;
use serde::Serialize;

pub use extrinsic_params::ExtrinsicParams;
pub use polkadot::PolkadotConfig;
pub use substrate::SubstrateConfig;

/// Runtime types.
// Note: the 'static bound isn't strictly required, but currently deriving TypeInfo
// automatically applies a 'static bound to all generic types (including this one),
// and so until that is resolved, we'll keep the (easy to satisfy) constraint here.
pub trait Config: 'static {
    /// Account index (aka nonce) type. This stores the number of previous
    /// transactions associated with a sender account.
    type Index: Parameter
        + Member
        + serde::de::DeserializeOwned
        + Default
        + Copy
        + scale_info::TypeInfo
        + Into<u64>;

    /// The block number type used by the runtime.
    type BlockNumber: Parameter
        + Member
        + Default
        + Copy
        + std::hash::Hash
        + std::str::FromStr
        + Into<u64>;

    /// The output of the `Hashing` function.
    type Hash: Parameter
        + Member
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Ord
        + Default
        + Copy
        + std::hash::Hash
        + AsRef<[u8]>
        + AsMut<[u8]>
        + scale_info::TypeInfo;

    /// The account ID type.
    type AccountId: Clone + Serialize;

    /// The address type.
    type Address: Encode + From<Self::AccountId>;

    /// The signature type.
    type Signature: Encode;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type Hasher: Hasher<Output = Self::Hash>;

    /// The block header.
    type Header: Parameter
        + Header<Number = Self::BlockNumber, Hasher = Self::Hasher>
        + Member
        + serde::de::DeserializeOwned;

    /// This type defines the extrinsic extra and additional parameters.
    type ExtrinsicParams: extrinsic_params::ExtrinsicParams<Self::Index, Self::Hash>;
}

/// Parameter trait copied from `substrate::frame_support`.
pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + Debug {}

/// A type that can be used in runtime structures. Copied from `sp_runtime::traits`.
pub trait Member: Send + Sync + Sized + Debug + Eq + PartialEq + Clone + 'static {}
impl<T: Send + Sync + Sized + Debug + Eq + PartialEq + Clone + 'static> Member for T {}

/// This represents the hasher used by a node to hash things like block headers
/// and extrinsics.
pub trait Hasher {
    /// The type given back from the hash operation
    type Output;

    /// Hash some bytes to the given output type.
    fn hash(s: &[u8]) -> Self::Output;

    /// Hash some SCALE encodable type to the given output type.
    fn hash_of<S: Encode>(s: &S) -> Self::Output {
        let out = s.encode();
        Self::hash(&out)
    }
}

/// This represents the block header type used by a node.
pub trait Header: Sized + Encode {
    /// The block number type for this header.
    type Number;
    /// The hasher used to hash this header.
    type Hasher: Hasher;

    /// Return the block number of this header.
    fn number(&self) -> Self::Number;

    /// Hash this header.
    fn hash(&self) -> <Self::Hasher as Hasher>::Output {
        Self::Hasher::hash_of(self)
    }
}

/// Take a type implementing [`Config`] (eg [`SubstrateConfig`]), and some type which describes the
/// additional and extra parameters to pass to an extrinsic (see [`crate::tx::ExtrinsicParams`]),
/// and returns a type implementing [`Config`] with those new `ExtrinsicParams`.
///
/// # Example
///
/// ```
/// use subxt::config::{ SubstrateConfig, WithExtrinsicParams };
/// use subxt::tx::PolkadotExtrinsicParams;
///
/// // This is how PolkadotConfig is implemented:
/// type PolkadotConfig = WithExtrinsicParams<SubstrateConfig, PolkadotExtrinsicParams<SubstrateConfig>>;
/// ```
pub struct WithExtrinsicParams<
    T: Config,
    E: extrinsic_params::ExtrinsicParams<T::Index, T::Hash>,
> {
    _marker: std::marker::PhantomData<(T, E)>,
}

impl<T: Config, E: extrinsic_params::ExtrinsicParams<T::Index, T::Hash>> Config
    for WithExtrinsicParams<T, E>
{
    type Index = T::Index;
    type BlockNumber = T::BlockNumber;
    type Hash = T::Hash;
    type AccountId = T::AccountId;
    type Address = T::Address;
    type Signature = T::Signature;
    type Hasher = T::Hasher;
    type Header = T::Header;
    type ExtrinsicParams = E;
}
