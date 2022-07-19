// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::Metadata;
use crate::{
    dynamic::DecodedValue,
    error::Error,
};
use codec::Decode;
use frame_metadata::StorageEntryType;

/// This trait is implemented for types which can be decoded with the help of metadata.
pub trait DecodeWithMetadata {
    /// The type that we'll get back from decoding.
    type Target;
    /// Given some metadata and a type ID, attempt to SCALE decode the provided bytes into `Self`.
    fn decode_with_metadata(
        bytes: &mut &[u8],
        type_id: u32,
        metadata: &Metadata,
    ) -> Result<Self::Target, Error>;

    /// Decode a storage item using metadata. By default, this uses the metadata to
    /// work out the type ID to use, but for static items we can short circuit this
    /// lookup.
    fn decode_storage_with_metadata(
        bytes: &mut &[u8],
        pallet_name: &str,
        storage_entry: &str,
        metadata: &Metadata,
    ) -> Result<Self::Target, Error> {
        let ty = &metadata.pallet(pallet_name)?.storage(storage_entry)?.ty;

        let id = match ty {
            StorageEntryType::Plain(ty) => ty.id(),
            StorageEntryType::Map { value, .. } => value.id(),
        };

        Self::decode_with_metadata(bytes, id, metadata)
    }
}

// Things can be dynamically decoded to our Value type:
impl DecodeWithMetadata for DecodedValue {
    type Target = Self;
    fn decode_with_metadata(
        bytes: &mut &[u8],
        type_id: u32,
        metadata: &Metadata,
    ) -> Result<Self::Target, Error> {
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
    ) -> Result<Self::Target, Error> {
        T::decode(bytes).map_err(|e| e.into())
    }

    fn decode_storage_with_metadata(
        bytes: &mut &[u8],
        _pallet_name: &str,
        _storage_entry: &str,
        _metadata: &Metadata,
    ) -> Result<Self::Target, Error> {
        T::decode(bytes).map_err(|e| e.into())
    }
}
