// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use core::marker::PhantomData;
use crate::utils::{Maybe, Yes, YesMaybe};
use crate::{error::StorageError};
use super::{address::Address, PrefixOf, StorageValue, StorageKeyValue};
use alloc::vec::Vec;
use frame_decode::storage::{IntoEncodableValues, StorageInfo};
use scale_info::PortableRegistry;
use subxt_metadata::Metadata;

/// Create a [`StorageEntry`] to work with a given storage entry.
pub fn entry<'info, Addr: Address>(address: Addr, metadata: &'info Metadata) -> Result<StorageEntry<'info, Addr, Addr::IsPlain>, StorageError> {
    super::validate(&address, &metadata)?;

    use frame_decode::storage::StorageTypeInfo;
    let types = metadata.types();
    let info = metadata
        .storage_info(address.pallet_name(), address.entry_name())
        .map_err(|e| StorageError::StorageInfoError(e.into_owned()))?;

    Ok(StorageEntry {
        address,
        info,
        types,
        marker: PhantomData,
    })
}

/// This represents a single storage entry (be it a plain value or map).
pub struct StorageEntry<'info, Addr, IsPlain> {
    address: Addr,
    info: StorageInfo<'info, u32>,
    types: &'info PortableRegistry,
    marker: core::marker::PhantomData<IsPlain>
}

impl <'info, Addr: Address, IsPlain> StorageEntry<'info, Addr, IsPlain> {
    /// Name of the pallet containing this storage entry.
    pub fn pallet_name(&self) -> &str {
        self.address.pallet_name()
    }

    /// Name of the storage entry.
    pub fn entry_name(&self) -> &str {
        self.address.entry_name()
    }

    /// Is the storage entry a plain value?
    pub fn is_plain(&self) -> bool {
        self.info.keys.is_empty()
    }

    /// Is the storage entry a map?
    pub fn is_map(&self) -> bool {
        !self.is_plain()
    }

    /// Instantiate a [`StorageKeyValue`] for this entry.
    /// 
    /// It is expected that the bytes are obtained by iterating key/value pairs at this address.
    pub fn key_value(&self, key_bytes: Vec<u8>, value_bytes: Vec<u8>) -> StorageKeyValue<'_, 'info, Addr> {
        StorageKeyValue::new(&self.info, self.types, key_bytes, value_bytes.into())
    }

    /// Instantiate a [`StorageValue`] for this entry.
    /// 
    /// It is expected that the bytes are obtained by fetching a value at this address.
    pub fn value(&self, bytes: Vec<u8>) -> StorageValue<'_, 'info, Addr::Value> {
        StorageValue::new(&self.info, &self.types, bytes)
    }

    /// Return the default [`StorageValue`] for this storage entry, if there is one.
    pub fn default_value(&self) -> Option<StorageValue<'_, 'info, Addr::Value>> {
        if let Some(default_bytes) = self.info.default_value.as_deref() {
            Some(StorageValue::new(&self.info, self.types, default_bytes))
        } else {
            None
        }
    }

    /// The keys for plain storage values are always 32 byte hashes. 
    pub fn key_prefix(&self) -> [u8; 32] {
        frame_decode::storage::encode_storage_key_prefix(
            self.address.pallet_name(),
            self.address.entry_name()
        )    
    }

    // This has a less "strict" type signature and so is just used under the hood.
    fn key<Keys: IntoEncodableValues>(&self, key_parts: Keys) -> Result<Vec<u8>, StorageError> {
        let key = frame_decode::storage::encode_storage_key_with_info(
            self.address.pallet_name(),
            self.address.entry_name(),
            key_parts,
            &self.info,
            self.types
        ).map_err(StorageError::StorageKeyEncodeError)?;

        Ok(key)
    }
}

impl <'info, Addr: Address> StorageEntry<'info, Addr, Yes> {
    /// This constructs a key suitable for fetching a value at the given plain storage address.
    pub fn fetch_key(&self) -> Vec<u8> {
        self.key_prefix().to_vec()
    }
}

impl <'info, Addr: Address> StorageEntry<'info, Addr, Maybe> {
    /// This constructs a key suitable for fetching a value at the given map storage address. This will error
    /// if we can see that the wrong number of key parts are provided.
    pub fn fetch_key(&self, key_parts: Addr::KeyParts) -> Result<Vec<u8>, StorageError> {
        if key_parts.num_encodable_values() != self.info.keys.len() {
            Err(StorageError::WrongNumberOfKeyPartsProvidedForFetching { 
                expected: self.info.keys.len(), 
                got: key_parts.num_encodable_values()
            })
        } else {
            self.key(key_parts)
        }
    }

    /// This constructs a key suitable for iterating at the given storage address. This will error
    /// if we can see that too many key parts are provided.
    pub fn iter_key<Keys: PrefixOf<Addr::KeyParts>>(&self, key_parts: Keys) -> Result<Vec<u8>, StorageError> {
        if Addr::IsPlain::is_yes() {
            Err(StorageError::CannotIterPlainEntry { 
                pallet_name: self.address.pallet_name().into(), 
                entry_name: self.address.entry_name().into(), 
            })
        } else if key_parts.num_encodable_values() >= self.info.keys.len() {
            Err(StorageError::WrongNumberOfKeyPartsProvidedForIterating { 
                max_expected: self.info.keys.len() - 1, 
                got: key_parts.num_encodable_values()
            })
        } else {
            self.key(key_parts)
        }
    }
}