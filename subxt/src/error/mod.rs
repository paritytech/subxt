// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the errors that can be returned.

mod dispatch_error;
mod hex;

crate::macros::cfg_unstable_light_client! {
    pub use subxt_lightclient::LightClientError;
}

// Re-export dispatch error types:
pub use dispatch_error::{
    ArithmeticError, DispatchError, ModuleError, TokenError, TransactionalError,
};

// Re-expose the errors we use from other crates here:
pub use crate::Metadata;
pub use hex::Hex;
pub use scale_decode::Error as DecodeError;
pub use scale_encode::Error as EncodeError;
pub use subxt_metadata::TryFromError as MetadataTryFromError;

// Re-export core error types we're just reusing.
pub use subxt_core::error::{
    ConstantError,
    CustomValueError,
    EventsError as CoreEventsError,
    // These errors are exposed as-is:
    ExtrinsicDecodeErrorAt,
    // These errors are wrapped:
    ExtrinsicError as CoreExtrinsicError,
    RuntimeApiError as CoreRuntimeApiError,
    StorageError as CoreStorageError,
    StorageKeyError,
    StorageValueError,
    ViewFunctionError as CoreViewFunctionError,
};

/// A global error type. Any of the errors exposed here can convert into this
/// error via `.into()`, but this error isn't itself exposed from anything.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Error {
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
    OnlineClientError(#[from] OnlineClientError),
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
}

impl From<std::convert::Infallible> for Error {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}

impl Error {
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
            Error::ExtrinsicDecodeErrorAt(_)
            | Error::ConstantError(_)
            | Error::CustomValueError(_)
            | Error::StorageKeyError(_)
            | Error::StorageValueError(_)
            | Error::BackendError(_)
            | Error::RuntimeUpdateeApplyError(_)
            | Error::TransactionStatusError(_)
            | Error::ModuleErrorDetailsError(_)
            | Error::ModuleErrorDecodeError(_)
            | Error::DispatchErrorDecodeError(_) => None,
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
        }
    }
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
pub enum OnlineClientError {
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
    #[error("Cannot access Runtime APIs at latest block: Cannot fetch latest finalized block: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("{0}")]
    OfflineError(#[from] CoreRuntimeApiError),
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
    #[error("{0}")]
    OfflineError(#[from] CoreEventsError),
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
    #[error("{0}")]
    OfflineError(#[from] CoreExtrinsicError),
    #[error("Could not download block body to extract extrinsics from: {0}")]
    CannotGetBlockBody(BackendError),
    #[error("Block not found: {0}")]
    BlockNotFound(Hex),
    #[error("{0}")]
    CouldNotDecodeExtrinsics(#[from] ExtrinsicDecodeErrorAt),
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

/// Error working with View Functions.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum ViewFunctionError {
    #[error("{0}")]
    OfflineError(#[from] CoreViewFunctionError),
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
#[allow(missing_docs)]
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
    #[error("{0}")]
    Offline(#[from] CoreStorageError),
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
