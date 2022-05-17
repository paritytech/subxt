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

//! This crate exposes the [`Value`] type and related subtypes, which are used as the runtime
//! representations of SCALE encoded data (much like `serde_json::Value` is a runtime representation
//! of JSON data).

mod scale_impls;
#[cfg(feature = "serde")]
mod serde_impls;
mod value_type;

pub use value_type::{
    BitSequence,
    Composite,
    Primitive,
    Value,
    ValueDef,
    Variant,
};

/// Serializing and deserializing a [`crate::Value`] into/from other types via serde.
#[cfg(feature = "serde")]
pub mod serde {
    pub use crate::serde_impls::DeserializeError;

    /// Attempt to deserialize a [`crate::Value`] into another type.
    pub fn from_value<'de, Ctx, T: serde::Deserialize<'de>>(
        value: crate::Value<Ctx>,
    ) -> Result<T, DeserializeError> {
        T::deserialize(value)
    }

    // TODO: Eventually let's implement Serializer on Value so that we can convert from some type into a Value:
    //
    // /// Attempt to serialize some type into a [`crate::Value`].
    // pub fn to_value<'de, Ctx, T: serde::Serialize>(ty: T) -> Result<crate::Value<()>, SerializeError> {
    //     ty.serialize(serializer)
    // }
}

/// Encoding and decoding SCALE bytes into a [`crate::Value`].
pub mod scale {
    pub use crate::scale_impls::{
        BitSequenceError,
        DecodeError,
        EncodeError,
        TypeId,
    };
    pub use scale_info::PortableRegistry;

    /// Attempt to decode some SCALE encoded bytes into a value, by providing a pointer
    /// to the bytes (which will be moved forwards as bytes are used in the decoding),
    /// a type ID, and a type registry from which we'll look up the relevant type information.
    pub fn decode_as_type<Id: Into<TypeId>>(
        data: &mut &[u8],
        ty_id: Id,
        types: &PortableRegistry,
    ) -> Result<crate::Value<TypeId>, DecodeError> {
        crate::scale_impls::decode_value_as_type(data, ty_id, types)
    }

    /// Attempt to encode some [`Value<T>`] into SCALE bytes, by providing a pointer to the
    /// type ID that we'd like to encode it as, a type registry from which we'll look
    /// up the relevant type information, and a buffer to encode the bytes to.
    pub fn encode_as_type<T, Id: Into<TypeId>>(
        value: crate::Value<T>,
        ty_id: Id,
        types: &PortableRegistry,
        buf: &mut Vec<u8>,
    ) -> Result<(), EncodeError<T>> {
        crate::scale_impls::encode_value_as_type(value, ty_id, types, buf)
    }
}
