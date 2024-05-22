// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The errors that can be emitted in this crate.

use alloc::boxed::Box;
use alloc::string::String;
use snafu::Snafu;
use subxt_metadata::StorageHasher;

use crate::error_utils::DisplayError;

/// The error emitted when something goes wrong.
#[derive(Debug, Snafu)]
pub enum Error {
    /// Codec error.
    #[snafu(display("Scale codec error: {source}"), context(false))]
    Codec {
        /// Error source
        #[snafu(source(from(codec::Error, DisplayError)))]
        source: DisplayError<codec::Error>,
    },
    /// Metadata error.
    #[snafu(display("Metadata Error: {source}"), context(false))]
    Metadata {
        /// Error source
        source: MetadataError,
    },
    /// Storage address error.
    #[snafu(display("Storage Error: {source}"), context(false))]
    StorageAddress {
        /// Error source
        source: StorageAddressError,
    },
    /// Error decoding to a [`crate::dynamic::Value`].
    #[snafu(display("Error decoding into dynamic value: {source}"), context(false))]
    Decode {
        /// Error source
        #[snafu(source(from(scale_decode::Error, DisplayError)))]
        source: DisplayError<scale_decode::Error>,
    },
    /// Error encoding from a [`crate::dynamic::Value`].
    #[snafu(display("Error encoding from dynamic value: {source}"), context(false))]
    Encode {
        /// Error source
        #[snafu(source(from(scale_encode::Error, DisplayError)))]
        source: DisplayError<scale_encode::Error>,
    },
    /// Error constructing the appropriate extrinsic params.
    #[snafu(display("Extrinsic params error: {source}"), context(false))]
    ExtrinsicParams {
        /// Error source
        source: ExtrinsicParamsError,
    },
    /// Block body error.
    #[snafu(display("Error working with block body: {source}"), context(false))]
    Block {
        /// Error source
        source: BlockError,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<scale_decode::visitor::DecodeError> for Error {
    fn from(value: scale_decode::visitor::DecodeError) -> Self {
        Error::Decode {
            source: DisplayError(value.into()),
        }
    }
}

/// Block error
#[derive(Clone, Debug, Snafu, Eq, PartialEq)]
pub enum BlockError {
    /// Extrinsic type ID cannot be resolved with the provided metadata.
    #[snafu(display(
        "Extrinsic type ID cannot be resolved with the provided metadata. Make sure this is a valid metadata"
    ))]
    MissingType,
    /// Unsupported signature.
    #[snafu(display("Unsupported extrinsic version, only version 4 is supported currently"))]
    /// The extrinsic has an unsupported version.
    UnsupportedVersion {
        /// Version of the extrinsic
        version: u8,
    },
    /// Decoding error.
    #[snafu(display("Cannot decode extrinsic: {source}"), context(false))]
    DecodingError {
        /// Decoding error source
        #[snafu(source(from(codec::Error, DisplayError)))]
        source: DisplayError<codec::Error>,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for BlockError {}

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq, Snafu)]
#[non_exhaustive]
pub enum MetadataError {
    /// The DispatchError type isn't available in the metadata
    #[snafu(display("The DispatchError type isn't available"))]
    DispatchErrorNotFound,
    /// Type not found in metadata.
    #[snafu(display("Type with ID {type_id} not found"))]
    TypeNotFound {
        /// Type id
        type_id: u32,
    },
    /// Pallet not found (index).
    #[snafu(display("Pallet with index {pallet_idx} not found"))]
    PalletIndexNotFound {
        /// Pallet index
        pallet_idx: u8,
    },
    /// Pallet not found (name).
    #[snafu(display("Pallet with name {name} not found"))]
    PalletNameNotFound {
        /// Pallet name
        name: String,
    },
    /// Variant not found.
    #[snafu(display("Variant with index {variant_idx} not found"))]
    VariantIndexNotFound {
        /// index of the variant being searched
        variant_idx: u8,
    },
    /// Constant not found.
    #[snafu(display("Constant with name {name} not found"))]
    ConstantNameNotFound {
        /// Name of the constant
        name: String,
    },
    /// Call not found.
    #[snafu(display("Call with name {name} not found"))]
    CallNameNotFound {
        /// Name of the call
        name: String,
    },
    /// Runtime trait not found.
    #[snafu(display("Runtime trait with name {name} not found"))]
    RuntimeTraitNotFound {
        /// Name of the trait being searched
        name: String,
    },
    /// Runtime method not found.
    #[snafu(display("Runtime method with name {name} not found"))]
    RuntimeMethodNotFound {
        /// Name of the method being searched
        name: String,
    },
    /// Call type not found in metadata.
    #[snafu(display("Call type not found in pallet with index {type_id}"))]
    CallTypeNotFoundInPallet {
        /// Type id of the call
        type_id: u8,
    },
    /// Event type not found in metadata.
    #[snafu(display("Event type not found in pallet with index {type_id}"))]
    EventTypeNotFoundInPallet {
        /// Type id
        type_id: u8,
    },
    /// Storage details not found in metadata.
    #[snafu(display("Storage details not found in pallet with name {name}"))]
    StorageNotFoundInPallet {
        /// Pallet name
        name: String,
    },
    /// Storage entry not found.
    #[snafu(display("Storage entry {entry_name} not found"))]
    StorageEntryNotFound {
        /// Name of the storage entry
        entry_name: String,
    },
    /// The generated interface used is not compatible with the node.
    #[snafu(display("The generated code is not compatible with the node"))]
    IncompatibleCodegen,
    /// Custom value not found.
    #[snafu(display("Custom value with name {name} not found"))]
    CustomValueNameNotFound {
        /// Custom name of the value
        name: String,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for MetadataError {}

/// Something went wrong trying to encode or decode a storage address.
#[derive(Clone, Debug, Snafu)]
#[non_exhaustive]
pub enum StorageAddressError {
    /// Storage lookup does not have the expected number of keys.
    #[snafu(display("Storage lookup requires {expected} keys but more keys have been provided."))]
    TooManyKeys {
        /// The number of keys provided in the storage address.
        expected: usize,
    },
    /// This storage entry in the metadata does not have the correct number of hashers to fields.
    #[snafu(display(
        "Storage entry in metadata does not have the correct number of hashers to fields"
    ))]
    WrongNumberOfHashers {
        /// The number of hashers in the metadata for this storage entry.
        hashers: usize,
        /// The number of fields in the metadata for this storage entry.
        fields: usize,
    },
    /// We weren't given enough bytes to decode the storage address/key.
    #[snafu(display("Not enough remaining bytes to decode the storage address/key"))]
    NotEnoughBytes,
    /// We have leftover bytes after decoding the storage address.
    #[snafu(display("We have leftover bytes after decoding the storage address"))]
    TooManyBytes,
    /// The bytes of a storage address are not the expected address for decoding the storage keys of the address.
    #[snafu(display(
        "Storage address bytes are not the expected format. Addresses need to be at least 16 bytes (pallet ++ entry) and follow a structure given by the hashers defined in the metadata"
    ))]
    UnexpectedAddressBytes,
    /// An invalid hasher was used to reconstruct a value from a chunk of bytes that is part of a storage address. Hashers where the hash does not contain the original value are invalid for this purpose.
    #[snafu(display(
        "An invalid hasher was used to reconstruct a value with type ID {ty_id} from a hash formed by a {hasher:?} hasher. This is only possible for concat-style hashers or the identity hasher"
    ))]
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
#[derive(Snafu, Debug)]
#[non_exhaustive]
pub enum ExtrinsicParamsError {
    /// Cannot find a type id in the metadata. The context provides some additional
    /// information about the source of the error (eg the signed extension name).
    #[snafu(display("Cannot find type id '{type_id} in the metadata (context: {context})"))]
    MissingTypeId {
        /// Type ID.
        type_id: u32,
        /// Some arbitrary context to help narrow the source of the error.
        context: &'static str,
    },
    /// A signed extension in use on some chain was not provided.
    #[snafu(display(
        "The chain expects a signed extension with the name {extension}, but we did not provide one"
    ))]
    UnknownSignedExtension {
        /// Extension name
        extension: String,
    },
    /// Some custom error.
    #[snafu(display("Error constructing extrinsic parameters: {source}"))]
    Custom {
        /// Error source
        #[snafu(source(from(Box<dyn CustomError>, DisplayError)))]
        source: DisplayError<Box<dyn CustomError>>,
    },
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
        ExtrinsicParamsError::Custom {
            source: DisplayError(value),
        }
    }
}
