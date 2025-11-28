// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the errors that can be returned.

mod dispatch_error;
mod hex;

use thiserror::Error as DeriveError;

#[cfg(feature = "unstable-light-client")]
pub use subxt_lightclient::LightClientError;

// Re-export dispatch error types:
pub use dispatch_error::{
    ArithmeticError, DispatchError, ModuleError, TokenError, TransactionalError,
};

// Re-expose the errors we use from other crates here:
pub use subxt_metadata::Metadata;
pub use hex::Hex;
pub use scale_decode::Error as DecodeError;
pub use scale_encode::Error as EncodeError;
pub use subxt_metadata::TryFromError as MetadataTryFromError;

/// A global error type. Any of the errors exposed here can convert into this
/// error via `.into()`, but this error isn't itself exposed from anything.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    OnlineClientError(#[from] OnlineClientError),
    #[error(transparent)]
    OfflineClientAtBlockError(#[from] OfflineClientAtBlockError),
    #[error(transparent)]
    OnlineClientAtBlockError(#[from] OnlineClientAtBlockError),
    #[error(transparent)]
    ExtrinsicDecodeErrorAt(#[from] ExtrinsicDecodeErrorAt),
    #[error(transparent)]
    ConstantError(#[from] ConstantError),
    #[error(transparent)]
    CustomValueError(#[from] CustomValueError),
    #[error(transparent)]
    StorageKeyError(#[from] StorageKeyError),
    #[error(transparent)]
    StorageValueError(#[from] StorageValueError),
    #[error(transparent)]
    BackendError(#[from] BackendError),
    #[error(transparent)]
    BlockError(#[from] BlockError),
    #[error(transparent)]
    AccountNonceError(#[from] AccountNonceError),
    #[error(transparent)]
    RuntimeUpdaterError(#[from] RuntimeUpdaterError),
    #[error(transparent)]
    RuntimeUpdateeApplyError(#[from] RuntimeUpdateeApplyError),
    #[error(transparent)]
    RuntimeApiError(#[from] RuntimeApiError),
    #[error(transparent)]
    EventsError(#[from] EventsError),
    #[error(transparent)]
    ExtrinsicError(#[from] ExtrinsicError),
    #[error(transparent)]
    ViewFunctionError(#[from] ViewFunctionError),
    #[error(transparent)]
    TransactionProgressError(#[from] TransactionProgressError),
    #[error(transparent)]
    TransactionStatusError(#[from] TransactionStatusError),
    #[error(transparent)]
    TransactionEventsError(#[from] TransactionEventsError),
    #[error(transparent)]
    TransactionFinalizedSuccessError(#[from] TransactionFinalizedSuccessError),
    #[error(transparent)]
    ModuleErrorDetailsError(#[from] ModuleErrorDetailsError),
    #[error(transparent)]
    ModuleErrorDecodeError(#[from] ModuleErrorDecodeError),
    #[error(transparent)]
    DispatchErrorDecodeError(#[from] DispatchErrorDecodeError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    // Dev note: Subxt doesn't directly return Raw* errors. These exist so that when
    // users use common crates (like parity-scale-codec and subxt-rpcs), errors returned
    // there can be handled automatically using ? when the expected error is subxt::Error.
    #[error("Other RPC client error: {0}")]
    OtherRpcClientError(#[from] subxt_rpcs::Error),
    #[error("Other codec error: {0}")]
    OtherCodecError(#[from] codec::Error),
    #[cfg(feature = "unstable-light-client")]
    #[error("Other light client error: {0}")]
    OtherLightClientError(#[from] subxt_lightclient::LightClientError),
    #[cfg(feature = "unstable-light-client")]
    #[error("Other light client RPC error: {0}")]
    OtherLightClientRpcError(#[from] subxt_lightclient::LightClientRpcError),
    // Dev note: Nothing in subxt should ever emit this error. It can instead be used
    // to easily map other errors into a subxt::Error for convenience. Some From impls
    // make this automatic for common "other" error types.
    #[error("Other error: {0}")]
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<std::convert::Infallible> for Error {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}

impl Error {
    /// Create a generic error. This is a quick workaround when you are using
    /// [`Error`] and have a non-Subxt error to return.
    pub fn other<E: std::error::Error + Send + Sync + 'static>(error: E) -> Error {
        Error::Other(Box::new(error))
    }

    /// Create a generic error from a string. This is a quick workaround when you are using
    /// [`Error`] and have a non-Subxt error to return.
    pub fn other_str(error: impl Into<String>) -> Error {
        #[derive(thiserror::Error, Debug, Clone)]
        #[error("{0}")]
        struct StrError(String);
        Error::Other(Box::new(StrError(error.into())))
    }

    /// Checks whether the error was caused by a RPC re-connection.
    pub fn is_disconnected_will_reconnect(&self) -> bool {
        matches!(
            self.backend_error(),
            Some(BackendError::Rpc(RpcError::ClientError(
                subxt_rpcs::Error::DisconnectedWillReconnect(_)
            )))
        )
    }

    /// Checks whether the error was caused by a RPC request being rejected.
    pub fn is_rpc_limit_reached(&self) -> bool {
        matches!(
            self.backend_error(),
            Some(BackendError::Rpc(RpcError::LimitReached))
        )
    }

    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            Error::BlockError(e) => e.backend_error(),
            Error::AccountNonceError(e) => e.backend_error(),
            Error::OnlineClientError(e) => e.backend_error(),
            Error::RuntimeUpdaterError(e) => e.backend_error(),
            Error::RuntimeApiError(e) => e.backend_error(),
            Error::EventsError(e) => e.backend_error(),
            Error::ExtrinsicError(e) => e.backend_error(),
            Error::ViewFunctionError(e) => e.backend_error(),
            Error::TransactionProgressError(e) => e.backend_error(),
            Error::TransactionEventsError(e) => e.backend_error(),
            Error::TransactionFinalizedSuccessError(e) => e.backend_error(),
            Error::StorageError(e) => e.backend_error(),
            // Any errors that **don't** return a BackendError anywhere will return None:
            _ => None,
        }
    }
}

/// Errors constructing an offline client at a specific block number.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OfflineClientAtBlockError {
    #[error(
        "Cannot construct OfflineClientAtBlock: spec version not found for block number {block_number}"
    )]
    SpecVersionNotFound {
        /// The block number for which the spec version was not found.
        block_number: u32,
    },
    #[error(
        "Cannot construct OfflineClientAtBlock: metadata not found for spec version {spec_version}"
    )]
    MetadataNotFound {
        /// The spec version for which the metadata was not found.
        spec_version: u32,
    },
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OnlineClientError {
    #[error("Cannot construct OnlineClient: The URL provided is invalid: {url}")]
    InvalidUrl {
        /// The URL that was invalid.
        url: String,
    },
    #[error("Cannot construct OnlineClient: {0}")]
    RpcError(#[from] subxt_rpcs::Error),
    #[error(
        "Cannot construct OnlineClient: Cannot fetch latest finalized block to obtain init details from: {0}"
    )]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot construct OnlineClient: Cannot fetch genesis hash: {0}")]
    CannotGetGenesisHash(BackendError),
    #[error("Cannot construct OnlineClient: Cannot fetch current runtime version: {0}")]
    CannotGetCurrentRuntimeVersion(BackendError),
    #[error("Cannot construct OnlineClient: Cannot fetch metadata: {0}")]
    CannotFetchMetadata(BackendError),
}

impl OnlineClientError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            OnlineClientError::CannotGetLatestFinalizedBlock(e)
            | OnlineClientError::CannotGetGenesisHash(e)
            | OnlineClientError::CannotGetCurrentRuntimeVersion(e)
            | OnlineClientError::CannotFetchMetadata(e) => Some(e),
            _ => None,
        }
    }
}

/// Errors constructing an online client at a specific block number.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OnlineClientAtBlockError {
    #[error(
        "Cannot construct OnlineClientAtBlock: failed to get block hash from node for block {block_number}: {reason}"
    )]
    CannotGetBlockHash {
        /// Block number we failed to get the hash for.
        block_number: u64,
        /// The error we encountered.
        reason: subxt_rpcs::Error,
    },
    #[error("Cannot construct OnlineClientAtBlock: block number {block_number} not found")]
    BlockNotFound {
        /// The block number for which a block was not found.
        block_number: u64,
    },
    #[error(
        "Cannot construct OnlineClientAtBlock: failed to get spec version for block hash {block_hash}: {reason}"
    )]
    CannotGetSpecVersion {
        /// The block hash for which we failed to get the spec version.
        block_hash: String,
        /// The error we encountered.
        reason: String,
    },
    #[error(
        "Cannot construct OnlineClientAtBlock: failed to get metadata for block hash {block_hash}: {reason}"
    )]
    CannotGetMetadata {
        /// The block hash for which we failed to get the metadata.
        block_hash: String,
        /// The error we encountered.
        reason: String,
    },
    #[error(
        "Cannot inject types from metadata: failure to parse a type found in the metadata: {parse_error}"
    )]
    CannotInjectMetadataTypes {
        /// Error parsing a type found in the metadata.
        parse_error: scale_info_legacy::lookup_name::ParseError,
    },
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum BackendError {
    #[error("Backend error: RPC error: {0}")]
    Rpc(#[from] RpcError),
    #[error("Backend error: Could not find metadata version {0}")]
    MetadataVersionNotFound(u32),
    #[error("Backend error: Could not codec::Decode Runtime API response: {0}")]
    CouldNotScaleDecodeRuntimeResponse(codec::Error),
    #[error("Backend error: Could not codec::Decode metadata bytes into subxt::Metadata: {0}")]
    CouldNotDecodeMetadata(codec::Error),
    // This is for errors in `Backend` implementations which aren't any of the "pre-defined" set above:
    #[error("Custom backend error: {0}")]
    Other(String),
}

impl BackendError {
    /// Checks whether the error was caused by a RPC re-connection.
    pub fn is_disconnected_will_reconnect(&self) -> bool {
        matches!(
            self,
            BackendError::Rpc(RpcError::ClientError(
                subxt_rpcs::Error::DisconnectedWillReconnect(_)
            ))
        )
    }

    /// Checks whether the error was caused by a RPC request being rejected.
    pub fn is_rpc_limit_reached(&self) -> bool {
        matches!(self, BackendError::Rpc(RpcError::LimitReached))
    }
}

impl From<subxt_rpcs::Error> for BackendError {
    fn from(value: subxt_rpcs::Error) -> Self {
        BackendError::Rpc(RpcError::ClientError(value))
    }
}

/// An RPC error. Since we are generic over the RPC client that is used,
/// the error is boxed and could be casted.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RpcError {
    /// Error related to the RPC client.
    #[error("RPC error: {0}")]
    ClientError(#[from] subxt_rpcs::Error),
    /// This error signals that we got back a [`subxt_rpcs::methods::chain_head::MethodResponse::LimitReached`],
    /// which is not technically an RPC error but is treated as an error in our own APIs.
    #[error("RPC error: limit reached")]
    LimitReached,
    /// The RPC subscription was dropped.
    #[error("RPC error: subscription dropped.")]
    SubscriptionDropped,
}

/// Block error
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum BlockError {
    #[error(
        "Could not find the block body with hash {block_hash} (perhaps it was on a non-finalized fork?)"
    )]
    BlockNotFound { block_hash: Hex },
    #[error("Could not download the block header with hash {block_hash}: {reason}")]
    CouldNotGetBlockHeader {
        block_hash: Hex,
        reason: BackendError,
    },
    #[error("Could not download the latest block header: {0}")]
    CouldNotGetLatestBlock(BackendError),
    #[error("Could not subscribe to all blocks: {0}")]
    CouldNotSubscribeToAllBlocks(BackendError),
    #[error("Could not subscribe to best blocks: {0}")]
    CouldNotSubscribeToBestBlocks(BackendError),
    #[error("Could not subscribe to finalized blocks: {0}")]
    CouldNotSubscribeToFinalizedBlocks(BackendError),
    #[error("Error getting account nonce at block {block_hash}")]
    AccountNonceError {
        block_hash: Hex,
        account_id: Hex,
        reason: AccountNonceError,
    },
}

impl BlockError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            BlockError::CouldNotGetBlockHeader { reason: e, .. }
            | BlockError::CouldNotGetLatestBlock(e)
            | BlockError::CouldNotSubscribeToAllBlocks(e)
            | BlockError::CouldNotSubscribeToBestBlocks(e)
            | BlockError::CouldNotSubscribeToFinalizedBlocks(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum AccountNonceError {
    #[error("Could not retrieve account nonce: {0}")]
    CouldNotRetrieve(#[from] BackendError),
    #[error("Could not decode account nonce: {0}")]
    CouldNotDecode(#[from] codec::Error),
    #[error("Wrong number of account nonce bytes returned: {0} (expected 2, 4 or 8)")]
    WrongNumberOfBytes(usize),
}

impl AccountNonceError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            AccountNonceError::CouldNotRetrieve(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum RuntimeUpdaterError {
    #[error("Error subscribing to runtime updates: The update stream ended unexpectedly")]
    UnexpectedEndOfUpdateStream,
    #[error("Error subscribing to runtime updates: The finalized block stream ended unexpectedly")]
    UnexpectedEndOfBlockStream,
    #[error("Error subscribing to runtime updates: Can't stream runtime version: {0}")]
    CannotStreamRuntimeVersion(BackendError),
    #[error("Error subscribing to runtime updates: Can't get next runtime version in stream: {0}")]
    CannotGetNextRuntimeVersion(BackendError),
    #[error("Error subscribing to runtime updates: Cannot stream finalized blocks: {0}")]
    CannotStreamFinalizedBlocks(BackendError),
    #[error("Error subscribing to runtime updates: Cannot get next finalized block in stream: {0}")]
    CannotGetNextFinalizedBlock(BackendError),
    #[error("Cannot fetch new metadata for runtime update: {0}")]
    CannotFetchNewMetadata(BackendError),
    #[error(
        "Error subscribing to runtime updates: Cannot find the System.LastRuntimeUpgrade storage entry"
    )]
    CantFindSystemLastRuntimeUpgrade,
    #[error("Error subscribing to runtime updates: Cannot fetch last runtime upgrade: {0}")]
    CantFetchLastRuntimeUpgrade(StorageError),
    #[error("Error subscribing to runtime updates: Cannot decode last runtime upgrade: {0}")]
    CannotDecodeLastRuntimeUpgrade(StorageValueError),
}

impl RuntimeUpdaterError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            RuntimeUpdaterError::CannotStreamRuntimeVersion(e)
            | RuntimeUpdaterError::CannotGetNextRuntimeVersion(e)
            | RuntimeUpdaterError::CannotStreamFinalizedBlocks(e)
            | RuntimeUpdaterError::CannotGetNextFinalizedBlock(e)
            | RuntimeUpdaterError::CannotFetchNewMetadata(e) => Some(e),
            _ => None,
        }
    }
}

/// Error that can occur during upgrade.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum RuntimeUpdateeApplyError {
    #[error("The proposed runtime update is the same as the current version")]
    SameVersion,
}

/// Error working with Runtime APIs
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
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
    #[error("Cannot access Runtime APIs at latest block: Cannot fetch latest finalized block: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot call the Runtime API: {0}")]
    CannotCallApi(BackendError),
}

impl RuntimeApiError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            RuntimeApiError::CannotGetLatestFinalizedBlock(e)
            | RuntimeApiError::CannotCallApi(e) => Some(e),
            _ => None,
        }
    }
}

/// Error working with events.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
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
    #[error(
        "Can't decode event: can't find variant with index {variant_index} in pallet {pallet_name}"
    )]
    CannotFindVariantWithIndex {
        pallet_name: String,
        variant_index: u8,
    },
    #[error("Can't decode field {field_name:?} in event {pallet_name}.{event_name}: {reason}")]
    CannotDecodeFieldInEvent {
        pallet_name: String,
        event_name: String,
        field_name: String,
        reason: scale_decode::visitor::DecodeError,
    },
    #[error("Can't decode event topics: {0}")]
    CannotDecodeEventTopics(codec::Error),
    #[error("Can't decode the fields of event {pallet_name}.{event_name}: {reason}")]
    CannotDecodeEventFields {
        pallet_name: String,
        event_name: String,
        reason: scale_decode::Error,
    },
    #[error("Can't decode event {pallet_name}.{event_name} to Event enum: {reason}")]
    CannotDecodeEventEnum {
        pallet_name: String,
        event_name: String,
        reason: scale_decode::Error,
    },
    #[error("Cannot access events at latest block: Cannot fetch latest finalized block: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot fetch event bytes: {0}")]
    CannotFetchEventBytes(BackendError),
}

