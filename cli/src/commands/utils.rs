// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::{fs, io::Read, path::PathBuf};

use color_eyre::eyre;
use subxt_codegen::utils::Uri;

// The source of the metadata.
pub enum MetadataSource {
    // Metadata is loaded from a file.
    FilePath(PathBuf),
    // Metadata is downloaded from a runtime node.
    Uri(Uri),
}

impl MetadataSource {
    /// Constructs a new [`MetadataSource`].
    pub fn new(url: Option<Uri>, file: Option<PathBuf>) -> color_eyre::Result<Self> {
        if let Some(file) = file {
            if url.is_some() {
                eyre::bail!("specify one of `--url` or `--file` but not both")
            };

            return Ok(Self::FilePath(file));
        }
        let url = url.unwrap_or_else(|| {
            "http://localhost:9933"
                .parse::<Uri>()
                .expect("default url is valid")
        });
        Ok(Self::Uri(url))
    }

    /// Fetch the metadata bytes.
    pub async fn fetch(&self) -> color_eyre::Result<Vec<u8>> {
        match &self {
            Self::FilePath(path) => {
                let mut file = fs::File::open(path)?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                Ok(bytes)
            }
            Self::Uri(url) => Ok(subxt_codegen::utils::fetch_metadata_bytes(url).await?),
        }
    }
}
