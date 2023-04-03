// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    dynamic::{DecodedValueThunk, Value},
    error::{Error, StorageAddressError},
    metadata::{DecodeWithMetadata, EncodeWithMetadata, Metadata},
    utils::{Encoded, Static},
};
use frame_metadata::{StorageEntryType, StorageHasher};
use scale_info::TypeDef;
use std::borrow::Cow;

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
    fn append_entry_bytes(&self, metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error>;

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

/// A concrete storage address. This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`] and [`dynamic_root`].
pub struct Address<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable> {
    pallet_name: Cow<'static, str>,
    entry_name: Cow<'static, str>,
    storage_entry_keys: Vec<StorageKey>,
    validation_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

/// A typical storage address constructed at runtime rather than via the `subxt` macro; this
/// has no restriction on what it can be used for (since we don't statically know).
pub type DynamicAddress<StorageKey> = Address<StorageKey, DecodedValueThunk, Yes, Yes, Yes>;

impl<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable>
    Address<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable>
where
    StorageKey: EncodeWithMetadata,
    ReturnTy: DecodeWithMetadata,
{
    /// Create a new [`Address`] to use to access a storage entry.
    pub fn new(
        pallet_name: impl Into<String>,
        entry_name: impl Into<String>,
        storage_entry_keys: Vec<StorageKey>,
    ) -> Self {
        Self {
            pallet_name: Cow::Owned(pallet_name.into()),
            entry_name: Cow::Owned(entry_name.into()),
            storage_entry_keys: storage_entry_keys.into_iter().collect(),
            validation_hash: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Create a new [`Address`] using static strings for the pallet and call name.
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        entry_name: &'static str,
        storage_entry_keys: Vec<StorageKey>,
        hash: [u8; 32],
    ) -> Self {
        Self {
            pallet_name: Cow::Borrowed(pallet_name),
            entry_name: Cow::Borrowed(entry_name),
            storage_entry_keys: storage_entry_keys.into_iter().collect(),
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

    /// Return bytes representing the root of this storage entry (ie a hash of
    /// the pallet and entry name). Use [`crate::storage::StorageClient::address_bytes()`]
    /// to obtain the bytes representing the entire address.
    pub fn to_root_bytes(&self) -> Vec<u8> {
        super::utils::storage_address_root_bytes(self)
    }
}

impl<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable> StorageAddress
    for Address<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable>
where
    StorageKey: EncodeWithMetadata,
    ReturnTy: DecodeWithMetadata,
{
    type Target = ReturnTy;
    type IsFetchable = Fetchable;
    type IsDefaultable = Defaultable;
    type IsIterable = Iterable;

    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn entry_name(&self) -> &str {
        &self.entry_name
    }

    fn append_entry_bytes(&self, metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error> {
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
                    .resolve_type(key.id)
                    .ok_or(StorageAddressError::TypeNotFound(key.id))?;

                // If the key is a tuple, we encode each value to the corresponding tuple type.
                // If the key is not a tuple, encode a single value to the key type.
                let type_ids = match &ty.type_def {
                    TypeDef::Tuple(tuple) => {
                        either::Either::Left(tuple.fields.iter().map(|f| f.id))
                    }
                    _other => either::Either::Right(std::iter::once(key.id)),
                };

                if type_ids.len() != self.storage_entry_keys.len() {
                    return Err(StorageAddressError::WrongNumberOfKeys {
                        expected: type_ids.len(),
                        actual: self.storage_entry_keys.len(),
                    }
                    .into());
                }

                if hashers.len() == 1 {
                    // One hasher; hash a tuple of all SCALE encoded bytes with the one hash function.
                    let mut input = Vec::new();
                    let iter = self.storage_entry_keys.iter().zip(type_ids);
                    for (key, type_id) in iter {
                        key.encode_with_metadata(type_id, metadata, &mut input)?;
                    }
                    hash_bytes(&input, &hashers[0], bytes);
                    Ok(())
                } else if hashers.len() == type_ids.len() {
                    let iter = self.storage_entry_keys.iter().zip(type_ids).zip(hashers);
                    // A hasher per field; encode and hash each field independently.
                    for ((key, type_id), hasher) in iter {
                        let mut input = Vec::new();
                        key.encode_with_metadata(type_id, metadata, &mut input)?;
                        hash_bytes(&input, hasher, bytes);
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

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

/// A static storage key; this is some pre-encoded bytes
/// likely provided by the generated interface.
pub type StaticStorageMapKey = Static<Encoded>;

// Used in codegen to construct the above.
#[doc(hidden)]
pub fn make_static_storage_map_key<T: codec::Encode>(t: T) -> StaticStorageMapKey {
    Static(Encoded(t.encode()))
}

/// Construct a new dynamic storage lookup to the root of some entry.
pub fn dynamic_root(
    pallet_name: impl Into<String>,
    entry_name: impl Into<String>,
) -> DynamicAddress<Value> {
    DynamicAddress::new(pallet_name, entry_name, vec![])
}

/// Construct a new dynamic storage lookup.
pub fn dynamic<StorageKey: EncodeWithMetadata>(
    pallet_name: impl Into<String>,
    entry_name: impl Into<String>,
    storage_entry_keys: Vec<StorageKey>,
) -> DynamicAddress<StorageKey> {
    DynamicAddress::new(pallet_name, entry_name, storage_entry_keys)
}

/// Take some SCALE encoded bytes and a [`StorageHasher`] and hash the bytes accordingly.
fn hash_bytes(input: &[u8], hasher: &StorageHasher, bytes: &mut Vec<u8>) {
    match hasher {
        StorageHasher::Identity => bytes.extend(input),
        StorageHasher::Blake2_128 => bytes.extend(sp_core_hashing::blake2_128(input)),
        StorageHasher::Blake2_128Concat => {
            bytes.extend(sp_core_hashing::blake2_128(input));
            bytes.extend(input);
        }
        StorageHasher::Blake2_256 => bytes.extend(sp_core_hashing::blake2_256(input)),
        StorageHasher::Twox128 => bytes.extend(sp_core_hashing::twox_128(input)),
        StorageHasher::Twox256 => bytes.extend(sp_core_hashing::twox_256(input)),
        StorageHasher::Twox64Concat => {
            bytes.extend(sp_core_hashing::twox_64(input));
            bytes.extend(input);
        }
    }
}