impl EventsError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            EventsError::CannotGetLatestFinalizedBlock(e)
            | EventsError::CannotFetchEventBytes(e) => Some(e),
            _ => None,
        }
    }
}

/// Error working with extrinsics.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum ExtrinsicError {
    #[error("The extrinsic payload is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Can't find extrinsic: pallet with name {0} not found")]
    PalletNameNotFound(String),
    #[error("Can't find extrinsic: call name {call_name} doesn't exist in pallet {pallet_name}")]
    CallNameNotFound {
        pallet_name: String,
        call_name: String,
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
        error: scale_decode::Error,
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
        error: scale_decode::Error,
    },
    #[error("Failed to decode the extrinsic at index {extrinsic_index} to a root enum: {error}")]
    CannotDecodeIntoRootExtrinsic {
        /// Index of the extrinsic that we failed to decode
        extrinsic_index: usize,
        /// The decode error.
        error: scale_decode::Error,
    },
    #[error("Could not download block body to extract extrinsics from: {0}")]
    CannotGetBlockBody(BackendError),
    #[error("Block not found: {0}")]
    BlockNotFound(Hex),
    #[error(
        "Extrinsic submission error: Cannot get latest finalized block to grab account nonce at: {0}"
    )]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot find block header for block {block_hash}")]
    CannotFindBlockHeader { block_hash: Hex },
    #[error("Error getting account nonce at block {block_hash}")]
    AccountNonceError {
        block_hash: Hex,
        account_id: Hex,
        reason: AccountNonceError,
    },
    #[error("Cannot submit extrinsic: {0}")]
    ErrorSubmittingTransaction(BackendError),
    #[error("A transaction status error was returned while submitting the extrinsic: {0}")]
    TransactionStatusError(TransactionStatusError),
    #[error(
        "The transaction status stream encountered an error while submitting the extrinsic: {0}"
    )]
    TransactionStatusStreamError(BackendError),
    #[error(
        "The transaction status stream unexpectedly ended, so we don't know the status of the submitted extrinsic"
    )]
    UnexpectedEndOfTransactionStatusStream,
    #[error("Cannot get fee info from Runtime API: {0}")]
    CannotGetFeeInfo(BackendError),
    #[error("Cannot get validation info from Runtime API: {0}")]
    CannotGetValidationInfo(BackendError),
    #[error("Cannot decode ValidationResult bytes: {0}")]
    CannotDecodeValidationResult(codec::Error),
    #[error("ValidationResult bytes could not be decoded")]
    UnexpectedValidationResultBytes(Vec<u8>),
}

