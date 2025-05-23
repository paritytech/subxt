// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::utils::{FileOrUrl, validate_url_security};
use clap::Parser as ClapParser;
use codec::{Decode, Encode};
use color_eyre::eyre::{self, bail};
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use std::{io::Write, path::PathBuf};
use subxt_utils_stripmetadata::StripMetadata;

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
    /// Allow insecure URLs e.g. URLs starting with ws:// or http:// without SSL encryption
    #[clap(long, short)]
    allow_insecure: bool,
}

pub async fn run(opts: Opts, output: &mut impl Write) -> color_eyre::Result<()> {
    validate_url_security(opts.file_or_url.url.as_ref(), opts.allow_insecure)?;
    let bytes = opts.file_or_url.fetch().await?;

    let mut metadata = RuntimeMetadataPrefixed::decode(&mut &bytes[..])?;

    // Strip pallets or runtime APIs if names are provided:
    if opts.pallets.is_some() || opts.runtime_apis.is_some() {
        let keep_pallets_fn: Box<dyn Fn(&str) -> bool> = match opts.pallets.as_ref() {
            Some(pallets) => Box::new(|name| pallets.iter().any(|p| &**p == name)),
            None => Box::new(|_| true),
        };
        let keep_runtime_apis_fn: Box<dyn Fn(&str) -> bool> = match opts.runtime_apis.as_ref() {
            Some(apis) => Box::new(|name| apis.iter().any(|p| &**p == name)),
            None => Box::new(|_| true),
        };

        match &mut metadata.1 {
            RuntimeMetadata::V14(md) => md.strip_metadata(keep_pallets_fn, keep_runtime_apis_fn),
            RuntimeMetadata::V15(md) => md.strip_metadata(keep_pallets_fn, keep_runtime_apis_fn),
            RuntimeMetadata::V16(md) => md.strip_metadata(keep_pallets_fn, keep_runtime_apis_fn),
            _ => {
                bail!(
                    "Unsupported metadata version for stripping pallets/runtime APIs: V14, V15 or V16 metadata is expected."
                )
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
