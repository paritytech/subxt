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
extern crate substrate_subxt_proc_macro;

pub use sp_core;
pub use sp_runtime;

use codec::{Codec, Decode, EncodeLike, Encode};
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

pub use crate::{
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
    substrate_subxt_proc_macro::*,
};
use crate::{
    rpc::{
        ChainBlock,
        Rpc,
    },
};
use crate::sp_runtime::traits::{Verify, Extrinsic, Member, Hash, Header, AtLeast32Bit, MaybeSerializeDeserialize};

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

/// Event trait.
pub trait Event<T>: Decode {
    /// Module name.
    const MODULE: &'static str;
    /// Event name.
    const EVENT: &'static str;
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
