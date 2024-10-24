// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use codec::Decode;
use color_eyre::eyre::WrapErr;
use jsonrpsee::client_transport::ws::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use subxt_metadata::Metadata;
use subxt_utils_fetchmetadata::MetadataVersion;

use crate::utils::validate_url_security;

/// Verify metadata compatibility between substrate nodes.
#[derive(Debug, ClapParser)]
pub struct Opts {
    /// Urls of the substrate nodes to verify for metadata compatibility.
    #[clap(name = "nodes", long, use_value_delimiter = true, value_parser)]
    nodes: Vec<Url>,
    /// Check the compatibility of metadata for a particular pallet.
    ///
    /// ### Note
    /// The validation will omit the full metadata check and focus instead on the pallet.
    #[clap(long, value_parser)]
    pallet: Option<String>,
    /// Specify the metadata version.
    ///
    ///  - unstable:
    ///
    ///    Use the latest unstable metadata of the node.
    ///
    ///  - number
    ///
    ///    Use this specific metadata version.
    ///
    /// Defaults to latest.
    #[clap(long = "version", default_value = "latest")]
    version: MetadataVersion,
    /// Allow insecure URLs e.g. URLs starting with ws:// or http:// without SSL encryption
    #[clap(long, short)]
    allow_insecure: bool,
}

pub async fn run(opts: Opts, output: &mut impl std::io::Write) -> color_eyre::Result<()> {
    for url in opts.nodes.iter() {
        validate_url_security(Some(url), opts.allow_insecure)?;
    }

    match opts.pallet {
        Some(pallet) => {
            handle_pallet_metadata(opts.nodes.as_slice(), pallet.as_str(), opts.version, output)
                .await
        }
        None => handle_full_metadata(opts.nodes.as_slice(), opts.version, output).await,
    }
}

async fn handle_pallet_metadata(
    nodes: &[Url],
    name: &str,
    version: MetadataVersion,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    #[derive(Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct CompatibilityPallet {
        pallet_present: HashMap<String, Vec<String>>,
        pallet_not_found: Vec<String>,
    }

    let mut compatibility: CompatibilityPallet = Default::default();
    for node in nodes.iter() {
        let metadata = fetch_runtime_metadata(node.clone(), version).await?;

        match metadata.pallet_by_name(name) {
            Some(pallet_metadata) => {
                let hash = pallet_metadata.hash();
                let hex_hash = hex::encode(hash);
                writeln!(
                    output,
                    "Node {node:?} has pallet metadata hash {hex_hash:?}"
                )?;

                compatibility
                    .pallet_present
                    .entry(hex_hash)
                    .or_default()
                    .push(node.to_string());
            }
            None => {
                compatibility.pallet_not_found.push(node.to_string());
            }
        }
    }

    writeln!(
        output,
        "\nCompatible nodes by pallet\n{}",
        serde_json::to_string_pretty(&compatibility)
            .context("Failed to parse compatibility map")?
    )?;

    Ok(())
}

async fn handle_full_metadata(
    nodes: &[Url],
    version: MetadataVersion,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let mut compatibility_map: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes.iter() {
        let metadata = fetch_runtime_metadata(node.clone(), version).await?;
        let hash = metadata.hasher().hash();
        let hex_hash = hex::encode(hash);
        writeln!(output, "Node {node:?} has metadata hash {hex_hash:?}",)?;

        compatibility_map
            .entry(hex_hash)
            .or_default()
            .push(node.to_string());
    }

    writeln!(
        output,
        "\nCompatible nodes\n{}",
        serde_json::to_string_pretty(&compatibility_map)
            .context("Failed to parse compatibility map")?
    )?;

    Ok(())
}

async fn fetch_runtime_metadata(
    url: Url,
    version: MetadataVersion,
) -> color_eyre::Result<Metadata> {
    let bytes = subxt_utils_fetchmetadata::from_url(url, version).await?;
    let metadata = Metadata::decode(&mut &bytes[..])?;
    Ok(metadata)
}
