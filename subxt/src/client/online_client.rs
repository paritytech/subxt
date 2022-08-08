// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    OfflineClient,
    OfflineClientT,
};
use crate::{
    constants::ConstantsClient,
    error::Error,
    events::EventsClient,
    rpc::{
        Rpc,
        RpcClient,
        RuntimeVersion,
    },
    storage::StorageClient,
    tx::TxClient,
    Config,
    Metadata,
};
use derivative::Derivative;
use futures::future;
use parking_lot::RwLock;
use std::sync::Arc;

/// A trait representing a client that can perform
/// online actions.
pub trait OnlineClientT<T: Config>: OfflineClientT<T> {
    /// Return an RPC client that can be used to communicate with a node.
    fn rpc(&self) -> &Rpc<T>;
}

/// A client that can be used to perform API calls (that is, either those
/// requiriing an [`OfflineClientT`] or those requiring an [`OnlineClientT`]).
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct OnlineClient<T: Config> {
    inner: Arc<RwLock<Inner<T>>>,
    rpc: Rpc<T>,
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
            .field("rpc", &"<Rpc>")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T: Config> OnlineClient<T> {
    /// Construct a new [`OnlineClient`] using default settings which
    /// point to a locally running node on `ws://127.0.0.1:9944`.
    pub async fn new() -> Result<OnlineClient<T>, Error> {
        let url = "ws://127.0.0.1:9944";
        OnlineClient::from_url(url).await
    }

    /// Construct a new [`OnlineClient`], providing a URL to connect to.
    pub async fn from_url(url: impl AsRef<str>) -> Result<OnlineClient<T>, Error> {
        let client = crate::rpc::ws_client(url.as_ref()).await?;
        OnlineClient::from_rpc_client(client).await
    }

    /// Construct a new [`OnlineClient`] by providing the underlying [`RpcClient`]
    /// to use to drive the connection.
    pub async fn from_rpc_client(
        rpc_client: impl Into<RpcClient>,
    ) -> Result<OnlineClient<T>, Error> {
        let rpc = Rpc::new(rpc_client.into());

        let (genesis_hash, runtime_version, metadata) = future::join3(
            rpc.genesis_hash(),
            rpc.runtime_version(None),
            rpc.metadata(),
        )
        .await;

        Ok(OnlineClient {
            inner: Arc::new(RwLock::new(Inner {
                genesis_hash: genesis_hash?,
                runtime_version: runtime_version?,
                metadata: metadata?,
            })),
            rpc,
        })
    }

    /// Create an object which can be used to keep the runtime uptodate
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
    /// let update_task = client.subscribe_to_updates();
    /// tokio::spawn(async move {
    ///     update_task.perform_runtime_updates().await;
    /// });
    /// # }
    /// ```
    pub fn subscribe_to_updates(&self) -> ClientRuntimeUpdater<T> {
        ClientRuntimeUpdater(self.clone())
    }

    /// Return the [`Metadata`] used in this client.
    pub fn metadata(&self) -> Metadata {
        let inner = self.inner.read();
        inner.metadata.clone()
    }

    /// Return the genesis hash.
    pub fn genesis_hash(&self) -> T::Hash {
        let inner = self.inner.read();
        inner.genesis_hash
    }

    /// Return the runtime version.
    pub fn runtime_version(&self) -> RuntimeVersion {
        let inner = self.inner.read();
        inner.runtime_version.clone()
    }

    /// Return an RPC client to make raw requests with.
    pub fn rpc(&self) -> &Rpc<T> {
        &self.rpc
    }

    /// Return an offline client with the same configuration as this.
    pub fn offline(&self) -> OfflineClient<T> {
        let inner = self.inner.read();
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
    fn rpc(&self) -> &Rpc<T> {
        &self.rpc
    }
}

/// Client wrapper for performing runtime updates. See [`OnlineClient::subscribe_to_updates()`]
/// for example usage.
pub struct ClientRuntimeUpdater<T: Config>(OnlineClient<T>);

impl<T: Config> ClientRuntimeUpdater<T> {
    fn is_runtime_version_different(&self, new: &RuntimeVersion) -> bool {
        let curr = self.0.inner.read();
        &curr.runtime_version != new
    }

    /// Performs runtime updates indefinitely unless encountering an error.
    ///
    /// *Note:* This will run indefinitely until it errors, so the typical usage
    /// would be to run it in a separate background task.
    pub async fn perform_runtime_updates(&self) -> Result<(), Error> {
        // Obtain an update subscription to further detect changes in the runtime version of the node.
        let mut update_subscription = self.0.rpc.subscribe_runtime_version().await?;

        while let Some(new_runtime_version) = update_subscription.next().await {
            // The Runtime Version obtained via subscription.
            let new_runtime_version = new_runtime_version?;

            // Ignore this update if there is no difference.
            if !self.is_runtime_version_different(&new_runtime_version) {
                continue
            }

            // Fetch new metadata.
            let new_metadata = self.0.rpc.metadata().await?;

            // Do the update.
            let mut writable = self.0.inner.write();
            writable.metadata = new_metadata;
            writable.runtime_version = new_runtime_version;
        }

        Ok(())
    }
}
