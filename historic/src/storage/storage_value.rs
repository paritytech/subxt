use super::storage_info::AnyStorageInfo;
use super::storage_info::with_info;
use crate::error::StorageValueError;
use scale_decode::DecodeAsType;
use std::borrow::Cow;

/// This represents a storage value.
pub struct StorageValue<'entry, 'atblock> {
    pub(crate) info: &'entry AnyStorageInfo<'atblock>,
    bytes: Cow<'atblock, [u8]>,
}

impl<'entry, 'atblock> StorageValue<'entry, 'atblock> {
    /// Create a new storage value.
    pub fn new(info: &'entry AnyStorageInfo<'atblock>, bytes: Cow<'atblock, [u8]>) -> Self {
        Self { info, bytes }
    }

    /// Get the raw bytes for this storage value.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume this storage value and return the raw bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// Decode this storage value.
    pub fn decode<T: DecodeAsType>(&self) -> Result<T, StorageValueError> {
        with_info!(info = &self.info => {
            let cursor = &mut &*self.bytes;

            let value = T::decode_as_type(
                cursor,
                info.info.value_id.clone(),
                info.resolver,
            ).map_err(|e| StorageValueError::DecodeError { reason: e })?;

            if !cursor.is_empty() {
                return Err(StorageValueError::LeftoverBytes {
                    leftover_bytes: cursor.to_vec(),
                });
            }

            Ok(value)
        })
    }
}
