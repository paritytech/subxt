mod background;
mod client;

use crate::error::Error;
#[cfg(feature = "jsonrpsee")]
use jsonrpsee::{
    async_client::ClientBuilder,
    client_transport::ws::{Uri, WsTransportClientBuilder},
    core::client::ClientT,
    rpc_params,
};
use smoldot_light::ChainId;
use std::num::NonZeroU32;

pub use client::LightClient;

/// Light client error.
#[derive(Debug, thiserror::Error)]
pub enum LightClientError {
    /// Error encountered while adding the chain to the light-client.
    #[error("Failed to add the chain to the light client: {0}.")]
    AddChainError(String),
    /// The background task is closed.
    #[error("Failed to communicate with the background task.")]
    BackgroundClosed,
    /// Invalid RPC parameters cannot be serialized as JSON string.
    #[error("RPC parameters cannot be serialized as JSON string.")]
    InvalidParams,
    /// Error originated while trying to submit a RPC request.
    #[error("RPC request cannot be sent: {0}.")]
    Request(String),
    /// The provided URL scheme is invalid.
    ///
    /// Supported versions: WS, WSS.
    #[error("The provided URL scheme is invalid.")]
    InvalidScheme,
    /// The provided URL is invalid.
    #[error("The provided URL scheme is invalid.")]
    InvalidUrl,
    /// The provided chain spec is invalid.
    #[error("The provided chain spec is not a valid JSON object.")]
    InvalidChainSpec,
    /// Handshake error while connecting to a node.
    #[error("WS handshake failed.")]
    Handshake,
}

/// Builder for [`LightClient`].
#[derive(Clone, Debug)]
pub struct LightClientBuilder {
    max_pending_requests: NonZeroU32,
    max_subscriptions: u32,
    bootnodes: Option<Vec<String>>,
    potential_relay_chains: Option<Vec<ChainId>>,
}

impl Default for LightClientBuilder {
    fn default() -> Self {
        Self {
            max_pending_requests: NonZeroU32::new(128)
                .expect("Valid number is greater than zero; qed"),
            max_subscriptions: 1024,
            bootnodes: None,
            potential_relay_chains: None,
        }
    }
}

impl LightClientBuilder {
    /// Create a new [`LightClientBuilder`].
    pub fn new() -> LightClientBuilder {
        LightClientBuilder::default()
    }

    /// Overwrite the bootnodes of the chain specification.
    ///
    /// Can be used to provide trusted entities to the chian spec, or for
    /// testing environments.
    pub fn bootnodes<'a>(mut self, bootnodes: impl IntoIterator<Item = &'a str>) -> Self {
        self.bootnodes = Some(bootnodes.into_iter().map(Into::into).collect());
        self
    }

    /// Maximum number of JSON-RPC in the queue of requests waiting to be processed.
    /// This parameter is necessary for situations where the JSON-RPC clients aren't
    /// trusted. If you control all the requests that are sent out and don't want them
    /// to fail, feel free to pass `u32::max_value()`.
    ///
    /// Default is 128.
    pub fn max_pending_requests(mut self, max_pending_requests: NonZeroU32) -> Self {
        self.max_pending_requests = max_pending_requests;
        self
    }

    /// Maximum number of active subscriptions before new ones are automatically
    /// rejected. Any JSON-RPC request that causes the server to generate notifications
    /// counts as a subscription.
    ///
    /// Default is 1024.
    pub fn max_subscriptions(mut self, max_subscriptions: u32) -> Self {
        self.max_subscriptions = max_subscriptions;
        self
    }

    /// If the chain spec defines a parachain, contains the list of relay chains to choose
    /// from. Ignored if not a parachain.
    ///
    /// This field is necessary because multiple different chain can have the same identity.
    ///
    /// For example: if user A adds a chain named "Kusama", then user B adds a different chain
    /// also named "Kusama", then user B adds a parachain whose relay chain is "Kusama", it would
    /// be wrong to connect to the "Kusama" created by user A.
    pub fn potential_relay_chains(
        mut self,
        potential_relay_chains: impl IntoIterator<Item = ChainId>,
    ) -> Self {
        self.potential_relay_chains = Some(potential_relay_chains.into_iter().collect());
        self
    }

    /// Build the light client with specified URL to connect to.
    /// You must provide the port number in the URL.
    ///
    /// ## Panics
    ///
    /// Panics if being called outside of `tokio` runtime context.
    #[cfg(feature = "jsonrpsee")]
    pub async fn build_from_url<Url: AsRef<str>>(self, url: Url) -> Result<LightClient, Error> {
        let chain_spec = fetch_url(url.as_ref()).await?;

        self.build_client(chain_spec).await
    }

    /// Build the light client from chain spec.
    ///
    /// The most important field of the configuration is the chain specification.
    /// This is a JSON document containing all the information necessary for the client to
    /// connect to said chain.
    ///
    /// The chain spec must be obtained from a trusted entity.
    ///
    /// It can be fetched from a trused node with the following command:
    /// ```bash
    /// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "sync_state_genSyncSpec", "params":[true]}' http://localhost:9944/ | jq .result > res.spec
    /// ```
    ///
    /// # Note
    ///
    /// For testing environments, please populate the "bootNodes" if the not already provided.
    /// See [`Self::bootnodes`] for more details.
    ///
    /// ```json
    ///   "bootNodes": [
    ///       "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
    ///    ],
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if being called outside of `tokio` runtime context.
    pub async fn build(self, chain_spec: &str) -> Result<LightClient, Error> {
        let chain_spec = serde_json::from_str(chain_spec)
            .map_err(|_| Error::LightClient(LightClientError::InvalidChainSpec))?;

        self.build_client(chain_spec).await
    }

    /// Build the light client.
    async fn build_client(self, mut chain_spec: serde_json::Value) -> Result<LightClient, Error> {
        // Set custom bootnodes if provided.
        if let Some(bootnodes) = self.bootnodes {
            let bootnodes = bootnodes
                .into_iter()
                .map(serde_json::Value::String)
                .collect();
            if let serde_json::Value::Object(map) = &mut chain_spec {
                map.insert("bootNodes".to_string(), serde_json::Value::Array(bootnodes));
            }
        }

        let config = smoldot_light::AddChainConfig {
            specification: &chain_spec.to_string(),
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: self.max_pending_requests,
                max_subscriptions: self.max_subscriptions,
            },
            potential_relay_chains: self.potential_relay_chains.unwrap_or_default().into_iter(),
            database_content: "",
            user_data: (),
        };

        LightClient::new(config)
    }
}

/// Fetch the chain spec from the URL.
#[cfg(feature = "jsonrpsee")]
async fn fetch_url(url: impl AsRef<str>) -> Result<serde_json::Value, Error> {
    let url = url
        .as_ref()
        .parse::<Uri>()
        .map_err(|_| Error::LightClient(LightClientError::InvalidUrl))?;

    if url.scheme_str() != Some("ws") && url.scheme_str() != Some("wss") {
        return Err(Error::LightClient(LightClientError::InvalidScheme));
    }

    let (sender, receiver) = WsTransportClientBuilder::default()
        .build(url)
        .await
        .map_err(|_| LightClientError::Handshake)?;

    let client = ClientBuilder::default()
        .request_timeout(core::time::Duration::from_secs(180))
        .max_notifs_per_subscription(4096)
        .build_with_tokio(sender, receiver);

    client
        .request("sync_state_genSyncSpec", rpc_params![true])
        .await
        .map_err(|err| Error::Rpc(crate::error::RpcError::ClientError(Box::new(err))))
}
