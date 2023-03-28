// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use color_eyre::eyre;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use jsonrpsee::client_transport::ws::Uri;
use scale::{Decode, Encode};
use std::{
    io::{self, Write},
    path::PathBuf,
};
use subxt_metadata::retain_metadata_pallets;

use super::utils::MetadataSource;

/// Download metadata from a substrate node, for use with `subxt` codegen.
#[derive(Debug, ClapParser)]
pub struct Opts {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(name = "url", long, value_parser)]
    url: Option<Uri>,
    /// The path to the encoded metadata file.
    #[clap(short, long, value_parser)]
    file: Option<PathBuf>,
    /// The format of the metadata to display: `json`, `hex` or `bytes`.
    #[clap(long, short, default_value = "bytes")]
    format: String,
    /// Generate a subset of the metadata that contains only the
    /// types needed to represent the provided pallets.
    #[clap(long, use_value_delimiter = true, value_parser)]
    pallets: Option<Vec<String>>,
}

pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    let source = MetadataSource::new(opts.url, opts.file)?;
    let bytes = source.fetch().await?;
    let mut metadata = <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;

    if let Some(pallets) = opts.pallets {
        let metadata_v14 = match &mut metadata.1 {
            RuntimeMetadata::V14(metadata_v14) => metadata_v14,
            _ => {
                return Err(eyre::eyre!(
                    "Unsupported metadata version {:?}, expected V14.",
                    metadata.1
                ))
            }
        };

        retain_metadata_pallets(metadata_v14, |pallet| pallets.contains(&pallet.name));
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
