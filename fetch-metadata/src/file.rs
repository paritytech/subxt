//! Fetch metadata from a file.

use crate::Error;
use std::io::Read;

/// Fetch metadata from a file in a blocking manner.
pub fn get_blocking(path: &std::path::Path) -> Result<Vec<u8>, Error> {
    let to_err = |err| Error::Io(path.to_string_lossy().into(), err);
    let mut file = std::fs::File::open(path).map_err(to_err)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(to_err)?;
    Ok(bytes)
}
