// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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

#![deny(unused_crate_dependencies)]

use color_eyre::eyre::{
    self,
    WrapErr,
};
use frame_metadata::{
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
    RuntimeMetadataV14,
    META_RESERVED,
};
use scale::{
    Decode,
    Input,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::HashMap,
    fs,
    io::{
        self,
        Read,
        Write,
    },
    path::PathBuf,
};
use structopt::StructOpt;
use subxt_codegen::DerivesRegistry;
use subxt_metadata::{
    get_metadata_hash,
    get_pallet_hash,
};

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
        #[structopt(long, short, default_value = "bytes")]
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
        /// Additional derives
        #[structopt(long = "derive")]
        derives: Vec<String>,
    },
    /// Verify metadata compatibility between substrate nodes.
    Compatibility {
        /// Urls of the substrate nodes to verify for metadata compatibility.
        #[structopt(name = "nodes", long, use_delimiter = true, parse(try_from_str))]
        nodes: Vec<url::Url>,
        /// Check the compatibility of metadata for a particular pallet.
        ///
        /// ### Note
        /// The validation will omit the full metadata check and focus instead on the pallet.
        #[structopt(long, parse(try_from_str))]
        pallet: Option<String>,
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
        Command::Codegen { url, file, derives } => {
            if let Some(file) = file.as_ref() {
                if url.is_some() {
                    eyre::bail!("specify one of `--url` or `--file` but not both")
                };

                let mut file = fs::File::open(file)?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                codegen(&mut &bytes[..], derives)?;
                return Ok(())
            }

            let url = url.unwrap_or_else(|| {
                url::Url::parse("http://localhost:9933").expect("default url is valid")
            });
            let (_, bytes) = fetch_metadata(&url)?;
            codegen(&mut &bytes[..], derives)?;
            Ok(())
        }
        Command::Compatibility { nodes, pallet } => {
            match pallet {
                Some(pallet) => handle_pallet_metadata(nodes.as_slice(), pallet.as_str()),
                None => handle_full_metadata(nodes.as_slice()),
            }
        }
    }
}

fn handle_pallet_metadata(nodes: &[url::Url], name: &str) -> color_eyre::Result<()> {
    #[derive(Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct CompatibilityPallet {
        pallet_present: HashMap<String, Vec<String>>,
        pallet_not_found: Vec<String>,
    }

    let mut compatibility: CompatibilityPallet = Default::default();
    for node in nodes.iter() {
        let metadata = fetch_runtime_metadata(node)?;

        match metadata.pallets.iter().find(|pallet| pallet.name == name) {
            Some(pallet_metadata) => {
                let hash = get_pallet_hash(&metadata.types, pallet_metadata);
                let hex_hash = hex::encode(hash);
                println!(
                    "Node {:?} has pallet metadata hash {:?}",
                    node.as_str(),
                    hex_hash
                );

                compatibility
                    .pallet_present
                    .entry(hex_hash)
                    .or_insert_with(Vec::new)
                    .push(node.as_str().to_string());
            }
            None => {
                compatibility
                    .pallet_not_found
                    .push(node.as_str().to_string());
            }
        }
    }

    println!(
        "\nCompatible nodes by pallet\n{}",
        serde_json::to_string_pretty(&compatibility)
            .context("Failed to parse compatibility map")?
    );

    Ok(())
}

fn handle_full_metadata(nodes: &[url::Url]) -> color_eyre::Result<()> {
    let mut compatibility_map: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes.iter() {
        let metadata = fetch_runtime_metadata(node)?;
        let hash = get_metadata_hash(&metadata);
        let hex_hash = hex::encode(hash);
        println!("Node {:?} has metadata hash {:?}", node.as_str(), hex_hash,);

        compatibility_map
            .entry(hex_hash)
            .or_insert_with(Vec::new)
            .push(node.as_str().to_string());
    }

    println!(
        "\nCompatible nodes\n{}",
        serde_json::to_string_pretty(&compatibility_map)
            .context("Failed to parse compatibility map")?
    );

    Ok(())
}

fn fetch_runtime_metadata(url: &url::Url) -> color_eyre::Result<RuntimeMetadataV14> {
    let (_, bytes) = fetch_metadata(url)?;

    let metadata = <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;
    if metadata.0 != META_RESERVED {
        return Err(eyre::eyre!(
            "Node {:?} has invalid metadata prefix: {:?} expected prefix: {:?}",
            url.as_str(),
            metadata.0,
            META_RESERVED
        ))
    }

    match metadata.1 {
        RuntimeMetadata::V14(v14) => Ok(v14),
        _ => {
            Err(eyre::eyre!(
                "Node {:?} with unsupported metadata version: {:?}",
                url.as_str(),
                metadata.1
            ))
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

fn codegen<I: Input>(
    encoded: &mut I,
    raw_derives: Vec<String>,
) -> color_eyre::Result<()> {
    let metadata = <RuntimeMetadataPrefixed as Decode>::decode(encoded)?;
    let generator = subxt_codegen::RuntimeGenerator::new(metadata);
    let item_mod = syn::parse_quote!(
        pub mod api {}
    );

    let p = raw_derives
        .iter()
        .map(|raw| syn::parse_str(raw))
        .collect::<Result<Vec<_>, _>>()?;
    let mut derives = DerivesRegistry::default();
    derives.extend_for_all(p.into_iter());

    let runtime_api = generator.generate_runtime(item_mod, derives);
    println!("{}", runtime_api);
    Ok(())
}
