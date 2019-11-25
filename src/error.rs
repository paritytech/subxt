// Copyright 2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    events::EventsError,
    metadata::MetadataError,
};
use jsonrpc_core_client::RpcError;
use parity_scale_codec::Error as CodecError;
use runtime_primitives::transaction_validity::TransactionValidityError;
use std::io::Error as IoError;
use substrate_primitives::crypto::SecretStringError;

/// Error enum.
#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum Error {
    /// Codec error.
    Codec(CodecError),
    /// Events error.
    Events(EventsError),
    /// Io error.
    Io(IoError),
    /// Rpc error.
    Rpc(RpcError),
    /// Secret string error.
    #[display(fmt = "Secret String Error")]
    SecretString(SecretStringError),
    /// Metadata error.
    Metadata(MetadataError),
    /// Extrinsic validity error
    #[display(fmt = "Transaction Validity Error: {:?}", _0)]
    Invalid(TransactionValidityError),
    /// Other error.
    Other(String),
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Other(error.into())
    }
}
