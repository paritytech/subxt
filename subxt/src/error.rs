// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the errors that can be returned.

use core::fmt::Debug;

// Re-expose the errors we use from other crates here:
pub use crate::metadata::{
    InvalidMetadataError,
    MetadataError,
};
pub use jsonrpsee::core::error::Error as RequestError;
pub use scale_value::scale::DecodeError;
pub use sp_core::crypto::SecretStringError;
pub use sp_runtime::transaction_validity::TransactionValidityError;

/// The underlying error enum, generic over the type held by the `Runtime`
/// variant. Prefer to use the [`Error<E>`] and [`Error`] aliases over
/// using this type directly.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Io error.
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    /// Codec error.
    #[error("Scale codec error: {0}")]
    Codec(#[from] codec::Error),
    /// Rpc error.
    #[error("Rpc error: {0}")]
    Rpc(#[from] RequestError),
    /// Serde serialization error
    #[error("Serde json error: {0}")]
    Serialization(#[from] serde_json::error::Error),
    /// Secret string error.
    #[error("Secret String Error")]
    SecretString(SecretStringError),
    /// Extrinsic validity error
    #[error("Transaction Validity Error: {0:?}")]
    Invalid(TransactionValidityError),
    /// Invalid metadata error
    #[error("Invalid Metadata: {0}")]
    InvalidMetadata(#[from] InvalidMetadataError),
    /// Invalid metadata error
    #[error("Metadata: {0}")]
    Metadata(#[from] MetadataError),
    /// Runtime error.
    #[error("Runtime error: {0:?}")]
    Runtime(DispatchError),
    /// Events decoding error.
    #[error("Events decoding error: {0}")]
    EventsDecoding(#[from] DecodeError),
    /// Transaction progress error.
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

impl From<SecretStringError> for Error {
    fn from(error: SecretStringError) -> Self {
        Error::SecretString(error)
    }
}

impl From<TransactionValidityError> for Error {
    fn from(error: TransactionValidityError) -> Self {
        Error::Invalid(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Other(error.into())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Other(error)
    }
}

impl From<DispatchError> for Error {
    fn from(error: DispatchError) -> Self {
        Error::Runtime(error)
    }
}

/// This is our attempt to decode a runtime DispatchError. We either
/// successfully decode it into a [`ModuleError`], or we fail and keep
/// hold of the bytes, which we can attempt to decode if we have an
/// appropriate static type to hand.
#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    /// An error was emitted from a specific pallet/module.
    #[error("Module error: {0}")]
    Module(ModuleError),
    /// Some other error was emitted.
    #[error("Undecoded dispatch error: {0:?}")]
    Other(Vec<u8>),
}

/// Transaction error.
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
pub enum TransactionError {
    /// The finality subscription expired (after ~512 blocks we give up if the
    /// block hasn't yet been finalized).
    #[error("The finality subscription expired")]
    FinalitySubscriptionTimeout,
    /// The block hash that the tranaction was added to could not be found.
    /// This is probably because the block was retracted before being finalized.
    #[error("The block containing the transaction can no longer be found (perhaps it was on a non-finalized fork?)")]
    BlockHashNotFound,
}

/// Details about a module error that has occurred.
#[derive(Clone, Debug, thiserror::Error)]
#[error("{pallet}: {error}\n\n{}", .description.join("\n"))]
pub struct ModuleError {
    /// The name of the pallet that the error came from.
    pub pallet: String,
    /// The name of the error.
    pub error: String,
    /// A description of the error.
    pub description: Vec<String>,
    /// A byte representation of the error.
    pub error_data: ModuleErrorData,
}

/// The error details about a module error that has occurred.
///
/// **Note**: Structure used to obtain the underlying bytes of a ModuleError.
#[derive(Clone, Debug, thiserror::Error)]
#[error("Pallet index {pallet_index}: raw error: {error:?}")]
pub struct ModuleErrorData {
    /// Index of the pallet that the error came from.
    pub pallet_index: u8,
    /// Raw error bytes.
    pub error: [u8; 4],
}

impl ModuleErrorData {
    /// Obtain the error index from the underlying byte data.
    pub fn error_index(&self) -> u8 {
        // Error index is utilized as the first byte from the error array.
        self.error[0]
    }
}
