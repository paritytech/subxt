// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the errors that can be returned.

mod dispatch_error;

use subxt_core::error::{BlockError as CoreBlockError, Error as CoreError};

crate::macros::cfg_unstable_light_client! {
    pub use subxt_lightclient::LightClientError;
}

// Re-export dispatch error types:
pub use dispatch_error::{
    ArithmeticError, DispatchError, ModuleError, TokenError, TransactionalError,
};

// Re-expose the errors we use from other crates here:
pub use crate::metadata::Metadata;
pub use scale_decode::Error as DecodeError;
pub use scale_encode::Error as EncodeError;
pub use subxt_core::error::{ExtrinsicParamsError, MetadataError, StorageAddressError};
pub use subxt_metadata::TryFromError as MetadataTryFromError;

/// The underlying error enum, generic over the type held by the `Runtime`
/// variant. Prefer to use the [`Error<E>`] and [`Error`] aliases over
/// using this type directly.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Io error.
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    /// Codec error.
    #[error("Scale codec error: {0}")]
    Codec(#[from] codec::Error),
    /// Rpc error.
    #[error(transparent)]
    Rpc(#[from] RpcError),
    /// Serde serialization error
    #[error("Serde json error: {0}")]
    Serialization(#[from] serde_json::error::Error),
    /// Error working with metadata.
    #[error("Metadata error: {0}")]
    Metadata(#[from] MetadataError),
    /// Error decoding metadata.
    #[error("Metadata Decoding error: {0}")]
    MetadataDecoding(#[from] MetadataTryFromError),
    /// Runtime error.
    #[error("Runtime error: {0}")]
    Runtime(#[from] DispatchError),
    /// Error decoding to a [`crate::dynamic::Value`].
    #[error("Error decoding into dynamic value: {0}")]
    Decode(#[from] DecodeError),
    /// Error encoding from a [`crate::dynamic::Value`].
    #[error("Error encoding from dynamic value: {0}")]
    Encode(#[from] EncodeError),
    /// Transaction progress error.
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
    /// Error constructing the appropriate extrinsic params.
    #[error("Extrinsic params error: {0}")]
    ExtrinsicParams(#[from] ExtrinsicParamsError),
    /// Block related error.
    #[error("Block error: {0}")]
    Block(#[from] BlockError),
    /// An error encoding a storage address.
    #[error("Error encoding storage address: {0}")]
    StorageAddress(#[from] StorageAddressError),
    /// The bytes representing an error that we were unable to decode.
    #[error("An error occurred but it could not be decoded: {0:?}")]
    Unknown(Vec<u8>),
    /// Light client error.
    #[cfg(feature = "unstable-light-client")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-light-client")))]
    #[error("An error occurred but it could not be decoded: {0}")]
    LightClient(#[from] LightClientError),
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

impl From<CoreError> for Error {
    fn from(value: CoreError) -> Self {
        match value {
            CoreError::Codec(e) => Error::Codec(e),
            CoreError::Metadata(e) => Error::Metadata(e),
            CoreError::StorageAddress(e) => Error::StorageAddress(e),
            CoreError::Decode(e) => Error::Decode(e),
            CoreError::Encode(e) => Error::Encode(e),
            CoreError::ExtrinsicParams(e) => Error::ExtrinsicParams(e),
            CoreError::Block(e) => Error::Block(e.into()),
        }
    }
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

impl From<scale_decode::visitor::DecodeError> for Error {
    fn from(value: scale_decode::visitor::DecodeError) -> Self {
        Error::Decode(value.into())
    }
}

impl Error {
    /// Checks whether the error was caused by a RPC re-connection.
    pub fn is_disconnected_will_reconnect(&self) -> bool {
        matches!(self, Error::Rpc(RpcError::DisconnectedWillReconnect(_)))
    }

    /// Checks whether the error was caused by a RPC request being rejected.
    pub fn is_rejected(&self) -> bool {
        matches!(self, Error::Rpc(RpcError::RequestRejected(_)))
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
    ClientError(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// This error signals that the request was rejected for some reason.
    /// The specific reason is provided.
    #[error("RPC error: request rejected: {0}")]
    RequestRejected(String),
    /// The RPC subscription dropped.
    #[error("RPC error: subscription dropped.")]
    SubscriptionDropped,
    /// The requested URL is insecure.
    #[error("RPC error: insecure URL: {0}")]
    InsecureUrl(String),
    /// The connection was lost and automatically reconnected.
    #[error("RPC error: the connection was lost `{0}`; reconnect automatically initiated")]
    DisconnectedWillReconnect(String),
}

impl RpcError {
    /// Create a `RequestRejected` error from anything that can be turned into a string.
    pub fn request_rejected<S: Into<String>>(s: S) -> RpcError {
        RpcError::RequestRejected(s.into())
    }
}

/// Block error
#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BlockError {
    /// An error containing the hash of the block that was not found.
    #[error("Could not find a block with hash {0} (perhaps it was on a non-finalized fork?)")]
    NotFound(String),
    /// Leftover bytes found after decoding the extrinsic.
    #[error("After decoding the exntrinsic at index {extrinsic_index}, {num_leftover_bytes} bytes were left, suggesting that decoding may have failed")]
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
        error: subxt_core::error::ExtrinsicDecodeError,
    },
}

impl From<CoreBlockError> for BlockError {
    fn from(value: CoreBlockError) -> Self {
        match value {
            CoreBlockError::LeftoverBytes {
                extrinsic_index,
                num_leftover_bytes,
            } => BlockError::LeftoverBytes {
                extrinsic_index,
                num_leftover_bytes,
            },
            CoreBlockError::ExtrinsicDecodeError {
                extrinsic_index,
                error,
            } => BlockError::ExtrinsicDecodeError {
                extrinsic_index,
                error,
            },
        }
    }
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
    #[error("The block containing the transaction can no longer be found (perhaps it was on a non-finalized fork?)")]
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
