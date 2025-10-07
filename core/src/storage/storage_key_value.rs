// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{Address, StorageKey, StorageValue};
use crate::error::StorageKeyError;
use frame_decode::storage::StorageInfo;
use scale_info::PortableRegistry;
use std::borrow::Cow;

/// This represents a storage key/value pair, which is typically returned from
/// iterating over values in some storage map.
pub struct StorageKeyValue<'entry, 'info, Addr: Address> {
    key: Vec<u8>,
    // This contains the storage information already:
    value: StorageValue<'entry, 'info, Addr::Value>,
}

impl<'entry, 'info, Addr: Address> StorageKeyValue<'entry, 'info, Addr> {
    pub(crate) fn new(
        info: &'entry StorageInfo<'info, u32>,
        types: &'info PortableRegistry,
        key_bytes: Vec<u8>,
        value_bytes: Cow<'info, [u8]>,
    ) -> Self {
        StorageKeyValue {
            key: key_bytes,
            value: StorageValue::new(info, types, value_bytes),
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
    pub fn key(&'_ self) -> Result<StorageKey<'_, 'info, Addr::KeyParts>, StorageKeyError> {
        StorageKey::new(self.value.info, self.value.types, &self.key)
    }

    /// Return the storage value.
    pub fn value(&self) -> &StorageValue<'entry, 'info, Addr::Value> {
        &self.value
    }
}
