// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::address::Address;
use super::storage_key::StorageKey;
use super::storage_value::StorageValue;
use crate::error::StorageKeyError;
use frame_decode::storage::StorageInfo;
use scale_info::PortableRegistry;
use std::sync::Arc;

/// This represents a storage key/value pair, which is typically returned from
/// iterating over values in some storage map.
#[derive(Debug)]
pub struct StorageKeyValue<'info, Addr: Address> {
    key: Arc<[u8]>,
    // This contains the storage information already:
    value: StorageValue<'info, Addr::Value>,
}

impl<'info, Addr: Address> StorageKeyValue<'info, Addr> {
    pub(crate) fn new(
        info: Arc<StorageInfo<'info, u32>>,
        types: &'info PortableRegistry,
        key_bytes: Arc<[u8]>,
        value_bytes: Vec<u8>,
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

    /// Decode the key for this storage entry. This gives back a type from which we can
    /// decode specific parts of the key hash (where applicable).
    pub fn key(&'_ self) -> Result<StorageKey<'info, Addr::KeyParts>, StorageKeyError> {
        StorageKey::new(&self.value.info, self.value.types, self.key.clone())
    }

    /// Return the storage value.
    pub fn value(&self) -> &StorageValue<'info, Addr::Value> {
        &self.value
    }
}
