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
#[derive(Default, Debug, Clone, Copy)]
pub enum MetadataVersion {
    /// Latest stable version of the metadata.
    #[default]
    Latest,
    /// Fetch a specified version of the metadata.
    Version(u32),
    /// Latest unstable version of the metadata.
    Unstable,
}

// Note: Implementation needed for the CLI tool.
impl std::str::FromStr for MetadataVersion {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "unstable" => Ok(MetadataVersion::Unstable),
            "latest" => Ok(MetadataVersion::Latest),
            version => {
                let num: u32 = version
                    .parse()
                    .map_err(|_| format!("Invalid metadata version specified {:?}", version))?;

                Ok(MetadataVersion::Version(num))
            }
        }
    }
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

/// The innermost call to fetch metadata:
async fn fetch_metadata(
    client: impl ClientT,
    version: MetadataVersion,
) -> Result<Vec<u8>, FetchMetadataError> {
    const UNSTABLE_METADATA_VERSION: u32 = u32::MAX;

    // Fetch metadata using the "new" state_call interface
    async fn fetch_inner(
        client: &impl ClientT,
        version: MetadataVersion,
    ) -> Result<Vec<u8>, FetchMetadataError> {
        // Look up supported versions:
        let supported_versions: Vec<u32> = {
            let res: String = client
                .request(
                    "state_call",
                    rpc_params!["Metadata_metadata_versions", "0x"],
                )
                .await?;
            let raw_bytes = hex::decode(res.trim_start_matches("0x"))?;
            Decode::decode(&mut &raw_bytes[..])?
        };

        // Return the version the user wants if it's supported:
        let version = match version {
            MetadataVersion::Latest => *supported_versions
                .iter()
                .filter(|&&v| v != UNSTABLE_METADATA_VERSION)
                .max()
                .ok_or_else(|| {
                    FetchMetadataError::Other("No valid metadata versions returned".to_string())
                })?,
            MetadataVersion::Unstable => {
                if supported_versions.contains(&UNSTABLE_METADATA_VERSION) {
                    UNSTABLE_METADATA_VERSION
                } else {
                    return Err(FetchMetadataError::Other(
                        "The node does not have an unstable metadata version available".to_string(),
                    ));
                }
            }
            MetadataVersion::Version(version) => {
                if supported_versions.contains(&version) {
                    version
                } else {
                    return Err(FetchMetadataError::Other(format!(
                        "The node does not have version {version} available"
                    )));
                }
            }
        };

        let bytes = version.encode();
        let version: String = format!("0x{}", hex::encode(&bytes));

        // Fetch the metadata at that version:
        let metadata_string: String = client
            .request(
                "state_call",
                rpc_params!["Metadata_metadata_at_version", &version],
            )
            .await?;
        // Decode the metadata.
        let metadata_bytes = hex::decode(metadata_string.trim_start_matches("0x"))?;
        let metadata: Option<frame_metadata::OpaqueMetadata> =
            Decode::decode(&mut &metadata_bytes[..])?;
        let Some(metadata) = metadata else {
            return Err(FetchMetadataError::Other(format!(
                "The node does not have version {version} available"
            )));
        };
        Ok(metadata.0)
    }

    // Fetch metadata using the "old" state_call interface
    async fn fetch_inner_legacy(
        client: &impl ClientT,
        version: MetadataVersion,
    ) -> Result<Vec<u8>, FetchMetadataError> {
        if !matches!(version, MetadataVersion::Version(14)) {
            return Err(FetchMetadataError::Other(
                "The node can only return version 14 metadata using the legacy API but you've asked for something else"
                    .to_string(),
            ));
        }

        // Fetch the metadata at that version:
        let metadata_string: String = client
            .request("state_call", rpc_params!["Metadata_metadata", "0x"])
            .await?;

        // Decode the metadata.
        let metadata_bytes = hex::decode(metadata_string.trim_start_matches("0x"))?;
        let metadata: frame_metadata::OpaqueMetadata = Decode::decode(&mut &metadata_bytes[..])?;
        Ok(metadata.0)
    }

    // Fetch using the new interface, falling back to trying old one if there's an error.
    match fetch_inner(&client, version).await {
        Ok(s) => Ok(s),
        Err(_) => fetch_inner_legacy(&client, version).await,
    }
}
