use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Metadata Error: {_0}")]
    Metadata(MetadataError),
    #[display(fmt = "Storage Error: {_0}")]
    Storage(StorageAddressError),
    /// Error decoding to a [`crate::dynamic::Value`].
    #[display(fmt = "Error decoding into dynamic value: {_0}")]
    Decode(scale_decode::Error),
    /// Error encoding from a [`crate::dynamic::Value`].
    #[display(fmt = "Error encoding from dynamic value: {_0}")]
    Encode(scale_encode::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq, Display)]
#[non_exhaustive]
pub enum MetadataError {
    /// The DispatchError type isn't available in the metadata
    #[display(fmt = "The DispatchError type isn't available")]
    DispatchErrorNotFound,
    /// Type not found in metadata.
    #[display(fmt = "Type with ID {_0} not found")]
    TypeNotFound(u32),
    /// Pallet not found (index).
    #[display(fmt = "Pallet with index {_0} not found")]
    PalletIndexNotFound(u8),
    /// Pallet not found (name).
    #[display(fmt = "Pallet with name {_0} not found")]
    PalletNameNotFound(String),
    /// Variant not found.
    #[display(fmt = "Variant with index {_0} not found")]
    VariantIndexNotFound(u8),
    /// Constant not found.
    #[display(fmt = "Constant with name {_0} not found")]
    ConstantNameNotFound(String),
    /// Call not found.
    #[display(fmt = "Call with name {_0} not found")]
    CallNameNotFound(String),
    /// Runtime trait not found.
    #[display(fmt = "Runtime trait with name {_0} not found")]
    RuntimeTraitNotFound(String),
    /// Runtime method not found.
    #[display(fmt = "Runtime method with name {_0} not found")]
    RuntimeMethodNotFound(String),
    /// Call type not found in metadata.
    #[display(fmt = "Call type not found in pallet with index {_0}")]
    CallTypeNotFoundInPallet(u8),
    /// Event type not found in metadata.
    #[display(fmt = "Event type not found in pallet with index {_0}")]
    EventTypeNotFoundInPallet(u8),
    /// Storage details not found in metadata.
    #[display(fmt = "Storage details not found in pallet with name {_0}")]
    StorageNotFoundInPallet(String),
    /// Storage entry not found.
    #[display(fmt = "Storage entry {_0} not found")]
    StorageEntryNotFound(String),
    /// The generated interface used is not compatible with the node.
    #[display(fmt = "The generated code is not compatible with the node")]
    IncompatibleCodegen,
    /// Custom value not found.
    #[display(fmt = "Custom value with name {_0} not found")]
    CustomValueNameNotFound(String),
}

#[cfg(feature = "std")]
impl std::error::Error for MetadataError {}

/// Something went wrong trying to encode a storage address.
#[derive(Clone, Debug, Display)]
#[non_exhaustive]
pub enum StorageAddressError {
    /// Storage map type must be a composite type.
    #[display(fmt = "Storage map type must be a composite type")]
    MapTypeMustBeTuple,
    /// Storage lookup does not have the expected number of keys.
    #[display(fmt = "Storage lookup requires {expected} keys but got {actual} keys")]
    WrongNumberOfKeys {
        /// The actual number of keys needed, based on the metadata.
        actual: usize,
        /// The number of keys provided in the storage address.
        expected: usize,
    },
    /// This storage entry in the metadata does not have the correct number of hashers to fields.
    #[display(
        fmt = "Storage entry in metadata does not have the correct number of hashers to fields"
    )]
    WrongNumberOfHashers {
        /// The number of hashers in the metadata for this storage entry.
        hashers: usize,
        /// The number of fields in the metadata for this storage entry.
        fields: usize,
    },
}
