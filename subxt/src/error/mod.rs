// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the errors that can be returned.

mod dispatch_error;

use subxt_core::error::Error as CoreError;

crate::macros::cfg_unstable_light_client! {
    pub use subxt_lightclient::LightClientError;
}

// Re-export dispatch error types:
pub use dispatch_error::{
    ArithmeticError, DispatchError, ModuleError, TokenError, TransactionalError,
};

// Re-expose the errors we use from other crates here:
pub use crate::Metadata;
pub use subxt_metadata::TryFromError as MetadataTryFromError;

// Re-export subxt-core error types that we'll use directly:
pub use subxt_core::error::{
    ConstantError as CoreConstantError,
    CustomValueError as CoreCustomValueError,
    EventsError as CoreEventsError,
    ExtrinsicError as CoreExtrinsicError,
    ExtrinsicParamsError,
    RuntimeApiError as CoreRuntimeApiError,
    StorageError as CoreStorageError,
    ViewFunctionError as CoreViewFunctionError,
};

/// The underlying error enum, generic over the type held by the `Runtime`
/// variant. Prefer to use the [`Error<E>`] and [`Error`] aliases over
/// using this type directly.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Io error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Rpc error.
    #[error(transparent)]
    Rpc(#[from] RpcError),
    /// Serde serialization error
    #[error(transparent)]
    Serialization(#[from] serde_json::error::Error),
    /// Error decoding metadata.
    #[error(transparent)]
    MetadataDecoding(#[from] MetadataTryFromError),
    /// Runtime error.
    #[error(transparent)]
    Runtime(#[from] DispatchError),
    /// Transaction progress error.
    #[error(transparent)]
    Transaction(#[from] TransactionError),
    /// Block related error.
    #[error(transparent)]
    Block(#[from] BlockError),
    /// Storage error.
    #[error(transparent)]
    Storage(#[from] StorageError),
    /// Storage key error.
    #[error(transparent)]
    StorageKey(#[from] StorageKeyError),
    /// Storage value error.
    #[error(transparent)]
    StorageValue(#[from] StorageValueError),
    /// Constant error.
    #[error(transparent)]
    Constant(#[from] ConstantError),
    /// Custom value error.
    #[error(transparent)]
    CustomValue(#[from] CustomValueError),
    /// Runtime API error.
    #[error(transparent)]
    RuntimeApi(#[from] RuntimeApiError),
    /// View function error.
    #[error(transparent)]
    ViewFunction(#[from] ViewFunctionError),
    /// Events error.
    #[error(transparent)]
    Events(#[from] EventsError),
    /// Extrinsic error.
    #[error(transparent)]
    Extrinsic(#[from] ExtrinsicError),
    /// The bytes representing an error that we were unable to decode.
    #[error("An error occurred but it could not be decoded: {0:?}")]
    Unknown(Vec<u8>),
    /// Light client error.
    #[cfg(feature = "unstable-light-client")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-light-client")))]
    #[error(transparent)]
    LightClient(#[from] LightClientError),
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

impl<'a> From<&'a str> for Error {
    fn from(error: &'a str) -> Self {
        Error::Other(error.into())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Other(error)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}

impl From<codec::Error> for Error {
    fn from(value: codec::Error) -> Self {
        // Codec errors typically happen during event/extrinsic decoding, so we map to Other
        Error::Other(format!("Codec error: {}", value))
    }
}

impl From<scale_decode::Error> for Error {
    fn from(value: scale_decode::Error) -> Self {
        // Scale decode errors typically happen during decoding, so we map to Other
        Error::Other(format!("Decode error: {}", value))
    }
}

impl From<subxt_rpcs::Error> for Error {
    fn from(value: subxt_rpcs::Error) -> Self {
        Error::Rpc(value.into())
    }
}

// Add From implementations for core error types through their module-specific wrappers
impl From<CoreStorageError> for Error {
    fn from(e: CoreStorageError) -> Self {
        Error::Storage(StorageError::from(e))
    }
}

impl From<CoreExtrinsicError> for Error {
    fn from(e: CoreExtrinsicError) -> Self {
        Error::Extrinsic(ExtrinsicError::from(e))
    }
}

impl From<CoreConstantError> for Error {
    fn from(e: CoreConstantError) -> Self {
        Error::Constant(ConstantError::from(e))
    }
}

impl From<CoreCustomValueError> for Error {
    fn from(e: CoreCustomValueError) -> Self {
        Error::CustomValue(CustomValueError::from(e))
    }
}

impl From<CoreRuntimeApiError> for Error {
    fn from(e: CoreRuntimeApiError) -> Self {
        Error::RuntimeApi(RuntimeApiError::from(e))
    }
}

impl From<CoreViewFunctionError> for Error {
    fn from(e: CoreViewFunctionError) -> Self {
        Error::ViewFunction(ViewFunctionError::from(e))
    }
}

impl From<CoreEventsError> for Error {
    fn from(e: CoreEventsError) -> Self {
        Error::Events(EventsError::from(e))
    }
}

// Add From implementations for frame_decode error types that go through StorageError
impl From<frame_decode::storage::StorageInfoError<'_>> for Error {
    fn from(e: frame_decode::storage::StorageInfoError<'_>) -> Self {
        Error::Storage(StorageError::from(e))
    }
}

impl From<frame_decode::storage::StorageKeyEncodeError> for Error {
    fn from(e: frame_decode::storage::StorageKeyEncodeError) -> Self {
        Error::Storage(StorageError::from(e))
    }
}

impl Error {
    /// Checks whether the error was caused by a RPC re-connection.
    pub fn is_disconnected_will_reconnect(&self) -> bool {
        matches!(
            self,
            Error::Rpc(RpcError::ClientError(
                subxt_rpcs::Error::DisconnectedWillReconnect(_)
            ))
        )
    }

    /// Checks whether the error was caused by a RPC request being rejected.
    pub fn is_rpc_limit_reached(&self) -> bool {
        matches!(self, Error::Rpc(RpcError::LimitReached))
    }
}

/// An RPC error. Since we are generic over the RPC client that is used,
/// the error is boxed and could be casted.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RpcError {
    // Dev note: We need the error to be safely sent between threads
    // for `subscribe_to_block_headers_filling_in_gaps` and friends.
    /// Error related to the RPC client.
    #[error("RPC error: {0}")]
    ClientError(#[from] subxt_rpcs::Error),
    /// This error signals that we got back a [`subxt_rpcs::methods::chain_head::MethodResponse::LimitReached`],
    /// which is not technically an RPC error but is treated as an error in our own APIs.
    #[error("RPC error: limit reached")]
    LimitReached,
    /// The RPC subscription dropped.
    #[error("RPC error: subscription dropped.")]
    SubscriptionDropped,
}

/// Block error
#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BlockError {
    /// An error containing the hash of the block that was not found.
    #[error("Could not find a block with hash {0} (perhaps it was on a non-finalized fork?)")]
    NotFound(String),
    /// Leftover bytes found after decoding the extrinsic.
    #[error(
        "After decoding the exntrinsic at index {extrinsic_index}, {num_leftover_bytes} bytes were left, suggesting that decoding may have failed"
    )]
    LeftoverBytes {
        /// Index of the extrinsic that failed to decode.
        extrinsic_index: usize,
        /// Number of bytes leftover after decoding the extrinsic.
        num_leftover_bytes: usize,
    },
    /// Decoding error.
    #[error("Cannot decode extrinsic at index {extrinsic_index}: {error}")]
    ExtrinsicDecodeError {
        /// Index of the extrinsic that failed to decode.
        extrinsic_index: usize,
        /// The decode error.
        error: frame_decode::extrinsics::ExtrinsicDecodeError,
    },
}

impl BlockError {
    /// Produce an error that a block with the given hash cannot be found.
    pub fn not_found(hash: impl AsRef<[u8]>) -> BlockError {
        let hash = format!("0x{}", hex::encode(hash));
        BlockError::NotFound(hash)
    }
}

/// Transaction error.
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum TransactionError {
    /// The block hash that the transaction was added to could not be found.
    /// This is probably because the block was retracted before being finalized.
    #[error(
        "The block containing the transaction can no longer be found (perhaps it was on a non-finalized fork?)"
    )]
    BlockNotFound,
    /// An error happened on the node that the transaction was submitted to.
    #[error("Error handling transaction: {0}")]
    Error(String),
    /// The transaction was deemed invalid.
    #[error("The transaction is not valid: {0}")]
    Invalid(String),
    /// The transaction was dropped.
    #[error("The transaction was dropped: {0}")]
    Dropped(String),
}

// Module-specific error types following the subxt-core pattern:

/// Errors that can occur when working with storage.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StorageError {
    #[error("Storage: The static storage address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Storage: Can't find storage value - pallet with name '{0}' not found")]
    PalletNameNotFound(String),
    #[error("Storage: Entry '{entry_name}' not found in pallet '{pallet_name}'")]
    StorageEntryNotFound {
        pallet_name: String,
        entry_name: String,
    },
    #[error("Storage: Cannot obtain storage information from metadata: {0}")]
    StorageInfoError(String),
    #[error("Storage: Cannot decode storage value: {0}")]
    StorageValueDecodeError(String),
    #[error("Storage: Cannot encode storage key: {0}")]
    StorageKeyEncodeError(#[from] frame_decode::storage::StorageKeyEncodeError),
    #[error("Storage: RPC error - {0}")]
    Rpc(#[from] RpcError),
    #[error("Storage: Could not fetch next entry from storage subscription - {reason}")]
    StorageEventError {
        reason: String,
    },
    #[error(
        "Storage: Wrong number of keys provided (expected {num_keys_expected}, got {num_keys_provided})"
    )]
    WrongNumberOfKeysProvidedForFetch {
        num_keys_provided: usize,
        num_keys_expected: usize,
    },
    #[error(
        "Storage: Too many keys provided for iteration (expected at most {max_keys_expected}, got {num_keys_provided})"
    )]
    TooManyKeysProvidedForIter {
        num_keys_provided: usize,
        max_keys_expected: usize,
    },
}

impl From<CoreStorageError> for StorageError {
    fn from(e: CoreStorageError) -> Self {
        match e {
            CoreStorageError::IncompatibleCodegen => StorageError::IncompatibleCodegen,
            CoreStorageError::PalletNameNotFound(name) => StorageError::PalletNameNotFound(name),
            CoreStorageError::StorageEntryNotFound { pallet_name, entry_name } => {
                StorageError::StorageEntryNotFound { pallet_name, entry_name }
            }
            CoreStorageError::StorageInfoError(e) => StorageError::StorageInfoError(e.to_string()),
            CoreStorageError::StorageValueDecodeError(e) => {
                StorageError::StorageValueDecodeError(e.to_string())
            }
            CoreStorageError::StorageKeyEncodeError(e) => StorageError::StorageKeyEncodeError(e),
            _ => StorageError::StorageInfoError(e.to_string()),
        }
    }
}

impl From<frame_decode::storage::StorageInfoError<'_>> for StorageError {
    fn from(e: frame_decode::storage::StorageInfoError<'_>) -> Self {
        StorageError::StorageInfoError(e.to_string())
    }
}

/// Errors that can occur when working with storage keys.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StorageKeyError {
    #[error("Storage: Could not decode storage key - {reason}")]
    DecodeError {
        reason: frame_decode::storage::StorageKeyDecodeError<String>,
    },
    #[error("Storage: Could not decode storage key - leftover bytes after decoding")]
    LeftoverBytes {
        leftover_bytes: Vec<u8>,
    },
    #[error("Storage: Could not decode part of storage key at index {index} - {reason}")]
    DecodePartError {
        index: usize,
        reason: scale_decode::Error,
    },
    #[error("Storage: Could not decode values from storage key - {reason}")]
    DecodeKeyValueError {
        reason: frame_decode::storage::StorageKeyValueDecodeError,
    },
}

/// Errors that can occur when working with storage values.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StorageValueError {
    #[error("Storage: Could not decode storage value - {reason}")]
    DecodeError {
        reason: scale_decode::Error,
    },
    #[error("Storage: Could not decode storage value - leftover bytes after decoding")]
    LeftoverBytes {
        leftover_bytes: Vec<u8>,
    },
}

/// Errors that can occur when working with constants.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ConstantError {
    #[error("Constant: The static constant address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Constant: Can't find constant - pallet with name '{0}' not found")]
    PalletNameNotFound(String),
    #[error("Constant: '{constant_name}' not found in pallet '{pallet_name}'")]
    ConstantNameNotFound {
        pallet_name: String,
        constant_name: String,
    },
    #[error("Constant: Failed to decode constant - {0}")]
    CouldNotDecodeConstant(String),
}

impl From<CoreConstantError> for ConstantError {
    fn from(e: CoreConstantError) -> Self {
        match e {
            CoreConstantError::IncompatibleCodegen => ConstantError::IncompatibleCodegen,
            CoreConstantError::PalletNameNotFound(name) => ConstantError::PalletNameNotFound(name),
            CoreConstantError::ConstantNameNotFound { pallet_name, constant_name } => {
                ConstantError::ConstantNameNotFound { pallet_name, constant_name }
            }
            CoreConstantError::CouldNotDecodeConstant(e) => {
                ConstantError::CouldNotDecodeConstant(e.to_string())
            }
            _ => ConstantError::CouldNotDecodeConstant(e.to_string()),
        }
    }
}

/// Errors that can occur when working with custom values.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CustomValueError {
    #[error("Custom Value: The static custom value address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Custom Value: '{0}' was not found")]
    NotFound(String),
    #[error("Custom Value: Failed to decode custom value - {0}")]
    CouldNotDecodeCustomValue(String),
}

impl From<CoreCustomValueError> for CustomValueError {
    fn from(e: CoreCustomValueError) -> Self {
        match e {
            CoreCustomValueError::IncompatibleCodegen => CustomValueError::IncompatibleCodegen,
            CoreCustomValueError::NotFound(name) => CustomValueError::NotFound(name),
            CoreCustomValueError::CouldNotDecodeCustomValue(e) => {
                CustomValueError::CouldNotDecodeCustomValue(e.to_string())
            }
            _ => CustomValueError::CouldNotDecodeCustomValue(e.to_string()),
        }
    }
}

/// Errors that can occur when working with runtime APIs.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum RuntimeApiError {
    #[error("Runtime API: The static Runtime API address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Runtime API: Trait '{0}' not found")]
    TraitNotFound(String),
    #[error("Runtime API: Method '{method_name}' not found in trait '{trait_name}'")]
    MethodNotFound {
        trait_name: String,
        method_name: String,
    },
    #[error("Runtime API: Failed to encode inputs - {0}")]
    CouldNotEncodeInputs(String),
    #[error("Runtime API: Failed to decode response - {0}")]
    CouldNotDecodeResponse(String),
    #[error("Runtime API: RPC error - {0}")]
    Rpc(#[from] RpcError),
}

impl From<CoreRuntimeApiError> for RuntimeApiError {
    fn from(e: CoreRuntimeApiError) -> Self {
        match e {
            CoreRuntimeApiError::IncompatibleCodegen => RuntimeApiError::IncompatibleCodegen,
            CoreRuntimeApiError::TraitNotFound(name) => RuntimeApiError::TraitNotFound(name),
            CoreRuntimeApiError::MethodNotFound { trait_name, method_name } => {
                RuntimeApiError::MethodNotFound { trait_name, method_name }
            }
            CoreRuntimeApiError::CouldNotEncodeInputs(e) => {
                RuntimeApiError::CouldNotEncodeInputs(e.to_string())
            }
            CoreRuntimeApiError::CouldNotDecodeResponse(e) => {
                RuntimeApiError::CouldNotDecodeResponse(e.to_string())
            }
            _ => RuntimeApiError::CouldNotDecodeResponse(e.to_string()),
        }
    }
}

/// Errors that can occur when working with view functions.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ViewFunctionError {
    #[error("View Function: The static View Function address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("View Function: Pallet '{0}' not found")]
    PalletNotFound(String),
    #[error("View Function: '{function_name}' not found in pallet '{pallet_name}'")]
    ViewFunctionNotFound {
        pallet_name: String,
        function_name: String,
    },
    #[error("View Function: Failed to encode inputs - {0}")]
    CouldNotEncodeInputs(String),
    #[error("View Function: Failed to decode response - {0}")]
    CouldNotDecodeResponse(String),
    #[error("View Function: RPC error - {0}")]
    Rpc(#[from] RpcError),
}

impl From<CoreViewFunctionError> for ViewFunctionError {
    fn from(e: CoreViewFunctionError) -> Self {
        match e {
            CoreViewFunctionError::IncompatibleCodegen => ViewFunctionError::IncompatibleCodegen,
            CoreViewFunctionError::PalletNotFound(name) => ViewFunctionError::PalletNotFound(name),
            CoreViewFunctionError::ViewFunctionNotFound { pallet_name, function_name } => {
                ViewFunctionError::ViewFunctionNotFound { pallet_name, function_name }
            }
            CoreViewFunctionError::CouldNotEncodeInputs(e) => {
                ViewFunctionError::CouldNotEncodeInputs(e.to_string())
            }
            CoreViewFunctionError::CouldNotDecodeResponse(e) => {
                ViewFunctionError::CouldNotDecodeResponse(e.to_string())
            }
            _ => ViewFunctionError::CouldNotDecodeResponse(e.to_string()),
        }
    }
}

/// Errors that can occur when working with events.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum EventsError {
    #[error("Events: Can't decode event - can't decode phase: {0}")]
    CannotDecodePhase(codec::Error),
    #[error("Events: Can't decode event - can't decode pallet index: {0}")]
    CannotDecodePalletIndex(codec::Error),
    #[error("Events: Can't decode event - can't decode variant index: {0}")]
    CannotDecodeVariantIndex(codec::Error),
    #[error("Events: Can't decode event - can't find pallet with index {0}")]
    CannotFindPalletWithIndex(u8),
    #[error("Events: Can't decode event - can't find variant with index {variant_index} in pallet {pallet_name}")]
    CannotFindVariantWithIndex {
        pallet_name: String,
        variant_index: u8,
    },
    #[error("Events: Can't decode field {field_name:?} in event {pallet_name}.{event_name} - {reason}")]
    CannotDecodeFieldInEvent {
        pallet_name: String,
        event_name: String,
        field_name: String,
        reason: scale_decode::visitor::DecodeError,
    },
    #[error("Events: Can't decode event topics: {0}")]
    CannotDecodeEventTopics(codec::Error),
    #[error("Events: Can't decode fields of event {pallet_name}.{event_name} - {reason}")]
    CannotDecodeEventFields {
        pallet_name: String,
        event_name: String,
        reason: scale_decode::Error,
    },
    #[error("Events: Can't decode event {pallet_name}.{event_name} to Event enum - {reason}")]
    CannotDecodeEventEnum {
        pallet_name: String,
        event_name: String,
        reason: scale_decode::Error,
    },
    #[error("Events: RPC error - {0}")]
    Rpc(#[from] RpcError),
}

impl From<CoreEventsError> for EventsError {
    fn from(e: CoreEventsError) -> Self {
        match e {
            CoreEventsError::CannotDecodePhase(err) => EventsError::CannotDecodePhase(err),
            CoreEventsError::CannotDecodePalletIndex(err) => EventsError::CannotDecodePalletIndex(err),
            CoreEventsError::CannotDecodeVariantIndex(err) => EventsError::CannotDecodeVariantIndex(err),
            CoreEventsError::CannotFindPalletWithIndex(idx) => EventsError::CannotFindPalletWithIndex(idx),
            CoreEventsError::CannotFindVariantWithIndex { pallet_name, variant_index } => {
                EventsError::CannotFindVariantWithIndex { pallet_name, variant_index }
            }
            CoreEventsError::CannotDecodeFieldInEvent { pallet_name, event_name, field_name, reason } => {
                EventsError::CannotDecodeFieldInEvent { pallet_name, event_name, field_name, reason }
            }
            CoreEventsError::CannotDecodeEventTopics(err) => EventsError::CannotDecodeEventTopics(err),
            CoreEventsError::CannotDecodeEventFields { pallet_name, event_name, reason } => {
                EventsError::CannotDecodeEventFields { pallet_name, event_name, reason }
            }
            CoreEventsError::CannotDecodeEventEnum { pallet_name, event_name, reason } => {
                EventsError::CannotDecodeEventEnum { pallet_name, event_name, reason }
            }
            _ => EventsError::CannotDecodeEventTopics(codec::Error::from("Unknown events error")),
        }
    }
}

/// Errors that can occur when working with extrinsics/transactions.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExtrinsicError {
    #[error("Extrinsic: The extrinsic payload is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Extrinsic: Can't find extrinsic - pallet with name '{0}' not found")]
    PalletNameNotFound(String),
    #[error("Extrinsic: Call '{call_name}' doesn't exist in pallet '{pallet_name}'")]
    CallNameNotFound {
        pallet_name: String,
        call_name: String,
    },
    #[error("Extrinsic: Can't encode call data - {0}")]
    CannotEncodeCallData(scale_encode::Error),
    #[error("Extrinsic: Unsupported extrinsic version")]
    UnsupportedVersion,
    #[error("Extrinsic: Cannot construct transaction extensions - {0}")]
    Params(#[from] ExtrinsicParamsError),
    #[error("Extrinsic: Cannot decode transaction extension '{name}' - {error}")]
    CouldNotDecodeTransactionExtension {
        name: String,
        error: scale_decode::Error,
    },
    #[error("Extrinsic: Leftover bytes after decoding extrinsic at index {extrinsic_index} ({num_leftover_bytes} bytes remaining)")]
    LeftoverBytes {
        extrinsic_index: usize,
        num_leftover_bytes: usize,
    },
    #[error("Extrinsic: Failed to decode extrinsic at index {extrinsic_index} - {error}")]
    ExtrinsicDecodeError {
        extrinsic_index: usize,
        error: frame_decode::extrinsics::ExtrinsicDecodeError,
    },
    #[error("Extrinsic: Failed to decode fields of extrinsic at index {extrinsic_index} - {error}")]
    CannotDecodeFields {
        extrinsic_index: usize,
        error: scale_decode::Error,
    },
    #[error("Extrinsic: Failed to decode extrinsic at index {extrinsic_index} to root enum - {error}")]
    CannotDecodeIntoRootExtrinsic {
        extrinsic_index: usize,
        error: scale_decode::Error,
    },
    #[error("Extrinsic: RPC error - {0}")]
    Rpc(#[from] RpcError),
}

impl From<CoreExtrinsicError> for ExtrinsicError {
    fn from(e: CoreExtrinsicError) -> Self {
        match e {
            CoreExtrinsicError::IncompatibleCodegen => ExtrinsicError::IncompatibleCodegen,
            CoreExtrinsicError::PalletNameNotFound(name) => ExtrinsicError::PalletNameNotFound(name),
            CoreExtrinsicError::CallNameNotFound { pallet_name, call_name } => {
                ExtrinsicError::CallNameNotFound { pallet_name, call_name }
            }
            CoreExtrinsicError::CannotEncodeCallData(err) => ExtrinsicError::CannotEncodeCallData(err),
            CoreExtrinsicError::UnsupportedVersion => ExtrinsicError::UnsupportedVersion,
            CoreExtrinsicError::Params(err) => ExtrinsicError::Params(err),
            CoreExtrinsicError::CouldNotDecodeTransactionExtension { name, error } => {
                ExtrinsicError::CouldNotDecodeTransactionExtension { name, error }
            }
            CoreExtrinsicError::LeftoverBytes { extrinsic_index, num_leftover_bytes } => {
                ExtrinsicError::LeftoverBytes { extrinsic_index, num_leftover_bytes }
            }
            CoreExtrinsicError::ExtrinsicDecodeError { extrinsic_index, error } => {
                ExtrinsicError::ExtrinsicDecodeError { extrinsic_index, error }
            }
            CoreExtrinsicError::CannotDecodeFields { extrinsic_index, error } => {
                ExtrinsicError::CannotDecodeFields { extrinsic_index, error }
            }
            CoreExtrinsicError::CannotDecodeIntoRootExtrinsic { extrinsic_index, error } => {
                ExtrinsicError::CannotDecodeIntoRootExtrinsic { extrinsic_index, error }
            }
            _ => ExtrinsicError::CannotEncodeCallData(scale_encode::Error::custom_string(e.to_string())),
        }
    }
}
