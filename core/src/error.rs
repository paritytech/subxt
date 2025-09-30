// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The errors that can be emitted in this crate.

use alloc::boxed::Box;
use alloc::string::String;
use thiserror::Error as DeriveError;

/// The error emitted when something goes wrong.
#[derive(Debug, DeriveError)]
#[allow(missing_docs)]
pub enum Error {
    // /// Codec error.
    // #[error("Codec error: {0}")]
    // Codec(codec::Error),
    #[error(transparent)]
    Metadata(#[from] MetadataError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    // /// Error decoding to a [`crate::dynamic::Value`].
    // #[error("Error decoding into dynamic value: {0}")]
    // Decode(#[from] scale_decode::Error),
    // /// Error encoding from a [`crate::dynamic::Value`].
    // #[error("Error encoding from dynamic value: {0}")]
    // Encode(#[from] scale_encode::Error),
    #[error(transparent)]
    Extrinsic(#[from] ExtrinsicError),
    #[error(transparent)]
    Constants(#[from] ConstantsError),
}

// impl From<scale_decode::visitor::DecodeError> for Error {
//     fn from(err: scale_decode::visitor::DecodeError) -> Error {
//         Error::Decode(err.into())
//     }
// }

// // TODO: when `codec::Error` implements `core::Error`
// // remove this impl and replace it by thiserror #[from]
// impl From<codec::Error> for Error {
//     fn from(err: codec::Error) -> Error {
//         Error::Codec(err)
//     }
// }

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MetadataError {
    // /// The DispatchError type isn't available in the metadata
    // #[error("The DispatchError type isn't available")]
    // DispatchErrorNotFound,
    // /// Type not found in metadata.
    // #[error("Type with ID {0} not found")]
    // TypeNotFound(u32),
    /// Pallet not found (index).
    // /// Call type not found in metadata.
    // #[error("Call type not found in pallet with index {0}")]
    // CallTypeNotFoundInPallet(u8),
    // /// Event type not found in metadata.
    // #[error("Event type not found in pallet with index {0}")]
    // EventTypeNotFoundInPallet(u8),
    // // /// Storage details not found in metadata.
    // // #[error("Storage details not found in pallet with name {0}")]
    // // StorageNotFoundInPallet(String),
    // // /// Storage entry not found.
    // // #[error("Storage entry {0} not found")]
    // // StorageEntryNotFound(String),
    #[error("Pallet with index {0} not found")]
    PalletIndexNotFound(u8),
    #[error("Pallet with name {0} not found")]
    PalletNameNotFound(String),
    #[error("Variant with index {0} not found")]
    VariantIndexNotFound(u8),
    #[error("Constant with name {0} not found")]
    ConstantNameNotFound(String),
    #[error("Call with name {0} not found")]
    CallNameNotFound(String),
    #[error("Runtime trait with name {0} not found")]
    RuntimeTraitNotFound(String),
    #[error("Runtime method with name {0} not found")]
    RuntimeMethodNotFound(String),
    #[error("View Function with query ID {} not found", hex::encode(.0))]
    ViewFunctionNotFound([u8; 32]),
    #[error("The generated code is not compatible with the node")]
    IncompatibleCodegen,
    #[error("Custom value with name {0} not found")]
    CustomValueNameNotFound(String),
}

/// Something went wrong working with a constant.
#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ConstantsError {
    #[error("The static constant address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Constant with name {0} not found in the live chain metadata")]
    ConstantNameNotFound(String),
}

/// Something went wrong trying to encode or decode a storage address.
#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StorageError {
    #[error("Cannot obtain storage information from metadata: {0}")]
    StorageInfoError(frame_decode::storage::StorageInfoError<'static>),
    #[error("Cannot decode storage value: {0}")]
    StorageValueDecodeError(frame_decode::storage::StorageValueDecodeError<u32>),
    #[error("Cannot encode storage key: {0}")]
    StorageKeyEncodeError(frame_decode::storage::StorageKeyEncodeError),
}

/// An error that can be encountered when constructing a transaction.
#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExtrinsicError {
    #[error("Subxt does not support the extrinsic versions expected by the chain")]
    UnsupportedVersion,
    #[error("Cannot construct the required transaction extensions: {0}")]
    Params(#[from] ExtrinsicParamsError),
    #[error("Cannot decode transaction extension '{name}': {error}")]
    CouldNotDecodeTransactionExtension {
        /// The extension name.
        name: String,
        /// The decode error.
        error: scale_decode::Error
    },
    #[error(
        "After decoding the extrinsic at index {extrinsic_index}, {num_leftover_bytes} bytes were left, suggesting that decoding may have failed"
    )]
    LeftoverBytes {
        /// Index of the extrinsic that failed to decode.
        extrinsic_index: usize,
        /// Number of bytes leftover after decoding the extrinsic.
        num_leftover_bytes: usize,
    },
    #[error("Failed to decode extrinsic at index {extrinsic_index}: {error}")]
    ExtrinsicDecodeError {
        /// Index of the extrinsic that failed to decode.
        extrinsic_index: usize,
        /// The decode error.
        error: ExtrinsicDecodeError,
    },
    #[error("Failed to decode the fields of an extrinsic at index {extrinsic_index}: {error}")]
    CannotDecodeFields {
        /// Index of the extrinsic whose fields we could not decode
        extrinsic_index: usize,
        /// The decode error.
        error: scale_decode::Error
    },
    #[error("Failed to decode the extrinsic at index {extrinsic_index} to a root enum: {error}")]
    CannotDecodeIntoRootExtrinsic {
        /// Index of the extrinsic that we failed to decode
        extrinsic_index: usize,
        /// The decode error.
        error: scale_decode::Error
    }
}

/// An alias for [`frame_decode::extrinsics::ExtrinsicDecodeError`].
///
pub type ExtrinsicDecodeError = frame_decode::extrinsics::ExtrinsicDecodeError;

/// An error that can be emitted when trying to construct an instance of [`crate::config::ExtrinsicParams`],
/// encode data from the instance, or match on signed extensions.
#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExtrinsicParamsError {
    #[error("Cannot find type id '{type_id} in the metadata (context: {context})")]
    MissingTypeId {
        /// Type ID.
        type_id: u32,
        /// Some arbitrary context to help narrow the source of the error.
        context: &'static str,
    },
    #[error("The chain expects a signed extension with the name {0}, but we did not provide one")]
    UnknownTransactionExtension(String),
    #[error("Error constructing extrinsic parameters: {0}")]
    Custom(Box<dyn core::error::Error + Send + Sync + 'static>),
}

impl ExtrinsicParamsError {
    /// Create a custom [`ExtrinsicParamsError`] from a string.
    pub fn custom<S: Into<String>>(error: S) -> Self {
        let error: String = error.into();
        let error: Box<dyn core::error::Error + Send + Sync + 'static> = Box::from(error);
        ExtrinsicParamsError::Custom(error)
    }
}

impl From<core::convert::Infallible> for ExtrinsicParamsError {
    fn from(value: core::convert::Infallible) -> Self {
        match value {}
    }
}
