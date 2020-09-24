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

//! Implements support for the frame_system module.

use codec::{
    Codec,
    Decode,
    Encode,
};
use core::marker::PhantomData;
use frame_support::{
    weights::DispatchInfo,
    Parameter,
};
use serde::de::DeserializeOwned;
use sp_runtime::{
    traits::{
        AtLeast32Bit,
        AtLeast32BitUnsigned,
        Bounded,
        CheckEqual,
        Extrinsic,
        Hash,
        Header,
        MaybeDisplay,
        MaybeMallocSizeOf,
        MaybeSerialize,
        MaybeSerializeDeserialize,
        Member,
        SimpleBitOps,
    },
    DispatchError,
};
use std::fmt::Debug;

/// The subset of the `frame::Trait` that a client must implement.
#[module]
pub trait System {
    /// Account index (aka nonce) type. This stores the number of previous
    /// transactions associated with a sender account.
    type Index: Parameter
        + Member
        + MaybeSerialize
        + Debug
        + Default
        + MaybeDisplay
        + AtLeast32Bit
        + Copy;

    /// The block number type used by the runtime.
    type BlockNumber: Parameter
        + Member
        + MaybeMallocSizeOf
        + MaybeSerializeDeserialize
        + Debug
        + MaybeDisplay
        + AtLeast32BitUnsigned
        + Default
        + Bounded
        + Copy
        + std::hash::Hash
        + std::str::FromStr;

    /// The output of the `Hashing` function.
    type Hash: Parameter
        + Member
        + MaybeMallocSizeOf
        + MaybeSerializeDeserialize
        + Debug
        + MaybeDisplay
        + Ord
        + SimpleBitOps
        + Default
        + Copy
        + CheckEqual
        + std::hash::Hash
        + AsRef<[u8]>
        + AsMut<[u8]>;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    #[module(ignore)]
    type Hashing: Hash<Output = Self::Hash>;

    /// The user account identifier type for the runtime.
    type AccountId: Parameter + Member + MaybeSerialize + MaybeDisplay + Ord + Default;

    /// The address type. This instead of `<frame_system::Trait::Lookup as StaticLookup>::Source`.
    #[module(ignore)]
    type Address: Codec + Clone + PartialEq + Debug + Send + Sync;

    /// The block header.
    #[module(ignore)]
    type Header: Parameter
        + Header<Number = Self::BlockNumber, Hash = Self::Hash>
        + DeserializeOwned;

    /// Extrinsic type within blocks.
    #[module(ignore)]
    type Extrinsic: Parameter + Member + Extrinsic + Debug + MaybeSerializeDeserialize;

    /// Data to be associated with an account (other than nonce/transaction counter, which this
    /// module does regardless).
    type AccountData: Member + Codec + Clone + Default;
}

/// Type used to encode the number of references an account has.
pub type RefCount = u32;

/// Information of an account.
#[derive(Clone, Debug, Eq, PartialEq, Default, Decode, Encode)]
pub struct AccountInfo<T: System> {
    /// The number of transactions this account has sent.
    pub nonce: T::Index,
    /// The number of other modules that currently depend on this account's existence. The account
    /// cannot be reaped until this is zero.
    pub refcount: RefCount,
    /// The additional data that belongs to this account. Used to store the balance(s) in a lot of
    /// chains.
    pub data: T::AccountData,
}

/// Account field of the `System` module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AccountStore<'a, T: System> {
    #[store(returns = AccountInfo<T>)]
    /// Account to retrieve the `AccountInfo<T>` for.
    pub account_id: &'a T::AccountId,
}

/// Arguments for updating the runtime code
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SetCodeCall<'a, T: System> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Runtime wasm blob.
    pub code: &'a [u8],
}

/// Arguments for updating the runtime code without checks
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SetCodeWithoutChecksCall<'a, T: System> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Runtime wasm blob.
    pub code: &'a [u8],
}

/// A phase of a block's execution.
#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub enum Phase {
    /// Applying an extrinsic.
    ApplyExtrinsic(u32),
    /// The end.
    Finalization,
}

/// An extrinsic completed successfully.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ExtrinsicSuccessEvent<T: System> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// The dispatch info.
    pub info: DispatchInfo,
}

/// An extrinsic failed.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ExtrinsicFailedEvent<T: System> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// The dispatch error.
    pub error: DispatchError,
    /// The dispatch info.
    pub info: DispatchInfo,
}

/// `:code` was updated.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct CodeUpdatedEvent<T: System> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

/// A new account was created.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewAccountEvent<T: System> {
    /// Created account id.
    pub account: T::AccountId,
}

/// An account was reaped.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct KilledAccountEvent<T: System> {
    /// Killed account id.
    pub account: T::AccountId,
}
