// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use color_eyre::eyre;
use frame_metadata::RuntimeMetadataPrefixed;
use scale::Decode;
use std::io::{
    self,
    Write,
};
use subxt_codegen::utils::fetch_metadata_hex;

use crate::CliOpts;

/// Download metadata from a substrate node, for use with `subxt` codegen.
#[derive(Debug, ClapParser)]
pub struct MetadataOpts {
    /// The format of the metadata to display: `json`, `hex` or `bytes`.
    #[clap(long, short, default_value = "bytes")]
    format: String,
}

pub async fn run(opts: &CliOpts, cmd_opts: &MetadataOpts) -> color_eyre::Result<()> {
    let hex_data = fetch_metadata_hex(&opts.url).await?;

    match cmd_opts.format.as_str() {
        "json" => {
            let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;
            let metadata = <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;
            let json = serde_json::to_string_pretty(&metadata)?;
            println!("{json}");
            Ok(())
        }
        "hex" => {
            println!("{hex_data}");
            Ok(())
        }
        "bytes" => {
            let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;
            Ok(io::stdout().write_all(&bytes)?)
        }
        _ => {
            Err(eyre::eyre!(
                "Unsupported format `{}`, expected `json`, `hex` or `bytes`",
                cmd_opts.format
            ))
        }
    }
}
