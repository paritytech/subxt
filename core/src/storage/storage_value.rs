// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use core::marker::PhantomData;
use frame_decode::storage::StorageInfo;
use alloc::borrow::Cow;
use scale_info::PortableRegistry;
use scale_decode::DecodeAsType;
use crate::error::StorageValueError;

/// Create a [`StorageValue`] to decode some storage value bytes.
/// 
/// The generic `Value` parameter determines what the default type
/// that the value will decode into is.
pub fn value<'entry, 'info, Value>(
    bytes: impl Into<Cow<'entry, [u8]>>,
    info: &'entry StorageInfo<'info, u32>,
    types: &'info PortableRegistry
) -> StorageValue<'entry, 'info, Value> {
    StorageValue {
        info,
        types,
        bytes: bytes.into(),
        marker: PhantomData,
    }
}

/// This represents a storage value.
pub struct StorageValue<'entry, 'info, Value> {
    pub(crate) info: &'entry StorageInfo<'info, u32>,
    pub(crate) types: &'info PortableRegistry,
    bytes: Cow<'entry, [u8]>,
    marker: PhantomData<Value>
}

impl<'entry, 'info, Value: DecodeAsType> StorageValue<'entry, 'info, Value> {
    /// Get the raw bytes for this storage value.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume this storage value and return the raw bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// Decode this storage value into the provided response type.
    pub fn decode(&self) -> Result<Value, StorageValueError> {
        self.decode_as::<Value>()
    }

    /// Decode this storage value into an arbitrary type.
    pub fn decode_as<T: DecodeAsType>(&self) -> Result<T, StorageValueError> {
        let cursor = &mut &*self.bytes;

        let value = frame_decode::storage::decode_storage_value_with_info(
            cursor, 
            self.info, 
            self.types, 
            T::into_visitor()
        ).map_err(StorageValueError::CannotDecode)?;

        if !cursor.is_empty() {
            return Err(StorageValueError::LeftoverBytes {
                bytes: cursor.to_vec()
            });
        }

        Ok(value)
    }
}