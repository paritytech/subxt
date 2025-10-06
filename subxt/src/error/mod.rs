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
pub use scale_decode::Error as DecodeError;
pub use scale_encode::Error as EncodeError;
pub use subxt_metadata::TryFromError as MetadataTryFromError;
pub use hex::Hex;

// Re-export core error types we're just reusing.
pub use subxt_core::error::{
    ExtrinsicDecodeErrorAt,
    ConstantError,
    CustomValueError,
    ExtrinsicError as CoreExtrinsicError,
    RuntimeApiError as CoreRuntimeApiError,
    EventsError as CoreEventsError,
    ViewFunctionError as CoreViewFunctionError,
};

// /// The underlying error enum, generic over the type held by the `Runtime`
// /// variant. Prefer to use the [`Error<E>`] and [`Error`] aliases over
// /// using this type directly.
// #[derive(Debug, thiserror::Error)]
// #[non_exhaustive]
// pub enum Error {
//     /// Io error.
//     #[error("Io error: {0}")]
//     Io(#[from] std::io::Error),
//     /// Codec error.
//     #[error("Scale codec error: {0}")]
//     Codec(#[from] codec::Error),
//     /// Rpc error.
//     #[error(transparent)]
//     Rpc(#[from] RpcError),
//     /// Serde serialization error
//     #[error("Serde json error: {0}")]
//     Serialization(#[from] serde_json::error::Error),
//     /// Error working with metadata.
//     #[error("Metadata error: {0}")]
//     Metadata(#[from] MetadataError),
//     /// Error decoding metadata.
//     #[error("Metadata Decoding error: {0}")]
//     MetadataDecoding(#[from] MetadataTryFromError),
//     /// Runtime error.
//     #[error("Runtime error: {0}")]
//     Runtime(#[from] DispatchError),
//     /// Error decoding to a [`crate::dynamic::Value`].
//     #[error("Error decoding into dynamic value: {0}")]
//     Decode(#[from] DecodeError),
//     /// Error encoding from a [`crate::dynamic::Value`].
//     #[error("Error encoding from dynamic value: {0}")]
//     Encode(#[from] EncodeError),
//     /// Transaction progress error.
//     #[error("Transaction error: {0}")]
//     Transaction(#[from] TransactionStatusError),
//     /// Error constructing the appropriate extrinsic params.
//     #[error("Extrinsic params error: {0}")]
//     Extrinsic(#[from] ExtrinsicError),
//     /// Block related error.
//     #[error("Block error: {0}")]
//     Block(#[from] BlockError),
//     /// An error encoding a storage address.
//     #[error("Error encoding storage address: {0}")]
//     StorageAddress(#[from] StorageError),
//     /// The bytes representing an error that we were unable to decode.
//     #[error("An error occurred but it could not be decoded: {0:?}")]
//     Unknown(Vec<u8>),
//     /// Light client error.
//     #[cfg(feature = "unstable-light-client")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "unstable-light-client")))]
//     #[error("An error occurred but it could not be decoded: {0}")]
//     LightClient(#[from] LightClientError),
//     /// Other error.
//     #[error("Other error: {0}")]
//     Other(String),
// }

// impl From<CoreError> for Error {
//     fn from(value: CoreError) -> Self {
//         match value {
//             CoreError::Codec(e) => Error::Codec(e),
//             CoreError::Metadata(e) => Error::Metadata(e),
//             CoreError::StorageError(e) => Error::StorageAddress(e),
//             CoreError::Decode(e) => Error::Decode(e),
//             CoreError::Encode(e) => Error::Encode(e),
//             CoreError::Extrinsic(e) => Error::Extrinsic(e),
//             CoreError::Block(e) => Error::Block(e.into()),
//         }
//     }
// }

// impl<'a> From<&'a str> for Error {
//     fn from(error: &'a str) -> Self {
//         Error::Other(error.into())
//     }
// }

// impl From<String> for Error {
//     fn from(error: String) -> Self {
//         Error::Other(error)
//     }
// }

// impl From<std::convert::Infallible> for Error {
//     fn from(value: std::convert::Infallible) -> Self {
//         match value {}
//     }
// }

