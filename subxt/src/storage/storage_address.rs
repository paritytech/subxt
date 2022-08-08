// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_map_key::StorageMapKey;
use crate::{
    dynamic::{
        DecodedValue,
        Value,
    },
    error::{
        Error,
        StorageAddressError,
    },
    metadata::{
        DecodeWithMetadata,
        EncodeWithMetadata,
        Metadata,
    },
};
use frame_metadata::StorageEntryType;
use scale_info::TypeDef;
use std::borrow::Cow;

// We use this type a bunch, so export it from here.
pub use frame_metadata::StorageHasher;

/// This represents a storage address. Anything implementing this trait
/// can be used to fetch and iterate over storage entries.
pub trait StorageAddress {
    /// The target type of the value that lives at this address.
    type Target: DecodeWithMetadata;
    /// Can an entry be fetched from this address?
    /// Set this type to [`Yes`] to enable the corresponding calls to be made.
    type IsFetchable;
    /// Can a default entry be obtained from this address?
    /// Set this type to [`Yes`] to enable the corresponding calls to be made.
    type IsDefaultable;
    /// Can this address be iterated over?
    /// Set this type to [`Yes`] to enable the corresponding calls to be made.
    type IsIterable;

    /// The name of the pallet that the entry lives under.
    fn pallet_name(&self) -> &str;

    /// The name of the entry in a given pallet that the item is at.
    fn entry_name(&self) -> &str;

    /// Output the non-prefix bytes; that is, any additional bytes that need
    /// to be appended to the key to dig into maps.
    fn append_entry_bytes(
        &self,
        metadata: &Metadata,
        bytes: &mut Vec<u8>,
    ) -> Result<(), Error>;

    /// An optional hash which, if present, will be checked against
    /// the node metadata to confirm that the return type matches what
    /// we are expecting.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

/// Used to signal whether a [`StorageAddress`] can be iterated,
/// fetched and returned with a default value in the type system.
pub struct Yes;

/// This represents a statically generated storage lookup address.
pub struct StaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable> {
    pallet_name: &'static str,
    entry_name: &'static str,
    // How to access the specific value at that storage address.
    storage_entry_keys: Vec<StorageMapKey>,
    // Hash provided from static code for validation.
    validation_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

impl<ReturnTy, Fetchable, Defaultable, Iterable>
    StaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable>
where
    ReturnTy: DecodeWithMetadata,
{
    /// Create a new [`StaticStorageAddress`] that will be validated
    /// against node metadata using the hash given.
    pub fn new(
        pallet_name: &'static str,
        entry_name: &'static str,
        storage_entry_keys: Vec<StorageMapKey>,
        hash: [u8; 32],
    ) -> Self {
        Self {
            pallet_name,
            entry_name,
            storage_entry_keys,
            validation_hash: Some(hash),
            _marker: std::marker::PhantomData,
        }
    }

    /// Do not validate this storage entry prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            validation_hash: None,
            ..self
        }
    }

    /// Return bytes representing this storage entry.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        super::utils::write_storage_address_root_bytes(self, &mut bytes);
        for entry in &self.storage_entry_keys {
            entry.to_bytes(&mut bytes);
        }
        bytes
    }

    /// Return bytes representing the root of this storage entry (ie a hash of
    /// the pallet and entry name).
    pub fn to_root_bytes(&self) -> Vec<u8> {
        super::utils::storage_address_root_bytes(self)
    }
}

impl<ReturnTy, Fetchable, Defaultable, Iterable> StorageAddress
    for StaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable>
