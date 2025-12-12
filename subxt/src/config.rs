// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides a [`Config`] type, which is used to define various
//! types that are important in order to speak to a particular chain.
//! [`SubstrateConfig`] provides a default set of these types suitable for the
//! default Substrate node implementation, and [`PolkadotConfig`] for a
//! Polkadot node.

mod default_extrinsic_params;

pub mod extrinsic_params;
pub mod polkadot;
pub mod substrate;
pub mod transaction_extensions;

use crate::metadata::{ArcMetadata, Metadata};
use codec::{Decode, Encode};
use core::fmt::Debug;
use scale_decode::DecodeAsType;
use scale_encode::EncodeAsType;
use scale_info_legacy::TypeRegistrySet;
use serde::{Serialize, de::DeserializeOwned};
use std::{fmt::Display, marker::PhantomData};
use subxt_rpcs::RpcConfig;

pub use default_extrinsic_params::{DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder};
pub use extrinsic_params::{ClientState, ExtrinsicParams, ExtrinsicParamsEncoder};
pub use polkadot::{PolkadotConfig, PolkadotExtrinsicParams, PolkadotExtrinsicParamsBuilder};
pub use substrate::{SubstrateConfig, SubstrateExtrinsicParams, SubstrateExtrinsicParamsBuilder};
pub use transaction_extensions::TransactionExtension;

/// Configuration for a given chain and the runtimes within. This consists of the
/// type information needed to work at the head of the chain (namely submitting
/// transactions), as well as functionality which we might wish to customize for a
/// given chain.
pub trait Config: Clone + Debug + Sized + Send + Sync + 'static {
    /// The account ID type; required for constructing extrinsics.
    type AccountId: Debug + Clone + Encode + Decode + Serialize + Send;

    /// The address type; required for constructing extrinsics.
    type Address: Debug + Encode + From<Self::AccountId>;

    /// The signature type.
    type Signature: Debug + Clone + Encode + Decode + Send;

    /// The block header.
    type Header: Header;

    /// This type defines the extrinsic extra and additional parameters.
    type ExtrinsicParams: ExtrinsicParams<Self>;

    /// This is used to identify an asset in the `ChargeAssetTxPayment` signed extension.
    type AssetId: Debug + Clone + Encode + DecodeAsType + EncodeAsType + Send;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    /// This is created on demand with the relevant metadata for a given block, and
    /// can then be used to hash things at that block.
    type Hasher: Hasher;

    /// The starting hash for the chain we're connecting to. This is required for constructing transactions.
    ///
    /// If not provided by the config implementation, it will be obtained from the chain in the case of the
    /// [`crate::client::OnlineClient`]. It must be provided to construct transactions via the
    /// [`crate::client::OfflineClient`], else an error will be returned.
    fn genesis_hash(&self) -> Option<HashFor<Self>> {
        None
    }

    /// Return a tuple of the spec version and then transaction version for a given block number, if available.
    ///
    /// The [`crate::client::OnlineClient`] will look this up on chain if it's not available here,
    /// but the [`crate::client::OfflineClient`] will error if this is not available for the required block number.
    fn spec_and_transaction_version_for_block_number(
        &self,
        _block_number: u64,
    ) -> Option<(u32, u32)> {
        None
    }

    /// Return the metadata for a given spec version, if available.
    ///
    /// The [`crate::client::OnlineClient`] will look this up on chain if it's not available here, and then
    /// call [`Config::set_metadata_for_spec_version`] to give the configuration the opportunity to cache it.
    /// The [`crate::client::OfflineClient`] will error if this is not available for the required spec version.
    fn metadata_for_spec_version(&self, _spec_version: u32) -> Option<ArcMetadata> {
        None
    }

    /// Set some metadata for a given spec version. the [`crate::client::OnlineClient`] will call this if it has
    /// to retrieve metadata from the chain, to give this the opportunity to cache it. The configuration can
    /// do nothing if it prefers.
    fn set_metadata_for_spec_version(&self, _spec_version: u32, _metadata: ArcMetadata) {}

    /// Return legacy types (ie types to use with Runtimes that return pre-V14 metadata) for a given spec version.
    /// If this returns `None`, [`subxt`] will return an error if type definitions are needed to access some older
    /// block.
    ///
    /// This doesn't need to live for long; it will be used to translate any older metadata returned from the node
    /// into our [`Metadata`] type, which will then be used.
    fn legacy_types_for_spec_version<'this>(
        &'this self,
        _spec_version: u32,
    ) -> Option<TypeRegistrySet<'this>> {
        None
    }
}

/// `RpcConfigFor<Config>` can be used anywhere which requires an implementation of [`subxt_rpcs::RpcConfig`].
/// This is only needed at the type level, and so there is no way to construct this.
pub struct RpcConfigFor<T> {
    marker: PhantomData<T>,
}

impl<T: Config> RpcConfig for RpcConfigFor<T> {
    type Hash = HashFor<T>;
    type Header = T::Header;
    type AccountId = T::AccountId;
}

/// Given some [`Config`], this returns the type of hash used.
pub type HashFor<T> = <<T as Config>::Hasher as Hasher>::Hash;

/// given some [`Config`], this return the other params needed for its `ExtrinsicParams`.
pub type ParamsFor<T> = <<T as Config>::ExtrinsicParams as ExtrinsicParams<T>>::Params;

/// Block hashes must conform to a bunch of things to be used in Subxt.
pub trait Hash:
    Debug
    + Display
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
        + Display
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
pub trait Hasher: Debug + Clone + Send + Sync + 'static {
    /// The type of hash produced by this hasher.
    type Hash: Hash;

    /// Construct a new hasher.
    fn new(metadata: &Metadata) -> Self;

    /// Hash some bytes to the given output type.
    fn hash(&self, s: &[u8]) -> Self::Hash;
}

/// This represents the block header type used by a node.
pub trait Header: Sized + Encode + Decode + Debug + Sync + Send + DeserializeOwned + Clone {
    /// Return the block number of this header.
    fn number(&self) -> u64;
}
