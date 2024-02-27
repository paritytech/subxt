// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{rpc::LightClientRpc, LightClient, LightClientError};
use crate::backend::legacy::{LegacyBackend,LegacyBackendBuilder};
use crate::backend::unstable::UnstableBackendBuilder;
use crate::backend::{self, rpc::RpcClient};
use crate::client::RawLightClient;
use crate::macros::{cfg_jsonrpsee_native, cfg_jsonrpsee_web};
use crate::{config::Config, error::Error, OnlineClient};
use std::num::NonZeroU32;
use subxt_lightclient::{smoldot, AddedChain};
use std::sync::Arc;
use std::future::Future;

#[cfg(feature = "jsonrpsee")]
use crate::utils::validate_url_is_secure;

/// Builder for [`LightClient`].
#[derive(Clone, Debug)]
pub struct LightClientBuilder<T: Config> {
    max_pending_requests: NonZeroU32,
    max_subscriptions: u32,
    bootnodes: Option<Vec<serde_json::Value>>,
    potential_relay_chains: Option<Vec<smoldot::ChainId>>,
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
        potential_relay_chains: impl IntoIterator<Item = smoldot::ChainId>,
    ) -> Self {
        self.potential_relay_chains = Some(potential_relay_chains.into_iter().collect());
        self
    }

    /// Build the light client with specified URL to connect to.
    /// You must provide the port number in the URL.
    ///
    /// ## Panics
    ///
    /// The panic behaviour depends on the feature flag being used:
    ///
    /// ### Native
    ///
    /// Panics when called outside of a `tokio` runtime context.
    ///
    /// ### Web
    ///
    /// If smoldot panics, then the promise created will be leaked. For more details, see
    /// https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/fn.future_to_promise.html.
    #[cfg(feature = "jsonrpsee")]
    #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee")))]
    pub async fn build_from_url<Url: AsRef<str>>(self, url: Url) -> Result<LightClient<T>, Error> {
        validate_url_is_secure(url.as_ref())?;
        self.build_from_insecure_url(url).await
    }

    /// Build the light client with specified URL to connect to. Allows insecure URLs (no SSL, ws:// or http://).
    ///
    /// For secure connections only, please use [`crate::LightClientBuilder::build_from_url`].
    #[cfg(feature = "jsonrpsee")]
    pub async fn build_from_insecure_url<Url: AsRef<str>>(
        self,
        url: Url,
    ) -> Result<LightClient<T>, Error> {
        let chain_spec = fetch_url(url.as_ref()).await?;
        let legacy_builder = LegacyBackend::builder();
        self.build_client(legacy_builder, chain_spec).await
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
    /// The panic behaviour depends on the feature flag being used:
    ///
    /// ### Native
    ///
    /// Panics when called outside of a `tokio` runtime context.
    ///
    /// ### Web
    ///
    /// If smoldot panics, then the promise created will be leaked. For more details, see
    /// https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/fn.future_to_promise.html.
    pub async fn build(self, chain_spec: &str) -> Result<LightClient<T>, Error> {
        let chain_spec = serde_json::from_str(chain_spec)
            .map_err(|_| Error::LightClient(LightClientError::InvalidChainSpec))?;

        let legacy_builder = LegacyBackend::builder();
        self.build_client(legacy_builder, chain_spec).await
    }

    /// Construct a light client given either an [`UnstableBackendBuilder`] or [`LegacyBackendBuilder`].
    ///
    /// # Examples
    ///
    /// To use the legacy backend which relies on the legacy RPC methods (this is the default), you can do this:
    ///
    /// ```no_run
    /// # async fn example() {
    /// use subxt::backend::legacy::LegacyBackend;
    /// use subxt::client::LightClient;
    /// use subxt::config::PolkadotConfig;
    ///
    /// let legacy_builder = LegacyBackend::<PolkadotConfig>::builder();
    /// let chain_spec = "...";
    ///
    /// let client = LightClient::builder()
    ///     .build_from_backend(legacy_builder, chain_spec)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    ///
    /// To use the unstable backend which relies on the unstable RPC V2 methods, you can do this:
    ///
    /// ```no_run
    /// # async fn example() {
    /// use subxt::backend::unstable::UnstableBackend;
    /// use subxt::client::LightClient;
    /// use subxt::config::PolkadotConfig;
    ///
    /// let unstable_builder = UnstableBackend::<PolkadotConfig>::builder();
    /// let chain_spec = "...";
    ///
    /// // The "driver" is an UnstableBackendDriver, and needs polling in order to
    /// // allow the UnstableBackend, and thus client, to progress.
    /// let (client, driver) = LightClient::builder()
    ///     .build_from_backend(unstable_builder, chain_spec)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn build_from_backend<B: BackendBuilderToLightClient<T>>(self, backend_builder: B, chain_spec: &str) -> Result<B::Output, Error> {
        let chain_spec = serde_json::from_str(chain_spec)
            .map_err(|_| Error::LightClient(LightClientError::InvalidChainSpec))?;

        self.build_client(backend_builder, chain_spec).await
    }

    // Build the light client.
    async fn build_client<B: BackendBuilderToLightClient<T>>(
        self,
        backend_builder: B,
        mut chain_spec: serde_json::Value,
    ) -> Result<B::Output, Error> {
        // Set custom bootnodes if provided.
        if let Some(bootnodes) = self.bootnodes {
            if let serde_json::Value::Object(map) = &mut chain_spec {
                map.insert("bootNodes".to_string(), serde_json::Value::Array(bootnodes));
            }
        }

        let config = smoldot::AddChainConfig {
            specification: &chain_spec.to_string(),
            json_rpc: smoldot::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: self.max_pending_requests,
                max_subscriptions: self.max_subscriptions,
            },
            potential_relay_chains: self.potential_relay_chains.unwrap_or_default().into_iter(),
            database_content: "",
            user_data: (),
        };

        let raw_rpc = LightClientRpc::new(config)?;
        let chain_id = raw_rpc.chain_id();
        let rpc_client = RpcClient::new(raw_rpc);
        let light_client = backend_builder.build_light_client(chain_id, rpc_client).await?;

        Ok(light_client)
    }
}

