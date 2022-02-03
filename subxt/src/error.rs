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

use crate::{
    events::EventsDecodingError,
    metadata::{
        InvalidMetadataError,
        MetadataError,
    },
};
use core::fmt::Debug;
use jsonrpsee::core::error::Error as RequestError;
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
    EventsDecoding(#[from] EventsDecodingError),
    /// Transaction progress error.
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
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
