// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{rpc::LightClientRpc, LightClient, LightClientError};
use crate::backend::rpc::RpcClient;
use crate::{config::Config, error::Error, OnlineClient};
use std::num::NonZeroU32;
use subxt_lightclient::{AddChainConfig, AddChainConfigJsonRpc, ChainId};

/// Builder for [`LightClient`].
#[derive(Clone, Debug)]
pub struct LightClientBuilder<T: Config> {
    max_pending_requests: NonZeroU32,
    max_subscriptions: u32,
    bootnodes: Option<Vec<serde_json::Value>>,
    potential_relay_chains: Option<Vec<ChainId>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Config> Default for LightClientBuilder<T> {
    fn default() -> Self {
        Self {
            max_pending_requests: NonZeroU32::new(128)
                .expect("Valid number is greater than zero; qed"),
            max_subscriptions: 1024,
            bootnodes: None,
            potential_relay_chains: None,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Config> LightClientBuilder<T> {
    /// Create a new [`LightClientBuilder`].
    pub fn new() -> LightClientBuilder<T> {
        LightClientBuilder::default()
    }

    /// Overwrite the bootnodes of the chain specification.
    ///
    /// Can be used to provide trusted entities to the chain spec, or for
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
    pub async fn build_from_url<Url: AsRef<str>>(self, url: Url) -> Result<LightClient<T>, Error> {
        let chain_spec = fetch_url(url.as_ref()).await?;

        self.build_client(chain_spec).await
    }

    /// Construct a [`LightClient`] from a raw smoldot instance.
    ///
    /// # Note
    ///
    /// This ignores all the configuration options provided by the builder
    /// and uses the raw client entirely. If you are unsure about what you are doing,
    /// please use [`Self::build`] instead.
    pub async fn build_from_raw<TPlatform: subxt_lightclient::PlatformRef>(
        self,
        client: subxt_lightclient::Client<TPlatform>,
        chains: impl Iterator<Item = subxt_lightclient::JsonRpcResponses>,
        chain_id: subxt_lightclient::ChainId,
    ) -> Result<LightClient<T>, Error> {
        // The raw subxt light client that spawns the smoldot background task.
        let raw_rpc = subxt_lightclient::LightClientRpc::new_from_client(client, chains, chain_id);
        // The crate implementation of `RpcClientT` over the raw subxt light client.
        let raw_rpc = LightClientRpc::from_inner(raw_rpc);

        Self::build_client_from_rpc(raw_rpc).await
    }

    /// Build the light client from chain spec.
    ///
    /// The most important field of the configuration is the chain specification.
    /// This is a JSON document containing all the information necessary for the client to
    /// connect to said chain.
    ///
    /// The chain spec must be obtained from a trusted entity.
    ///
    /// It can be fetched from a trusted node with the following command:
    /// ```bash
    /// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "sync_state_genSyncSpec", "params":[true]}' http://localhost:9944/ | jq .result > res.spec
    /// ```
    ///
    /// # Note
    ///
    /// For testing environments, please populate the "bootNodes" if the not already provided.
    /// See [`Self::bootnodes`] for more details.
    ///
    /// ## Panics
    ///
    /// Panics if being called outside of `tokio` runtime context.
    pub async fn build(self, chain_spec: &str) -> Result<LightClient<T>, Error> {
        let chain_spec = serde_json::from_str(chain_spec)
            .map_err(|_| Error::LightClient(LightClientError::InvalidChainSpec))?;

        self.build_client(chain_spec).await
    }

    /// Build the light client.
    async fn build_client(
        self,
        mut chain_spec: serde_json::Value,
    ) -> Result<LightClient<T>, Error> {
        // Set custom bootnodes if provided.
        if let Some(bootnodes) = self.bootnodes {
            if let serde_json::Value::Object(map) = &mut chain_spec {
                map.insert("bootNodes".to_string(), serde_json::Value::Array(bootnodes));
            }
        }

        let config = AddChainConfig {
            specification: &chain_spec.to_string(),
            json_rpc: AddChainConfigJsonRpc::Enabled {
                max_pending_requests: self.max_pending_requests,
                max_subscriptions: self.max_subscriptions,
            },
            potential_relay_chains: self.potential_relay_chains.unwrap_or_default().into_iter(),
            database_content: "",
            user_data: (),
        };

        let raw_rpc = LightClientRpc::new(config)?;
        Self::build_client_from_rpc(raw_rpc).await
    }

    /// Build the light client from a raw rpc client.
    async fn build_client_from_rpc(raw_rpc: LightClientRpc) -> Result<LightClient<T>, Error> {
        let rpc_client = RpcClient::new(raw_rpc.clone());
        let client = OnlineClient::<T>::from_rpc_client(rpc_client).await?;

        Ok(LightClient { client, raw_rpc })
    }
}

/// Fetch the chain spec from the URL.
#[cfg(feature = "jsonrpsee")]
async fn fetch_url(url: impl AsRef<str>) -> Result<serde_json::Value, Error> {
    use jsonrpsee::core::client::ClientT;

    let client = jsonrpsee_helpers::client(url.as_ref()).await?;

    client
        .request("sync_state_genSyncSpec", jsonrpsee::rpc_params![true])
        .await
        .map_err(|err| Error::Rpc(crate::error::RpcError::ClientError(Box::new(err))))
}

#[cfg(all(feature = "jsonrpsee", feature = "native"))]
mod jsonrpsee_helpers {
    use crate::error::{Error, LightClientError};
    pub use jsonrpsee::{
        client_transport::ws::{Receiver, Sender, Url, WsTransportClientBuilder},
        core::client::{Client, ClientBuilder},
    };

    /// Build WS RPC client from URL
    pub async fn client(url: &str) -> Result<Client, Error> {
        let url = Url::parse(url).map_err(|_| Error::LightClient(LightClientError::InvalidUrl))?;

        if url.scheme() != "ws" && url.scheme() != "wss" {
            return Err(Error::LightClient(LightClientError::InvalidScheme));
        }

        let (sender, receiver) = ws_transport(url).await?;

        Ok(Client::builder()
            .max_buffer_capacity_per_subscription(4096)
            .build_with_tokio(sender, receiver))
    }

    async fn ws_transport(url: Url) -> Result<(Sender, Receiver), Error> {
        WsTransportClientBuilder::default()
            .build(url)
            .await
            .map_err(|_| Error::LightClient(LightClientError::Handshake))
    }
}

#[cfg(all(feature = "jsonrpsee", feature = "web"))]
mod jsonrpsee_helpers {
    use crate::error::{Error, LightClientError};
    pub use jsonrpsee::{
        client_transport::web,
        core::client::{Client, ClientBuilder},
    };

    /// Build web RPC client from URL
    pub async fn client(url: &str) -> Result<Client, Error> {
        let (sender, receiver) = web::connect(url)
            .await
            .map_err(|_| Error::LightClient(LightClientError::Handshake))?;

        Ok(ClientBuilder::default()
            .max_buffer_capacity_per_subscription(4096)
            .build_with_wasm(sender, receiver))
    }
}
