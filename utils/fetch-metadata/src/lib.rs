// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Subxt utils fetch metadata.

mod error;

#[cfg(feature = "url")]
mod url;
#[cfg(feature = "url")]
pub use url::{MetadataVersion, Url, from_url, from_url_blocking};

pub use error::Error;

/// Fetch metadata from a file in a blocking manner.
pub fn from_file_blocking(path: &std::path::Path) -> Result<Vec<u8>, error::Error> {
    use std::io::Read;

    let to_err = |err| error::Error::Io(path.to_string_lossy().into(), err);
    let mut file = std::fs::File::open(path).map_err(to_err)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(to_err)?;
    Ok(bytes)
}
