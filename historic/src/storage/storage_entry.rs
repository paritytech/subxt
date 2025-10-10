use super::storage_info::AnyStorageInfo;
use super::storage_key::StorageKey;
use super::storage_value::StorageValue;
use crate::error::StorageKeyError;
use std::borrow::Cow;
use std::sync::Arc;

/// This represents a storage entry, which is a key-value pair in the storage.
pub struct StorageEntry<'atblock> {
    key: Vec<u8>,
    // This contains the storage information already:
    value: StorageValue<'atblock>,
}

impl<'atblock> StorageEntry<'atblock> {
    /// Create a new storage entry.
    pub fn new(
        info: Arc<AnyStorageInfo<'atblock>>,
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

    /// Consume this storage entry and return the raw bytes for the key and value.
    pub fn into_key_and_value_bytes(self) -> (Vec<u8>, Vec<u8>) {
        (self.key, self.value.into_bytes())
    }

    /// Decode the key for this storage entry. This gives back a type from which we can
    /// decode specific parts of the key hash (where applicable).
    pub fn key(&'_ self) -> Result<StorageKey<'_, 'atblock>, StorageKeyError> {
        StorageKey::new(&self.value.info, &self.key)
    }

    /// Return the storage value.
    pub fn value(&self) -> &StorageValue<'atblock> {
        &self.value
    }
}
