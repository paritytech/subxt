// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use jsonrpsee::{
    client::RequestError,
    transport::ws::WsNewDnsError,
};
use sp_core::crypto::SecretStringError;
use sp_runtime::transaction_validity::TransactionValidityError;

use crate::{
    events::EventsError,
    metadata::MetadataError,
};

/// Error enum.
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
    /// Error that can happen during the initial websocket handshake
    #[error("Rpc error: {0}")]
    WsHandshake(#[from] WsNewDnsError),
    /// Serde serialization error
    #[error("Serde json error: {0}")]
    Serialization(#[from] serde_json::error::Error),
    /// Secret string error.
    #[error("Secret String Error")]
    SecretString(SecretStringError),
    /// Extrinsic validity error
    #[error("Transaction Validity Error: {0:?}")]
    Invalid(TransactionValidityError),
    /// Events error.
    #[error("Event error: {0}")]
    Events(#[from] EventsError),
    /// Metadata error.
    #[error("Metadata error: {0}")]
    Metadata(#[from] MetadataError),
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
