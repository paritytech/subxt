// Copyright 2019-2024 Parity Technologies (UK) Ltd.
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
pub mod substrate;
pub mod transaction_extensions;

use codec::{Decode, Encode};
use core::fmt::Debug;
use scale_decode::DecodeAsType;
use scale_encode::EncodeAsType;
use serde::{Serialize, de::DeserializeOwned};
use subxt_metadata::Metadata;

pub use default_extrinsic_params::{DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder};
pub use extrinsic_params::{ExtrinsicParams, ExtrinsicParamsEncoder};
pub use polkadot::{PolkadotConfig, PolkadotExtrinsicParams, PolkadotExtrinsicParamsBuilder};
pub use substrate::{SubstrateConfig, SubstrateExtrinsicParams, SubstrateExtrinsicParamsBuilder};
pub use transaction_extensions::TransactionExtension;

/// Runtime types.
// Note: the `Send + Sync + 'static` bound isn't strictly required, but currently deriving
// TypeInfo automatically applies a 'static bound to all generic types (including this one),
// And we want the compiler to infer `Send` and `Sync` OK for things which have `T: Config`
// rather than having to `unsafe impl` them ourselves.
pub trait Config: Sized + Send + Sync + 'static {
    /// The account ID type.
    type AccountId: Debug + Clone + Encode + Decode + Serialize + Send;

    /// The address type.
    type Address: Debug + Encode + From<Self::AccountId>;

    /// The signature type.
    type Signature: Debug + Clone + Encode + Decode + Send;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type Hasher: Debug + Clone + Copy + Hasher + Send + Sync;

    /// The block header.
    type Header: Debug + Header<Hasher = Self::Hasher> + Sync + Send + DeserializeOwned + Clone;

    /// This type defines the extrinsic extra and additional parameters.
    type ExtrinsicParams: ExtrinsicParams<Self>;

    /// This is used to identify an asset in the `ChargeAssetTxPayment` signed extension.
    type AssetId: Debug + Clone + Encode + DecodeAsType + EncodeAsType + Send;
}

/// Given some [`Config`], this returns the type of hash used.
pub type HashFor<T> = <<T as Config>::Hasher as Hasher>::Output;

/// given some [`Config`], this return the other params needed for its `ExtrinsicParams`.
pub type ParamsFor<T> = <<T as Config>::ExtrinsicParams as ExtrinsicParams<T>>::Params;

/// Block hashes must conform to a bunch of things to be used in Subxt.
pub trait Hash:
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
    + core::hash::Hash
{
}
impl<T> Hash for T where
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
        + core::hash::Hash
{
}

/// This represents the hasher used by a node to hash things like block headers
/// and extrinsics.
pub trait Hasher {
    /// The type given back from the hash operation
    type Output: Hash;

    /// Construct a new hasher.
    fn new(metadata: &Metadata) -> Self;

    /// Hash some bytes to the given output type.
    fn hash(&self, s: &[u8]) -> Self::Output;

    /// Hash some SCALE encodable type to the given output type.
    fn hash_of<S: Encode>(&self, s: &S) -> Self::Output {
        let out = s.encode();
        self.hash(&out)
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
    fn hash_with(&self, hasher: Self::Hasher) -> <Self::Hasher as Hasher>::Output {
        hasher.hash_of(self)
    }
}
