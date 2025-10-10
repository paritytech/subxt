// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{PrefixOf, StorageKeyValue, StorageValue, address::Address};
use crate::error::StorageError;
use crate::utils::{Maybe, Yes, YesMaybe};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::marker::PhantomData;
use frame_decode::storage::{IntoEncodableValues, StorageInfo};
use scale_info::PortableRegistry;
use subxt_metadata::Metadata;

/// Create a [`StorageEntry`] to work with a given storage entry.
pub fn entry<'info, Addr: Address>(
    address: Addr,
    metadata: &'info Metadata,
) -> Result<StorageEntry<'info, Addr, Addr::IsPlain>, StorageError> {
    super::validate(&address, &metadata)?;

    use frame_decode::storage::StorageTypeInfo;
    let types = metadata.types();
    let info = metadata
        .storage_info(address.pallet_name(), address.entry_name())
        .map_err(|e| StorageError::StorageInfoError(e.into_owned()))?;

    Ok(StorageEntry(Arc::new(StorageEntryInner {
        address,
        info: Arc::new(info),
        types,
        marker: PhantomData,
    })))
}

/// This represents a single storage entry (be it a plain value or map).
pub struct StorageEntry<'info, Addr, IsPlain>(Arc<StorageEntryInner<'info, Addr, IsPlain>>);

impl<'info, Addr, IsPlain> Clone for StorageEntry<'info, Addr, IsPlain> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct StorageEntryInner<'info, Addr, IsPlain> {
    address: Addr,
    info: Arc<StorageInfo<'info, u32>>,
    types: &'info PortableRegistry,
    marker: core::marker::PhantomData<IsPlain>,
}

impl<'info, Addr: Address, IsPlain> StorageEntry<'info, Addr, IsPlain> {
    /// Name of the pallet containing this storage entry.
    pub fn pallet_name(&self) -> &str {
        self.0.address.pallet_name()
    }

    /// Name of the storage entry.
    pub fn entry_name(&self) -> &str {
        self.0.address.entry_name()
    }

    /// Is the storage entry a plain value?
    pub fn is_plain(&self) -> bool {
        self.0.info.keys.is_empty()
    }

    /// Is the storage entry a map?
    pub fn is_map(&self) -> bool {
        !self.is_plain()
    }

    /// Instantiate a [`StorageKeyValue`] for this entry.
    ///
    /// It is expected that the bytes are obtained by iterating key/value pairs at this address.
    pub fn key_value(
        &self,
        key_bytes: impl Into<Arc<[u8]>>,
        value_bytes: Vec<u8>,
    ) -> StorageKeyValue<'info, Addr> {
        StorageKeyValue::new(
            self.0.info.clone(),
            self.0.types,
            key_bytes.into(),
            value_bytes,
        )
    }

    /// Instantiate a [`StorageValue`] for this entry.
    ///
    /// It is expected that the bytes are obtained by fetching a value at this address.
    pub fn value(&self, bytes: Vec<u8>) -> StorageValue<'info, Addr::Value> {
        StorageValue::new(self.0.info.clone(), self.0.types, bytes)
    }

    /// Return the default [`StorageValue`] for this storage entry, if there is one.
    pub fn default_value(&self) -> Option<StorageValue<'info, Addr::Value>> {
        if let Some(default_bytes) = self.0.info.default_value.as_deref() {
            Some(StorageValue::new(
                self.0.info.clone(),
                self.0.types,
                default_bytes.to_vec(),
            ))
        } else {
            None
        }
    }

    /// The keys for plain storage values are always 32 byte hashes.
    pub fn key_prefix(&self) -> [u8; 32] {
        frame_decode::storage::encode_storage_key_prefix(
            self.0.address.pallet_name(),
            self.0.address.entry_name(),
        )
    }

    // This has a less "strict" type signature and so is just used under the hood.
    fn key<Keys: IntoEncodableValues>(&self, key_parts: Keys) -> Result<Vec<u8>, StorageError> {
        let key = frame_decode::storage::encode_storage_key_with_info(
            self.0.address.pallet_name(),
            self.0.address.entry_name(),
            key_parts,
            &self.0.info,
            self.0.types,
        )
        .map_err(StorageError::StorageKeyEncodeError)?;

        Ok(key)
    }
}

impl<'info, Addr: Address> StorageEntry<'info, Addr, Yes> {
    /// This constructs a key suitable for fetching a value at the given plain storage address.
    pub fn fetch_key(&self) -> Vec<u8> {
        self.key_prefix().to_vec()
    }
}

impl<'info, Addr: Address> StorageEntry<'info, Addr, Maybe> {
    /// This constructs a key suitable for fetching a value at the given map storage address. This will error
    /// if we can see that the wrong number of key parts are provided.
    pub fn fetch_key(&self, key_parts: Addr::KeyParts) -> Result<Vec<u8>, StorageError> {
        if key_parts.num_encodable_values() != self.0.info.keys.len() {
            Err(StorageError::WrongNumberOfKeyPartsProvidedForFetching {
                expected: self.0.info.keys.len(),
                got: key_parts.num_encodable_values(),
            })
        } else {
            self.key(key_parts)
        }
    }

    /// This constructs a key suitable for iterating at the given storage address. This will error
    /// if we can see that too many key parts are provided.
    pub fn iter_key<Keys: PrefixOf<Addr::KeyParts>>(
        &self,
        key_parts: Keys,
    ) -> Result<Vec<u8>, StorageError> {
        if Addr::IsPlain::is_yes() {
            Err(StorageError::CannotIterPlainEntry {
                pallet_name: self.0.address.pallet_name().into(),
                entry_name: self.0.address.entry_name().into(),
            })
        } else if key_parts.num_encodable_values() >= self.0.info.keys.len() {
            Err(StorageError::WrongNumberOfKeyPartsProvidedForIterating {
                max_expected: self.0.info.keys.len() - 1,
                got: key_parts.num_encodable_values(),
            })
        } else {
            self.key(key_parts)
        }
    }
}
