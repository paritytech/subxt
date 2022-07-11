// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{
    Encode,
};
use sp_core::storage::{
    StorageKey,
};
pub use sp_runtime::traits::SignedExtension;
use std::{
    borrow::Cow,
};

// We use this type a bunch, so export it from here.
pub use frame_metadata::StorageHasher;

/// This is returned from storage accesses in the statically generated
/// code, and contains the information needed to find, validate and decode
/// the storage entry.
pub struct StorageAddress <'a, ReturnTy, Iterable, Defaultable> {
    pallet_name: Cow<'a, str>,
    entry_name: Cow<'a, str>,
    // How to access the specific value at that storage address.
    storage_entry_key: StorageEntryKey,
    // Hash provided from static code for validation.
    validation_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<(ReturnTy, Iterable, Defaultable)>
}

impl <'a, ReturnTy, Iterable, Defaultable> StorageAddress<'a, ReturnTy, Iterable, Defaultable> {
    /// Create a new [`StorageAddress`] that will be validated
    /// against node metadata using the hash given.
    pub fn new_with_validation(
        pallet_name: impl Into<Cow<'a, str>>,
        storage_name: impl Into<Cow<'a, str>>,
        storage_entry_key: StorageEntryKey,
        hash: [u8; 32]
    ) -> Self {
        Self {
            pallet_name: pallet_name.into(),
            entry_name: storage_name.into(),
            storage_entry_key: storage_entry_key.into(),
            validation_hash: Some(hash),
            _marker: std::marker::PhantomData
        }
    }

    /// Do not validate this storage prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            pallet_name: self.pallet_name,
            entry_name: self.entry_name,
            storage_entry_key: self.storage_entry_key,
            validation_hash: None,
            _marker: self._marker
        }
    }

    /// Strip any map keys from a storage entry. If the storage entry is pointing at
    /// a plain, non-map storage value, then this will have no effect.
    pub fn root(&mut self) {
        self.storage_entry_key = StorageEntryKey::Plain.into();
    }

    /// Append a map key to the existing storage address.
    pub fn append_map_key(&mut self, key: StorageMapKey) {
        match &mut self.storage_entry_key {
            StorageEntryKey::Plain => {
                self.storage_entry_key = StorageEntryKey::Map(vec![key]);
            },
            StorageEntryKey::Map(keys) => {
                keys.push(key);
            }
        }
    }

    /// Convert this address into bytes that we can pass to a node to look up
    /// the associated value at this address.
    pub fn to_bytes(&self) -> Vec<u8> {
        // First encode the pallet/name part:
        let mut bytes = sp_core::twox_128(self.pallet_name.as_bytes()).to_vec();
        bytes.extend(&sp_core::twox_128(self.entry_name.as_bytes())[..]);

        // Then encode any additional params to dig further into the entry:
        self.storage_entry_key.to_bytes(&mut bytes);

        bytes
    }

    /// Take a storage address and return an owned storage address.
    pub fn to_owned(self) -> StorageAddress<'static, ReturnTy, Iterable, Defaultable> {
        StorageAddress {
            pallet_name: Cow::Owned(self.pallet_name.into_owned()),
            entry_name: Cow::Owned(self.entry_name.into_owned()),
            storage_entry_key: self.storage_entry_key,
            validation_hash: self.validation_hash,
            _marker: self._marker
        }
    }

    /// Pallet name the entry lives at.
    pub fn pallet_name(&self) -> &str {
        &*self.pallet_name
    }

    /// The name of the storage entry in the pallet.
    pub fn entry_name(&self) -> &str {
        &*self.entry_name
    }

    /// A hash which validates that the address is valid.
    pub fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

impl <'a, ReturnTy, Iterable, Defaultable> From<&StorageAddress<'a, ReturnTy, Iterable, Defaultable>> for StorageKey {
    fn from(address: &StorageAddress<'a, ReturnTy, Iterable, Defaultable>) -> Self {
        StorageKey(address.to_bytes())
    }
}

/// Storage key.
#[derive(Clone)]
pub enum StorageEntryKey {
    /// Plain key.
    Plain,
    /// Map key(s).
    Map(Vec<StorageMapKey>),
}

impl StorageEntryKey {
    /// Convert this [`StorageEntryKey`] into bytes and append them to some existing bytes.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) {
        if let StorageEntryKey::Map(map) = self {
            for entry in map {
                entry.to_bytes(bytes);
            }
        }
    }
}

impl <'a> From<StorageEntryKey> for Cow<'a, StorageEntryKey> {
    fn from(k: StorageEntryKey) -> Self {
        Cow::Owned(k)
    }
}

/// Storage key for a Map.
#[derive(Clone)]
pub struct StorageMapKey {
    value: Vec<u8>,
    hasher: StorageHasher,
}

impl StorageMapKey {
    /// Create a new [`StorageMapKey`] with the encoded data and the hasher.
    pub fn new<T: Encode>(value: &T, hasher: StorageHasher) -> Self {
        Self {
            value: value.encode(),
            hasher,
        }
    }

    /// Convert this [`StorageMapKey`] into bytes and append them to some existing bytes.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) {
        match &self.hasher {
            StorageHasher::Identity => bytes.extend(&self.value),
            StorageHasher::Blake2_128 => bytes.extend(sp_core::blake2_128(bytes)),
            StorageHasher::Blake2_128Concat => {
                // adapted from substrate Blake2_128Concat::hash since StorageHasher is not public
                let v = sp_core::blake2_128(&self.value);
                let v = v.iter().chain(&self.value).cloned();
                bytes.extend(v);
            }
            StorageHasher::Blake2_256 => bytes.extend(sp_core::blake2_256(&self.value)),
            StorageHasher::Twox128 => bytes.extend(sp_core::twox_128(&self.value)),
            StorageHasher::Twox256 => bytes.extend(sp_core::twox_256(&self.value)),
            StorageHasher::Twox64Concat => {
                let v = sp_core::twox_64(&self.value);
                let v = v.iter().chain(&self.value).cloned();
                bytes.extend(v);
            }
        }
    }
}

/// If a [`StorageAddress`] is annotated with this, we can iterate over it.
pub struct AddressIsIterable;


/// If a [`StorageAddress`] is annotated with this, it has a default value
/// that we can use if it's not been set.
pub struct AddressHasDefaultValue;