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
use std::collections::HashMap;
use structopt::StructOpt;
use subxt_metadata::{
    get_metadata_hash,
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
    },
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Opts::from_args();

    match args.command {
        Command::Compatibility { nodes } => {
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
        }
    }
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
