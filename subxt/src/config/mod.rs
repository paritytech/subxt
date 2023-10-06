// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides a [`Config`] type, which is used to define various
//! types that are important in order to speak to a particular chain.
//! [`SubstrateConfig`] provides a default set of these types suitable for the
//! default Substrate node implementation, and [`PolkadotConfig`] for a
//! Polkadot node.

mod default_extrinsic_params;
mod extrinsic_params;

pub mod polkadot;
pub mod signed_extensions;
pub mod substrate;

use codec::{Decode, Encode};
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

pub use default_extrinsic_params::{DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder};
pub use extrinsic_params::{ExtrinsicParams, ExtrinsicParamsEncoder, ExtrinsicParamsError};
pub use polkadot::{PolkadotConfig, PolkadotExtrinsicParams, PolkadotExtrinsicParamsBuilder};
pub use signed_extensions::SignedExtension;
pub use substrate::{SubstrateConfig, SubstrateExtrinsicParams, SubstrateExtrinsicParamsBuilder};

/// Runtime types.
// Note: the `Send + Sync + 'static` bound isn't strictly required, but currently deriving
// TypeInfo automatically applies a 'static bound to all generic types (including this one),
// And we want the compiler to infer `Send` and `Sync` OK for things which have `T: Config`
// rather than having to `unsafe impl` them ourselves.
pub trait Config: Sized + Send + Sync + 'static {
    /// The output of the `Hasher` function.
    type Hash: BlockHash;

    /// The account ID type.
    type AccountId: Debug + Clone + Encode;

    /// The address type.
    type Address: Debug + Encode + From<Self::AccountId>;

    /// The signature type.
    type Signature: Debug + Encode;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type Hasher: Debug + Hasher<Output = Self::Hash>;

    /// The block header.
    type Header: Debug + Header<Hasher = Self::Hasher> + Sync + Send + DeserializeOwned;

    /// This type defines the extrinsic extra and additional parameters.
    type ExtrinsicParams: ExtrinsicParams<Self>;

    /// Asset Id
    type AssetId: Encode;
}

/// given some [`Config`], this return the other params needed for its `ExtrinsicParams`.
pub type OtherParamsFor<T> = <<T as Config>::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams;

/// Block hashes must conform to a bunch of things to be used in Subxt.
pub trait BlockHash:
    Debug
    + Copy
    + Send
    + Sync
    + Decode
    + AsRef<[u8]>
    + Serialize
    + DeserializeOwned
    + Encode
    + PartialEq
    + Eq
    + std::hash::Hash
{
}
impl<T> BlockHash for T where
    T: Debug
        + Copy
        + Send
        + Sync
        + Decode
        + AsRef<[u8]>
        + Serialize
        + DeserializeOwned
        + Encode
        + PartialEq
        + Eq
        + std::hash::Hash
{
}

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
pub trait Header: Sized + Encode + Decode {
    /// The block number type for this header.
    type Number: Into<u64>;
    /// The hasher used to hash this header.
    type Hasher: Hasher;

    /// Return the block number of this header.
    fn number(&self) -> Self::Number;

    /// Hash this header.
    fn hash(&self) -> <Self::Hasher as Hasher>::Output {
        Self::Hasher::hash_of(self)
    }
}

/// implement subxt's Hasher and Header traits for some substrate structs
#[cfg(feature = "substrate-compat")]
mod substrate_impls {
    use super::*;
    use primitive_types::{H256, U256};

    impl<N, H> Header for sp_runtime::generic::Header<N, H>
    where
        Self: Encode + Decode,
        N: Copy + Into<U256> + Into<u64> + TryFrom<U256>,
        H: sp_runtime::traits::Hash + Hasher,
    {
        type Number = N;
        type Hasher = H;

        fn number(&self) -> Self::Number {
            self.number
        }
    }

    impl Hasher for sp_core::Blake2Hasher {
        type Output = H256;

        fn hash(s: &[u8]) -> Self::Output {
            <Self as sp_core::Hasher>::hash(s)
        }
    }

    impl Hasher for sp_runtime::traits::BlakeTwo256 {
        type Output = H256;

        fn hash(s: &[u8]) -> Self::Output {
            <Self as sp_core::Hasher>::hash(s)
        }
    }

    impl Hasher for sp_core::KeccakHasher {
        type Output = H256;

        fn hash(s: &[u8]) -> Self::Output {
            <Self as sp_core::Hasher>::hash(s)
        }
    }

    impl Hasher for sp_runtime::traits::Keccak256 {
        type Output = H256;

        fn hash(s: &[u8]) -> Self::Output {
            <Self as sp_core::Hasher>::hash(s)
        }
    }
}
