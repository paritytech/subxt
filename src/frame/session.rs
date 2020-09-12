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

//! Session support
use crate::frame::system::{
    System,
    SystemEventsDecoder as _,
};
use codec::Encode;
use frame_support::Parameter;
use sp_runtime::traits::{
    Member,
    OpaqueKeys,
};
use std::{
    fmt::Debug,
    marker::PhantomData,
};
use substrate_subxt_proc_macro::Store;

/// Impls `Default::default` for some types that have a `_runtime` field of type
/// `PhantomData` as their only field.
macro_rules! default_impl {
    ($name:ident) => {
        impl<T: Session> Default for $name<T> {
            fn default() -> Self {
                Self {
                    _runtime: PhantomData,
                }
            }
        }
    };
}

/// The trait needed for this module.
#[module]
pub trait Session: System {
    /// The validator account identifier type for the runtime.
    type ValidatorId: Parameter + Debug + Ord + Default + Send + Sync + 'static;

    /// The keys.
    type Keys: OpaqueKeys + Member + Parameter + Default;
}

/// The current set of validators.
#[derive(Encode, Store, Debug)]
pub struct ValidatorsStore<T: Session> {
    #[store(returns = Vec<<T as Session>::ValidatorId>)]
    /// Marker for the runtime
    pub _runtime: PhantomData<T>,
}

default_impl!(ValidatorsStore);

/// Set the session keys for a validator.
#[derive(Encode, Call, Debug)]
pub struct SetKeysCall<T: Session> {
    /// The keys
    pub keys: T::Keys,
    /// The proof. This is not currently used and can be set to an empty vector.
    pub proof: Vec<u8>,
}
