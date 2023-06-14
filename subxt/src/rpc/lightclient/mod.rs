mod background;
mod client;

use crate::error::{Error, RpcError};
use core::time::Duration;
#[cfg(feature = "jsonrpsee-ws")]
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
    /// Handshake error while connecting to a node.
    #[error("WS handshake failed.")]
    Handshake,
    /// The configuration provided to the builder is compatible.
    ///
    /// The light client can be constructed with either a chain config
    /// or an URL to a trusted node.
    #[error("The configuration provided to the builder is compatible. Please provide a chain spec or URL.")]
    IncompatibleConfig,
}

/// Builder for [`LightClient`].
#[derive(Clone, Debug)]
pub struct LightClientBuilder<'a> {
    chain_spec: Option<&'a str>,
    max_pending_requests: NonZeroU32,
    max_subscriptions: u32,
    database_content: Option<&'a str>,
    url: Option<&'a str>,
    bootnodes: Option<Vec<String>>,
    potential_relay_chains: Option<Vec<ChainId>>,
}

impl<'a> Default for LightClientBuilder<'a> {
    fn default() -> Self {
        Self {
            chain_spec: None,
            max_pending_requests: NonZeroU32::new(128)
                .expect("Valid number is greater than zero; qed"),
            max_subscriptions: 1024,
            database_content: None,
            url: None,
            bootnodes: None,
            potential_relay_chains: None,
        }
    }
}

impl<'a> LightClientBuilder<'a> {
    /// Create a new [`LightClientBuilder`].
    pub fn new() -> LightClientBuilder<'a> {
        LightClientBuilder::default()
    }

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
    ///
    /// ```json
    ///   "bootNodes": [
    ///       "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
    ///    ],
    /// ```
    pub fn chain_spec(mut self, chain_spec: &'a str) -> Self {
        self.chain_spec = Some(chain_spec);
        self
    }

    /// Overwrite the bootnodes of the chain specification.
    ///
    /// Can be used to provide trusted entities to the chian spec, or for
    /// testing environments.
    pub fn bootnodes(mut self, bootnodes: impl Iterator<Item = &'a str>) -> Self {
        self.bootnodes = Some(bootnodes.map(Into::into).collect());
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

    /// After a chain has been added, it is possible to extract a "database" (in the form of a
    /// simple string). This database can later be passed back the next time the same chain is
    /// added again.
    /// A database with an invalid format is simply ignored by the client.
    pub fn database_content(mut self, database_content: &'a str) -> Self {
        self.database_content = Some(database_content);
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
        potential_relay_chains: impl Iterator<Item = ChainId>,
    ) -> Self {
        self.potential_relay_chains = Some(potential_relay_chains.collect());
        self
    }

    /// The URL of a trusted node from which the chain spec is fetched.
    ///
    /// # Note
    ///
    /// Incompatible with [`Self::chain_spec`].
    #[cfg(feature = "jsonrpsee-ws")]
    pub fn trusted_url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    /// Build the light client with specified URL to connect to.
    /// You must provide the port number in the URL.
    ///
    /// ## Panics
    ///
    /// Panics if being called outside of `tokio` runtime context.
    pub async fn build(self) -> Result<LightClient, Error> {
        let mut chain_spec = match (self.chain_spec, self.url) {
            (Some(chain_spec), None) => serde_json::from_str(chain_spec).unwrap(),
            (None, Some(url)) => fetch_url(url).await?,
            _ => return Err(Error::LightClient(LightClientError::IncompatibleConfig)),
        };

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
            database_content: self.database_content.unwrap_or_default(),
            user_data: (),
        };

        LightClient::new(config)
    }
}

/// Fetch the chain spec from the URL.
#[cfg(feature = "jsonrpsee-ws")]
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
        .request_timeout(Duration::from_secs(180))
        .max_notifs_per_subscription(4096)
        .build_with_tokio(sender, receiver);

    client
        .request("sync_state_genSyncSpec", rpc_params![true])
        .await
        .map_err(|err| Error::Rpc(RpcError::ClientError(Box::new(err))))
}
