// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::FetchMetadataError;
use codec::{Decode, Encode};
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use jsonrpsee::{
    async_client::ClientBuilder,
    client_transport::ws::{Uri, WsTransportClientBuilder},
    core::{client::ClientT, Error},
    http_client::HttpClientBuilder,
    rpc_params,
};
use std::time::Duration;
use subxt_metadata::metadata_v14_to_latest;

/// Returns the metadata bytes from the provided URL, blocking the current thread.
pub fn fetch_metadata_bytes_blocking(url: &Uri) -> Result<Vec<u8>, FetchMetadataError> {
    tokio_block_on(fetch_metadata_bytes(url))
}

/// Returns the raw, 0x prefixed metadata hex from the provided URL, blocking the current thread.
pub fn fetch_metadata_hex_blocking(url: &Uri) -> Result<String, FetchMetadataError> {
    tokio_block_on(fetch_metadata_hex(url))
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
pub async fn fetch_metadata_bytes(url: &Uri) -> Result<Vec<u8>, FetchMetadataError> {
    let bytes = match url.scheme_str() {
        Some("http") | Some("https") => fetch_metadata_http(url).await,
        Some("ws") | Some("wss") => fetch_metadata_ws(url).await,
        invalid_scheme => {
            let scheme = invalid_scheme.unwrap_or("no scheme");
            Err(FetchMetadataError::InvalidScheme(scheme.to_owned()))
        }
    }?;

    Ok(bytes)
}

/// Returns the raw, 0x prefixed metadata hex from the provided URL.
pub async fn fetch_metadata_hex(url: &Uri) -> Result<String, FetchMetadataError> {
    let bytes = fetch_metadata_bytes(url).await?;
    let hex_data = format!("0x{}", hex::encode(bytes));
    Ok(hex_data)
}

/// Execute runtime API call and return the specified runtime metadata version.
async fn fetch_latest_metadata(client: impl ClientT) -> Result<Vec<u8>, FetchMetadataError> {
    const V15_METADATA_VERSION: u32 = u32::MAX;
    let bytes = V15_METADATA_VERSION.encode();

    // Runtime API arguments are scale encoded hex encoded.
    let version: String = format!("0x{}", hex::encode(&bytes));

    // Returns a hex(Option<frame_metadata::OpaqueMetadata>).

    let result: Result<String, _> = client
        .request(
            "state_call",
            rpc_params!["Metadata_metadata_at_version", &version],
        )
        .await;

    if let Ok(raw) = result {
        let raw_bytes = hex::decode(raw.trim_start_matches("0x"))?;

        let opaque: Option<frame_metadata::OpaqueMetadata> = Decode::decode(&mut &raw_bytes[..])?;
        let bytes = opaque.ok_or(FetchMetadataError::Other(
            "Metadata version not found".into(),
        ))?;

        return Ok(bytes.0);
    }

    // The `Metadata_metadata_at_version` RPC failed, fall back to the old `state_getMetadata`.
    let raw: String = client.request("state_getMetadata", rpc_params![]).await?;

    let raw_bytes = hex::decode(raw.trim_start_matches("0x"))?;
    // Decode the metadata to inspect the RuntimeMetadata variant.
    let meta: RuntimeMetadataPrefixed = Decode::decode(&mut &raw_bytes[..])?;
    // Update available versions to V15.
    let metadata = match meta.1 {
        RuntimeMetadata::V14(v14) => metadata_v14_to_latest(v14),
        RuntimeMetadata::V15(v15) => v15,
        _ => {
            return Err(FetchMetadataError::Other(
                "Metadata version not found".into(),
            ))
        }
    };
    // Convert back to bytes.
    let meta: RuntimeMetadataPrefixed = metadata.into();
    Ok(meta.encode())
}

async fn fetch_metadata_ws(url: &Uri) -> Result<Vec<u8>, FetchMetadataError> {
    let (sender, receiver) = WsTransportClientBuilder::default()
        .build(url.to_string().parse::<Uri>().unwrap())
        .await
        .map_err(|e| Error::Transport(e.into()))?;

    let client = ClientBuilder::default()
        .request_timeout(Duration::from_secs(180))
        .max_notifs_per_subscription(4096)
        .build_with_tokio(sender, receiver);

    fetch_latest_metadata(client).await
}

async fn fetch_metadata_http(url: &Uri) -> Result<Vec<u8>, FetchMetadataError> {
    let client = HttpClientBuilder::default()
        .request_timeout(Duration::from_secs(180))
        .build(url.to_string())?;

    fetch_latest_metadata(client).await
}
