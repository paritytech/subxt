// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Construct addresses to access storage entries with.

use alloc::borrow::Cow;
use alloc::vec::Vec;
use frame_decode::storage::{IntoDecodableValues, IntoEncodableValues};
use scale_decode::DecodeAsType;

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

    /// Return the input key parts needed to point to this storage entry / entries.
    fn key_parts(&self) -> impl IntoEncodableValues;

    /// Return a unique hash for this address which can be used to validate it against metadata.
    fn validation_hash(&self) -> Option<[u8; 32]>;
}

/// This trait represents any storage address which points to a single value we can fetch.
pub trait FetchableAddress: Address {
    /// Does the address have a default value defined for it.
    /// Set to [`Yes`] to enable APIs which require one.
    type HasDefaultValue;
}

/// This trait represents any storage address which points to multiple 0 or more values to iterate over.
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
    marker: core::marker::PhantomData<(Value, HasDefaultValue)>,
}

impl<KeyParts, Value, HasDefaultValue> StaticFetchableAddress<KeyParts, Value, HasDefaultValue> {
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

impl<KeyParts, Value, HasDefaultValue> Address
    for StaticFetchableAddress<KeyParts, Value, HasDefaultValue>
where
    KeyParts: IntoEncodableValues,
    Value: DecodeAsType,
{
    type KeyParts = KeyParts;
    type Value = Value;

    fn key_parts(&self) -> impl IntoEncodableValues {
        &self.key_parts
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

impl<KeyParts, Value, HasDefaultValue> FetchableAddress
    for StaticFetchableAddress<KeyParts, Value, HasDefaultValue>
where
    KeyParts: IntoEncodableValues,
    Value: DecodeAsType,
{
    type HasDefaultValue = HasDefaultValue;
}

/// An address which points to a set of storage values.
pub struct StaticIterableAddress<InputKeyParts, OutputKeyParts, Value> {
    pallet_name: Cow<'static, str>,
    entry_name: Cow<'static, str>,
    input_key_parts: InputKeyParts,
    validation_hash: Option<[u8; 32]>,
    marker: core::marker::PhantomData<(OutputKeyParts, Value)>,
}

impl<InputKeyParts, OutputKeyParts, Value>
    StaticIterableAddress<InputKeyParts, OutputKeyParts, Value>
{
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

impl<InputKeyParts, OutputKeyParts, Value> Address
    for StaticIterableAddress<InputKeyParts, OutputKeyParts, Value>
where
    InputKeyParts: IntoEncodableValues,
    Value: DecodeAsType,
{
    type KeyParts = InputKeyParts;
    type Value = Value;

    fn key_parts(&self) -> impl IntoEncodableValues {
        &self.input_key_parts
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

impl<InputKeyParts, OutputKeyParts, Value> IterableAddress
    for StaticIterableAddress<InputKeyParts, OutputKeyParts, Value>
where
    InputKeyParts: IntoEncodableValues,
    OutputKeyParts: IntoDecodableValues,
    Value: DecodeAsType,
{
    type OutputKeys = OutputKeyParts;
}

/// Construct a new dynamic storage fetch address.
pub fn dynamic_fetch<Keys: IntoEncodableValues>(
    pallet_name: impl Into<Cow<'static, str>>,
    entry_name: impl Into<Cow<'static, str>>,
    storage_entry_keys: Keys,
) -> impl FetchableAddress {
    StaticFetchableAddress::<Keys, scale_value::Value, ()>::new(
        pallet_name,
        entry_name,
        storage_entry_keys,
    )
}

/// Construct a new dynamic storage iter address.
pub fn dynamic_iter<Keys: IntoEncodableValues>(
    pallet_name: impl Into<Cow<'static, str>>,
    entry_name: impl Into<Cow<'static, str>>,
    storage_entry_keys: Keys,
) -> impl IterableAddress {
    StaticIterableAddress::<Keys, Vec<scale_value::Value>, scale_value::Value>::new(
        pallet_name,
        entry_name,
        storage_entry_keys,
    )
}
