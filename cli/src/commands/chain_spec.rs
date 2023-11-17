// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use jsonrpsee::{
    async_client::ClientBuilder,
    client_transport::ws::WsTransportClientBuilder,
    core::{client::ClientT, Error},
    http_client::HttpClientBuilder,
};
use std::{io::Write, path::PathBuf, time::Duration};
use subxt_codegen::fetch_metadata::Url;

/// Download chainSpec from a substrate node.
#[derive(Debug, ClapParser)]
pub struct Opts {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(long)]
    url: Url,
    /// Write the output of the command to the provided file path.
    #[clap(long, short, value_parser)]
    pub output_file: Option<PathBuf>,
}

pub async fn run(opts: Opts, output: &mut impl Write) -> color_eyre::Result<()> {
    let url = opts.url;

    let spec = fetch_chain_spec(url).await?;

    let mut output: Box<dyn Write> = match opts.output_file {
        Some(path) => Box::new(std::fs::File::create(path)?),
        None => Box::new(output),
    };

    let json = serde_json::to_string_pretty(&spec)?;
    write!(output, "{json}")?;
    Ok(())
}

/// Returns the node's chainSpec from the provided URL.
pub async fn fetch_chain_spec(url: Url) -> Result<serde_json::Value, FetchSpecError> {
    async fn fetch_ws(url: Url) -> Result<serde_json::Value, Error> {
        let (sender, receiver) = WsTransportClientBuilder::default()
            .build(url)
            .await
            .map_err(|e| Error::Transport(e.into()))?;

        let client = ClientBuilder::default()
            .request_timeout(Duration::from_secs(180))
            .max_buffer_capacity_per_subscription(4096)
            .build_with_tokio(sender, receiver);

        inner_fetch(client).await
    }

    async fn fetch_http(url: Url) -> Result<serde_json::Value, Error> {
        let client = HttpClientBuilder::default()
            .request_timeout(Duration::from_secs(180))
            .build(url)?;

        inner_fetch(client).await
    }

    async fn inner_fetch(client: impl ClientT) -> Result<serde_json::Value, Error> {
        client
            .request("sync_state_genSyncSpec", jsonrpsee::rpc_params![true])
            .await
    }

    let spec = match url.scheme() {
        "http" | "https" => fetch_http(url)
            .await
            .map_err(|err| FetchSpecError::RequestError(err)),
        "ws" | "wss" => fetch_ws(url)
            .await
            .map_err(|err| FetchSpecError::RequestError(err)),
        invalid_scheme => Err(FetchSpecError::InvalidScheme(invalid_scheme.to_owned())),
    }?;

    Ok(spec)
}

/// Error attempting to fetch chainSpec.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FetchSpecError {
    /// JSON-RPC error fetching metadata.
    #[error("Request error: {0}")]
    RequestError(#[from] jsonrpsee::core::Error),
    /// URL scheme is not http, https, ws or wss.
    #[error("'{0}' not supported, supported URI schemes are http, https, ws or wss.")]
    InvalidScheme(String),
}
