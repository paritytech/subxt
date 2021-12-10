// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of subxt.
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
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

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

pub use frame_metadata::StorageHasher;
pub use subxt_macro::subxt;

pub use bitvec;
pub use codec;
pub use sp_core;
pub use sp_runtime;

use codec::{
    Decode,
    DecodeAll,
    Encode,
};
use core::{
    fmt::Debug,
    marker::PhantomData,
};

mod client;
mod config;
mod error;
mod events;
pub mod extrinsic;
mod metadata;
pub mod rpc;
pub mod storage;
mod subscription;
mod transaction;

pub use crate::{
    client::{
        Client,
        ClientBuilder,
        SubmittableExtrinsic,
    },
    config::{
        AccountData,
        Config,
        ExtrinsicExtraData,
    },
    error::{
        Error,
        PalletError,
        RuntimeError,
        TransactionError,
    },
    events::{
        EventsDecoder,
        RawEvent,
    },
    extrinsic::{
        DefaultExtra,
        PairSigner,
        SignedExtra,
        Signer,
        UncheckedExtrinsic,
    },
    metadata::{
        Metadata,
        MetadataError,
        PalletMetadata,
    },
    rpc::{
        BlockNumber,
        ReadProof,
        RpcClient,
        SystemProperties,
    },
    storage::{
        KeyIter,
        StorageEntry,
        StorageEntryKey,
        StorageMapKey,
    },
    subscription::{
        EventStorageSubscription,
        EventSubscription,
        FinalizedEventStorageSubscription,
    },
    transaction::{
        TransactionEvents,
        TransactionInBlock,
        TransactionProgress,
        TransactionStatus,
    },
};

/// Call trait.
pub trait Call: Encode {
    /// Pallet name.
    const PALLET: &'static str;
    /// Function name.
    const FUNCTION: &'static str;

    /// Returns true if the given pallet and function names match this call.
    fn is_call(pallet: &str, function: &str) -> bool {
        Self::PALLET == pallet && Self::FUNCTION == function
    }
}

/// Event trait.
pub trait Event: Decode {
    /// Pallet name.
    const PALLET: &'static str;
    /// Event name.
    const EVENT: &'static str;

    /// Returns true if the given pallet and event names match this event.
    fn is_event(pallet: &str, event: &str) -> bool {
        Self::PALLET == pallet && Self::EVENT == event
    }
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

/// A wrapper for any type `T` which implement encode/decode in a way compatible with `Vec<u8>`.
///
/// [`WrapperKeepOpaque`] stores the type only in its opaque format, aka as a `Vec<u8>`. To
/// access the real type `T` [`Self::try_decode`] needs to be used.
#[derive(Debug, Eq, PartialEq, Default, Clone, Decode, Encode)]
pub struct WrapperKeepOpaque<T> {
    data: Vec<u8>,
    _phantom: PhantomData<T>,
}

impl<T: Decode> WrapperKeepOpaque<T> {
    /// Try to decode the wrapped type from the inner `data`.
    ///
    /// Returns `None` if the decoding failed.
    pub fn try_decode(&self) -> Option<T> {
        T::decode_all(&self.data[..]).ok()
    }

    /// Returns the length of the encoded `T`.
    pub fn encoded_len(&self) -> usize {
        self.data.len()
    }

    /// Returns the encoded data.
    pub fn encoded(&self) -> &[u8] {
        &self.data
    }

    /// Create from the given encoded `data`.
    pub fn from_encoded(data: Vec<u8>) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
}
