// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Args;
use color_eyre::eyre;

use std::str::FromStr;
use std::{fs, io::Read, path::PathBuf};

use subxt_codegen::utils::{MetadataVersion, Uri};

pub mod type_description;
pub mod type_example;

/// The source of the metadata.
#[derive(Debug, Args, Clone)]
pub struct FileOrUrl {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(long, value_parser)]
    pub url: Option<Uri>,
    /// The path to the encoded metadata file.
    #[clap(long, value_parser)]
    pub file: Option<PathBuf>,
    /// Specify the metadata version.
    ///
    ///  - unstable:
    ///
    ///    Use the latest unstable metadata of the node.
    ///
    ///  - number
    ///
    ///    Use this specific metadata version.
    ///
    /// Defaults to 14.
    #[clap(long)]
    pub version: Option<MetadataVersion>,
}

impl FileOrUrl {
    /// Fetch the metadata bytes.
    pub async fn fetch(&self) -> color_eyre::Result<Vec<u8>> {
        match (&self.file, &self.url, self.version) {
            // Can't provide both --file and --url
            (Some(_), Some(_), _) => {
                eyre::bail!("specify one of `--url` or `--file` but not both")
            }
            // Load from --file path
            (Some(path), None, None) => {
                let mut file = fs::File::open(path)?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                Ok(bytes)
            }
            // Cannot load the metadata from the file and specify a version to fetch.
            (Some(_), None, Some(_)) => {
                // Note: we could provide the ability to convert between metadata versions
                // but that would be involved because we'd need to convert
                // from each metadata to the latest one and from the
                // latest one to each metadata version. For now, disable the conversion.
                eyre::bail!("`--file` is incompatible with `--version`")
            }
            // Fetch from --url
            (None, Some(uri), version) => Ok(subxt_codegen::utils::fetch_metadata_bytes(
                uri,
                version.unwrap_or_default(),
            )
            .await?),
            // Default if neither is provided; fetch from local url
            (None, None, version) => {
                let uri = Uri::from_static("ws://localhost:9944");
                Ok(
                    subxt_codegen::utils::fetch_metadata_bytes(&uri, version.unwrap_or_default())
                        .await?,
                )
            }
        }
    }
}

pub fn print_first_paragraph_with_indent(docs: &[String], indent: usize) -> String {
    // take at most the first paragraph of documentation, such that it does not get too long.
    let docs_str = docs
        .iter()
        .map(|e| e.trim())
        .take_while(|e| !e.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    with_indent(docs_str, indent)
}

pub fn with_indent(s: String, indent: usize) -> String {
    let indent_str = " ".repeat(indent);
    s.lines()
        .map(|line| format!("{indent_str}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

impl FromStr for FileOrUrl {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = std::path::Path::new(s);
        if path.exists() {
            Ok(FileOrUrl {
                url: None,
                file: Some(PathBuf::from(s)),
                version: None,
            })
        } else {
            Uri::from_str(s)
                .map_err(|_| "no path or uri could be crated")
                .map(|uri| FileOrUrl {
                    url: Some(uri),
                    file: None,
                    version: None,
                })
        }
    }
}
