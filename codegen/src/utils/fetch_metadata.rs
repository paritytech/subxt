use jsonrpsee::{
    async_client::ClientBuilder,
    client_transport::ws::{Uri, WsTransportClientBuilder},
    core::{client::ClientT, Error},
    http_client::HttpClientBuilder,
    rpc_params,
};
use std::time::Duration;

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
    let hex = fetch_metadata_hex(url).await?;
    let bytes = hex::decode(hex.trim_start_matches("0x"))?;
    Ok(bytes)
}

/// Returns the raw, 0x prefixed metadata hex from the provided URL.
pub async fn fetch_metadata_hex(url: &Uri) -> Result<String, FetchMetadataError> {
    let hex_data = match url.scheme_str() {
        Some("http") | Some("https") => fetch_metadata_http(url).await,
        Some("ws") | Some("wss") => fetch_metadata_ws(url).await,
        invalid_scheme => {
            let scheme = invalid_scheme.unwrap_or("no scheme");
            Err(FetchMetadataError::InvalidScheme(scheme.to_owned()))
        }
    }?;
    Ok(hex_data)
}

async fn fetch_metadata_ws(url: &Uri) -> Result<String, FetchMetadataError> {
    let (sender, receiver) = WsTransportClientBuilder::default()
        .build(url.to_string().parse::<Uri>().unwrap())
        .await
        .map_err(|e| Error::Transport(e.into()))?;

    let client = ClientBuilder::default()
        .request_timeout(Duration::from_secs(180))
        .max_notifs_per_subscription(4096)
        .build_with_tokio(sender, receiver);

    Ok(client.request("state_getMetadata", rpc_params![]).await?)
}

async fn fetch_metadata_http(url: &Uri) -> Result<String, FetchMetadataError> {
    let client = HttpClientBuilder::default()
        .request_timeout(Duration::from_secs(180))
        .build(url.to_string())?;

    Ok(client.request("state_getMetadata", rpc_params![]).await?)
}

#[derive(Debug)]
pub enum FetchMetadataError {
    DecodeError(hex::FromHexError),
    RequestError(jsonrpsee::core::Error),
    InvalidScheme(String),
}

impl std::fmt::Display for FetchMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchMetadataError::DecodeError(e) => {
                write!(f, "Cannot decode hex value: {e}")
            }
            FetchMetadataError::RequestError(e) => write!(f, "Request error: {e}"),
            FetchMetadataError::InvalidScheme(s) => {
                write!(
                    f,
                    "'{s}' not supported, supported URI schemes are http, https, ws or wss."
                )
            }
        }
    }
}

impl std::error::Error for FetchMetadataError {}

impl From<hex::FromHexError> for FetchMetadataError {
    fn from(e: hex::FromHexError) -> Self {
        FetchMetadataError::DecodeError(e)
    }
}
impl From<jsonrpsee::core::Error> for FetchMetadataError {
    fn from(e: jsonrpsee::core::Error) -> Self {
        FetchMetadataError::RequestError(e)
    }
}
