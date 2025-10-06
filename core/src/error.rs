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
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    Extrinsic(#[from] ExtrinsicError),
    #[error(transparent)]
    Constant(#[from] ConstantError),
    #[error(transparent)]
    CustomValue(#[from] CustomValueError),
    #[error(transparent)]
    RuntimeApi(#[from] RuntimeApiError),
    #[error(transparent)]
    ViewFunction(#[from] ViewFunctionError),
    #[error(transparent)]
    Events(#[from] EventsError),
}

#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum EventsError {
    #[error("Can't decode event: can't decode phase: {0}")]
    CannotDecodePhase(codec::Error),
    #[error("Can't decode event: can't decode pallet index: {0}")]
    CannotDecodePalletIndex(codec::Error),
    #[error("Can't decode event: can't decode variant index: {0}")]
    CannotDecodeVariantIndex(codec::Error),
    #[error("Can't decode event: can't find pallet with index {0}")]
    CannotFindPalletWithIndex(u8),
    #[error("Can't decode event: can't find variant with index {variant_index} in pallet {pallet_name}")]
    CannotFindVariantWithIndex {
        pallet_name: String,
        variant_index: u8
    },
    #[error("Can't decode field {field_name:?} in event {pallet_name}.{event_name}: {reason}")]
    CannotDecodeFieldInEvent {
        pallet_name: String,
        event_name: String,
        field_name: String,
        reason: scale_decode::visitor::DecodeError
    },
    #[error("Can't decode event topics: {0}")]
    CannotDecodeEventTopics(codec::Error),
    #[error("Can't decode the fields of event {pallet_name}.{event_name}: {reason}")]
    CannotDecodeEventFields {
        pallet_name: String,
        event_name: String,
        reason: scale_decode::Error
    },
    #[error("Can't decode event {pallet_name}.{event_name} to Event enum: {reason}")]
    CannotDecodeEventEnum {
        pallet_name: String,
        event_name: String,
        reason: scale_decode::Error
    }
}

#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ViewFunctionError {
    #[error("The static View Function address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Can't find View Function: pallet {0} not found")]
    PalletNotFound(String),
    #[error("Can't find View Function {function_name} in pallet {pallet_name}")]
    ViewFunctionNotFound {
        pallet_name: String,
        function_name: String,
    },
    #[error("Failed to encode View Function inputs: {0}")]
    CouldNotEncodeInputs(frame_decode::view_functions::ViewFunctionInputsEncodeError),
    #[error("Failed to decode View Function: {0}")]
    CouldNotDecodeResponse(frame_decode::view_functions::ViewFunctionDecodeError<u32>),
}

#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum RuntimeApiError {
    #[error("The static Runtime API address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Runtime API trait not found: {0}")]
    TraitNotFound(String),
    #[error("Runtime API method {method_name} not found in trait {trait_name}")]
    MethodNotFound {
        trait_name: String,
        method_name: String,
    },
    #[error("Failed to encode Runtime API inputs: {0}")]
    CouldNotEncodeInputs(frame_decode::runtime_apis::RuntimeApiInputsEncodeError),
    #[error("Failed to decode Runtime API: {0}")]
    CouldNotDecodeResponse(frame_decode::runtime_apis::RuntimeApiDecodeError<u32>),
}

#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CustomValueError {
    #[error("The static custom value address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("The custom value '{0}' was not found")]
    NotFound(String),
    #[error("Failed to decode custom value: {0}")]
    CouldNotDecodeCustomValue(frame_decode::custom_values::CustomValueDecodeError<u32>),
}

/// Something went wrong working with a constant.
#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ConstantError {
    #[error("The static constant address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Can't find constant: pallet with name {0} not found")]
    PalletNameNotFound(String),
    #[error("Constant '{constant_name}' not found in pallet {pallet_name} in the live chain metadata")]
    ConstantNameNotFound {
        pallet_name: String,
        constant_name: String
    },
    #[error("Failed to decode constant: {0}")]
    CouldNotDecodeConstant(frame_decode::constants::ConstantDecodeError<u32>)
}

/// Something went wrong trying to encode or decode a storage address.
#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StorageError {
    #[error("The static storage address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Can't find storage value: pallet with name {0} not found")]
    PalletNameNotFound(String),
    #[error("Storage entry '{entry_name}' not found in pallet {pallet_name} in the live chain metadata")]
    StorageEntryNotFound {
        pallet_name: String,
        entry_name: String
    },
    #[error("Cannot obtain storage information from metadata: {0}")]
    StorageInfoError(frame_decode::storage::StorageInfoError<'static>),
    #[error("Cannot decode storage value: {0}")]
    StorageValueDecodeError(frame_decode::storage::StorageValueDecodeError<u32>),
    #[error("Cannot encode storage key: {0}")]
    StorageKeyEncodeError(frame_decode::storage::StorageKeyEncodeError),
}

/// An error that can be encountered when constructing a transaction.
#[derive(Debug, DeriveError)]
#[allow(missing_docs)]
pub enum ExtrinsicError {
    #[error("The extrinsic payload is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Can't find extrinsic: pallet with name {0} not found")]
    PalletNameNotFound(String),
    #[error("Can't find extrinsic: call name {call_name} doesn't exist in pallet {pallet_name}")]
    CallNameNotFound {
        pallet_name: String,
        call_name: String
    },
    #[error("Can't encode the extrinsic call data: {0}")]
    CannotEncodeCallData(scale_encode::Error),
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
    #[error("{0}")]
    ExtrinsicDecodeErrorAt(#[from] ExtrinsicDecodeErrorAt),
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

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
#[error("Cannot decode extrinsic at index {extrinsic_index}: {error}")]
pub struct ExtrinsicDecodeErrorAt {
    pub extrinsic_index: usize,
    pub error: ExtrinsicDecodeErrorAtReason
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExtrinsicDecodeErrorAtReason {
    #[error("{0}")]
    DecodeError(frame_decode::extrinsics::ExtrinsicDecodeError),
    #[error("Leftover bytes")]
    LeftoverBytes(Vec<u8>)
}

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
