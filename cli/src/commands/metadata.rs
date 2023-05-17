// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::utils::FileOrUrl;
use clap::Parser as ClapParser;
use codec::{Decode, Encode};
use color_eyre::eyre;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use std::io::{self, Write};
use subxt_metadata::{metadata_v14_to_latest, retain_metadata};

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
}

pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    let bytes = opts.file_or_url.fetch().await?;
    let mut metadata = <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;

    if opts.pallets.is_some() || opts.runtime_apis.is_some() {
        let mut metadata_v15 = match metadata.1 {
            RuntimeMetadata::V14(metadata_v14) => metadata_v14_to_latest(metadata_v14),
            RuntimeMetadata::V15(metadata_v15) => metadata_v15,
            _ => {
                return Err(eyre::eyre!(
                    "Unsupported metadata version {:?}, expected V14.",
                    metadata.1
                ));
            }
        };
        match (opts.pallets.as_ref(), opts.runtime_apis.as_ref()) {
            (Some(pallets), Some(runtime_apis)) => retain_metadata(
                &mut metadata_v15,
                |pallet_name| pallets.iter().any(|p| &**p == pallet_name),
                |runtime_api_name| runtime_apis.iter().any(|p| &**p == runtime_api_name),
            ),
            (Some(pallets), None) => retain_metadata(
                &mut metadata_v15,
                |pallet_name| pallets.iter().any(|p| &**p == pallet_name),
                |_| true,
            ),
            (None, Some(runtime_apis)) => retain_metadata(
                &mut metadata_v15,
                |_| true,
                |runtime_api_name| runtime_apis.iter().any(|p| &**p == runtime_api_name),
            ),
            (None, None) => {}
        }
        metadata = metadata_v15.into();
    }

    match opts.format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&metadata)?;
            println!("{json}");
            Ok(())
        }
        "hex" => {
            let hex_data = format!("0x{:?}", hex::encode(metadata.encode()));
            println!("{hex_data}");
            Ok(())
        }
        "bytes" => {
            let bytes = metadata.encode();
            Ok(io::stdout().write_all(&bytes)?)
        }
        _ => Err(eyre::eyre!(
            "Unsupported format `{}`, expected `json`, `hex` or `bytes`",
            opts.format
        )),
    }
}
