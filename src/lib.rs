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

//! A library to **sub**mit e**xt**rinsics to a
//! [substrate](https://github.com/paritytech/substrate) node via RPC.

#![deny(
    bad_style,
    const_err,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    clippy::all
)]
#![allow(clippy::type_complexity)]

#[macro_use]
extern crate subxt_proc_macro;

pub use sp_core;
pub use sp_runtime;

use codec::{
    Codec,
    Decode,
    Encode,
    EncodeLike,
};
use serde::de::DeserializeOwned;
use std::{
    fmt::Debug,
    marker::PhantomData,
    sync::Arc,
};

mod client;
mod error;
mod events;
pub mod extrinsic;
mod metadata;
mod rpc;
mod subscription;

use crate::rpc::{
    ChainBlock,
    Rpc,
};
pub use crate::{
    client::{
        Client,
        ClientBuilder,
    },
    error::{
        Error,
        ModuleError,
        RuntimeError,
    },
    events::{
        EventsDecoder,
        RawEvent,
    },
    extrinsic::{
        PairSigner,
        SignedExtra,
        Signer,
        UncheckedExtrinsic,
    },
    metadata::Metadata,
    rpc::{
        BlockNumber,
        ExtrinsicSuccess,
        ReadProof,
        RpcClient,
        SystemProperties,
    },
    subscription::{
        EventStorageSubscription,
        EventSubscription,
        FinalizedEventStorageSubscription,
    },
};
pub use frame_metadata::StorageHasher;
pub use subxt_proc_macro::subxt;

use sp_runtime::traits::{
    AtLeast32Bit,
    Extrinsic,
    Hash,
    Header,
    MaybeSerializeDeserialize,
    Member,
    Verify,
};

/// Parameter trait compied from substrate::frame_support
pub trait Parameter: Codec + EncodeLike + Clone + Eq + std::fmt::Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + std::fmt::Debug {}

/// Runtime types.
pub trait Runtime: Clone + Sized + Send + Sync + 'static {
    /// Account index (aka nonce) type. This stores the number of previous
    /// transactions associated with a sender account.
    type Index: Parameter
        + Member
        + Default
        // + MaybeDisplay
        + AtLeast32Bit
        + Copy;

    /// The block number type used by the runtime.
    type BlockNumber: Parameter
        + Member
        // + MaybeMallocSizeOf
        // + MaybeSerializeDeserialize
        // + Debug
        // + MaybeDisplay
        // + AtLeast32BitUnsigned
        + Default
        // + Bounded
        + Copy
        + std::hash::Hash
        + std::str::FromStr;

    /// The output of the `Hashing` function.
    type Hash: Parameter
        + Member
        + MaybeSerializeDeserialize
        + Ord
        + Default
        + Copy
        + std::hash::Hash
        + AsRef<[u8]>
        + AsMut<[u8]>;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type Hashing: Hash<Output = Self::Hash>;

    /// The user account identifier type for the runtime.
    type AccountId: Parameter + Member; // + MaybeSerialize + MaybeDisplay + Ord + Default;

    /// The address type. This instead of `<frame_system::Trait::Lookup as StaticLookup>::Source`.
    type Address: Codec + Clone + PartialEq;
    // + Debug + Send + Sync;

    /// Data to be associated with an account (other than nonce/transaction counter, which this
    /// pallet does regardless).
    type AccountData: AccountData<Self>;

    /// The block header.
    type Header: Parameter
        + Header<Number = Self::BlockNumber, Hash = Self::Hash>
        + DeserializeOwned;

    /// Transaction extras.
    type Extra: SignedExtra<Self> + Send + Sync + 'static;

    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;

    /// Extrinsic type within blocks.
    type Extrinsic: Parameter + Extrinsic + Debug + MaybeSerializeDeserialize;
}

/// Trait to fetch data about an account.
pub trait AccountData<T: Runtime>: StorageEntry {
    /// Construct a storage entry type with the account id for the key.
    fn new(account_id: T::AccountId) -> Self;

    /// Get the nonce from the storage entry value.
    fn nonce(result: &<Self as StorageEntry>::Value) -> T::Index;
}

/// Call trait.
pub trait Call: Encode {
    /// Pallet name.
    const PALLET: &'static str;
    /// Function name.
    const FUNCTION: &'static str;
}

/// Event trait.
pub trait Event: Decode {
    /// Pallet name.
    const PALLET: &'static str;
    /// Event name.
    const EVENT: &'static str;
}

/// Storage entry trait.
pub trait StorageEntry {
    /// Pallet name.
    const PALLET: &'static str;
    /// Storage name.
    const STORAGE: &'static str;
    /// Type of the storage entry value.
    type Value: Decode;
    /// Get the key data for the storage.
    fn key(&self) -> StorageEntryKey;
}

/// Storage key.
pub enum StorageEntryKey {
    /// Plain key.
    Plain,
    /// Map key(s).
    Map(Vec<StorageMapKey>),
}

impl StorageEntryKey {
    /// Construct the final [`sp_core::storage::StorageKey`] for the storage entry.
    pub fn final_key<T: StorageEntry>(&self) -> sp_core::storage::StorageKey {
        let mut bytes = sp_core::twox_128(T::PALLET.as_bytes()).to_vec();
        bytes.extend(&sp_core::twox_128(T::STORAGE.as_bytes())[..]);
        if let Self::Map(map_keys) = self {
            for map_key in map_keys {
                bytes.extend(Self::hash(&map_key.hasher, &map_key.value))
            }
        }
        sp_core::storage::StorageKey(bytes)
    }

    fn hash(hasher: &StorageHasher, bytes: &[u8]) -> Vec<u8> {
        match hasher {
            StorageHasher::Identity => bytes.to_vec(),
            StorageHasher::Blake2_128 => sp_core::blake2_128(bytes).to_vec(),
            StorageHasher::Blake2_128Concat => {
                // copied from substrate Blake2_128Concat::hash since StorageHasher is not public
                sp_core::blake2_128(bytes)
                    .iter()
                    .chain(bytes)
                    .cloned()
                    .collect()
            }
            StorageHasher::Blake2_256 => sp_core::blake2_256(bytes).to_vec(),
            StorageHasher::Twox128 => sp_core::twox_128(bytes).to_vec(),
            StorageHasher::Twox256 => sp_core::twox_256(bytes).to_vec(),
            StorageHasher::Twox64Concat => {
                sp_core::twox_64(bytes)
                    .iter()
                    .chain(bytes)
                    .cloned()
                    .collect()
            }
        }
    }
}

/// Storage key for a Map.
pub struct StorageMapKey {
    value: Vec<u8>,
    hasher: StorageHasher,
}

impl StorageMapKey {
    /// Create a new [`StorageMapKey`] with the encoded data and the hasher.
    pub fn new<T: Encode>(value: &T, hasher: StorageHasher) -> Self {
        Self {
            value: value.encode(),
            hasher,
        }
    }
}

/// A phase of a block's execution.
#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub enum Phase {
    /// Applying an extrinsic.
    ApplyExtrinsic(u32),
    /// Finalizing the block.
    Finalization,
    /// Initializing the block.
    Initialization,
}

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}
