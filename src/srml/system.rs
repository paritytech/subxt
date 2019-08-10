use crate::{
    error::Error,
    Client,
};
use futures::future::Future;
use runtime_primitives::traits::{
    Bounded,
    CheckEqual,
    Hash,
    MaybeDisplay,
    MaybeSerializeDebug,
    MaybeSerializeDebugButNotDeserialize,
    Member,
    SignedExtension,
    SimpleArithmetic,
    SimpleBitOps,
    StaticLookup,
};
use runtime_support::Parameter;
use srml_system::Event;

/// The subset of the `srml_system::Trait` that a client must implement.
pub trait System {
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

    /// Converting trait to take a source type and convert to `AccountId`.
    ///
    /// Used to define the type and conversion mechanism for referencing
    /// accounts in transactions. It's perfectly reasonable for this to be an
    /// identity conversion (with the source type being `AccountId`), but other
    /// modules (e.g. Indices module) may provide more functional/efficient
    /// alternatives.
    type Lookup: StaticLookup<Target = Self::AccountId>;

    /// The aggregated event type of the runtime.
    type Event: Parameter + Member + From<Event>;

    type SignedExtra: SignedExtension;

    fn extra(nonce: Self::Index) -> Self::SignedExtra;
}

pub trait SystemStore {
    type System: System;

    fn account_nonce(
        &self,
        account_id: <Self::System as System>::AccountId,
    ) -> Box<dyn Future<Item = <Self::System as System>::Index, Error = Error> + Send>;
}

impl<T: System + 'static> SystemStore for Client<T> {
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
            Err(err) => return Box::new(futures::future::err(err)),
        };
        Box::new(self.fetch_or(map.key(account_id), map.default()))
    }
}
