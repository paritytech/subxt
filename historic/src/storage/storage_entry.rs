use super::storage_info::AnyStorageInfo;
use super::storage_key::StorageKey;
use super::storage_value::StorageValue;
use crate::error::{StorageKeyError, StorageValueError};
use scale_decode::DecodeAsType;
use std::borrow::Cow;

/// This represents a storage entry, which is a key-value pair in the storage.
pub struct StorageEntry<'entry, 'atblock> {
    key: Vec<u8>,
    // This contains the storage information already:
    value: StorageValue<'entry, 'atblock>,
}

impl<'entry, 'atblock> StorageEntry<'entry, 'atblock> {
    /// Create a new storage entry.
    pub fn new(
        info: &'entry AnyStorageInfo<'atblock>,
        key: Vec<u8>,
        value: Cow<'atblock, [u8]>,
    ) -> Self {
        Self {
            key,
            value: StorageValue::new(info, value),
        }
    }

    /// Get the raw bytes for this storage entry's key.
    pub fn key_bytes(&self) -> &[u8] {
        &self.key
    }

    /// Get the raw bytes for this storage entry's value.
    pub fn value_bytes(&self) -> &[u8] {
        self.value.bytes()
    }

    /// Consume this storage entry and return the raw bytes for the key and value.
    pub fn into_key_and_value_bytes(self) -> (Vec<u8>, Vec<u8>) {
        (self.key, self.value.into_bytes())
    }

    /// Decode the key for this storage entry. This gives back a type from which we can
    /// decode specific parts of the key hash (where applicable).
    pub fn decode_key(&'_ self) -> Result<StorageKey<'_, 'atblock>, StorageKeyError> {
        StorageKey::new(self.value.info, &self.key)
    }

    /// Decode this storage value.
    pub fn decode_value<T: DecodeAsType>(&self) -> Result<T, StorageValueError> {
        self.value.decode::<T>()
    }
}
