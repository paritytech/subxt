// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::FetchMetadataError;
use codec::{Decode, Encode};
use jsonrpsee::{
    async_client::ClientBuilder,
    client_transport::ws::{Uri, WsTransportClientBuilder},
    core::{client::ClientT, Error},
    http_client::HttpClientBuilder,
    rpc_params,
};
use std::time::Duration;

/// The metadata version that is fetched from the node.
#[derive(Default)]
pub enum MetadataVersion {
    /// Latest stable version of the metadata.
    #[default]
    Latest,
    /// Fetch a specified version of the metadata.
    Version(u32),
    /// Latest unstable version of the metadata.
    Unstable,
}

/// Returns the metadata bytes from the provided URL, blocking the current thread.
pub fn fetch_metadata_bytes_blocking(
    url: &Uri,
    version: MetadataVersion,
) -> Result<Vec<u8>, FetchMetadataError> {
    tokio_block_on(fetch_metadata_bytes(url, version))
}

/// Returns the raw, 0x prefixed metadata hex from the provided URL, blocking the current thread.
pub fn fetch_metadata_hex_blocking(
    url: &Uri,
    version: MetadataVersion,
) -> Result<String, FetchMetadataError> {
    tokio_block_on(fetch_metadata_hex(url, version))
}

// Block on some tokio runtime for sync contexts
fn tokio_block_on<T, Fut: std::future::Future<Output = T>>(fut: Fut) -> T {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(fut)
}

/// Returns the metadata bytes from the provided URL.
pub async fn fetch_metadata_bytes(
    url: &Uri,
    version: MetadataVersion,
) -> Result<Vec<u8>, FetchMetadataError> {
    let bytes = match url.scheme_str() {
        Some("http") | Some("https") => fetch_metadata_http(url, version).await,
        Some("ws") | Some("wss") => fetch_metadata_ws(url, version).await,
        invalid_scheme => {
            let scheme = invalid_scheme.unwrap_or("no scheme");
            Err(FetchMetadataError::InvalidScheme(scheme.to_owned()))
        }
    }?;

    Ok(bytes)
}

/// Returns the raw, 0x prefixed metadata hex from the provided URL.
pub async fn fetch_metadata_hex(
    url: &Uri,
    version: MetadataVersion,
) -> Result<String, FetchMetadataError> {
    let bytes = fetch_metadata_bytes(url, version).await?;
    let hex_data = format!("0x{}", hex::encode(bytes));
    Ok(hex_data)
}

/// Execute runtime API call and return the specified runtime metadata version.
async fn fetch_metadata(
    client: impl ClientT,
    version: MetadataVersion,
) -> Result<Vec<u8>, FetchMetadataError> {
    const UNSTABLE_METADATA_VERSION: u32 = u32::MAX;

    let version_num = match version {
        MetadataVersion::Latest => {
            // Fetch latest stable metadata of a node via `Metadata_metadata`.
            let raw: String = client
                .request("state_call", rpc_params!["Metadata_metadata", "0x"])
                .await?;
            let raw_bytes = hex::decode(raw.trim_start_matches("0x"))?;
            let bytes: frame_metadata::OpaqueMetadata = Decode::decode(&mut &raw_bytes[..])?;
            return Ok(bytes.0);
        }
        MetadataVersion::Version(version) => version,
        MetadataVersion::Unstable => UNSTABLE_METADATA_VERSION,
    };

    // Other versions (including unstable) are fetched with `Metadata_metadata_at_version`.
    let bytes = version_num.encode();
    let version: String = format!("0x{}", hex::encode(&bytes));

    let raw: String = client
        .request(
            "state_call",
            rpc_params!["Metadata_metadata_at_version", &version],
        )
        .await?;

    let raw_bytes = hex::decode(raw.trim_start_matches("0x"))?;

    let opaque: Option<frame_metadata::OpaqueMetadata> = Decode::decode(&mut &raw_bytes[..])?;
    let bytes = opaque.ok_or(FetchMetadataError::Other(
        "Metadata version not found".into(),
    ))?;

    Ok(bytes.0)
}

async fn fetch_metadata_ws(
    url: &Uri,
    version: MetadataVersion,
) -> Result<Vec<u8>, FetchMetadataError> {
    let (sender, receiver) = WsTransportClientBuilder::default()
        .build(url.to_string().parse::<Uri>().unwrap())
        .await
        .map_err(|e| Error::Transport(e.into()))?;

    let client = ClientBuilder::default()
        .request_timeout(Duration::from_secs(180))
        .max_notifs_per_subscription(4096)
        .build_with_tokio(sender, receiver);

    fetch_metadata(client, version).await
}

async fn fetch_metadata_http(
    url: &Uri,
    version: MetadataVersion,
) -> Result<Vec<u8>, FetchMetadataError> {
    let client = HttpClientBuilder::default()
        .request_timeout(Duration::from_secs(180))
        .build(url.to_string())?;

    fetch_metadata(client, version).await
}
