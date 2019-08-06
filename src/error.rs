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

use jsonrpc_core_client::RpcError;
use std::io::Error as IoError;
use substrate_primitives::crypto::SecretStringError;

#[derive(Debug, derive_more::From)]
pub enum Error {
    Io(IoError),
    Rpc(RpcError),
    SecretString(SecretStringError),
    Other(String),
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Other(error.into())
    }
}
