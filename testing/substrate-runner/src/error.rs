// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    CouldNotExtractPort(String),
    CouldNotExtractP2pAddress(String),
    CouldNotExtractP2pPort(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {err}"),
            Error::CouldNotExtractPort(log) => write!(
                f,
                "could not extract port from running substrate node's stdout: {log}"
            ),
            Error::CouldNotExtractP2pAddress(log) => write!(
                f,
                "could not extract p2p address from running substrate node's stdout: {log}"
            ),
            Error::CouldNotExtractP2pPort(log) => write!(
                f,
                "could not extract p2p port from running substrate node's stdout: {log}"
            ),
        }
    }
}

impl std::error::Error for Error {}
