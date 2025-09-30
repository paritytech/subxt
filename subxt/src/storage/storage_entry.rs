// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_value::StorageValue;
use super::storage_key::StorageKey;
use subxt_core::storage::address::Address;
use frame_decode::storage::StorageInfo;
use scale_info::PortableRegistry;
use std::borrow::Cow;

/// This represents a storage entry, which is a key-value pair in the storage.
pub struct StorageEntry<'entry, 'atblock, Addr: Address> {
    key: Vec<u8>,
    // This contains the storage information already:
    value: StorageValue<'entry, 'atblock, Addr::Value>,
}

impl<'entry, 'atblock, Addr: Address> StorageEntry<'entry, 'atblock, Addr> {
    /// Create a new storage entry.
    pub fn new(
        info: &'entry StorageInfo<'atblock, u32>,
        types: &'atblock PortableRegistry,
        key: Vec<u8>,
        value: Cow<'atblock, [u8]>,
    ) -> Self {
        Self {
            key,
            value: StorageValue::new(info, types, value),
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
    pub fn key(&'_ self) -> Result<StorageKey<'_, 'atblock, Addr::KeyParts>, StorageKeyError> {
        StorageKey::new(self.value.info, self.value.types, &self.key)
    }

    /// Return the storage value.
    pub fn value(&self) -> &StorageValue<'entry, 'atblock, Addr::Value> {
        &self.value
    }
}
