// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Args;
use std::{fs, io::Read, path::PathBuf};

use color_eyre::eyre;
use subxt_codegen::utils::Uri;

/// The source of the metadata.
#[derive(Debug, Args)]
pub struct FileOrUrl {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(name = "url", long, value_parser)]
    url: Option<Uri>,
    /// The path to the encoded metadata file.
    #[clap(short, long, value_parser)]
    file: Option<PathBuf>,
}

impl FileOrUrl {
    /// Fetch the metadata bytes.
    pub async fn fetch(&self) -> color_eyre::Result<Vec<u8>> {
        if let Some(path) = &self.file {
            if self.url.is_some() {
                eyre::bail!("specify one of `--url` or `--file` but not both")
            };

            let mut file = fs::File::open(path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            return Ok(bytes);
        }

        let url = self.url.clone().unwrap_or_else(|| {
            "http://localhost:9933"
                .parse::<Uri>()
                .expect("default url is valid")
        });
        Ok(subxt_codegen::utils::fetch_metadata_bytes(&url).await?)
    }
}
