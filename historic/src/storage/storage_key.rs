use super::AnyStorageInfo;
use crate::{error::StorageKeyError, storage::storage_info::with_info};
use scale_info_legacy::{LookupName, TypeRegistrySet};

// This is part of our public interface.
pub use frame_decode::storage::{IntoDecodableValues, StorageHasher};

enum AnyStorageKeyInfo<'atblock> {
    Legacy(StorageKeyInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(StorageKeyInfo<'atblock, u32, scale_info::PortableRegistry>),
}

impl<'atblock> From<StorageKeyInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>>
    for AnyStorageKeyInfo<'atblock>
{
    fn from(info: StorageKeyInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>) -> Self {
        AnyStorageKeyInfo::Legacy(info)
    }
}
impl<'atblock> From<StorageKeyInfo<'atblock, u32, scale_info::PortableRegistry>>
    for AnyStorageKeyInfo<'atblock>
{
    fn from(info: StorageKeyInfo<'atblock, u32, scale_info::PortableRegistry>) -> Self {
        AnyStorageKeyInfo::Current(info)
    }
}

struct StorageKeyInfo<'atblock, TypeId, Resolver> {
    info: frame_decode::storage::StorageKey<TypeId>,
    resolver: &'atblock Resolver,
}

macro_rules! with_key_info {
    ($info:ident = $original_info:expr => $fn:expr) => {{
        #[allow(clippy::clone_on_copy)]
        let info = match $original_info {
            AnyStorageKeyInfo::Legacy($info) => $fn,
            AnyStorageKeyInfo::Current($info) => $fn,
        };
        info
    }};
}

/// This represents the different parts of a storage key.
pub struct StorageKey<'entry, 'atblock> {
    info: AnyStorageKeyInfo<'atblock>,
    bytes: &'entry [u8],
}

impl<'entry, 'atblock> StorageKey<'entry, 'atblock> {
    pub(crate) fn new(
        info: &AnyStorageInfo<'atblock>,
        bytes: &'entry [u8],
    ) -> Result<Self, StorageKeyError> {
        with_info!(info = info => {
            let cursor = &mut &*bytes;
            let storage_key_info = frame_decode::storage::decode_storage_key_with_info(
                cursor,
                &info.info,
                info.resolver,
            ).map_err(|e| {
                StorageKeyError::DecodeError { reason: e.map_type_id(|id| id.to_string()) }
            })?;

            if !cursor.is_empty() {
                return Err(StorageKeyError::LeftoverBytes {
                    leftover_bytes: cursor.to_vec(),
                });
            }

            Ok(StorageKey {
                info: StorageKeyInfo {
                    info: storage_key_info,
                    resolver: info.resolver,
                }.into(),
                bytes,
            })
        })
    }

    /// Attempt to decode the values contained within this storage key to the `Target` type
    /// provided. This type is typically a tuple of types which each implement [`scale_decode::DecodeAsType`]
    /// and correspond to each of the key types present, in order.
    pub fn decode_as<Target: IntoDecodableValues>(&self) -> Result<Target, StorageKeyError> {
        with_key_info!(info = &self.info => {
            let values = frame_decode::storage::decode_storage_key_values(
                self.bytes,
                &info.info,
                info.resolver
            ).map_err(|e| {
                StorageKeyError::DecodeKeyValueError { reason: e }
            })?;

            Ok(values)
        })
    }

    /// Iterate over the parts of this storage key. Each part of a storage key corresponds to a
    /// single value that has been hashed.
    pub fn parts(&'_ self) -> impl ExactSizeIterator<Item = StorageKeyPart<'_, 'entry, 'atblock>> {
        let parts_len = with_key_info!(info = &self.info => info.info.parts().len());
        (0..parts_len).map(move |index| StorageKeyPart {
            index,
            info: &self.info,
            bytes: self.bytes,
        })
    }

    /// Return the part of the storage key at the provided index, or `None` if the index is out of bounds.
    pub fn part(&self, index: usize) -> Option<StorageKeyPart<'_, 'entry, 'atblock>> {
        if index < self.parts().len() {
            Some(StorageKeyPart {
                index,
                info: &self.info,
                bytes: self.bytes,
            })
        } else {
            None
        }
    }
}

/// This represents a part of a storage key.
pub struct StorageKeyPart<'key, 'entry, 'atblock> {
    index: usize,
    info: &'key AnyStorageKeyInfo<'atblock>,
    bytes: &'entry [u8],
}

impl<'key, 'entry, 'atblock> StorageKeyPart<'key, 'entry, 'atblock> {
    /// Get the raw bytes for this part of the storage key.
    pub fn bytes(&self) -> &'entry [u8] {
        with_key_info!(info = &self.info => {
            let part = &info.info[self.index];
            let hash_range = part.hash_range();
            let value_range = part
                .value()
                .map(|v| v.range())
                .unwrap_or(std::ops::Range { start: hash_range.end, end: hash_range.end });
            let combined_range = std::ops::Range {
                start: hash_range.start,
                end: value_range.end,
            };
            &self.bytes[combined_range]
        })
    }

    /// Get the hasher that was used to construct this part of the storage key.
    pub fn hasher(&self) -> StorageHasher {
        with_key_info!(info = &self.info => info.info[self.index].hasher())
    }

    /// For keys that were produced using "concat" or "identity" hashers, the value
    /// is available as a part of the key hash, allowing us to decode it into anything
    /// implementing [`scale_decode::DecodeAsType`]. If the key was produced using a
    /// different hasher, this will return `None`.
    pub fn decode_as<T: scale_decode::DecodeAsType>(&self) -> Result<Option<T>, StorageKeyError> {
        with_key_info!(info = &self.info => {
            let part_info = &info.info[self.index];
            let Some(value_info) = part_info.value() else {
                return Ok(None);
            };

            let value_bytes = &self.bytes[value_info.range()];
            let value_ty = value_info.ty().clone();

            let decoded_key_part = T::decode_as_type(
                &mut &*value_bytes,
                value_ty,
                info.resolver,
            ).map_err(|e| StorageKeyError::DecodePartError { index: self.index, reason: e })?;

            Ok(Some(decoded_key_part))
        })
    }
}
