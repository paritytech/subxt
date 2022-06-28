// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

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
use jsonrpsee::{
    async_client::ClientBuilder,
    client_transport::ws::{
        Uri,
        WsTransportClientBuilder,
    },
    core::{
        client::ClientT,
        Error,
    },
    http_client::HttpClientBuilder,
    rpc_params,
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
        /// The url of the substrate node to query for metadata.
        #[structopt(
            name = "url",
            long,
            parse(try_from_str),
            default_value = "http://localhost:9933"
        )]
        url: Uri,
        /// The format of the metadata to display: `json`, `hex` or `bytes`.
        #[structopt(long, short, default_value = "bytes")]
        format: String,
    },
    /// Generate runtime API client code from metadata.
    ///
    /// # Example (with code formatting)
    ///
    /// `subxt codegen | rustfmt --edition=2018 --emit=stdout`
    Codegen {
        /// The url of the substrate node to query for metadata for codegen.
        #[structopt(name = "url", long, parse(try_from_str))]
        url: Option<Uri>,
        /// The path to the encoded metadata file.
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
        nodes: Vec<Uri>,
        /// Check the compatibility of metadata for a particular pallet.
        ///
        /// ### Note
        /// The validation will omit the full metadata check and focus instead on the pallet.
        #[structopt(long, parse(try_from_str))]
        pallet: Option<String>,
    },
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Opts::from_args();

    match args.command {
        Command::Metadata { url, format } => {
            let (hex_data, bytes) = fetch_metadata(&url).await?;

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
                "http://localhost:9933"
                    .parse::<Uri>()
                    .expect("default url is valid")
            });
            let (_, bytes) = fetch_metadata(&url).await?;
            codegen(&mut &bytes[..], derives)?;
            Ok(())
        }
        Command::Compatibility { nodes, pallet } => {
            match pallet {
                Some(pallet) => {
                    handle_pallet_metadata(nodes.as_slice(), pallet.as_str()).await
                }
                None => handle_full_metadata(nodes.as_slice()).await,
            }
        }
    }
}

async fn handle_pallet_metadata(nodes: &[Uri], name: &str) -> color_eyre::Result<()> {
    #[derive(Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct CompatibilityPallet {
        pallet_present: HashMap<String, Vec<String>>,
        pallet_not_found: Vec<String>,
    }

    let mut compatibility: CompatibilityPallet = Default::default();
    for node in nodes.iter() {
        let metadata = fetch_runtime_metadata(node).await?;

        match metadata.pallets.iter().find(|pallet| pallet.name == name) {
            Some(pallet_metadata) => {
                let hash = get_pallet_hash(&metadata.types, pallet_metadata);
                let hex_hash = hex::encode(hash);
                println!("Node {:?} has pallet metadata hash {:?}", node, hex_hash);

                compatibility
                    .pallet_present
                    .entry(hex_hash)
                    .or_insert_with(Vec::new)
                    .push(node.to_string());
            }
            None => {
                compatibility.pallet_not_found.push(node.to_string());
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

async fn handle_full_metadata(nodes: &[Uri]) -> color_eyre::Result<()> {
    let mut compatibility_map: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes.iter() {
        let metadata = fetch_runtime_metadata(node).await?;
        let hash = get_metadata_hash(&metadata);
        let hex_hash = hex::encode(hash);
        println!("Node {:?} has metadata hash {:?}", node, hex_hash,);

        compatibility_map
            .entry(hex_hash)
            .or_insert_with(Vec::new)
            .push(node.to_string());
    }

    println!(
        "\nCompatible nodes\n{}",
        serde_json::to_string_pretty(&compatibility_map)
            .context("Failed to parse compatibility map")?
    );

    Ok(())
}

async fn fetch_runtime_metadata(url: &Uri) -> color_eyre::Result<RuntimeMetadataV14> {
    let (_, bytes) = fetch_metadata(url).await?;

    let metadata = <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;
    if metadata.0 != META_RESERVED {
        return Err(eyre::eyre!(
            "Node {:?} has invalid metadata prefix: {:?} expected prefix: {:?}",
            url,
            metadata.0,
            META_RESERVED
        ))
    }

    match metadata.1 {
        RuntimeMetadata::V14(v14) => Ok(v14),
        _ => {
            Err(eyre::eyre!(
                "Node {:?} with unsupported metadata version: {:?}",
                url,
                metadata.1
            ))
        }
    }
}

async fn fetch_metadata_ws(url: &Uri) -> color_eyre::Result<String> {
    let (sender, receiver) = WsTransportClientBuilder::default()
        .build(url.to_string().parse::<Uri>().unwrap())
        .await
        .map_err(|e| Error::Transport(e.into()))?;

    let client = ClientBuilder::default()
        .max_notifs_per_subscription(4096)
        .build_with_tokio(sender, receiver);

    Ok(client.request("state_getMetadata", rpc_params![]).await?)
}

async fn fetch_metadata_http(url: &Uri) -> color_eyre::Result<String> {
    let client = HttpClientBuilder::default().build(url.to_string())?;

    Ok(client.request::<String>("state_getMetadata", None).await?)
}

async fn fetch_metadata(url: &Uri) -> color_eyre::Result<(String, Vec<u8>)> {
    let hex_data = match url.scheme_str() {
        Some("http") => fetch_metadata_http(url).await,
        Some("ws") | Some("wss") => fetch_metadata_ws(url).await,
        invalid_scheme => {
            let scheme = invalid_scheme.unwrap_or("no scheme");
            Err(eyre::eyre!(format!(
                "`{}` not supported, expects 'http', 'ws', or 'wss'",
                scheme
            )))
        }
    }?;

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
