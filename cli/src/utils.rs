// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Args;
use color_eyre::eyre;
use std::{fs, io::Read, path::PathBuf};
use subxt_codegen::utils::{MetadataVersion as CodegenMetadataVersion, Uri};

/// The source of the metadata.
#[derive(Debug, Args)]
pub struct FileOrUrl {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(long, value_parser)]
    url: Option<Uri>,
    /// The path to the encoded metadata file.
    #[clap(long, value_parser)]
    file: Option<PathBuf>,
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
    version: Option<MetadataVersion>,
}

impl FileOrUrl {
    /// Fetch the metadata bytes.
    pub async fn fetch(&self) -> color_eyre::Result<Vec<u8>> {
        match (&self.file, &self.url, &self.version) {
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
                version.map(Into::into).unwrap_or_default(),
            )
            .await?),
            // Default if neither is provided; fetch from local url
            (None, None, version) => {
                let uri = Uri::from_static("http://localhost:9933");
                Ok(subxt_codegen::utils::fetch_metadata_bytes(
                    &uri,
                    version.map(Into::into).unwrap_or_default(),
                )
                .await?)
            }
        }
    }
}

/// The metadata version to fetch from the node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetadataVersion {
    version: u32,
}

impl std::str::FromStr for MetadataVersion {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        const SUPPORTED_VERSIONS: &[u32] = &[14];

        match input {
            "unstable" => Ok(MetadataVersion { version: u32::MAX }),
            version => {
                let num: u32 = version
                    .parse()
                    .map_err(|_| format!("Invalid metadata version specified {:?}. Subxt supports the following versions {:?}", version, SUPPORTED_VERSIONS))?;

                if !SUPPORTED_VERSIONS.iter().any(|&value| value == num) {
                    return Err(format!("Invalid metadata version specified {:?}. Subxt supports the following versions {:?}", version, SUPPORTED_VERSIONS));
                }

                Ok(MetadataVersion { version: num })
            }
        }
    }
}

impl From<MetadataVersion> for CodegenMetadataVersion {
    fn from(input: MetadataVersion) -> CodegenMetadataVersion {
        match input.version {
            14 => CodegenMetadataVersion::V14,
            u32::MAX => CodegenMetadataVersion::Unstable,
            _ => panic!("MetadataVersion and CodegenMetadataVersion are not in sync!"),
        }
    }
}
