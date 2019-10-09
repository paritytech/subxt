//! Implements support for the srml_system module.
use crate::{
    error::Error,
    srml::{
        Call,
        balances::Balances,
    },
    Client,
};
use futures::future::{
    self,
    Future,
};
use parity_scale_codec::Codec;
use runtime_primitives::traits::{
    Bounded,
    CheckEqual,
    Hash,
    Header,
    MaybeDebug,
    MaybeDisplay,
    MaybeSerializeDebug,
    MaybeSerializeDebugButNotDeserialize,
    Member,
    SimpleArithmetic,
    SimpleBitOps,
    StaticLookup,
};
use runtime_support::Parameter;
use serde::de::DeserializeOwned;

/// The subset of the `srml_system::Trait` that a client must implement.
pub trait System: 'static + Eq + Clone + std::fmt::Debug {
    /// Account index (aka nonce) type. This stores the number of previous
    /// transactions associated with a sender account.
    type Index: Parameter
        + Member
        + MaybeSerializeDebugButNotDeserialize
        + Default
        + MaybeDisplay
        + SimpleArithmetic
        + Copy;

    /// The block number type used by the runtime.
    type BlockNumber: Parameter
        + Member
        + MaybeSerializeDebug
        + MaybeDisplay
        + SimpleArithmetic
        + Default
        + Bounded
        + Copy
        + std::hash::Hash;

    /// The output of the `Hashing` function.
    type Hash: Parameter
        + Member
        + MaybeSerializeDebug
        + MaybeDisplay
        + SimpleBitOps
        + Default
        + Copy
        + CheckEqual
        + std::hash::Hash
        + AsRef<[u8]>
        + AsMut<[u8]>;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type Hashing: Hash<Output = Self::Hash>;

    /// The user account identifier type for the runtime.
    type AccountId: Parameter
        + Member
        + MaybeSerializeDebug
        + MaybeDisplay
        + Ord
        + Default;

    /// The address type. This instead of `<srml_system::Trait::Lookup as StaticLookup>::Source`.
    type Address: Codec + Clone + PartialEq + MaybeDebug;

    /// The block header.
    type Header: Parameter
        + Header<Number = Self::BlockNumber, Hash = Self::Hash>
        + DeserializeOwned;
}

/// Blanket impl for using existing runtime types
impl<T: srml_system::Trait + std::fmt::Debug> System for T
where
    <T as srml_system::Trait>::Header: DeserializeOwned,
{
    type Index = T::Index;
    type BlockNumber = T::BlockNumber;
    type Hash = T::Hash;
    type Hashing = T::Hashing;
    type AccountId = T::AccountId;
    type Address = <T::Lookup as StaticLookup>::Source;
    type Header = T::Header;
}

/// The System extension trait for the Client.
pub trait SystemStore {
    /// System type.
    type System: System;

    /// Returns the account nonce for an account_id.
    fn account_nonce(
        &self,
        account_id: <Self::System as System>::AccountId,
    ) -> Box<dyn Future<Item = <Self::System as System>::Index, Error = Error> + Send>;
}

impl<T: System + Balances + 'static> SystemStore for Client<T> {
    type System = T;

    fn account_nonce(
        &self,
        account_id: <Self::System as System>::AccountId,
    ) -> Box<dyn Future<Item = <Self::System as System>::Index, Error = Error> + Send>
    {
        let account_nonce_map = || {
            Ok(self
                .metadata
                .module("System")?
                .storage("AccountNonce")?
                .get_map()?)
        };
        let map = match account_nonce_map() {
            Ok(map) => map,
            Err(err) => return Box::new(future::err(err)),
        };
        Box::new(self.fetch_or(map.key(account_id), map.default()))
    }
}

const MODULE: &str = "System";
const SET_CODE: &str = "set_code";

pub type SetCode = Vec<u8>;

/// Sets the new code.
pub fn set_code(code: Vec<u8>) -> Call<SetCode> {
    Call::new(MODULE, SET_CODE, code)
}

/// Event for the System module.
#[derive(Clone, Debug, parity_scale_codec::Decode)]
pub enum SystemEvent {
    /// An extrinsic completed successfully.
    ExtrinsicSuccess,
    /// An extrinsic failed.
    ExtrinsicFailed(runtime_primitives::DispatchError),
}
