// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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
    Metadata,
};
use jsonrpsee::types::Error as RequestError;
use sp_core::crypto::SecretStringError;
use sp_runtime::{
    transaction_validity::TransactionValidityError,
    DispatchError,
};
use thiserror::Error;

/// Error enum.
#[derive(Debug, Error)]
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
    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeError),
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

/// Runtime error.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RuntimeError {
    /// Module error.
    #[error("Runtime module error: {0}")]
    Module(PalletError),
    /// At least one consumer is remaining so the account cannot be destroyed.
    #[error("At least one consumer is remaining so the account cannot be destroyed.")]
    ConsumerRemaining,
    /// There are no providers so the account cannot be created.
    #[error("There are no providers so the account cannot be created.")]
    NoProviders,
    /// There are too many consumers so the account cannot be created.
    #[error("There are too many consumers so the account cannot be created.")]
    TooManyConsumers,
    /// Bad origin.
    #[error("Bad origin: throw by ensure_signed, ensure_root or ensure_none.")]
    BadOrigin,
    /// Cannot lookup.
    #[error("Cannot lookup some information required to validate the transaction.")]
    CannotLookup,
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

impl RuntimeError {
    /// Converts a `DispatchError` into a subxt error.
    pub fn from_dispatch(
        metadata: &Metadata,
        error: DispatchError,
    ) -> Result<Self, Error> {
        match error {
            DispatchError::Module {
                index,
                error,
                message: _,
            } => {
                let error = metadata.error(index, error)?;
                Ok(Self::Module(PalletError {
                    pallet: error.pallet().to_string(),
                    error: error.error().to_string(),
                    description: error.description().to_vec(),
                }))
            }
            DispatchError::BadOrigin => Ok(Self::BadOrigin),
            DispatchError::CannotLookup => Ok(Self::CannotLookup),
            DispatchError::ConsumerRemaining => Ok(Self::ConsumerRemaining),
            DispatchError::NoProviders => Ok(Self::NoProviders),
            DispatchError::TooManyConsumers => Ok(Self::TooManyConsumers),
            DispatchError::Arithmetic(_math_error) => {
                Ok(Self::Other("math_error".into()))
            }
            DispatchError::Token(_token_error) => Ok(Self::Other("token error".into())),
            DispatchError::Other(msg) => Ok(Self::Other(msg.to_string())),
        }
    }
}

/// Module error.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{error} from {pallet}")]
pub struct PalletError {
    /// The module where the error originated.
    pub pallet: String,
    /// The actual error code.
    pub error: String,
    /// The error description.
    pub description: Vec<String>,
}

/// Transaction error.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
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