impl ExtrinsicError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            ExtrinsicError::CannotGetBlockBody(e)
            | ExtrinsicError::CannotGetLatestFinalizedBlock(e)
            | ExtrinsicError::ErrorSubmittingTransaction(e)
            | ExtrinsicError::TransactionStatusStreamError(e)
            | ExtrinsicError::CannotGetFeeInfo(e)
            | ExtrinsicError::CannotGetValidationInfo(e) => Some(e),
            ExtrinsicError::AccountNonceError { reason, .. } => reason.backend_error(),
            _ => None,
        }
    }
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

/// Error working with View Functions.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
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
    #[error(
        "Cannot access View Functions at latest block: Cannot fetch latest finalized block: {0}"
    )]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot call the View Function Runtime API: {0}")]
    CannotCallApi(BackendError),
}

impl ViewFunctionError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            ViewFunctionError::CannotGetLatestFinalizedBlock(e)
            | ViewFunctionError::CannotCallApi(e) => Some(e),
            _ => None,
        }
    }
}

/// Error during the transaction progress.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum TransactionProgressError {
    #[error("Cannot get the next transaction progress update: {0}")]
    CannotGetNextProgressUpdate(BackendError),
    #[error("Error during transaction progress: {0}")]
    TransactionStatusError(#[from] TransactionStatusError),
    #[error(
        "The transaction status stream unexpectedly ended, so we have no further transaction progress updates"
    )]
    UnexpectedEndOfTransactionStatusStream,
}

