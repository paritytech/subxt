// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Fetch metadata from a URL.

use crate::Error;
use codec::{Decode, Encode}; 
use jsonrpsee::{
    core::client::ClientT, http_client::HttpClientBuilder, rpc_params, ws_client::WsClientBuilder,
};

pub use url::Url;

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
                    .map_err(|_| format!("Invalid metadata version specified {version:?}"))?;

                Ok(MetadataVersion::Version(num))
            }
        }
    }
}

/// Returns the metadata bytes from the provided URL.
pub async fn from_url(url: Url, version: MetadataVersion, at_block_hash: Option<&str>) -> Result<Vec<u8>, Error> {
    let bytes = match url.scheme() {
        "http" | "https" => fetch_metadata_http(url, version, at_block_hash).await,
        "ws" | "wss" => fetch_metadata_ws(url, version, at_block_hash).await,
        invalid_scheme => Err(Error::InvalidScheme(invalid_scheme.to_owned())),
    }?;

    Ok(bytes)
}

/// Returns the metadata bytes from the provided URL, blocking the current thread.
pub fn from_url_blocking(url: Url, version: MetadataVersion, at_block_hash: Option<&str>) -> Result<Vec<u8>, Error> {
    tokio_block_on(from_url(url, version, at_block_hash))
}

// Block on some tokio runtime for sync contexts
fn tokio_block_on<T, Fut: std::future::Future<Output = T>>(fut: Fut) -> T {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(fut)
}

async fn fetch_metadata_ws(url: Url, version: MetadataVersion, at_block_hash: Option<&str>) -> Result<Vec<u8>, Error> {
    let client = WsClientBuilder::default()
        .request_timeout(std::time::Duration::from_secs(180))
        .max_buffer_capacity_per_subscription(4096)
        .build(url)
        .await?;

    fetch_metadata(client, version, at_block_hash).await
}

async fn fetch_metadata_http(url: Url, version: MetadataVersion, at_block_hash: Option<&str>) -> Result<Vec<u8>, Error> {
    let client = HttpClientBuilder::default()
        .request_timeout(std::time::Duration::from_secs(180))
        .build(url)?;

    fetch_metadata(client, version, at_block_hash).await
}

/// The innermost call to fetch metadata:
async fn fetch_metadata(client: impl ClientT, version: MetadataVersion, at_block_hash: Option<&str>) -> Result<Vec<u8>, Error> {
    const UNSTABLE_METADATA_VERSION: u32 = u32::MAX;

    // Ensure always 0x prefix.
    let at_block_hash = at_block_hash
        .map(|hash| format!("0x{}", hash.strip_prefix("0x").unwrap_or(hash)));
    let at_block_hash = at_block_hash.as_deref();

    // Fetch available metadata versions. If error, revert to legacy metadata code.
    async fn fetch_available_versions(
        client: &impl ClientT,
        at_block_hash: Option<&str>,
    ) -> Result<Vec<u32>, Error> {
        let res: String = client
            .request("state_call", rpc_params!["Metadata_metadata_versions", "0x", at_block_hash])
            .await?;
        let raw_bytes = hex::decode(res.trim_start_matches("0x"))?;
        Decode::decode(&mut &raw_bytes[..]).map_err(Into::into)
    }

    // Fetch metadata using the "new" state_call interface
    async fn fetch_inner(
        client: &impl ClientT,
        version: MetadataVersion,
        supported_versions: Vec<u32>,
        at_block_hash: Option<&str>,
    ) -> Result<Vec<u8>, Error> {
        // Return the version the user wants if it's supported:
        let version = match version {
            MetadataVersion::Latest => *supported_versions
                .iter()
                .filter(|&&v| v != UNSTABLE_METADATA_VERSION)
                .max()
                .ok_or_else(|| Error::Other("No valid metadata versions returned".to_string()))?,
            MetadataVersion::Unstable => {
                if supported_versions.contains(&UNSTABLE_METADATA_VERSION) {
                    UNSTABLE_METADATA_VERSION
                } else {
                    return Err(Error::Other(
                        "The node does not have an unstable metadata version available".to_string(),
                    ));
                }
            }
            MetadataVersion::Version(version) => {
                if supported_versions.contains(&version) {
                    version
                } else {
                    return Err(Error::Other(format!(
                        "The node does not have metadata version {version} available"
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
                rpc_params!["Metadata_metadata_at_version", &version, at_block_hash],
            )
            .await?;
        // Decode the metadata.
        let metadata_bytes = hex::decode(metadata_string.trim_start_matches("0x"))?;
        let metadata: Option<frame_metadata::OpaqueMetadata> =
            Decode::decode(&mut &metadata_bytes[..])?;
        let Some(metadata) = metadata else {
            return Err(Error::Other(format!(
                "The node does not have metadata version {version} available"
            )));
        };
        Ok(metadata.0)
    }

    // Fetch metadata using the "old" state_call interface
    async fn fetch_inner_legacy(
        client: &impl ClientT,
        at_block_hash: Option<&str>,
    ) -> Result<Vec<u8>, Error> {
        // Fetch the metadata.
        let metadata_string: String = client
            .request("state_call", rpc_params!["Metadata_metadata", "0x", at_block_hash])
            .await?;

        // Decode the metadata.
        let metadata_bytes = hex::decode(metadata_string.trim_start_matches("0x"))?;
        let metadata: frame_metadata::OpaqueMetadata = Decode::decode(&mut &metadata_bytes[..])?;
        Ok(metadata.0)
    }

    match fetch_available_versions(&client, at_block_hash).await {
        Ok(supported_versions) => {
            fetch_inner(&client, version, supported_versions, at_block_hash).await
        },
        Err(e) => {
            // The "new" interface failed. if the user is asking for V14 or the "latest"
            // metadata then try the legacy interface instead. Else, just return the
            // reason for failure.
            if matches!(version, MetadataVersion::Version(14) | MetadataVersion::Latest) {
                fetch_inner_legacy(&client, at_block_hash).await
            } else {
                Err(e)
            }
        }
    }
}
