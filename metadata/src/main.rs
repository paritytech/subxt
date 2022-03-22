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

use crate::eyre::Context;
use codec::Decode;
use color_eyre::eyre;
use frame_metadata::{
    RuntimeMetadata,
    RuntimeMetadataLastVersion,
    RuntimeMetadataPrefixed,
    META_RESERVED,
};
use scale_info::{
    form::PortableForm,
    PortableRegistry,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;
use structopt::StructOpt;
use subxt_metadata::{
    get_metadata_hash,
    get_pallet_hash,
    MetadataHasherCache,
};

/// Utilities for validating metadata compatibility between multiple nodes.
#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
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
        Command::Compatibility { nodes, pallet } => {
            match pallet {
                Some(pallet) => handle_pallet_metadata(nodes.as_slice(), pallet.as_str()),
                None => handle_full_metadata(nodes.as_slice()),
            }
        }
    }
}

fn handle_pallet_metadata(nodes: &[url::Url], name: &str) -> color_eyre::Result<()> {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CompatibilityPallet {
        pallet_present: HashMap<String, Vec<String>>,
        pallet_not_found: Vec<String>,
    }

    let mut compatibility = CompatibilityPallet {
        pallet_present: Default::default(),
        pallet_not_found: vec![],
    };

    for node in nodes.iter() {
        let metadata = fetch_metadata(node)?;

        match metadata.pallets.iter().find(|pallet| pallet.name == name) {
            Some(pallet_metadata) => {
                let hash = pallet_hash(&metadata.types, pallet_metadata);
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

fn pallet_hash(
    registry: &PortableRegistry,
    pallet: &frame_metadata::PalletMetadata<PortableForm>,
) -> [u8; 32] {
    let mut cache = MetadataHasherCache::new();
    get_pallet_hash(registry, pallet, &mut cache)
}

fn handle_full_metadata(nodes: &[url::Url]) -> color_eyre::Result<()> {
    let mut compatibility_map: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes.iter() {
        let metadata = fetch_metadata(node)?;
        let hash = metadata_hash(&metadata);
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

fn metadata_hash(metadata: &RuntimeMetadataLastVersion) -> [u8; 32] {
    // Cached value cannot be shared between different metadata.
    let mut cache = MetadataHasherCache::new();
    get_metadata_hash(metadata, &mut cache)
}

fn fetch_metadata(url: &url::Url) -> color_eyre::Result<RuntimeMetadataLastVersion> {
    let resp = ureq::post(url.as_str())
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "state_getMetadata",
            "id": 1
        }))
        .context(format!(
            "Error fetching metadata from node {:?}",
            url.as_str()
        ))?;
    let json: serde_json::Value = resp.into_json()?;

    let hex_data = json["result"]
        .as_str()
        .map(ToString::to_string)
        .ok_or_else(|| eyre::eyre!("metadata result field should be a string"))?;
    let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;

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
