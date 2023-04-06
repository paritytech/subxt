// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the errors that can be returned.

mod dispatch_error;

use core::fmt::Debug;

// Re-export dispatch error types:
pub use dispatch_error::{
    ArithmeticError, DispatchError, ModuleError, RawModuleError, TokenError, TransactionalError,
};

// Re-expose the errors we use from other crates here:
pub use crate::metadata::{InvalidMetadataError, MetadataError};
pub use scale_decode::Error as DecodeError;
pub use scale_encode::Error as EncodeError;

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
    #[error("Rpc error: {0}")]
    Rpc(#[from] RpcError),
    /// Serde serialization error
    #[error("Serde json error: {0}")]
    Serialization(#[from] serde_json::error::Error),
    /// Invalid metadata error
    #[error("Invalid Metadata: {0}")]
    InvalidMetadata(#[from] InvalidMetadataError),
    /// Invalid metadata error
    #[error("Metadata: {0}")]
    Metadata(#[from] MetadataError),
    /// Runtime error.
    #[error("Runtime error: {0:?}")]
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
    /// Block related error.
    #[error("Block error: {0}")]
    Block(#[from] BlockError),
    /// An error encoding a storage address.
    #[error("Error encoding storage address: {0}")]
    StorageAddress(#[from] StorageAddressError),
    /// The bytes representing an error that we were unable to decode.
    #[error("An error occurred but it could not be decoded: {0:?}")]
    Unknown(Vec<u8>),
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
    /// The RPC subscription dropped.
    #[error("RPC error: subscription dropped.")]
    SubscriptionDropped,
}

/// Block error
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum BlockError {
    /// An error containing the hash of the block that was not found.
    #[error("Could not find a block with hash {0} (perhaps it was on a non-finalized fork?)")]
    NotFound(String),
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
    /// The finality subscription expired (after ~512 blocks we give up if the
    /// block hasn't yet been finalized).
    #[error("The finality subscription expired")]
    FinalityTimeout,
    /// The block hash that the transaction was added to could not be found.
    /// This is probably because the block was retracted before being finalized.
    #[error("The block containing the transaction can no longer be found (perhaps it was on a non-finalized fork?)")]
    BlockNotFound,
    /// The transaction was deemed invalid in the current chain state.
    #[error("The transaction is no longer valid")]
    Invalid,
    /// The transaction was replaced by a transaction with the same (sender, nonce) pair but with higher priority
    #[error("The transaction was replaced by a transaction with the same (sender, nonce) pair but with higher priority.")]
    Usurped,
    /// The transaction was dropped because of some limit
    #[error("The transaction was dropped from the pool because of a limit.")]
    Dropped,
}

/// Something went wrong trying to encode a storage address.
#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StorageAddressError {
    /// Storage map type must be a composite type.
    #[error("Storage map type must be a composite type")]
    MapTypeMustBeTuple,
    /// Storage lookup does not have the expected number of keys.
    #[error("Storage lookup requires {expected} keys but got {actual} keys")]
    WrongNumberOfKeys {
        /// The actual number of keys needed, based on the metadata.
        actual: usize,
        /// The number of keys provided in the storage address.
        expected: usize,
    },
    /// Storage lookup requires a type that wasn't found in the metadata.
    #[error("Storage lookup requires type {0} to exist in the metadata, but it was not found")]
    TypeNotFound(u32),
    /// This storage entry in the metadata does not have the correct number of hashers to fields.
    #[error("Storage entry in metadata does not have the correct number of hashers to fields")]
    WrongNumberOfHashers {
        /// The number of hashers in the metadata for this storage entry.
        hashers: usize,
        /// The number of fields in the metadata for this storage entry.
        fields: usize,
    },
}
