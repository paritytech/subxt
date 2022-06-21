// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::metadata::{
    InvalidMetadataError,
    MetadataError,
};
use core::fmt::Debug;
use jsonrpsee::core::error::Error as RequestError;
use scale_value::scale::DecodeError;
use sp_core::crypto::SecretStringError;
use sp_runtime::transaction_validity::TransactionValidityError;

/// An error that may contain some runtime error `E`
pub type Error<E> = GenericError<RuntimeError<E>>;

/// An error that will never contain a runtime error.
pub type BasicError = GenericError<std::convert::Infallible>;

/// The underlying error enum, generic over the type held by the `Runtime`
/// variant. Prefer to use the [`Error<E>`] and [`BasicError`] aliases over
/// using this type directly.
#[derive(Debug, thiserror::Error)]
pub enum GenericError<E> {
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
    Runtime(E),
    /// Events decoding error.
    #[error("Events decoding error: {0}")]
    EventsDecoding(#[from] DecodeError),
    /// Transaction progress error.
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
    #[error("Module error: {0}")]
    /// An error from the `Module` variant of the generated `DispatchError`.
    Module(ModuleError),
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

impl<E> GenericError<E> {
    /// [`GenericError`] is parameterised over the type that it holds in the `Runtime`
    /// variant. This function allows us to map the Runtime error contained within (if present)
    /// to a different type.
    pub fn map_runtime_err<F, NewE>(self, f: F) -> GenericError<NewE>
    where
        F: FnOnce(E) -> NewE,
    {
        match self {
            GenericError::Io(e) => GenericError::Io(e),
            GenericError::Codec(e) => GenericError::Codec(e),
            GenericError::Rpc(e) => GenericError::Rpc(e),
            GenericError::Serialization(e) => GenericError::Serialization(e),
            GenericError::SecretString(e) => GenericError::SecretString(e),
            GenericError::Invalid(e) => GenericError::Invalid(e),
            GenericError::InvalidMetadata(e) => GenericError::InvalidMetadata(e),
            GenericError::Metadata(e) => GenericError::Metadata(e),
            GenericError::EventsDecoding(e) => GenericError::EventsDecoding(e),
            GenericError::Transaction(e) => GenericError::Transaction(e),
            GenericError::Module(e) => GenericError::Module(e),
            GenericError::Other(e) => GenericError::Other(e),
            // This is the only branch we really care about:
            GenericError::Runtime(e) => GenericError::Runtime(f(e)),
        }
    }
}

impl BasicError {
    /// Convert an [`BasicError`] into any
    /// arbitrary [`Error<E>`].
    pub fn into_error<E>(self) -> Error<E> {
        self.map_runtime_err(|e| match e {})
    }
}

impl<E> From<BasicError> for Error<E> {
    fn from(err: BasicError) -> Self {
        err.into_error()
    }
}

impl<E> From<SecretStringError> for GenericError<E> {
    fn from(error: SecretStringError) -> Self {
        GenericError::SecretString(error)
    }
}

impl<E> From<TransactionValidityError> for GenericError<E> {
    fn from(error: TransactionValidityError) -> Self {
        GenericError::Invalid(error)
    }
}

impl<E> From<&str> for GenericError<E> {
    fn from(error: &str) -> Self {
        GenericError::Other(error.into())
    }
}

impl<E> From<String> for GenericError<E> {
    fn from(error: String) -> Self {
        GenericError::Other(error)
    }
}

/// This is used in the place of the `E` in [`GenericError<E>`] when we may have a
/// Runtime Error. We use this wrapper so that it is possible to implement
/// `From<Error<Infallible>` for `Error<RuntimeError<E>>`.
///
/// This should not be used as a type; prefer to use the alias [`Error<E>`] when referring
/// to errors which may contain some Runtime error `E`.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeError<E>(pub E);

impl<E> RuntimeError<E> {
    /// Extract the actual runtime error from this struct.
    pub fn inner(self) -> E {
        self.0
    }
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

/// This trait is automatically implemented for the generated `DispatchError`,
/// so that we can pluck out information about the `Module` error variant, if`
/// it exists.
pub trait HasModuleError {
    /// If the error has a `Module` variant, return a tuple of the
    /// pallet index and error index. Else, return `None`.
    fn module_error_data(&self) -> Option<ModuleErrorData>;
}
