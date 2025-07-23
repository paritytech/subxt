use super::storage_info::AnyStorageInfo;
use crate::error::StorageError;
use scale_decode::DecodeAsType;
use super::storage_info::with_info;

/// This represents a storage value.
pub struct StorageValue<'entry, 'atblock> {
    pub(crate) info: &'entry AnyStorageInfo<'atblock>,
    bytes: Vec<u8>,
}

impl <'entry, 'atblock> StorageValue<'entry, 'atblock> {
    /// Create a new storage value.
    pub fn new(info: &'entry AnyStorageInfo<'atblock>, bytes: Vec<u8>) -> Self {
        Self { info, bytes }
    }

    /// Get the raw bytes for this storage value.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume this storage value and return the raw bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Decode this storage value.
    pub fn decode<T: DecodeAsType>(&self) -> Result<T, StorageError> {
        with_info!(&self.info => {
            let cursor = &mut &*self.bytes;

            let value = T::decode_as_type(
                cursor,
                info.info.value_id.clone(),
                info.resolver,
            ).map_err(|e| StorageError::DecodeError { reason: e })?;

            if !cursor.is_empty() {
                return Err(StorageError::ValueLeftoverBytes {
                    leftover_bytes: cursor.to_vec(),
                });
            }

            Ok(value)
        })
    }
}


