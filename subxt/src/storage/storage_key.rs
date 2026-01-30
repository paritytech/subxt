// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::StorageKeyError;
use core::marker::PhantomData;
use frame_decode::storage::{IntoDecodableValues, StorageInfo, StorageKey as StorageKeyPartInfo};
use scale_info::PortableRegistry;
use std::sync::Arc;

pub use frame_decode::storage::StorageHasher;

/// This represents the different parts of a storage key.
pub struct StorageKey<'info, KeyParts> {
    info: Arc<StorageKeyPartInfo<u32>>,
    types: &'info PortableRegistry,
    bytes: Arc<[u8]>,
    marker: PhantomData<KeyParts>,
}

impl<'info, KeyParts: IntoDecodableValues> StorageKey<'info, KeyParts> {
    pub(crate) fn new(
        info: &StorageInfo<'info, u32>,
        types: &'info PortableRegistry,
        bytes: Arc<[u8]>,
    ) -> Result<Self, StorageKeyError> {
        let cursor = &mut &*bytes;
        let storage_key_info = frame_decode::storage::decode_storage_key_with_info(
            cursor, info, types,
        )
        .map_err(|e| StorageKeyError::StorageKeyDecodeError {
            bytes: bytes.to_vec(),
            error: e,
        })?;

        if !cursor.is_empty() {
            return Err(StorageKeyError::LeftoverBytes {
                bytes: cursor.to_vec(),
            });
        }

        Ok(StorageKey {
            info: Arc::new(storage_key_info),
            types,
            bytes,
            marker: PhantomData,
        })
    }

    /// Attempt to decode the values contained within this storage key. The target type is
    /// given by the storage address used to access this entry. To decode into a custom type,
    /// use [`Self::parts()`] or [`Self::part()`] and decode each part.
    pub fn decode(&self) -> Result<KeyParts, StorageKeyError> {
        let values =
            frame_decode::storage::decode_storage_key_values(&self.bytes, &self.info, self.types)
                .map_err(StorageKeyError::CannotDecodeValuesInKey)?;

        Ok(values)
    }

    /// Iterate over the parts of this storage key. Each part of a storage key corresponds to a
    /// single value that has been hashed.
    pub fn parts(&self) -> impl ExactSizeIterator<Item = StorageKeyPart<'info>> {
        let parts_len = self.info.parts().len();
        (0..parts_len).map(move |index| StorageKeyPart {
            index,
            info: self.info.clone(),
            types: self.types,
            bytes: self.bytes.clone(),
        })
    }

    /// Return the part of the storage key at the provided index, or `None` if the index is out of bounds.
    pub fn part(&self, index: usize) -> Option<StorageKeyPart<'info>> {
        if index < self.parts().len() {
            Some(StorageKeyPart {
                index,
                info: self.info.clone(),
                types: self.types,
                bytes: self.bytes.clone(),
            })
        } else {
            None
        }
    }
}

/// This represents a part of a storage key.
pub struct StorageKeyPart<'info> {
    index: usize,
    info: Arc<StorageKeyPartInfo<u32>>,
    types: &'info PortableRegistry,
    bytes: Arc<[u8]>,
}

impl<'info> StorageKeyPart<'info> {
    /// Get all of the raw bytes for this part of the storage key.
    pub fn bytes(&self) -> &[u8] {
        let part = &self.info[self.index];
        let hash_range = part.hash_range();
        let value_range = part.value().map(|v| v.range()).unwrap_or(core::ops::Range {
            start: hash_range.end,
            end: hash_range.end,
        });
        let combined_range = core::ops::Range {
            start: hash_range.start,
            end: value_range.end,
        };
        &self.bytes[combined_range]
    }

    /// Get the bytes corresponding to the hash part of the key.
    ///
    /// - For `Blake2_128Concat` and `Twox64Concat` hashers, this will return only the
    ///   BlakeTwo128 or Twox64 hash and _not_ the value that's been concatenated onto it.
    /// - For the `Identity` hasher, no bytes will be returned (since it contains only the
    ///   encoded value and no hash)
    /// - For other hashers, all of the bytes will be returned since they all correspond to
    ///   the hash and there is no additional value concatenated.
    pub fn hash_bytes(&self) -> &[u8] {
        let part = &self.info[self.index];
        let hash_range = part.hash_range();
        &self.bytes[hash_range]
    }

    /// Get the bytes corresponding to the value part of the key.
    ///
    /// - For `Blake2_128Concat` and `Twox64Concat` hashers, this will return only the
    ///   bytes corresponding to the value that is concatenated after the hash.
    /// - For the `Identity` hasher, all of the bytes are returned since they all
    ///   correspond to the value.
    /// - For other hashers, no bytes will be returned since there is no value concatenated.
    pub fn value_bytes(&self) -> &[u8] {
        let part = &self.info[self.index];
        part.value()
            .map(|v| &self.bytes[v.range()])
            .unwrap_or_default()
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
        let value_ty = *value_info.ty();

        let decoded_key_part = T::decode_as_type(&mut &*value_bytes, value_ty, self.types)
            .map_err(|e| StorageKeyError::CannotDecodeValueInKey {
                index: self.index,
                error: e,
            })?;

        Ok(Some(decoded_key_part))
    }
}
