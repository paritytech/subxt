// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use color_eyre::eyre::{
    self,
    WrapErr,
};
use frame_metadata::RuntimeMetadataPrefixed;
use scale::{
    Decode,
    Input,
};
use std::{
    fs,
    io::{
        self,
        Read,
        Write,
    },
    path::PathBuf,
};
use structopt::StructOpt;

/// Utilities for working with substrate metadata for subxt.
#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Download metadata from a substrate node, for use with `subxt` codegen.
    #[structopt(name = "metadata")]
    Metadata {
        /// the url of the substrate node to query for metadata
        #[structopt(
            name = "url",
            long,
            parse(try_from_str),
            default_value = "http://localhost:9933"
        )]
        url: url::Url,
        /// the format of the metadata to display: `json`, `hex` or `bytes`
        #[structopt(long, short, default_value = "json")]
        format: String,
    },
    /// Generate runtime API client code from metadata.
    ///
    /// # Example (with code formatting)
    ///
    /// `subxt codegen | rustfmt --edition=2018 --emit=stdout`
    Codegen {
        /// the url of the substrate node to query for metadata for codegen.
        #[structopt(name = "url", long, parse(try_from_str))]
        url: Option<url::Url>,
        /// the path to the encoded metadata file.
        #[structopt(short, long, parse(from_os_str))]
        file: Option<PathBuf>,
    },
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Opts::from_args();

    match args.command {
        Command::Metadata { url, format } => {
            let (hex_data, bytes) = fetch_metadata(&url)?;

            match format.as_str() {
                "json" => {
                    let metadata =
                        <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;
                    let json = serde_json::to_string_pretty(&metadata)?;
                    println!("{}", json);
                    Ok(())
                }
                "hex" => {
                    println!("{}", hex_data);
                    Ok(())
                }
                "bytes" => Ok(io::stdout().write_all(&bytes)?),
                _ => {
                    Err(eyre::eyre!(
                        "Unsupported format `{}`, expected `json`, `hex` or `bytes`",
                        format
                    ))
                }
            }
        }
        Command::Codegen { url, file } => {
            if let Some(file) = file.as_ref() {
                if url.is_some() {
                    eyre::bail!("specify one of `--url` or `--file` but not both")
                };

                let mut file = fs::File::open(file)?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                codegen(&mut &bytes[..])?;
                return Ok(())
            }

            let url = url.unwrap_or_else(|| {
                url::Url::parse("http://localhost:9933").expect("default url is valid")
            });
            let (_, bytes) = fetch_metadata(&url)?;
            codegen(&mut &bytes[..])?;
            Ok(())
        }
    }
}

fn fetch_metadata(url: &url::Url) -> color_eyre::Result<(String, Vec<u8>)> {
    let resp = ureq::post(url.as_str())
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "state_getMetadata",
            "id": 1
        }))
        .context("error fetching metadata from the substrate node")?;
    let json: serde_json::Value = resp.into_json()?;

    let hex_data = json["result"]
        .as_str()
        .map(ToString::to_string)
        .ok_or_else(|| eyre::eyre!("metadata result field should be a string"))?;
    let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;

    Ok((hex_data, bytes))
}

fn codegen<I: Input>(encoded: &mut I) -> color_eyre::Result<()> {
    let metadata = <RuntimeMetadataPrefixed as Decode>::decode(encoded)?;
    let generator = subxt_codegen::RuntimeGenerator::new(metadata);
    let item_mod = syn::parse_quote!(
        pub mod api {}
    );
    let runtime_api = generator.generate_runtime(item_mod, Default::default());
    println!("{}", runtime_api);
    Ok(())
}
