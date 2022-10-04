// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use color_eyre::eyre;
use frame_metadata::RuntimeMetadataPrefixed;
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
use scale::Decode;
use std::io::{
    self,
    Write,
};

/// Download metadata from a substrate node, for use with `subxt` codegen.
#[derive(Debug, ClapParser)]
pub struct Opts {
    /// The url of the substrate node to query for metadata.
    #[clap(
        name = "url",
        long,
        value_parser,
        default_value = "http://localhost:9933"
    )]
    url: Uri,
    /// The format of the metadata to display: `json`, `hex` or `bytes`.
    #[clap(long, short, default_value = "bytes")]
    format: String,
}

pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    let (hex_data, bytes) = fetch_metadata(&opts.url).await?;

    match opts.format.as_str() {
        "json" => {
            let metadata = <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;
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
                opts.format
            ))
        }
    }
}

pub async fn fetch_metadata(url: &Uri) -> color_eyre::Result<(String, Vec<u8>)> {
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
