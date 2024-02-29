// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{OfflineClient, OfflineClientT};
use crate::custom_values::CustomValuesClient;
use crate::{
    backend::{
        legacy::LegacyBackend, rpc::RpcClient, Backend, BackendExt, RuntimeVersion, StreamOfResults,
    },
    blocks::{BlockRef, BlocksClient},
    constants::ConstantsClient,
    error::Error,
    events::EventsClient,
    runtime_api::RuntimeApiClient,
    storage::StorageClient,
    tx::TxClient,
    Config, Metadata,
};
use derivative::Derivative;
use futures::future;
use std::sync::{Arc, RwLock};

/// A trait representing a client that can perform
/// online actions.
pub trait OnlineClientT<T: Config>: OfflineClientT<T> {
    /// Return a backend that can be used to communicate with a node.
    fn backend(&self) -> &dyn Backend<T>;
}

/// A client that can be used to perform API calls (that is, either those
/// requiring an [`OfflineClientT`] or those requiring an [`OnlineClientT`]).
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct OnlineClient<T: Config> {
    inner: Arc<RwLock<Inner<T>>>,
    backend: Arc<dyn Backend<T>>,
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
struct Inner<T: Config> {
    genesis_hash: T::Hash,
    runtime_version: RuntimeVersion,
    metadata: Metadata,
}

impl<T: Config> std::fmt::Debug for OnlineClient<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("rpc", &"RpcClient")
            .field("inner", &self.inner)
            .finish()
    }
}

// The default constructors assume Jsonrpsee.
#[cfg(feature = "jsonrpsee")]
#[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee")))]
impl<T: Config> OnlineClient<T> {
    /// Construct a new [`OnlineClient`] using default settings which
    /// point to a locally running node on `ws://127.0.0.1:9944`.
    pub async fn new() -> Result<OnlineClient<T>, Error> {
        let url = "ws://127.0.0.1:9944";
        OnlineClient::from_url(url).await
    }

    /// Construct a new [`OnlineClient`], providing a URL to connect to.
    pub async fn from_url(url: impl AsRef<str>) -> Result<OnlineClient<T>, Error> {
        crate::utils::validate_url_is_secure(url.as_ref())?;
        OnlineClient::from_insecure_url(url).await
    }

    /// Construct a new [`OnlineClient`], providing a URL to connect to.
    ///
    /// Allows insecure URLs without SSL encryption, e.g. (http:// and ws:// URLs).
    pub async fn from_insecure_url(url: impl AsRef<str>) -> Result<OnlineClient<T>, Error> {
        let client = RpcClient::from_insecure_url(url).await?;
        let backend = LegacyBackend::builder().build(client);
        OnlineClient::from_backend(Arc::new(backend)).await
    }
}

impl<T: Config> OnlineClient<T> {
    /// Construct a new [`OnlineClient`] by providing an [`RpcClient`] to drive the connection.
    /// This will use the current default [`Backend`], which may change in future releases.
    pub async fn from_rpc_client(rpc_client: RpcClient) -> Result<OnlineClient<T>, Error> {
        let backend = Arc::new(LegacyBackend::builder().build(rpc_client));
        OnlineClient::from_backend(backend).await
    }

    /// Construct a new [`OnlineClient`] by providing an RPC client along with the other
    /// necessary details. This will use the current default [`Backend`], which may change
    /// in future releases.
    ///
    /// # Warning
    ///
    /// This is considered the most primitive and also error prone way to
    /// instantiate a client; the genesis hash, metadata and runtime version provided will
    /// entirely determine which node and blocks this client will be able to interact with,
    /// and whether it will be able to successfully do things like submit transactions.
    ///
    /// If you're unsure what you're doing, prefer one of the alternate methods to instantiate
    /// a client.
    pub fn from_rpc_client_with(
        genesis_hash: T::Hash,
        runtime_version: RuntimeVersion,
        metadata: impl Into<Metadata>,
        rpc_client: RpcClient,
    ) -> Result<OnlineClient<T>, Error> {
        let backend = Arc::new(LegacyBackend::builder().build(rpc_client));
        OnlineClient::from_backend_with(genesis_hash, runtime_version, metadata, backend)
    }

