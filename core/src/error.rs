use alloc::string::String;
use derive_more::{Display, From};
use subxt_metadata::StorageHasher;

#[derive(Debug, Display, From)]
pub enum Error {
    /// Codec error.
    #[display(fmt = "Scale codec error: {_0}")]
    Codec(codec::Error),
    #[display(fmt = "Metadata Error: {_0}")]
    Metadata(MetadataError),
    #[display(fmt = "Storage Error: {_0}")]
    StorageAddress(StorageAddressError),
    /// Error decoding to a [`crate::dynamic::Value`].
    #[display(fmt = "Error decoding into dynamic value: {_0}")]
    Decode(scale_decode::Error),
    /// Error encoding from a [`crate::dynamic::Value`].
    #[display(fmt = "Error encoding from dynamic value: {_0}")]
    Encode(scale_encode::Error),
    /// Error constructing the appropriate extrinsic params.
    #[display(fmt = "Extrinsic params error: {_0}")]
    ExtrinsicParams(ExtrinsicParamsError),
}

impl From<scale_decode::visitor::DecodeError> for Error {
    fn from(value: scale_decode::visitor::DecodeError) -> Self {
        Error::Decode(value.into())
    }
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

/// Something went wrong trying to encode or decode a storage address.
#[derive(Clone, Debug, Display)]
#[non_exhaustive]
pub enum StorageAddressError {
    /// Storage lookup does not have the expected number of keys.
    #[display(fmt = "Storage lookup requires {expected} keys but more keys have been provided.")]
    TooManyKeys {
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
    /// We weren't given enough bytes to decode the storage address/key.
    #[display(fmt = "Not enough remaining bytes to decode the storage address/key")]
    NotEnoughBytes,
    /// We have leftover bytes after decoding the storage address.
    #[display(fmt = "We have leftover bytes after decoding the storage address")]
    TooManyBytes,
    /// The bytes of a storage address are not the expected address for decoding the storage keys of the address.
    #[display(
        fmt = "Storage address bytes are not the expected format. Addresses need to be at least 16 bytes (pallet ++ entry) and follow a structure given by the hashers defined in the metadata"
    )]
    UnexpectedAddressBytes,
    /// An invalid hasher was used to reconstruct a value from a chunk of bytes that is part of a storage address. Hashers where the hash does not contain the original value are invalid for this purpose.
    #[display(
        fmt = "An invalid hasher was used to reconstruct a value with type ID {ty_id} from a hash formed by a {hasher:?} hasher. This is only possible for concat-style hashers or the identity hasher"
    )]
    HasherCannotReconstructKey {
        /// Type id of the key's type.
        ty_id: u32,
        /// The invalid hasher that caused this error.
        hasher: StorageHasher,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for StorageAddressError {}

/// An error that can be emitted when trying to construct an instance of [`crate::config::ExtrinsicParams`],
/// encode data from the instance, or match on signed extensions.
#[derive(Display, Debug)]
#[non_exhaustive]
pub enum ExtrinsicParamsError {
    /// Cannot find a type id in the metadata. The context provides some additional
    /// information about the source of the error (eg the signed extension name).
    #[display(fmt = "Cannot find type id '{type_id} in the metadata (context: {context})")]
    MissingTypeId {
        /// Type ID.
        type_id: u32,
        /// Some arbitrary context to help narrow the source of the error.
        context: &'static str,
    },
    /// A signed extension in use on some chain was not provided.
    #[display(
        fmt = "The chain expects a signed extension with the name {_0}, but we did not provide one"
    )]
    UnknownSignedExtension(String),
    /// Some custom error.
    #[display(fmt = "Error constructing extrinsic parameters: {_0}")]
    Custom(Box<dyn CustomError>),
}

/// Anything implementing this trait can be used in [`ExtrinsicParamsError::Custom`].
#[cfg(feature = "std")]
pub trait CustomError: std::error::Error + Send + Sync + 'static {}
#[cfg(feature = "std")]
impl<T: std::error::Error + Send + Sync + 'static> CustomError for T {}

/// Anything implementing this trait can be used in [`ExtrinsicParamsError::Custom`].
#[cfg(not(feature = "std"))]
pub trait CustomError: core::fmt::Debug + core::fmt::Display + Send + Sync + 'static {}
#[cfg(not(feature = "std"))]
impl<T: core::fmt::Debug + core::fmt::Display + Send + Sync + 'static> CustomError for T {}

#[cfg(feature = "std")]
impl std::error::Error for ExtrinsicParamsError {}

impl From<core::convert::Infallible> for ExtrinsicParamsError {
    fn from(value: core::convert::Infallible) -> Self {
        match value {}
    }
}

impl From<Box<dyn CustomError>> for ExtrinsicParamsError {
    fn from(value: Box<dyn CustomError>) -> Self {
        ExtrinsicParamsError::Custom(value)
    }
}