impl TransactionProgressError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            TransactionProgressError::CannotGetNextProgressUpdate(e) => Some(e),
            TransactionProgressError::TransactionStatusError(_) => None,
            TransactionProgressError::UnexpectedEndOfTransactionStatusStream => None,
        }
    }
}

/// An error emitted as the result of a transaction progress update.
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TransactionStatusError {
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

/// Error fetching events for a just-submitted transaction
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TransactionEventsError {
    #[error(
        "The block containing the submitted transaction ({block_hash}) could not be downloaded: {error}"
    )]
    CannotFetchBlockBody {
        block_hash: Hex,
        error: BackendError,
    },
    #[error(
        "Cannot find the the submitted transaction (hash: {transaction_hash}) in the block (hash: {block_hash}) it is supposed to be in."
    )]
    CannotFindTransactionInBlock {
        block_hash: Hex,
        transaction_hash: Hex,
    },
    #[error("The block containing the submitted transaction ({block_hash}) could not be found")]
    BlockNotFound { block_hash: Hex },
    #[error(
        "Could not decode event at index {event_index} for the submitted transaction at block {block_hash}: {error}"
    )]
    CannotDecodeEventInBlock {
        event_index: usize,
        block_hash: Hex,
        error: EventsError,
    },
    #[error("Could not fetch events for the submitted transaction: {error}")]
    CannotFetchEventsForTransaction {
        block_hash: Hex,
        transaction_hash: Hex,
        error: EventsError,
    },
    #[error("The transaction led to a DispatchError, but we failed to decode it: {error}")]
    CannotDecodeDispatchError {
        error: DispatchErrorDecodeError,
        bytes: Vec<u8>,
    },
    #[error("The transaction failed with the following dispatch error: {0}")]
    ExtrinsicFailed(#[from] DispatchError),
}