where
    ReturnTy: DecodeWithMetadata,
{
    type Target = ReturnTy;
    type IsDefaultable = Defaultable;
    type IsIterable = Iterable;
    type IsFetchable = Fetchable;

    fn pallet_name(&self) -> &str {
        self.pallet_name
    }

    fn entry_name(&self) -> &str {
        self.entry_name
    }

    fn append_entry_bytes(
        &self,
        _metadata: &Metadata,
        bytes: &mut Vec<u8>,
    ) -> Result<(), Error> {
        for entry in &self.storage_entry_keys {
            entry.to_bytes(bytes);
        }
        Ok(())
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

/// This represents a dynamically generated storage address.
pub struct DynamicStorageAddress<'a, Encodable> {
    pallet_name: Cow<'a, str>,
    entry_name: Cow<'a, str>,
    storage_entry_keys: Vec<Encodable>,
}

/// Construct a new dynamic storage lookup to the root of some entry.
pub fn dynamic_root<'a>(
    pallet_name: impl Into<Cow<'a, str>>,
    entry_name: impl Into<Cow<'a, str>>,
) -> DynamicStorageAddress<'a, Value> {
    DynamicStorageAddress {
        pallet_name: pallet_name.into(),
        entry_name: entry_name.into(),
        storage_entry_keys: vec![],
    }
}

/// Construct a new dynamic storage lookup.
pub fn dynamic<'a, Encodable: EncodeWithMetadata>(
    pallet_name: impl Into<Cow<'a, str>>,
    entry_name: impl Into<Cow<'a, str>>,
    storage_entry_keys: Vec<Encodable>,
) -> DynamicStorageAddress<'a, Encodable> {
    DynamicStorageAddress {
        pallet_name: pallet_name.into(),
        entry_name: entry_name.into(),
        storage_entry_keys,
    }
}

impl<'a, Encodable> StorageAddress for DynamicStorageAddress<'a, Encodable>
where
    Encodable: EncodeWithMetadata,
{
    type Target = DecodedValue;

    // For dynamic types, we have no static guarantees about any of
    // this stuff, so we just allow it and let it fail at runtime:
    type IsFetchable = Yes;
    type IsDefaultable = Yes;
    type IsIterable = Yes;

    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn entry_name(&self) -> &str {
        &self.entry_name
    }

    fn append_entry_bytes(
        &self,
        metadata: &Metadata,
        bytes: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let pallet = metadata.pallet(&self.pallet_name)?;
        let storage = pallet.storage(&self.entry_name)?;

        match &storage.ty {
            StorageEntryType::Plain(_) => {
                if !self.storage_entry_keys.is_empty() {
                    Err(StorageAddressError::WrongNumberOfKeys {
                        expected: 0,
                        actual: self.storage_entry_keys.len(),
                    }
                    .into())
                } else {
                    Ok(())
                }
            }
            StorageEntryType::Map { hashers, key, .. } => {
                let ty = metadata
                    .resolve_type(key.id())
                    .ok_or_else(|| StorageAddressError::TypeNotFound(key.id()))?;

                // If the key is a tuple, we encode each value to the corresponding tuple type.
                // If the key is not a tuple, encode a single value to the key type.
                let type_ids = match ty.type_def() {
                    TypeDef::Tuple(tuple) => {
                        tuple.fields().iter().map(|f| f.id()).collect()
                    }
                    _other => {
                        vec![key.id()]
                    }
                };

                if type_ids.len() != self.storage_entry_keys.len() {
                    return Err(StorageAddressError::WrongNumberOfKeys {
                        expected: type_ids.len(),
                        actual: self.storage_entry_keys.len(),
                    }
                    .into())
                }

                if hashers.len() == 1 {
                    // One hasher; hash a tuple of all SCALE encoded bytes with the one hash function.
                    let mut input = Vec::new();
                    for (key, type_id) in self.storage_entry_keys.iter().zip(type_ids) {
                        key.encode_with_metadata(type_id, metadata, &mut input)?;
                    }
                    super::storage_map_key::hash_bytes(&input, &hashers[0], bytes);
                    Ok(())
                } else if hashers.len() == type_ids.len() {
                    // A hasher per field; encode and hash each field independently.
                    for ((key, type_id), hasher) in
                        self.storage_entry_keys.iter().zip(type_ids).zip(hashers)
                    {
                        let mut input = Vec::new();
                        key.encode_with_metadata(type_id, metadata, &mut input)?;
                        super::storage_map_key::hash_bytes(&input, hasher, bytes);
                    }
                    Ok(())
                } else {
                    // Mismatch; wrong number of hashers/fields.
                    Err(StorageAddressError::WrongNumberOfHashers {
                        hashers: hashers.len(),
                        fields: type_ids.len(),
                    }
                    .into())
                }
            }
        }
    }
}
