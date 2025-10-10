// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::StorageValueError;
use alloc::sync::Arc;
use core::marker::PhantomData;
use frame_decode::storage::StorageInfo;
use scale_decode::DecodeAsType;
use scale_info::PortableRegistry;

/// This represents a storage value.
pub struct StorageValue<'info, Value> {
    pub(crate) info: Arc<StorageInfo<'info, u32>>,
    pub(crate) types: &'info PortableRegistry,
    bytes: Vec<u8>,
    marker: PhantomData<Value>,
}

impl<'info, Value: DecodeAsType> StorageValue<'info, Value> {
    pub(crate) fn new(
        info: Arc<StorageInfo<'info, u32>>,
        types: &'info PortableRegistry,
        bytes: Vec<u8>,
    ) -> StorageValue<'info, Value> {
        StorageValue {
            info,
            types,
            bytes,
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
            &self.info,
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