    /// Construct a new [`OnlineClient`] by providing an underlying [`Backend`]
    /// implementation to power it. Other details will be obtained from the chain.
    pub async fn from_backend<B: Backend<T>>(backend: Arc<B>) -> Result<OnlineClient<T>, Error> {
        let latest_block = backend.latest_finalized_block_ref().await?;

        let (genesis_hash, runtime_version, metadata) = future::join3(
            backend.genesis_hash(),
            backend.current_runtime_version(),
            OnlineClient::fetch_metadata(&*backend, latest_block.hash()),
        )
        .await;

        OnlineClient::from_backend_with(genesis_hash?, runtime_version?, metadata?, backend)
    }

    /// Construct a new [`OnlineClient`] by providing all of the underlying details needed
    /// to make it work.
    ///
    /// # Warning
    ///
    /// This is considered the most primitive and also error prone way to
    /// instantiate a client; the genesis hash, metadata and runtime version provided will
    /// entirely determine which node and blocks this client will be able to interact with,
    /// and whether it will be able to successfully do things like submit transactions.
    ///
    /// If you're unsure what you're doing, prefer one of the alternate methods to instantiate
    /// a client.
    pub fn from_backend_with<B: Backend<T>>(
        genesis_hash: T::Hash,
        runtime_version: RuntimeVersion,
        metadata: impl Into<Metadata>,
        backend: Arc<B>,
    ) -> Result<OnlineClient<T>, Error> {
        Ok(OnlineClient {
            inner: Arc::new(RwLock::new(Inner {
                genesis_hash,
                runtime_version,
                metadata: metadata.into(),
            })),
            backend,
        })
    }

    /// Fetch the metadata from substrate using the runtime API.
    async fn fetch_metadata(
        backend: &dyn Backend<T>,
        block_hash: T::Hash,
    ) -> Result<Metadata, Error> {
        #[cfg(feature = "unstable-metadata")]
        {
            /// The unstable metadata version number.
            const UNSTABLE_METADATA_VERSION: u32 = u32::MAX;

            // Try to fetch the latest unstable metadata, if that fails fall back to
            // fetching the latest stable metadata.
            match backend
                .metadata_at_version(UNSTABLE_METADATA_VERSION, block_hash)
                .await
            {
                Ok(bytes) => Ok(bytes),
                Err(_) => OnlineClient::fetch_latest_stable_metadata(backend, block_hash).await,
            }
        }

        #[cfg(not(feature = "unstable-metadata"))]
        OnlineClient::fetch_latest_stable_metadata(backend, block_hash).await
    }

    /// Fetch the latest stable metadata from the node.
    async fn fetch_latest_stable_metadata(
        backend: &dyn Backend<T>,
        block_hash: T::Hash,
    ) -> Result<Metadata, Error> {
        // This is the latest stable metadata that subxt can utilize.
        const V15_METADATA_VERSION: u32 = 15;

        // Try to fetch the metadata version.
        if let Ok(bytes) = backend
            .metadata_at_version(V15_METADATA_VERSION, block_hash)
            .await
        {
            return Ok(bytes);
        }

        // If that fails, fetch the metadata V14 using the old API.
        backend.legacy_metadata(block_hash).await
    }

    /// Create an object which can be used to keep the runtime up to date
    /// in a separate thread.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// use subxt::{ OnlineClient, PolkadotConfig };
    ///
    /// let client = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    ///
    /// // high level API.
    ///
    /// let update_task = client.updater();
    /// tokio::spawn(async move {
    ///     update_task.perform_runtime_updates().await;
    /// });
    ///
    ///
    /// // low level API.
    ///
    /// let updater = client.updater();
    /// tokio::spawn(async move {
    ///     let mut update_stream = updater.runtime_updates().await.unwrap();
    ///
    ///     while let Some(Ok(update)) = update_stream.next().await {
    ///         let version = update.runtime_version().spec_version;
    ///
    ///         match updater.apply_update(update) {
    ///             Ok(()) => {
    ///                 println!("Upgrade to version: {} successful", version)
    ///             }
    ///             Err(e) => {
    ///                println!("Upgrade to version {} failed {:?}", version, e);
    ///             }
    ///        };
    ///     }
    /// });
    /// # }
    /// ```
    pub fn updater(&self) -> ClientRuntimeUpdater<T> {
        ClientRuntimeUpdater(self.clone())
    }

    /// Return the [`Metadata`] used in this client.
    pub fn metadata(&self) -> Metadata {
        let inner = self.inner.read().expect("shouldn't be poisoned");
        inner.metadata.clone()
    }

    /// Change the [`Metadata`] used in this client.
    ///
    /// # Warning
    ///
    /// Setting custom metadata may leave Subxt unable to work with certain blocks,
    /// subscribe to latest blocks or submit valid transactions.
    pub fn set_metadata(&self, metadata: impl Into<Metadata>) {
        let mut inner = self.inner.write().expect("shouldn't be poisoned");
        inner.metadata = metadata.into();
    }

