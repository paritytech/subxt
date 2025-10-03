// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use frame_decode::storage::StorageInfo;
use scale_decode::DecodeAsType;
use scale_info::PortableRegistry;
use core::marker::PhantomData;
use std::borrow::Cow;

use crate::error::StorageValueError;

/// This represents a storage value.
pub struct StorageValue<'entry, 'atblock, Value> {
    pub(crate) info: &'entry StorageInfo<'atblock, u32>,
    pub(crate) types: &'atblock PortableRegistry,
    bytes: Cow<'entry, [u8]>,
    marker: PhantomData<Value>
}

impl<'entry, 'atblock, Value: DecodeAsType> StorageValue<'entry, 'atblock, Value> {
    /// Create a new storage value.
    pub fn new(
        info: &'entry StorageInfo<'atblock, u32>,
        types: &'atblock PortableRegistry,
        bytes: impl Into<Cow<'entry, [u8]>>,
    ) -> Self {
        Self { info, types, bytes: bytes.into(), marker: PhantomData }
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

        let value = T::decode_as_type(
            cursor,
            self.info.value_id,
            self.types,
        ).map_err(|reason| StorageValueError::DecodeError { reason })?;

        if !cursor.is_empty() {
            return Err(StorageValueError::LeftoverBytes {
                leftover_bytes: cursor.to_vec(),
            });
        }

        Ok(value)
    }
}