impl TransactionEventsError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            TransactionEventsError::CannotFetchBlockBody { error, .. } => Some(error),
            TransactionEventsError::CannotDecodeEventInBlock { error, .. }
            | TransactionEventsError::CannotFetchEventsForTransaction { error, .. } => {
                error.backend_error()
            }
            _ => None,
        }
    }
}

/// Error waiting for the transaction to be finalized and successful.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs, clippy::large_enum_variant)]
pub enum TransactionFinalizedSuccessError {
    #[error("Could not finalize the transaction: {0}")]
    FinalizationError(#[from] TransactionProgressError),
    #[error("The transaction did not succeed: {0}")]
    SuccessError(#[from] TransactionEventsError),
}

impl TransactionFinalizedSuccessError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            TransactionFinalizedSuccessError::FinalizationError(e) => e.backend_error(),
            TransactionFinalizedSuccessError::SuccessError(e) => e.backend_error(),
        }
    }
}

/// Error decoding the [`DispatchError`]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ModuleErrorDetailsError {
    #[error(
        "Could not get details for the DispatchError: could not find pallet index {pallet_index}"
    )]
    PalletNotFound { pallet_index: u8 },
    #[error(
        "Could not get details for the DispatchError: could not find error index {error_index} in pallet {pallet_name}"
    )]
    ErrorVariantNotFound {
        pallet_name: String,
        error_index: u8,
    },
}

