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

async fn fetch_latest_stable(client: impl ClientT) -> Result<Vec<u8>, FetchMetadataError> {
    // Fetch latest stable metadata of a node via `Metadata_metadata`.
    let raw: String = client
        .request("state_call", rpc_params!["Metadata_metadata", "0x"])
        .await?;
    let raw_bytes = hex::decode(raw.trim_start_matches("0x"))?;
    let bytes: frame_metadata::OpaqueMetadata = Decode::decode(&mut &raw_bytes[..])?;
    Ok(bytes.0)
}

/// Execute runtime API call and return the specified runtime metadata version.
async fn fetch_metadata(
    client: impl ClientT,
    version: MetadataVersion,
) -> Result<Vec<u8>, FetchMetadataError> {
    const UNSTABLE_METADATA_VERSION: u32 = u32::MAX;

    // Note: `Metadata_metadata_versions` may not be present on all nodes.
    // Once every node supports the new RPC methods, we can simplify the code a bit.
    let supported: Option<Vec<u32>> = client
        .request(
            "state_call",
            rpc_params!["Metadata_metadata_versions", "0x"],
        )
        .await
        .ok()
        .map(|raw: String| {
            let raw_bytes = hex::decode(raw.trim_start_matches("0x"))?;
            let versions: Vec<u32> = Decode::decode(&mut &raw_bytes[..])?;
            Ok::<Vec<u32>, FetchMetadataError>(versions)
        })
        .transpose()?;

    // Ensure the user requested a valid version if done implicitly.
    if let (Some(supported_versions), MetadataVersion::Version(request_version)) =
        (&supported, &version)
    {
        if !supported_versions.is_empty()
            && !supported_versions
                .iter()
                .any(|value| value == request_version)
        {
            return Err(FetchMetadataError::Other(format!(
                "Metadata version {} not supported",
                request_version
            )))?;
        }
    }

    let mut is_latest = false;
    let version_num = match version {
        MetadataVersion::Latest => {
            // If he have a valid supported version, find the latest stable version number.
            let version_num = if let Some(supported_versions) = &supported {
                supported_versions
                    .iter()
                    .fold(None, |max, value| match (max, value) {
                        (None, value) => Some(value),
                        (Some(old_max), value) => {
                            if value != &UNSTABLE_METADATA_VERSION && value > old_max {
                                Some(value)
                            } else {
                                Some(old_max)
                            }
                        }
                    })
            } else {
                // List not exposed by node.
                None
            };

            if let Some(version_num) = version_num {
                // Use the latest stable from the provided list.
                is_latest = true;
                *version_num
            } else {
                // List is empty or the node does not expose the list of supported versions.
                return fetch_latest_stable(client).await;
            }
        }
        MetadataVersion::Version(version) => version,
        MetadataVersion::Unstable => UNSTABLE_METADATA_VERSION,
    };

    // Other versions (including unstable) are fetched with `Metadata_metadata_at_version`.
    let bytes = version_num.encode();
    let version: String = format!("0x{}", hex::encode(&bytes));

    let result: Result<String, _> = client
        .request(
            "state_call",
            rpc_params!["Metadata_metadata_at_version", &version],
        )
        .await;

    match result {
        Ok(raw) => {
            let raw_bytes = hex::decode(raw.trim_start_matches("0x"))?;

            let opaque: Option<frame_metadata::OpaqueMetadata> =
                Decode::decode(&mut &raw_bytes[..])?;
            let bytes = opaque.ok_or(FetchMetadataError::Other(
                "Metadata version not found".into(),
            ))?;

            Ok(bytes.0)
        }
        Err(err) => {
            // Try to fetch the latest with `Metadata_metadata`.
            if is_latest {
                fetch_latest_stable(client).await
            } else {
                Err(err.into())
            }
        }
    }
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
