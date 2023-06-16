// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::utils::FileOrUrl;
use clap::Parser as ClapParser;
use codec::{Decode, Encode};
use color_eyre::eyre::{self, bail};
use frame_metadata::{v15::RuntimeMetadataV15, RuntimeMetadata, RuntimeMetadataPrefixed};
use std::{io::Write, path::PathBuf};
use subxt_metadata::Metadata;

/// Download metadata from a substrate node, for use with `subxt` codegen.
#[derive(Debug, ClapParser)]
pub struct Opts {
    #[command(flatten)]
    file_or_url: FileOrUrl,
    /// The format of the metadata to display: `json`, `hex` or `bytes`.
    #[clap(long, short, default_value = "bytes")]
    format: String,
    /// Generate a subset of the metadata that contains only the
    /// types needed to represent the provided pallets.
    ///
    /// The returned metadata is updated to the latest available version
    /// when using the option.
    #[clap(long, use_value_delimiter = true, value_parser)]
    pallets: Option<Vec<String>>,
    /// Generate a subset of the metadata that contains only the
    /// runtime APIs needed.
    ///
    /// The returned metadata is updated to the latest available version
    /// when using the option.
    #[clap(long, use_value_delimiter = true, value_parser)]
    runtime_apis: Option<Vec<String>>,
    /// Write the output of the metadata command to the provided file path.
    #[clap(long, short, value_parser)]
    pub output_file: Option<PathBuf>,
}

pub async fn run(opts: Opts, output: &mut impl Write) -> color_eyre::Result<()> {
    let bytes = opts.file_or_url.fetch().await?;
    let mut metadata = RuntimeMetadataPrefixed::decode(&mut &bytes[..])?;

    let version = match &metadata.1 {
        RuntimeMetadata::V14(_) => Version::V14,
        RuntimeMetadata::V15(_) => Version::V15,
        _ => Version::Unknown,
    };

    if opts.pallets.is_some() || opts.runtime_apis.is_some() {
        // convert to internal type:
        let mut md = Metadata::try_from(metadata)?;

        // retain pallets and/or runtime APIs given:
        let retain_pallets_fn: Box<dyn Fn(&str) -> bool> = match opts.pallets.as_ref() {
            Some(pallets) => Box::new(|name| pallets.iter().any(|p| &**p == name)),
            None => Box::new(|_| true),
        };
        let retain_runtime_apis_fn: Box<dyn Fn(&str) -> bool> = match opts.runtime_apis.as_ref() {
            Some(apis) => Box::new(|name| apis.iter().any(|p| &**p == name)),
            None => Box::new(|_| true),
        };
        md.retain(retain_pallets_fn, retain_runtime_apis_fn);

        // Convert back to wire format, preserving version:
        metadata = match version {
            Version::V14 => RuntimeMetadataV15::from(md).into(),
            Version::V15 => RuntimeMetadataV15::from(md).into(),
            Version::Unknown => {
                bail!("Unsupported metadata version; V14 or V15 metadata is expected.")
            }
        }
    }

    let mut output: Box<dyn Write> = match opts.output_file {
        Some(path) => Box::new(std::fs::File::create(path)?),
        None => Box::new(output),
    };

    match opts.format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&metadata)?;
            write!(output, "{json}")?;
            Ok(())
        }
        "hex" => {
            let hex_data = format!("0x{}", hex::encode(metadata.encode()));
            write!(output, "{hex_data}")?;
            Ok(())
        }
        "bytes" => {
            let bytes = metadata.encode();
            output.write_all(&bytes)?;
            Ok(())
        }
        _ => Err(eyre::eyre!(
            "Unsupported format `{}`, expected `json`, `hex` or `bytes`",
            opts.format
        )),
    }
}

enum Version {
    V14,
    V15,
    Unknown,
}
