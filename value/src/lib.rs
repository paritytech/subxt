// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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

/*!
This crate exposes the [`Value`] type and related subtypes, which are used as the runtime
representations of SCALE encoded data (much like `serde_json::Value` is a runtime representation
of JSON data).
*/

mod value_type;
mod scale;
#[cfg(feature = "serde")]
mod serde_impls;
#[cfg(feature = "serde")]
pub use serde_impls::{ DeserializeError, deserialize_from_value };

pub use value_type::{
    BitSequence,
    Composite,
    Primitive,
    Value,
    ValueDef,
    Variant,
};

/// Items related to decoding SCALE bytes into a [`crate::Value`].
pub mod decode {
    pub use crate::scale::{
        DecodeValueError,
        TypeId,
        BitSequenceError,
    };
    pub use scale_info::PortableRegistry;
}

/// Items related to SCALE encoding a [`crate::Value`].
pub mod encode {
    pub use crate::scale::{
        EncodeValueError,
        TypeId,
        BitSequenceError,
    };
    pub use scale_info::PortableRegistry;
}

impl Value<scale::TypeId> {
    /// Attempt to decode some SCALE encoded bytes into a value, by providing a pointer
    /// to the bytes (which will be moved forwards as bytes are used in the decoding),
    /// a type ID, and a type registry from which we'll look up the relevant type information.
    pub fn decode_as_type<Id: Into<decode::TypeId>>(
        data: &mut &[u8],
        ty_id: Id,
        types: &decode::PortableRegistry,
    ) -> Result<Value<decode::TypeId>, decode::DecodeValueError> {
        scale::decode_value_as_type(data, ty_id, types)
    }

    /// Attempt to encode some [`Value<T>`] into SCALE bytes, by providing a pointer to the
    /// type ID that we'd like to encode it as, a type registry from which we'll look
    /// up the relevant type information, and a buffer to encode the bytes to.
    pub fn encode_as_type<T, Id: Into<encode::TypeId>>(
        value: Value<T>,
        ty_id: Id,
        types: &encode::PortableRegistry,
        buf: &mut Vec<u8>
    ) -> Result<(), encode::EncodeValueError<T>> {
        scale::encode_value_as_type(value, ty_id, types, buf)
    }
}