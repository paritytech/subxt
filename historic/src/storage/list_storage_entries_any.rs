use frame_decode::storage::StorageEntryInfo;
use frame_metadata::RuntimeMetadata;

pub use frame_decode::storage::StorageEntry;

/// Returns an iterator listing the available storage entries in some metadata.
///
/// This basically calls [`StorageEntryInfo::storage_entries()`] for each metadata version,
/// returning an empty iterator where applicable (ie when passing legacy metadata and the
/// `legacy` features flag is not enabled).
pub fn list_storage_entries_any(
    metadata: &RuntimeMetadata,
) -> impl Iterator<Item = StorageEntry<'_>> {
    match metadata {
        RuntimeMetadata::V0(_deprecated_metadata)
        | RuntimeMetadata::V1(_deprecated_metadata)
        | RuntimeMetadata::V2(_deprecated_metadata)
        | RuntimeMetadata::V3(_deprecated_metadata)
        | RuntimeMetadata::V4(_deprecated_metadata)
        | RuntimeMetadata::V5(_deprecated_metadata)
        | RuntimeMetadata::V6(_deprecated_metadata)
        | RuntimeMetadata::V7(_deprecated_metadata) => {
            Box::new(core::iter::empty()) as Box<dyn Iterator<Item = StorageEntry<'_>>>
        }
        RuntimeMetadata::V8(m) => Box::new(m.storage_entries()),
        RuntimeMetadata::V9(m) => Box::new(m.storage_entries()),
        RuntimeMetadata::V10(m) => Box::new(m.storage_entries()),
        RuntimeMetadata::V11(m) => Box::new(m.storage_entries()),
        RuntimeMetadata::V12(m) => Box::new(m.storage_entries()),
        RuntimeMetadata::V13(m) => Box::new(m.storage_entries()),
        RuntimeMetadata::V14(m) => Box::new(m.storage_entries()),
        RuntimeMetadata::V15(m) => Box::new(m.storage_entries()),
        RuntimeMetadata::V16(m) => Box::new(m.storage_entries()),
    }
}
