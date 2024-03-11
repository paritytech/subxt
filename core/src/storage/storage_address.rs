// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    dynamic::DecodedValueThunk,
    error::{Error, MetadataError},
    metadata::{DecodeWithMetadata, Metadata},
    Yes
};
use derivative::Derivative;

use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
use alloc::vec::Vec;


use super::{storage_key::StorageHashers, StorageKey};

/// This represents a storage address. Anything implementing this trait
/// can be used to fetch and iterate over storage entries.
pub trait StorageAddress {
    /// The target type of the value that lives at this address.
    type Target: DecodeWithMetadata;
    /// The keys type used to construct this address.
    type Keys: StorageKey;
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

/// A concrete storage address. This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`].
#[derive(Derivative)]
#[derivative(
    Clone(bound = "Keys: Clone"),
    Debug(bound = "Keys: core::fmt::Debug"),
    Eq(bound = "Keys: core::cmp::Eq"),
    Ord(bound = "Keys: core::cmp::Ord"),
    PartialEq(bound = "Keys: core::cmp::PartialEq"),
    PartialOrd(bound = "Keys: core::cmp::PartialOrd")
)]
pub struct Address<Keys: StorageKey, ReturnTy, Fetchable, Defaultable, Iterable> {
    pallet_name: Cow<'static, str>,
    entry_name: Cow<'static, str>,
    keys: Keys,
    validation_hash: Option<[u8; 32]>,
    _marker: core::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

/// A typical storage address constructed at runtime rather than via the `subxt` macro; this
/// has no restriction on what it can be used for (since we don't statically know).
pub type DynamicAddress<Keys> = Address<Keys, DecodedValueThunk, Yes, Yes, Yes>;

impl<Keys: StorageKey> DynamicAddress<Keys> {
    /// Creates a new dynamic address. As `Keys` you can use a `Vec<scale_value::Value>`
    pub fn new(pallet_name: impl Into<String>, entry_name: impl Into<String>, keys: Keys) -> Self {
        Self {
            pallet_name: Cow::Owned(pallet_name.into()),
            entry_name: Cow::Owned(entry_name.into()),
            keys,
            validation_hash: None,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
    Address<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
where
    Keys: StorageKey,
    ReturnTy: DecodeWithMetadata,
{
    /// Create a new [`Address`] using static strings for the pallet and call name.
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        entry_name: &'static str,
        keys: Keys,
        hash: [u8; 32],
    ) -> Self {
        Self {
            pallet_name: Cow::Borrowed(pallet_name),
            entry_name: Cow::Borrowed(entry_name),
            keys,
            validation_hash: Some(hash),
            _marker: core::marker::PhantomData,
        }
    }
}

impl<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
    Address<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
where
    Keys: StorageKey,
    ReturnTy: DecodeWithMetadata,
{
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

impl<Keys, ReturnTy, Fetchable, Defaultable, Iterable> StorageAddress
    for Address<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
where
    Keys: StorageKey,
    ReturnTy: DecodeWithMetadata,
{
    type Target = ReturnTy;
    type Keys = Keys;
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
        let pallet = metadata.pallet_by_name_err(self.pallet_name())?;
        let storage = pallet
            .storage()
            .ok_or_else(|| MetadataError::StorageNotFoundInPallet(self.pallet_name().to_owned()))?;
        let entry = storage
            .entry_by_name(self.entry_name())
            .ok_or_else(|| MetadataError::StorageEntryNotFound(self.entry_name().to_owned()))?;

        let hashers = StorageHashers::new(entry.entry_type(), metadata.types())?;
        self.keys
            .encode_storage_key(bytes, &mut hashers.iter(), metadata.types())?;
        Ok(())
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

/// Construct a new dynamic storage lookup.
pub fn dynamic<Keys: StorageKey>(
    pallet_name: impl Into<String>,
    entry_name: impl Into<String>,
    storage_entry_keys: Keys,
) -> DynamicAddress<Keys> {
    DynamicAddress::new(pallet_name, entry_name, storage_entry_keys)
}
