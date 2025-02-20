// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The errors that can be emitted in this crate.

use alloc::boxed::Box;
use alloc::string::String;
use subxt_metadata::StorageHasher;
use thiserror::Error as DeriveError;

/// The error emitted when something goes wrong.
#[derive(Debug, DeriveError)]
pub enum Error {
    /// Codec error.
    #[error("Codec error: {0}")]
    Codec(codec::Error),
    /// Metadata error.
    #[error(transparent)]
    Metadata(#[from] MetadataError),
    /// Storage address error.
    #[error(transparent)]
    StorageAddress(#[from] StorageAddressError),
    /// Error decoding to a [`crate::dynamic::Value`].
    #[error("Error decoding into dynamic value: {0}")]
    Decode(#[from] scale_decode::Error),
    /// Error encoding from a [`crate::dynamic::Value`].
    #[error("Error encoding from dynamic value: {0}")]
    Encode(#[from] scale_encode::Error),
    /// Error constructing the appropriate extrinsic params.
    #[error(transparent)]
    ExtrinsicParams(#[from] ExtrinsicParamsError),
    /// Block body error.
    #[error("Error working with block_body: {0}")]
    Block(#[from] BlockError),
}

impl From<scale_decode::visitor::DecodeError> for Error {
    fn from(err: scale_decode::visitor::DecodeError) -> Error {
        Error::Decode(err.into())
    }
}

// TODO: when `codec::Error` implements `core::Error`
// remove this impl and replace it by thiserror #[from]
impl From<codec::Error> for Error {
    fn from(err: codec::Error) -> Error {
        Error::Codec(err)
    }
}

/// Block error
#[derive(Debug, DeriveError)]
pub enum BlockError {
    /// Leftover bytes found after decoding the extrinsic.
    #[error("After decoding the extrinsic at index {extrinsic_index}, {num_leftover_bytes} bytes were left, suggesting that decoding may have failed")]
    LeftoverBytes {
        /// Index of the extrinsic that failed to decode.
        extrinsic_index: usize,
        /// Number of bytes leftover after decoding the extrinsic.
        num_leftover_bytes: usize,
    },
    /// Something went wrong decoding the extrinsic.
    #[error("Failed to decode extrinsic at index {extrinsic_index}: {error}")]
    ExtrinsicDecodeError {
        /// Index of the extrinsic that failed to decode.
        extrinsic_index: usize,
        /// The decode error.
        error: ExtrinsicDecodeError,
    },
}

/// An alias for [`frame_decode::extrinsics::ExtrinsicDecodeError`].
///
pub type ExtrinsicDecodeError = frame_decode::extrinsics::ExtrinsicDecodeError;

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq, DeriveError)]
#[non_exhaustive]
pub enum MetadataError {
    /// The DispatchError type isn't available in the metadata
    #[error("The DispatchError type isn't available")]
    DispatchErrorNotFound,
    /// Type not found in metadata.
    #[error("Type with ID {0} not found")]
    TypeNotFound(u32),
    /// Pallet not found (index).
    #[error("Pallet with index {0} not found")]
    PalletIndexNotFound(u8),
    /// Pallet not found (name).
    #[error("Pallet with name {0} not found")]
    PalletNameNotFound(String),
    /// Variant not found.
    #[error("Variant with index {0} not found")]
    VariantIndexNotFound(u8),
    /// Constant not found.
    #[error("Constant with name {0} not found")]
    ConstantNameNotFound(String),
    /// Call not found.
    #[error("Call with name {0} not found")]
    CallNameNotFound(String),
    /// Runtime trait not found.
    #[error("Runtime trait with name {0} not found")]
    RuntimeTraitNotFound(String),
    /// Runtime method not found.
    #[error("Runtime method with name {0} not found")]
    RuntimeMethodNotFound(String),
    /// Call type not found in metadata.
    #[error("Call type not found in pallet with index {0}")]
    CallTypeNotFoundInPallet(u8),
    /// Event type not found in metadata.
    #[error("Event type not found in pallet with index {0}")]
    EventTypeNotFoundInPallet(u8),
    /// Storage details not found in metadata.
    #[error("Storage details not found in pallet with name {0}")]
    StorageNotFoundInPallet(String),
    /// Storage entry not found.
    #[error("Storage entry {0} not found")]
    StorageEntryNotFound(String),
    /// The generated interface used is not compatible with the node.
    #[error("The generated code is not compatible with the node")]
    IncompatibleCodegen,
    /// Custom value not found.
    #[error("Custom value with name {0} not found")]
    CustomValueNameNotFound(String),
}

/// Something went wrong trying to encode or decode a storage address.
#[derive(Clone, Debug, DeriveError)]
#[non_exhaustive]
pub enum StorageAddressError {
    /// Storage lookup does not have the expected number of keys.
    #[error("Storage lookup requires {expected} keys but more keys have been provided.")]
    TooManyKeys {
        /// The number of keys provided in the storage address.
        expected: usize,
    },
    /// This storage entry in the metadata does not have the correct number of hashers to fields.
    #[error("Storage entry in metadata does not have the correct number of hashers to fields")]
    WrongNumberOfHashers {
        /// The number of hashers in the metadata for this storage entry.
        hashers: usize,
        /// The number of fields in the metadata for this storage entry.
        fields: usize,
    },
    /// We weren't given enough bytes to decode the storage address/key.
    #[error("Not enough remaining bytes to decode the storage address/key")]
    NotEnoughBytes,
    /// We have leftover bytes after decoding the storage address.
    #[error("We have leftover bytes after decoding the storage address")]
    TooManyBytes,
    /// The bytes of a storage address are not the expected address for decoding the storage keys of the address.
    #[error("Storage address bytes are not the expected format. Addresses need to be at least 16 bytes (pallet ++ entry) and follow a structure given by the hashers defined in the metadata")]
    UnexpectedAddressBytes,
    /// An invalid hasher was used to reconstruct a value from a chunk of bytes that is part of a storage address. Hashers where the hash does not contain the original value are invalid for this purpose.
    #[error("An invalid hasher was used to reconstruct a value with type ID {ty_id} from a hash formed by a {hasher:?} hasher. This is only possible for concat-style hashers or the identity hasher")]
    HasherCannotReconstructKey {
        /// Type id of the key's type.
        ty_id: u32,
        /// The invalid hasher that caused this error.
        hasher: StorageHasher,
    },
}

/// An error that can be emitted when trying to construct an instance of [`crate::config::ExtrinsicParams`],
/// encode data from the instance, or match on signed extensions.
#[derive(Debug, DeriveError)]
#[non_exhaustive]
pub enum ExtrinsicParamsError {
    /// Cannot find a type id in the metadata. The context provides some additional
    /// information about the source of the error (eg the signed extension name).
    #[error("Cannot find type id '{type_id} in the metadata (context: {context})")]
    MissingTypeId {
        /// Type ID.
        type_id: u32,
        /// Some arbitrary context to help narrow the source of the error.
        context: &'static str,
    },
    /// A signed extension in use on some chain was not provided.
    #[error("The chain expects a signed extension with the name {0}, but we did not provide one")]
    UnknownTransactionExtension(String),
    /// Some custom error.
    #[error("Error constructing extrinsic parameters: {0}")]
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
