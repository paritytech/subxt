use crate::error::StorageError;
use frame_decode::storage::StorageTypeInfo;
use frame_metadata::RuntimeMetadata;
use scale_info_legacy::{LookupName, TypeRegistrySet};

pub enum AnyStorageInfo<'atblock> {
    Legacy(StorageInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(StorageInfo<'atblock, u32, scale_info::PortableRegistry>),
}

impl<'atblock> AnyStorageInfo<'atblock> {
    /// For a slice of storage entries, return a vec of information about each one.
    pub fn new(
        pallet_name: &str,
        entry_name: &str,
        metadata: &'atblock RuntimeMetadata,
        legacy_types: &'atblock TypeRegistrySet<'atblock>,
    ) -> Result<Self, StorageError> {
        let info = match metadata {
            RuntimeMetadata::V8(m) => storage_info_inner(pallet_name, entry_name, m, legacy_types),
            RuntimeMetadata::V9(m) => storage_info_inner(pallet_name, entry_name, m, legacy_types),
            RuntimeMetadata::V10(m) => storage_info_inner(pallet_name, entry_name, m, legacy_types),
            RuntimeMetadata::V11(m) => storage_info_inner(pallet_name, entry_name, m, legacy_types),
            RuntimeMetadata::V12(m) => storage_info_inner(pallet_name, entry_name, m, legacy_types),
            RuntimeMetadata::V13(m) => storage_info_inner(pallet_name, entry_name, m, legacy_types),
            RuntimeMetadata::V14(m) => storage_info_inner(pallet_name, entry_name, m, &m.types),
            RuntimeMetadata::V15(m) => storage_info_inner(pallet_name, entry_name, m, &m.types),
            RuntimeMetadata::V16(m) => storage_info_inner(pallet_name, entry_name, m, &m.types),
            unknown => {
                return Err(StorageError::UnsupportedMetadataVersion {
                    version: unknown.version(),
                });
            }
        }?;

        fn storage_info_inner<'atblock, Info, Resolver>(
            pallet_name: &str,
            entry_name: &str,
            m: &'atblock Info,
            type_resolver: &'atblock Resolver,
        ) -> Result<AnyStorageInfo<'atblock>, StorageError>
        where
            Info: StorageTypeInfo,
            Resolver: scale_type_resolver::TypeResolver<TypeId = Info::TypeId>,
            AnyStorageInfo<'atblock>: From<StorageInfo<'atblock, Info::TypeId, Resolver>>,
        {
            m.storage_info(pallet_name, entry_name)
                .map(|frame_storage_info| {
                    let info = StorageInfo {
                        info: frame_storage_info,
                        resolver: type_resolver,
                    };
                    AnyStorageInfo::from(info)
                })
                .map_err(|e| StorageError::ExtractStorageInfoError {
                    reason: e.into_owned(),
                })
        }

        Ok(info)
    }

    /// Is the storage entry a map (ie something we'd provide extra keys to access a value, or otherwise iterate over)?
    pub fn is_map(&self) -> bool {
        match self {
            AnyStorageInfo::Legacy(info) => !info.info.keys.is_empty(),
            AnyStorageInfo::Current(info) => !info.info.keys.is_empty(),
        }
    }
}

impl<'atblock> From<StorageInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>>
    for AnyStorageInfo<'atblock>
{
    fn from(info: StorageInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>) -> Self {
        AnyStorageInfo::Legacy(info)
    }
}
impl<'atblock> From<StorageInfo<'atblock, u32, scale_info::PortableRegistry>>
    for AnyStorageInfo<'atblock>
{
    fn from(info: StorageInfo<'atblock, u32, scale_info::PortableRegistry>) -> Self {
        AnyStorageInfo::Current(info)
    }
}

pub struct StorageInfo<'atblock, TypeId: Clone, Resolver> {
    pub info: frame_decode::storage::StorageInfo<'atblock, TypeId>,
    pub resolver: &'atblock Resolver,
}

macro_rules! with_info {
    ($info:ident = $original_info:expr => $fn:expr) => {{
        #[allow(clippy::clone_on_copy)]
        let info = match $original_info {
            AnyStorageInfo::Legacy($info) => $fn,
            AnyStorageInfo::Current($info) => $fn,
        };
        info
    }};
}
pub(crate) use with_info;
