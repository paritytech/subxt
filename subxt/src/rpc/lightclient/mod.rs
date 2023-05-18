mod background;
mod client;

pub use client::LightClient;

/// Light client error.
#[derive(Debug, thiserror::Error)]
pub enum LightClientError {
    /// Error encountered while adding the chain to the light-client.
    #[error("Failed to add the chain to the light client: {0}")]
    AddChainError(String),
    /// The background task is closed.
    #[error("Failed to communicate with the background task.")]
    BackgroundClosed,
    /// Invalid RPC parameters cannot be serialized as JSON string.
    #[error("RPC parameters cannot be serialized as JSON string.")]
    InvalidParams,
    /// Error originated while trying to submit a RPC request.
    #[error("RPC request cannot be sent: {0}")]
    Request(String),
    /// The provided URL scheme is invalid.
    ///
    /// Supported versions: WS, WSS.
    #[error("The provided URL scheme is invalid.")]
    InvalidScheme,
    /// The provided URL is invalid.
    #[error("The provided URL scheme is invalid.")]
    InvalidUrl,
    /// Handshake error while connecting to a node.
    #[error("WS handshake failed")]
    Handshake,
}
