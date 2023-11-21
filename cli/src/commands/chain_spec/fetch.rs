// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use jsonrpsee::{
    async_client::ClientBuilder,
    client_transport::ws::WsTransportClientBuilder,
    core::{client::ClientT, Error},
    http_client::HttpClientBuilder,
};
use std::time::Duration;
use subxt_codegen::fetch_metadata::Url;

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
            .map_err(FetchSpecError::RequestError),
        "ws" | "wss" => fetch_ws(url)
            .await
            .map_err(FetchSpecError::RequestError),
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