// impl From<scale_decode::visitor::DecodeError> for Error {
//     fn from(value: scale_decode::visitor::DecodeError) -> Self {
//         Error::Decode(value.into())
//     }
// }

// impl From<subxt_rpcs::Error> for Error {
//     fn from(value: subxt_rpcs::Error) -> Self {
//         Error::Rpc(value.into())
//     }
// }

// impl Error {
//     /// Checks whether the error was caused by a RPC re-connection.
//     pub fn is_disconnected_will_reconnect(&self) -> bool {
//         matches!(
//             self,
//             Error::Rpc(RpcError::ClientError(
//                 subxt_rpcs::Error::DisconnectedWillReconnect(_)
//             ))
//         )
//     }

//     /// Checks whether the error was caused by a RPC request being rejected.
//     pub fn is_rpc_limit_reached(&self) -> bool {
//         matches!(self, Error::Rpc(RpcError::LimitReached))
//     }
// }

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
    Other(String)
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
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BlockError {
    /// An error containing the hash of the block that was not found.
    #[error("Could not find the block body with hash {block_hash} (perhaps it was on a non-finalized fork?)")]
    BlockNotFound {
        block_hash: Hex,
    },
    // #[error("Could not download the block body with hash {block_hash}: {reason}")]
    // CouldNotGetBlockBody {
    //     block_hash: String,
    //     reason: BackendError
    // },
    #[error("Could not download the block header with hash {block_hash}: {reason}")]
    CouldNotGetBlockHeader {
        block_hash: Hex,
        reason: BackendError
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
        reason: AccountNonceError
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AccountNonceError {
    #[error("Could not retrieve account nonce: {0}")]
    CouldNotRetrieve(#[from] BackendError),
    #[error("Could not decode account nonce: {0}")]
    CouldNotDecode(#[from] codec::Error),
    #[error("Wrong number of account nonce bytes returned: {0} (expected 2, 4 or 8)")]
    WrongNumberOfBytes(usize),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OnlineClientError {
    #[error("Cannot construct OnlineClient: {0}")]
    RpcError(#[from] subxt_rpcs::Error),
    #[error("Cannot construct OnlineClient: Cannot fetch latest finalized block to obtain init details from: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot construct OnlineClient: Cannot fetch genesis hash: {0}")]
    CannotGetGenesisHash(BackendError),
    #[error("Cannot construct OnlineClient: Cannot fetch current runtime version: {0}")]
    CannotGetCurrentRuntimeVersion(BackendError),
    #[error("Cannot construct OnlineClient: Cannot fetch metadata: {0}")]
    CannotFetchMetadata(BackendError),
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
    #[error("Error subscribing to runtime updates: Cannot find the System.LastRuntimeUpgrade storage entry")]
    CantFindSystemLastRuntimeUpgrade,
    #[error("Error subscribing to runtime updates: Cannot fetch last runtime upgrade: {0}")]
    CantFetchLastRuntimeUpgrade(),
    #[error("Error subscribing to runtime updates: Cannot decode last runtime upgrade: {0}")]
    CannotDecodeLastRuntimeUpgrade(scale_decode::Error),
}

/// Error that can occur during upgrade.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum RuntimeUpdateeApplyError {
    #[error("The proposed runtime update is the same as the current version")]
    SameVersion,
}

/// Error working with Runtime APIs
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum RuntimeApiError {
    #[error("Cannot access Runtime APIs at latest block: Cannot fetch latest finalized block: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("{0}")]
    OfflineError(#[from] CoreRuntimeApiError),
    #[error("Cannot call the Runtime API: {0}")]
    CannotCallApi(BackendError),
}

/// Error working with events.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum EventsError {
    #[error("{0}")]
    OfflineError(#[from] CoreEventsError),
    #[error("Cannot access events at latest block: Cannot fetch latest finalized block: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot fetch event bytes: {0}")]
    CannotFetchEventBytes(BackendError),
}

/// Error working with extrinsics.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ExtrinsicError {
    #[error("{0}")]
    OfflineError(#[from] CoreExtrinsicError),
    #[error("Could not download block body to extract extrinsics from: {0}")]
    CannotGetBlockBody(BackendError),
    #[error("Block not found: {0}")]
    BlockNotFound(Hex),
    #[error("{0}")]
    CouldNotDecodeExtrinsics(#[from] ExtrinsicDecodeErrorAt),
    #[error("Extrinsic submission error: Cannot get latest finalized block to grab account nonce at: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot find block header for block {block_hash}")]
    CannotFindBlockHeader {
        block_hash: Hex
    },
    #[error("Error getting account nonce at block {block_hash}")]
    AccountNonceError {
        block_hash: Hex,
        account_id: Hex,
        reason: AccountNonceError
    },
    #[error("Cannot submit extrinsic: {0}")]
    ErrorSubmittingTransaction(BackendError),
    #[error("A transaction status error was returned while submitting the extrinsic: {0}")]
    TransactionStatusError(TransactionStatusError),
    #[error("The transaction status stream encountered an error while submitting the extrinsic: {0}")]
    TransactionStatusStreamError(BackendError),
    #[error("The transaction status stream unexpectedly ended, so we don't know the status of the submitted extrinsic")]
    UnexpectedEndOfTransactionStatusStream,
    #[error("Cannot get fee info from Runtime API: {0}")]
    CannotGetFeeInfo(BackendError),
    #[error("Cannot get validation info from Runtime API: {0}")]
    CannotGetValidationInfo(BackendError),
}

/// Error working with View Functions.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ViewFunctionError {
    #[error("{0}")]
    OfflineError(#[from] CoreViewFunctionError),
    #[error("Cannot access View Functions at latest block: Cannot fetch latest finalized block: {0}")]
    CannotGetLatestFinalizedBlock(BackendError),
    #[error("Cannot call the View Function Runtime API: {0}")]
    CannotCallApi(BackendError),
}

/// Error during the transaction progress.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum TransactionProgressError {
    #[error("Cannot get the next transaction progress update: {0}")]
    CannotGetNextProgressUpdate(BackendError),
    #[error("Error during transaction progress: {0}")]
    TransactionStatusError(#[from] TransactionStatusError),
    #[error("The transaction status stream unexpectedly ended, so we have no further transaction progress updates")]
    UnexpectedEndOfTransactionStatusStream,
}

/// An error emitted as the result of a transaction progress update.
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum TransactionStatusError {
    // /// The block hash that the transaction was added to could not be found.
    // /// This is probably because the block was retracted before being finalized.
    // #[error(
    //     "The block containing the transaction can no longer be found (perhaps it was on a non-finalized fork?)"
    // )]
    // BlockNotFound,
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
pub enum TransactionEventsError {
    #[error("The block containing the submitted transaction ({block_hash}) could not be downloaded: {error}")]
    CannotFetchBlockBody {
        block_hash: Hex,
        error: BackendError,
    },
    #[error("Cannot find the the submitted transaction (hash: {transaction_hash}) in the block (hash: {block_hash}) it is supposed to be in.")]
    CannotFindTransactionInBlock {
        block_hash: Hex,
        transaction_hash: Hex,
    },
    #[error("The block containing the submitted transaction ({block_hash}) could not be found")]
    BlockNotFound {
        block_hash: Hex
    },
    #[error("Could not decode event at index {event_index} for the submitted transaction at block {block_hash}: {error}")]
    CannotDecodeEventInBlock {
        event_index: usize,
        block_hash: Hex,
        error: EventsError
    },
    #[error("Could not fetch events for the submitted transaction: {error}")]
    CannotFetchEventsForTransaction {
        block_hash: Hex,
        transaction_hash: Hex,
        error: EventsError
    },
    #[error("The transaction failed with the following dispatch error: {0}")]
    ExtrinsicFailed(#[from] DispatchError),
}

/// Error waiting for the transaction to be finalized and successful.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TransactionFinalizedSuccessError {
    #[error("Could not finalize the transaction: {0}")]
    FinalizationError(#[from] TransactionProgressError),
    #[error("The transaction did not succeed: {0}")]
    SuccessError(#[from] TransactionEventsError)
}