/// Raw builder for [`RawLightClient`].
#[derive(Default)]
pub struct RawLightClientBuilder {
    chains: Vec<AddedChain>,
}

impl RawLightClientBuilder {
    /// Create a new [`RawLightClientBuilder`].
    pub fn new() -> RawLightClientBuilder {
        RawLightClientBuilder::default()
    }

    /// Adds a new chain to the list of chains synchronized by the light client.
    pub fn add_chain(
        mut self,
        chain_id: smoldot::ChainId,
        rpc_responses: smoldot::JsonRpcResponses,
    ) -> Self {
        self.chains.push(AddedChain {
            chain_id,
            rpc_responses,
        });
        self
    }

    /// Construct a [`RawLightClient`] from a raw smoldot client.
    ///
    /// The provided `chain_id` is the chain with which the current instance of light client will interact.
    /// To target a different chain call the [`LightClient::target_chain`] method.
    pub async fn build<TPlatform: smoldot::PlatformRef>(
        self,
        client: smoldot::Client<TPlatform>,
    ) -> Result<RawLightClient, Error> {
        // The raw subxt light client that spawns the smoldot background task.
        let raw_rpc: subxt_lightclient::RawLightClientRpc =
            subxt_lightclient::LightClientRpc::new_from_client(client, self.chains.into_iter());

        // The crate implementation of `RpcClientT` over the raw subxt light client.
        let raw_rpc = crate::client::light_client::rpc::RawLightClientRpc::from_inner(raw_rpc);

        Ok(RawLightClient { raw_rpc })
    }
}

/// Fetch the chain spec from the URL.
#[cfg(feature = "jsonrpsee")]
async fn fetch_url(url: impl AsRef<str>) -> Result<serde_json::Value, Error> {
    use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
    use jsonrpsee::rpc_params;
    use serde_json::value::RawValue;

    let client = jsonrpsee_helpers::client(url.as_ref()).await?;

    let result = client
        .request("sync_state_genSyncSpec", jsonrpsee::rpc_params![true])
        .await
        .map_err(|err| Error::Rpc(crate::error::RpcError::ClientError(Box::new(err))))?;

    // Subscribe to the finalized heads of the chain.
    let mut subscription = SubscriptionClientT::subscribe::<Box<RawValue>, _>(
        &client,
        "chain_subscribeFinalizedHeads",
        rpc_params![],
        "chain_unsubscribeFinalizedHeads",
    )
    .await
    .map_err(|err| Error::Rpc(crate::error::RpcError::ClientError(Box::new(err))))?;

    // We must ensure that the finalized block of the chain is not the block included
    // in the chainSpec.
    // This is a temporary workaround for: https://github.com/smol-dot/smoldot/issues/1562.
    // The first finalized block that is received might by the finalized block could be the one
    // included in the chainSpec. Decoding the chainSpec for this purpose is too complex.
    let _ = subscription.next().await;
    let _ = subscription.next().await;

    Ok(result)
}

// To instantiate a light client, we first build an RPC client that knows how to talk to
// Smoldot, and then we need a way to create a Backend which uses that RPC client. This
// trait provides a means to do this.
#[doc(hidden)]
pub trait BackendBuilderToLightClient<T>: sealed::Sealed {
    type Output;
    fn build_light_client(self, chain_id: smoldot::ChainId, client: RpcClient) -> impl Future<Output = Result<Self::Output, Error>>;
}
impl <T: Config> BackendBuilderToLightClient<T> for LegacyBackendBuilder<T> {
    type Output = LightClient<T>;
    async fn build_light_client(self, chain_id: smoldot::ChainId, client: RpcClient) -> Result<Self::Output, Error> {
        let legacy_backend = self.build(client);
        let client = OnlineClient::<T>::from_backend(Arc::new(legacy_backend)).await?;
        Ok(LightClient { client, chain_id })
    }
}
impl <T: Config> BackendBuilderToLightClient<T> for UnstableBackendBuilder<T> {
    type Output = (LightClient<T>, backend::unstable::UnstableBackendDriver<T>);
    async fn build_light_client(self, chain_id: smoldot::ChainId, client: RpcClient) -> Result<Self::Output, Error> {
        let (unstable_backend, driver) = self.build(client);
        let client = OnlineClient::<T>::from_backend(Arc::new(unstable_backend)).await?;
        Ok((LightClient { client, chain_id }, driver))
    }
}

// Prevent additional implementations of the above trait, since it's not something
mod sealed {
    pub trait Sealed {}
    impl <T> Sealed for super::LegacyBackendBuilder<T> {}
    impl <T> Sealed for super::UnstableBackendBuilder<T> {}
}

cfg_jsonrpsee_native! {
    mod jsonrpsee_helpers {
        use crate::error::{Error, LightClientError};
        use tokio_util::compat::Compat;

        pub use jsonrpsee::{
            client_transport::ws::{self, EitherStream, Url, WsTransportClientBuilder},
            core::client::Client,
        };

        pub type Sender = ws::Sender<Compat<EitherStream>>;
        pub type Receiver = ws::Receiver<Compat<EitherStream>>;

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
}

cfg_jsonrpsee_web! {
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
}