    /// Return the genesis hash.
    pub fn genesis_hash(&self) -> T::Hash {
        let inner = self.inner.read().expect("shouldn't be poisoned");
        inner.genesis_hash
    }

    /// Change the genesis hash used in this client.
    ///
    /// # Warning
    ///
    /// Setting a custom genesis hash may leave Subxt unable to
    /// submit valid transactions.
    pub fn set_genesis_hash(&self, genesis_hash: T::Hash) {
        let mut inner = self.inner.write().expect("shouldn't be poisoned");
        inner.genesis_hash = genesis_hash;
    }

    /// Return the runtime version.
    pub fn runtime_version(&self) -> RuntimeVersion {
        let inner = self.inner.read().expect("shouldn't be poisoned");
        inner.runtime_version.clone()
    }

    /// Change the [`RuntimeVersion`] used in this client.
    ///
    /// # Warning
    ///
    /// Setting a custom runtime version may leave Subxt unable to
    /// submit valid transactions.
    pub fn set_runtime_version(&self, runtime_version: RuntimeVersion) {
        let mut inner = self.inner.write().expect("shouldn't be poisoned");
        inner.runtime_version = runtime_version;
    }

    /// Return an RPC client to make raw requests with.
    pub fn backend(&self) -> &dyn Backend<T> {
        &*self.backend
    }

    /// Return an offline client with the same configuration as this.
    pub fn offline(&self) -> OfflineClient<T> {
        let inner = self.inner.read().expect("shouldn't be poisoned");
        OfflineClient::new(
            inner.genesis_hash,
            inner.runtime_version.clone(),
            inner.metadata.clone(),
        )
    }

    // Just a copy of the most important trait methods so that people
    // don't need to import the trait for most things:

    /// Work with transactions.
    pub fn tx(&self) -> TxClient<T, Self> {
        <Self as OfflineClientT<T>>::tx(self)
    }

    /// Work with events.
    pub fn events(&self) -> EventsClient<T, Self> {
        <Self as OfflineClientT<T>>::events(self)
    }

    /// Work with storage.
    pub fn storage(&self) -> StorageClient<T, Self> {
        <Self as OfflineClientT<T>>::storage(self)
    }

    /// Access constants.
    pub fn constants(&self) -> ConstantsClient<T, Self> {
        <Self as OfflineClientT<T>>::constants(self)
    }

    /// Access custom types.
    pub fn custom_values(&self) -> CustomValuesClient<T, Self> {
        <Self as OfflineClientT<T>>::custom_values(self)
    }

    /// Work with blocks.
    pub fn blocks(&self) -> BlocksClient<T, Self> {
        <Self as OfflineClientT<T>>::blocks(self)
    }

    /// Work with runtime API.
    pub fn runtime_api(&self) -> RuntimeApiClient<T, Self> {
        <Self as OfflineClientT<T>>::runtime_api(self)
    }
}

impl<T: Config> OfflineClientT<T> for OnlineClient<T> {
    fn metadata(&self) -> Metadata {
        self.metadata()
    }
    fn genesis_hash(&self) -> T::Hash {
        self.genesis_hash()
    }
    fn runtime_version(&self) -> RuntimeVersion {
        self.runtime_version()
    }
}

impl<T: Config> OnlineClientT<T> for OnlineClient<T> {
    fn backend(&self) -> &dyn Backend<T> {
        &*self.backend
    }
}

/// Client wrapper for performing runtime updates. See [`OnlineClient::updater()`]
/// for example usage.
pub struct ClientRuntimeUpdater<T: Config>(OnlineClient<T>);

impl<T: Config> ClientRuntimeUpdater<T> {
    fn is_runtime_version_different(&self, new: &RuntimeVersion) -> bool {
        let curr = self.0.inner.read().expect("shouldn't be poisoned");
        &curr.runtime_version != new
    }

    fn do_update(&self, update: Update) {
        let mut writable = self.0.inner.write().expect("shouldn't be poisoned");
        writable.metadata = update.metadata;
        writable.runtime_version = update.runtime_version;
    }

    /// Tries to apply a new update.
    pub fn apply_update(&self, update: Update) -> Result<(), UpgradeError> {
        if !self.is_runtime_version_different(&update.runtime_version) {
            return Err(UpgradeError::SameVersion);
        }

        self.do_update(update);

        Ok(())
    }

