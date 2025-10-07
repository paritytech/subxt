// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use frame_decode::storage::{StorageInfo, StorageKey as StorageKeyPartInfo, IntoDecodableValues};
use scale_info::PortableRegistry;
use core::marker::PhantomData;
use crate::error::StorageKeyError;

pub use frame_decode::storage::StorageHasher;

/// This represents the different parts of a storage key.
pub struct StorageKey<'bytes, 'info, KeyParts> {
    info: StorageKeyPartInfo<u32>,
    types: &'info PortableRegistry,
    bytes: &'bytes [u8],
    marker: PhantomData<KeyParts>
}

impl<'bytes, 'info, KeyParts: IntoDecodableValues> StorageKey<'bytes, 'info, KeyParts> {
    pub(crate) fn new(
        info: &StorageInfo<'info, u32>,
        types: &'info PortableRegistry,
        bytes: &'bytes [u8],
    ) -> Result<Self, StorageKeyError> {
        let cursor = &mut &*bytes;
        let storage_key_info = frame_decode::storage::decode_storage_key_with_info(
            cursor,
            &info,
            types,
        ).map_err(|e| StorageKeyError::StorageKeyDecodeError { 
            bytes: bytes.to_vec(), 
            error: e 
        })?;

        if !cursor.is_empty() {
            return Err(StorageKeyError::LeftoverBytes {
                bytes: cursor.to_vec(),
            });
        }

        Ok(StorageKey {
            info: storage_key_info,
            types,
            bytes,
            marker: PhantomData
        })
    }

    /// Attempt to decode the values contained within this storage key. The target type is
    /// given by the storage address used to access this entry. To decode into a custom type,
    /// use [`Self::parts()`] or [`Self::part()`] and decode each part.
    pub fn decode(&self) -> Result<KeyParts,StorageKeyError> {
        let values = frame_decode::storage::decode_storage_key_values(
            self.bytes, 
            &self.info, 
            self.types
        ).map_err(StorageKeyError::CannotDecodeValuesInKey)?;

        Ok(values)
    }

    /// Iterate over the parts of this storage key. Each part of a storage key corresponds to a
    /// single value that has been hashed.
    pub fn parts(&'_ self) -> impl ExactSizeIterator<Item = StorageKeyPart<'_, 'bytes, 'info>> {
        let parts_len = self.info.parts().len();
        (0..parts_len).map(move |index| StorageKeyPart {
            index,
            info: &self.info,
            types: self.types,
            bytes: self.bytes,
        })
    }

    /// Return the part of the storage key at the provided index, or `None` if the index is out of bounds.
    pub fn part(&self, index: usize) -> Option<StorageKeyPart<'_, 'bytes, 'info>> {
        if index < self.parts().len() {
            Some(StorageKeyPart {
                index,
                info: &self.info,
                types: self.types,
                bytes: self.bytes,
            })
        } else {
            None
        }
    }
}

/// This represents a part of a storage key.
pub struct StorageKeyPart<'key, 'bytes, 'info> {
    index: usize,
    info: &'key StorageKeyPartInfo<u32>,
    types: &'info PortableRegistry,
    bytes: &'bytes [u8],
}

impl<'key, 'bytes, 'info> StorageKeyPart<'key, 'bytes, 'info> {
    /// Get the raw bytes for this part of the storage key.
    pub fn bytes(&self) -> &'bytes [u8] {
        let part = &self.info[self.index];
        let hash_range = part.hash_range();
        let value_range = part
            .value()
            .map(|v| v.range())
            .unwrap_or(std::ops::Range { start: hash_range.end, end: hash_range.end });
        let combined_range = std::ops::Range {
            start: hash_range.start,
            end: value_range.end,
        };
        &self.bytes[combined_range]
    }

    /// Get the hasher that was used to construct this part of the storage key.
    pub fn hasher(&self) -> StorageHasher {
        self.info[self.index].hasher()
    }

    /// For keys that were produced using "concat" or "identity" hashers, the value
    /// is available as a part of the key hash, allowing us to decode it into anything
    /// implementing [`scale_decode::DecodeAsType`]. If the key was produced using a
    /// different hasher, this will return `None`.
    pub fn decode_as<T: scale_decode::DecodeAsType>(&self) -> Result<Option<T>, StorageKeyError> {
        let part_info = &self.info[self.index];
        let Some(value_info) = part_info.value() else {
            return Ok(None);
        };

        let value_bytes = &self.bytes[value_info.range()];
        let value_ty = value_info.ty().clone();

        let decoded_key_part = T::decode_as_type(
            &mut &*value_bytes,
            value_ty,
            self.types,
        ).map_err(|e| StorageKeyError::CannotDecodeValueInKey { index: self.index, error: e })?;

        Ok(Some(decoded_key_part))
    }
}
