// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::StorageValueError;
use alloc::borrow::Cow;
use core::marker::PhantomData;
use frame_decode::storage::StorageInfo;
use scale_decode::DecodeAsType;
use scale_info::PortableRegistry;

/// This represents a storage value.
pub struct StorageValue<'entry, 'info, Value> {
    pub(crate) info: &'entry StorageInfo<'info, u32>,
    pub(crate) types: &'info PortableRegistry,
    bytes: Cow<'entry, [u8]>,
    marker: PhantomData<Value>,
}

impl<'entry, 'info, Value: DecodeAsType> StorageValue<'entry, 'info, Value> {
    pub(crate) fn new(
        info: &'entry StorageInfo<'info, u32>,
        types: &'info PortableRegistry,
        bytes: impl Into<Cow<'entry, [u8]>>,
    ) -> StorageValue<'entry, 'info, Value> {
        StorageValue {
            info,
            types,
            bytes: bytes.into(),
            marker: PhantomData,
        }
    }

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
            T::into_visitor(),
        )
        .map_err(StorageValueError::CannotDecode)?;

        if !cursor.is_empty() {
            return Err(StorageValueError::LeftoverBytes {
                bytes: cursor.to_vec(),
            });
        }

        Ok(value)
    }
}
