// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/// Error attempting to fetch metadata.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Error decoding from a hex value.
    #[error("Cannot decode hex value: {0}")]
    DecodeError(#[from] hex::FromHexError),
    /// Some SCALE codec error.
    #[error("Cannot scale encode/decode value: {0}")]
    CodecError(#[from] codec::Error),
    /// JSON-RPC error fetching metadata.
    #[cfg(feature = "url")]
    #[error("Request error: {0}")]
    RequestError(#[from] jsonrpsee::core::ClientError),
    /// Failed IO when fetching from a file.
    #[error("Failed IO for {0}, make sure that you are providing the correct file path for metadata: {1}")]
    Io(String, std::io::Error),
    /// URL scheme is not http, https, ws or wss.
    #[error("'{0}' not supported, supported URI schemes are http, https, ws or wss.")]
    InvalidScheme(String),
    /// Some other error.
    #[error("Other error: {0}")]
    Other(String),
}