/// Error decoding the [`ModuleError`]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
#[error("Could not decode the DispatchError::Module payload into the given type: {0}")]
pub struct ModuleErrorDecodeError(scale_decode::Error);

/// Error decoding the [`DispatchError`]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum DispatchErrorDecodeError {
    #[error(
        "Could not decode the DispatchError: could not find the corresponding type ID in the metadata"
    )]
    DispatchErrorTypeIdNotFound,
    #[error("Could not decode the DispatchError: {0}")]
    CouldNotDecodeDispatchError(scale_decode::Error),
    #[error("Could not decode the DispatchError::Module variant")]
    CouldNotDecodeModuleError {
        /// The bytes corresponding to the Module variant we were unable to decode:
        bytes: Vec<u8>,
    },
}

/// Error working with storage.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StorageError {
    #[error("The static storage address used is not compatible with the live chain")]
    IncompatibleCodegen,
    #[error("Can't find storage value: pallet with name {0} not found")]
    PalletNameNotFound(String),
    #[error(
        "Storage entry '{entry_name}' not found in pallet {pallet_name} in the live chain metadata"
    )]
    StorageEntryNotFound {
        pallet_name: String,
        entry_name: String,
    },
    #[error("Cannot obtain storage information from metadata: {0}")]
    StorageInfoError(frame_decode::storage::StorageInfoError<'static>),
    #[error("Cannot encode storage key: {0}")]
    StorageKeyEncodeError(frame_decode::storage::StorageKeyEncodeError),
    #[error("Cannot create a key to iterate over a plain entry")]
    CannotIterPlainEntry {
        pallet_name: String,
        entry_name: String,
    },
    #[error(
        "Wrong number of key parts provided to iterate a storage address. We expected at most {max_expected} key parts but got {got} key parts"
    )]
    WrongNumberOfKeyPartsProvidedForIterating { max_expected: usize, got: usize },
    #[error(
        "Wrong number of key parts provided to fetch a storage address. We expected {expected} key parts but got {got} key parts"
    )]
    WrongNumberOfKeyPartsProvidedForFetching { expected: usize, got: usize },
    #[error("Cannot access storage at latest block: Cannot fetch latest finalized block: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error(
        "No storage value found at the given address, and no default value to fall back to using."
    )]
    NoValueFound,
    #[error("Cannot fetch the storage value: {0}")]
    CannotFetchValue(BackendError),
    #[error("Cannot iterate storage values: {0}")]
    CannotIterateValues(BackendError),
    #[error("Encountered an error iterating over storage values: {0}")]
    StreamFailure(BackendError),
    #[error("Cannot decode the storage version for a given entry: {0}")]
    CannotDecodeStorageVersion(codec::Error),
}

