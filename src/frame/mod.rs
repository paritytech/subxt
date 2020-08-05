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

//! Implements support for built-in runtime modules.

use crate::{
    events::EventsDecoder,
    metadata::{
        Metadata,
        MetadataError,
    },
};
use codec::{
    Decode,
    Encode,
};
use sp_core::storage::StorageKey;

pub mod balances;
pub mod contracts;
pub mod session;
pub mod staking;
pub mod sudo;
pub mod system;

/// Store trait.
pub trait Store<T>: Encode {
    /// Module name.
    const MODULE: &'static str;
    /// Field name.
    const FIELD: &'static str;
    /// Return type.
    type Returns: Decode;
    /// Returns the key prefix for storage maps
    fn prefix(metadata: &Metadata) -> Result<StorageKey, MetadataError>;
    /// Returns the `StorageKey`.
    fn key(&self, metadata: &Metadata) -> Result<StorageKey, MetadataError>;
    /// Returns the default value.
    fn default(&self, metadata: &Metadata) -> Result<Self::Returns, MetadataError> {
        Ok(metadata
            .module(Self::MODULE)?
            .storage(Self::FIELD)?
            .default()?)
    }
}

/// Call trait.
pub trait Call<T>: Encode {
    /// Module name.
    const MODULE: &'static str;
    /// Function name.
    const FUNCTION: &'static str;
    /// Load event decoder.
    fn events_decoder(_decoder: &mut EventsDecoder<T>) {}
}

/// Event trait.
pub trait Event<T>: Decode {
    /// Module name.
    const MODULE: &'static str;
    /// Event name.
    const EVENT: &'static str;
}