    /// Performs runtime updates indefinitely unless encountering an error.
    ///
    /// *Note:* This will run indefinitely until it errors, so the typical usage
    /// would be to run it in a separate background task.
    pub async fn perform_runtime_updates(&self) -> Result<(), Error> {
        // Obtain an update subscription to further detect changes in the runtime version of the node.
        let mut runtime_version_stream = self.runtime_updates().await?;

        while let Some(update) = runtime_version_stream.next().await {
            let update = update?;

            // This only fails if received the runtime version is the same the current runtime version
            // which might occur because that runtime subscriptions in substrate sends out the initial
            // value when they created and not only when runtime upgrades occurs.
            // Thus, fine to ignore here as it strictly speaking isn't really an error
            let _ = self.apply_update(update);
        }

        Ok(())
    }

    /// Low-level API to get runtime updates as a stream but it's doesn't check if the
    /// runtime version is newer or updates the runtime.
    ///
    /// Instead that's up to the user of this API to decide when to update and
    /// to perform the actual updating.
    pub async fn runtime_updates(&self) -> Result<RuntimeUpdaterStream<T>, Error> {
        let stream = self.0.backend().stream_runtime_version().await?;
        Ok(RuntimeUpdaterStream {
            stream,
            client: self.0.clone(),
        })
    }
}

/// Stream to perform runtime upgrades.
pub struct RuntimeUpdaterStream<T: Config> {
    stream: StreamOfResults<RuntimeVersion>,
    client: OnlineClient<T>,
}

impl<T: Config> RuntimeUpdaterStream<T> {
    /// Wait for the next runtime update.
    pub async fn next(&mut self) -> Option<Result<Update, Error>> {
        let runtime_version = match self.stream.next().await? {
            Ok(runtime_version) => runtime_version,
            Err(err) => return Some(Err(err)),
        };

        let at =
            match wait_runtime_upgrade_in_finalized_block(&self.client, &runtime_version).await? {
                Ok(at) => at,
                Err(err) => return Some(Err(err)),
            };

        let metadata = match OnlineClient::fetch_metadata(self.client.backend(), at.hash()).await {
            Ok(metadata) => metadata,
            Err(err) => return Some(Err(err)),
        };

        Some(Ok(Update {
            metadata,
            runtime_version,
        }))
    }
}

/// Error that can occur during upgrade.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum UpgradeError {
    /// The version is the same as the current version.
    SameVersion,
}

/// Represents the state when a runtime upgrade occurred.
pub struct Update {
    runtime_version: RuntimeVersion,
    metadata: Metadata,
}

impl Update {
    /// Get the runtime version.
    pub fn runtime_version(&self) -> &RuntimeVersion {
        &self.runtime_version
    }

    /// Get the metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// Helper to wait until the runtime upgrade is applied on at finalized block.
async fn wait_runtime_upgrade_in_finalized_block<T: Config>(
    client: &OnlineClient<T>,
    runtime_version: &RuntimeVersion,
) -> Option<Result<BlockRef<T::Hash>, Error>> {
    use scale_value::At;

    let mut block_sub = match client.backend().stream_finalized_block_headers().await {
        Ok(s) => s,
        Err(err) => return Some(Err(err)),
    };

    let block_ref = loop {
        let (_, block_ref) = match block_sub.next().await? {
            Ok(n) => n,
            Err(err) => return Some(Err(err)),
        };

        let key: Vec<scale_value::Value> = vec![];
        let addr = crate::dynamic::storage("System", "LastRuntimeUpgrade", key);

        let chunk = match client.storage().at(block_ref.hash()).fetch(&addr).await {
            Ok(Some(v)) => v,
            Ok(None) => {
                // The storage `system::lastRuntimeUpgrade` should always exist.
                // <https://github.com/paritytech/polkadot-sdk/blob/master/substrate/frame/system/src/lib.rs#L958>
                unreachable!("The storage item `system::lastRuntimeUpgrade` should always exist")
            }
            Err(e) => return Some(Err(e)),
        };

        let scale_val = match chunk.to_value() {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };

        let Some(Ok(spec_version)) = scale_val
            .at("spec_version")
            .and_then(|v| v.as_u128())
            .map(u32::try_from)
        else {
            return Some(Err(Error::Other(
                "Decoding `RuntimeVersion::spec_version` as u32 failed".to_string(),
            )));
        };

        // We are waiting for the chain to have the same spec version
        // as sent out via the runtime subscription.
        if spec_version == runtime_version.spec_version {
            break block_ref;
        }
    };

    Some(Ok(block_ref))
}