impl StorageError {
    fn backend_error(&self) -> Option<&BackendError> {
        match self {
            StorageError::CannotGetLatestFinalizedBlock(e)
            | StorageError::CannotFetchValue(e)
            | StorageError::CannotIterateValues(e)
            | StorageError::StreamFailure(e) => Some(e),
            _ => None,
        }
    }
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
    #[error(
        "Constant '{constant_name}' not found in pallet {pallet_name} in the live chain metadata"
    )]
    ConstantNameNotFound {
        pallet_name: String,
        constant_name: String,
    },
    #[error("Failed to decode constant: {0}")]
    CouldNotDecodeConstant(frame_decode::constants::ConstantDecodeError<u32>),
    #[error("Cannot obtain constant information from metadata: {0}")]
    ConstantInfoError(frame_decode::constants::ConstantInfoError<'static>),
}

#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StorageKeyError {
    #[error("Can't decode the storage key: {error}")]
    StorageKeyDecodeError {
        bytes: Vec<u8>,
        error: frame_decode::storage::StorageKeyDecodeError<u32>,
    },
    #[error("Can't decode the values from the storage key: {0}")]
    CannotDecodeValuesInKey(frame_decode::storage::StorageKeyValueDecodeError),
    #[error(
        "Cannot decode storage key: there were leftover bytes, indicating that the decoding failed"
    )]
    LeftoverBytes { bytes: Vec<u8> },
    #[error("Can't decode a single value from the storage key part at index {index}: {error}")]
    CannotDecodeValueInKey {
        index: usize,
        error: scale_decode::Error,
    },
}

#[derive(Debug, DeriveError)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StorageValueError {
    #[error("Cannot decode storage value: {0}")]
    CannotDecode(frame_decode::storage::StorageValueDecodeError<u32>),
    #[error(
        "Cannot decode storage value: there were leftover bytes, indicating that the decoding failed"
    )]
    LeftoverBytes { bytes: Vec<u8> },
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
#[error("Cannot decode extrinsic at index {extrinsic_index}: {error}")]
pub struct ExtrinsicDecodeErrorAt {
    pub extrinsic_index: usize,
    pub error: ExtrinsicDecodeErrorAtReason,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExtrinsicDecodeErrorAtReason {
    #[error("{0}")]
    DecodeError(frame_decode::extrinsics::ExtrinsicDecodeError),
    #[error("Leftover bytes")]
    LeftoverBytes(Vec<u8>),
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
