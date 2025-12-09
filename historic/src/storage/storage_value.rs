use super::storage_info::AnyStorageInfo;
use super::storage_info::with_info;
use crate::error::StorageValueError;
use crate::utils::{AnyResolver, AnyTypeId};
use scale_decode::DecodeAsType;
use std::borrow::Cow;
use std::sync::Arc;

/// This represents a storage value.
pub struct StorageValue<'atblock> {
    pub(crate) info: Arc<AnyStorageInfo<'atblock>>,
    bytes: Cow<'atblock, [u8]>,
    resolver: AnyResolver<'atblock, 'atblock>,
}

impl<'atblock> StorageValue<'atblock> {
    /// Create a new storage value.
    pub(crate) fn new(info: Arc<AnyStorageInfo<'atblock>>, bytes: Cow<'atblock, [u8]>) -> Self {
        let resolver = match &*info {
            AnyStorageInfo::Current(info) => AnyResolver::A(info.resolver),
            AnyStorageInfo::Legacy(info) => AnyResolver::B(info.resolver),
        };

        Self {
            info,
            bytes,
            resolver,
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

    /// Visit the given field with a [`scale_decode::visitor::Visitor`]. This is like a lower level
    /// version of [`StorageValue::decode_as`], as the visitor is able to preserve lifetimes
    /// and has access to more type information than is available via [`StorageValue::decode_as`].
    pub fn visit<
        V: scale_decode::visitor::Visitor<TypeResolver = AnyResolver<'atblock, 'atblock>>,
    >(
        &self,
        visitor: V,
    ) -> Result<V::Value<'_, '_>, V::Error> {
        let type_id = match &*self.info {
            AnyStorageInfo::Current(info) => AnyTypeId::A(info.info.value_id),
            AnyStorageInfo::Legacy(info) => AnyTypeId::B(info.info.value_id.clone()),
        };
        let cursor = &mut self.bytes();

        scale_decode::visitor::decode_with_visitor(cursor, type_id, &self.resolver, visitor)
    }

    /// Decode this storage value.
    pub fn decode_as<T: DecodeAsType>(&self) -> Result<T, StorageValueError> {
        with_info!(info = &*self.info => {
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
