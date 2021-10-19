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

use argh::FromArgs;
use color_eyre::eyre::{self, WrapErr};
use frame_metadata::{
    RuntimeMetadata, RuntimeMetadataPrefixed,
};
use std::io::{self, Write};

#[derive(FromArgs, PartialEq, Debug)]
/// Utilities for working with substrate metadata for subxt.
struct SubXt {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    Metadata(MetadataCommand),
    Codegen(CodegenCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "metadata")]
/// Download metadata from a substrate node, for use with `subxt` codegen.
struct MetadataCommand {
    /// the url of the substrate node to query for metadata
    #[argh(option, default = "String::from(\"http://localhost:9933\")")]
    url: String,
    /// the format of the metadata to display: `json`, `hex` or `bytes`
    #[argh(option, short = 'f', default = "\"json\".to_string()")]
    format: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "codegen")]
/// Invoke subxt codegen from metadata.
struct CodegenCommand {
    // todo: from file or download directly from node
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: SubXt = argh::from_env();

    match args.command {
        Command::Metadata(args) => {
            let json = fetch_metadata(&args.url)?;
            let hex_data = json["result"]
                .as_str()
                .ok_or(eyre::eyre!("metadata result field should be a string"))?;
            let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;

            match args.format.as_str() {
                "json" => {
                    let metadata = scale::Decode::decode(&mut &bytes[..])?;
                    let json = serde_json::to_string_pretty(&metadata)?;
                    println!("{}", json);
                    Ok(())
                }
                "hex" => {
                    println!("{}", hex_data);
                    Ok(())
                }
                "bytes" => Ok(io::stdout().write_all(&bytes)?),
                _ => Err(eyre::eyre!(
                        "Unsupported format `{}`, expected `json`, `hex` or `bytes`",
                        args.format
                    )),
            }
        },
        Command::Codegen(_codegen) => todo!()
    }
}

fn fetch_metadata(url: &str) -> color_eyre::Result<serde_json::Value> {
    let resp = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "state_getMetadata",
            "id": 1
        }))
        .context("error fetching metadata from the substrate node")?;

    Ok(resp.into_json()?)
}
