// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Construct addresses to access storage entries with.

use crate::{
    dynamic::DecodedValueThunk,
    error::{Error, MetadataError},
    metadata::{DecodeWithMetadata, Metadata},
    utils::Yes,
};
use scale_decode::DecodeAsType;
use derive_where::derive_where;
use frame_decode::storage::{IntoEncodableValues, IntoDecodableValues};
use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
use alloc::vec::Vec;

/// A storage address. Concrete addresses are expected to implement either [`FetchableAddress`]
/// or [`IterableAddress`], which extends this to define fetchable and iterable storage keys.
pub trait Address {
    /// A set of types we'll hash and append to the prefix to build the storage key.
    type KeyParts: IntoEncodableValues;
    /// Type of the storage value at this location.
    type Value: DecodeAsType;

    /// The pallet containing this storage entry.
    fn pallet_name(&self) -> &str;

    /// The name of the storage entry.
    fn entry_name(&self) -> &str;

    /// Encode the suffix of the storage key for this address
    fn encode_key_suffix(&self, metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error>;

    /// Return a unique hash for this address which can be used to validate it against metadata.
    fn validation_hash(&self) -> Option<[u8; 32]>;
}

pub trait FetchableAddress: Address {
    /// Does the address have a default value defined for it. 
    /// Set to [`Yes`] to enable APIs which require one.
    type HasDefaultValue;
}

pub trait IterableAddress: Address {
    /// The storage key values that we'll decode for each value
    type OutputKeys: IntoDecodableValues;
}

/// An address which points to an individual storage value.
pub struct StaticFetchableAddress<KeyParts, Value, HasDefaultValue> {
    pallet_name: Cow<'static, str>,
    entry_name: Cow<'static, str>,
    key_parts: KeyParts,
    validation_hash: Option<[u8; 32]>,
    marker: core::marker::PhantomData<(Value, HasDefaultValue)>
}

impl <KeyParts, Value, HasDefaultValue> StaticFetchableAddress<KeyParts, Value, HasDefaultValue> {
    /// Create a new [`StaticFetchableAddress`] using static strings for the pallet and call name.
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        entry_name: &'static str,
        key_parts: KeyParts,
        hash: [u8; 32],
    ) -> Self {
        Self {
            pallet_name: Cow::Borrowed(pallet_name),
            entry_name: Cow::Borrowed(entry_name),
            key_parts,
            validation_hash: Some(hash),
            marker: core::marker::PhantomData,
        }
    }

    /// Create a new [`StaticFetchableAddress`].
    pub fn new(
        pallet_name: impl Into<Cow<'static, str>>,
        entry_name: impl Into<Cow<'static, str>>,
        key_parts: KeyParts,
    ) -> Self {
        Self {
            pallet_name: pallet_name.into(),
            entry_name: entry_name.into(),
            key_parts,
            validation_hash: None,
            marker: core::marker::PhantomData,
        }
    }

    /// Do not validate this storage entry prior to accessing it.
    pub fn unvalidated(mut self) -> Self {
        self.validation_hash = None;
        self
    }
}

