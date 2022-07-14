// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::Metadata;
use crate::error::BasicError;
use codec::Decode;

/// This trait is implemented for types which can be decoded with the help of metadata.
pub trait DecodeWithMetadata: Sized {
    /// The type that we'll get back from decoding.
    type Target;
    /// Given some metadata and a type ID, attempt to SCALE decode the provided bytes into `Self`.
    fn decode_with_metadata(
        bytes: &mut &[u8],
        type_id: u32,
        metadata: &Metadata,
    ) -> Result<Self::Target, BasicError>;
}

// Things can be dynamically decoded to our Value type:
impl DecodeWithMetadata for scale_value::Value<scale_value::scale::TypeId> {
    type Target = Self;
    fn decode_with_metadata(
        bytes: &mut &[u8],
        type_id: u32,
        metadata: &Metadata,
    ) -> Result<Self::Target, BasicError> {
        let res = scale_value::scale::decode_as_type(bytes, type_id, metadata.types())?;
        Ok(res)
    }
}

/// Any type implementing [`Decode`] can also be decoded with the help of metadata.
pub struct DecodeStaticType<T>(std::marker::PhantomData<T>);

impl<T: Decode> DecodeWithMetadata for DecodeStaticType<T> {
    type Target = T;

    fn decode_with_metadata(
        bytes: &mut &[u8],
        _type_id: u32,
        _metadata: &Metadata,
    ) -> Result<Self::Target, BasicError> {
        T::decode(bytes).map_err(|e| e.into())
    }
}
