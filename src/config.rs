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

use codec::{
    Codec,
    Encode,
    EncodeLike,
};
use crate::{
    SignedExtra,
    StorageEntry,
};
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
pub trait Config: Clone + Sized + Send + Sync + 'static {
    /// Account index (aka nonce) type. This stores the number of previous
    /// transactions associated with a sender account.
    type Index: Parameter + Member + Default + AtLeast32Bit + Copy + scale_info::TypeInfo;

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
        + core::hash::Hash
        + core::str::FromStr;

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
        + serde::de::DeserializeOwned;

    /// Transaction extras.
    type Extra: SignedExtra<Self> + Send + Sync + 'static;

    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;

    /// Extrinsic type within blocks.
    type Extrinsic: Parameter + Extrinsic + core::fmt::Debug + MaybeSerializeDeserialize;
}

/// Parameter trait compied from substrate::frame_support
pub trait Parameter: Codec + EncodeLike + Clone + Eq + std::fmt::Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + std::fmt::Debug {}

/// Trait to fetch data about an account.
pub trait AccountData<T: Config>: StorageEntry + From<T::AccountId> {
    /// Get the nonce from the storage entry value.
    fn nonce(result: &<Self as StorageEntry>::Value) -> T::Index;
}