impl <KeyParts, Value, HasDefaultValue> Address for StaticFetchableAddress<KeyParts, Value, HasDefaultValue> 
where
    KeyParts: IntoEncodableValues,
    Value: DecodeAsType
{
    type KeyParts = KeyParts;
    type Value = Value;
    
    fn encode_key_suffix(&self, metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error> {
        frame_decode::storage::encode_storage_key_suffix(
            &self.pallet_name, 
            &self.entry_name, 
            &self.key_parts,
            metadata.types(),
            metadata
        ).map_err(Into::into)
    }

    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn entry_name(&self) -> &str {
        &self.entry_name
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

impl <KeyParts, Value, HasDefaultValue> FetchableAddress for StaticFetchableAddress<KeyParts, Value, HasDefaultValue> 
where
    KeyParts: IntoEncodableValues,
    Value: DecodeAsType
{
    type HasDefaultValue = HasDefaultValue;
}

/// An address which points to a set of storage values.
pub struct StaticIterableAddress<InputKeyParts, OutputKeyParts, Value> {
    pallet_name: Cow<'static, str>,
    entry_name: Cow<'static, str>,
    input_key_parts: InputKeyParts,
    validation_hash: Option<[u8; 32]>,
    marker: core::marker::PhantomData<(OutputKeyParts, Value)>
}

impl <InputKeyParts, OutputKeyParts, Value> StaticIterableAddress<InputKeyParts, OutputKeyParts, Value> {
    /// Create a new [`StaticIterableAddress`] using static strings for the pallet and call name.
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        entry_name: &'static str,
        input_key_parts: InputKeyParts,
        hash: [u8; 32],
    ) -> Self {
        Self {
            pallet_name: Cow::Borrowed(pallet_name),
            entry_name: Cow::Borrowed(entry_name),
            input_key_parts,
            validation_hash: Some(hash),
            marker: core::marker::PhantomData,
        }
    }

    /// Create a new [`StaticIterableAddress`].
    pub fn new(
        pallet_name: impl Into<Cow<'static, str>>,
        entry_name: impl Into<Cow<'static, str>>,
        input_key_parts: InputKeyParts,
    ) -> Self {
        Self {
            pallet_name: pallet_name.into(),
            entry_name: entry_name.into(),
            input_key_parts,
            validation_hash: None,
            marker: core::marker::PhantomData,
        }
    }

    /// Do not validate this storage entry prior to accessing it.
    pub fn unvalidated(mut self) -> Self {
        self.validation_hash = None;
        self
    }
}

impl <InputKeyParts, OutputKeyParts, Value> Address for StaticIterableAddress<InputKeyParts, OutputKeyParts, Value> 
where
    InputKeyParts: IntoEncodableValues,
    Value: DecodeAsType
{
    type KeyParts = InputKeyParts;
    type Value = Value;
    
    fn encode_key_suffix(&self, metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error> {
        frame_decode::storage::encode_storage_key_suffix(
            &self.pallet_name, 
            &self.entry_name, 
            &self.input_key_parts,
            metadata.types(),
            metadata
        ).map_err(Into::into)
    }

    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn entry_name(&self) -> &str {
        &self.entry_name
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

impl <InputKeyParts, OutputKeyParts, Value> IterableAddress for StaticIterableAddress<InputKeyParts, OutputKeyParts, Value> 
where
    InputKeyParts: IntoEncodableValues,
    OutputKeyParts: IntoDecodableValues,
    Value: DecodeAsType
{
    type OutputKeys = OutputKeyParts;
}





/// A concrete storage address. This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`].
#[derive_where(Clone, Debug, Eq, Ord, PartialEq, PartialOrd; Keys)]
pub struct DefaultAddress<Keys: StorageKey, ReturnTy, Fetchable, Defaultable, Iterable> {
    pallet_name: Cow<'static, str>,
    entry_name: Cow<'static, str>,
    keys: Keys,
    validation_hash: Option<[u8; 32]>,
    _marker: core::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

/// A storage address constructed by the static codegen.
pub type StaticAddress<Keys, ReturnTy, Fetchable, Defaultable, Iterable> =
    DefaultAddress<Keys, ReturnTy, Fetchable, Defaultable, Iterable>;
/// A typical storage address constructed at runtime rather than via the `subxt` macro; this
/// has no restriction on what it can be used for (since we don't statically know).
pub type DynamicAddress<Keys> = DefaultAddress<Keys, DecodedValueThunk, Yes, Yes, Yes>;

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
    DefaultAddress<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
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
    DefaultAddress<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
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

    /// Return bytes representing the root of this storage entry (a hash of the pallet and entry name).
    pub fn to_root_bytes(&self) -> Vec<u8> {
        super::get_address_root_bytes(self)
    }
}

impl<Keys, ReturnTy, Fetchable, Defaultable, Iterable> Address
    for DefaultAddress<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
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
