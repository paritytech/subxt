// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a backend trait for Subxt which allows us to get and set
//! the necessary information (probably from a JSON-RPC API, but that's up to the
//! implementation).

pub mod legacy;
pub mod rpc;
pub mod unstable;
pub mod utils;

use subxt_core::client::RuntimeVersion;

use crate::error::Error;
use crate::metadata::Metadata;
use crate::Config;
use async_trait::async_trait;
use codec::{Decode, Encode};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;

/// Prevent the backend trait being implemented externally.
#[doc(hidden)]
pub(crate) mod sealed {
    pub trait Sealed {}
}

/// This trait exposes the interface that Subxt will use to communicate with
/// a backend. Its goal is to be as minimal as possible.
#[async_trait]
pub trait Backend<T: Config>: sealed::Sealed + Send + Sync + 'static {
    /// Fetch values from storage.
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: T::Hash,
    ) -> Result<StreamOfResults<StorageResponse>, Error>;

    /// Fetch keys underneath the given key from storage.
    async fn storage_fetch_descendant_keys(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<StreamOfResults<Vec<u8>>, Error>;

    /// Fetch values underneath the given key from storage.
    async fn storage_fetch_descendant_values(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<StreamOfResults<StorageResponse>, Error>;

    /// Fetch the genesis hash
    async fn genesis_hash(&self) -> Result<T::Hash, Error>;

    /// Get a block header
    async fn block_header(&self, at: T::Hash) -> Result<Option<T::Header>, Error>;

    /// Return the extrinsics found in the block. Each extrinsic is represented
    /// by a vector of bytes which has _not_ been SCALE decoded (in other words, the
    /// first bytes in the vector will decode to the compact encoded length of the extrinsic)
    async fn block_body(&self, at: T::Hash) -> Result<Option<Vec<Vec<u8>>>, Error>;

    /// Get the most recent finalized block hash.
    /// Note: needed only in blocks client for finalized block stream; can prolly be removed.
    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<T::Hash>, Error>;

    /// Get information about the current runtime.
    async fn current_runtime_version(&self) -> Result<RuntimeVersion, Error>;

    /// A stream of all new runtime versions as they occur.
    async fn stream_runtime_version(&self) -> Result<StreamOfResults<RuntimeVersion>, Error>;

    /// A stream of all new block headers as they arrive.
    async fn stream_all_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error>;

    /// A stream of best block headers.
    async fn stream_best_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error>;

    /// A stream of finalized block headers.
    async fn stream_finalized_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error>;

    /// Submit a transaction. This will return a stream of events about it.
    async fn submit_transaction(
        &self,
        bytes: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<T::Hash>>, Error>;

    /// Make a call to some runtime API.
    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: T::Hash,
    ) -> Result<Vec<u8>, Error>;
}

/// helpful utility methods derived from those provided on [`Backend`]
#[async_trait]
pub trait BackendExt<T: Config>: Backend<T> {
    /// Fetch a single value from storage.
    async fn storage_fetch_value(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<Option<Vec<u8>>, Error> {
        self.storage_fetch_values(vec![key], at)
            .await?
            .next()
            .await
            .transpose()
            .map(|o| o.map(|s| s.value))
    }

    /// The same as a [`Backend::call()`], but it will also attempt to decode the
    /// result into the given type, which is a fairly common operation.
    async fn call_decoding<D: codec::Decode>(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: T::Hash,
    ) -> Result<D, Error> {
        let bytes = self.call(method, call_parameters, at).await?;
        let res = D::decode(&mut &*bytes)?;
        Ok(res)
    }

    /// Return the metadata at some version.
    async fn metadata_at_version(&self, version: u32, at: T::Hash) -> Result<Metadata, Error> {
        let param = version.encode();

        let opaque: Option<frame_metadata::OpaqueMetadata> = self
            .call_decoding("Metadata_metadata_at_version", Some(&param), at)
            .await?;
        let Some(opaque) = opaque else {
            return Err(Error::Other("Metadata version not found".into()));
        };

        let metadata: Metadata = Decode::decode(&mut &opaque.0[..])?;
        Ok(metadata)
    }

    /// Return V14 metadata from the legacy `Metadata_metadata` call.
    async fn legacy_metadata(&self, at: T::Hash) -> Result<Metadata, Error> {
        let opaque: frame_metadata::OpaqueMetadata =
            self.call_decoding("Metadata_metadata", None, at).await?;
        let metadata: Metadata = Decode::decode(&mut &opaque.0[..])?;
        Ok(metadata)
    }
}

#[async_trait]
impl<B: Backend<T> + ?Sized, T: Config> BackendExt<T> for B {}

/// An opaque struct which, while alive, indicates that some references to a block
/// still exist. This gives the backend the opportunity to keep the corresponding block
/// details around for a while if it likes and is able to. No guarantees can be made about
/// how long the corresponding details might be available for, but if no references to a block
/// exist, then the backend is free to discard any details for it.
#[derive(Clone)]
pub struct BlockRef<H> {
    hash: H,
    // We keep this around so that when it is dropped, it has the
    // opportunity to tell the backend.
    _pointer: Option<Arc<dyn BlockRefT>>,
}

impl<H> From<H> for BlockRef<H> {
    fn from(value: H) -> Self {
        BlockRef::from_hash(value)
    }
}

impl<H: PartialEq> PartialEq for BlockRef<H> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
impl<H: Eq> Eq for BlockRef<H> {}

// Manual implementation to work around https://github.com/mcarton/rust-derivative/issues/115.
impl<H: PartialOrd> PartialOrd for BlockRef<H> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl<H: Ord> Ord for BlockRef<H> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}

impl<H: std::fmt::Debug> std::fmt::Debug for BlockRef<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BlockRef").field(&self.hash).finish()
    }
}

impl<H: std::hash::Hash> std::hash::Hash for BlockRef<H> {
    fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
        self.hash.hash(state);
    }
}

impl<H> BlockRef<H> {
    /// A [`BlockRef`] that doesn't reference a given block, but does have an associated hash.
    /// This is used in the legacy backend, which has no notion of pinning blocks.
    pub fn from_hash(hash: H) -> Self {
        Self {
            hash,
            _pointer: None,
        }
    }
    /// Construct a [`BlockRef`] from an instance of the underlying trait. It's expected
    /// that the [`Backend`] implementation will call this if it wants to track which blocks
    /// are potentially in use.
    pub fn new<P: BlockRefT>(hash: H, inner: P) -> Self {
        Self {
            hash,
            _pointer: Some(Arc::new(inner)),
        }
    }

    /// Return the hash of the referenced block.
    pub fn hash(&self) -> H
    where
        H: Copy,
    {
        self.hash
    }
}

/// A trait that a [`Backend`] can implement to know when some block
/// can be unpinned: when this is dropped, there are no remaining references
/// to the block that it's associated with.
pub trait BlockRefT: Send + Sync + 'static {}

/// A stream of some item.
pub struct StreamOf<T>(Pin<Box<dyn Stream<Item = T> + Send + 'static>>);

impl<T> Stream for StreamOf<T> {
    type Item = T;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.0.poll_next_unpin(cx)
    }
}

impl<T> std::fmt::Debug for StreamOf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StreamOf").field(&"<stream>").finish()
    }
}

impl<T> StreamOf<T> {
    /// Construct a new stream.
    pub fn new(inner: Pin<Box<dyn Stream<Item = T> + Send + 'static>>) -> Self {
        StreamOf(inner)
    }

    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<T> {
        StreamExt::next(self).await
    }
}

/// A stream of [`Result<Item, Error>`].
pub type StreamOfResults<T> = StreamOf<Result<T, Error>>;

/// The status of the transaction.
///
/// If the status is [`TransactionStatus::InFinalizedBlock`], [`TransactionStatus::Error`],
/// [`TransactionStatus::Invalid`] or [`TransactionStatus::Dropped`], then no future
/// events will be emitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionStatus<Hash> {
    /// Transaction is part of the future queue.
    Validated,
    /// The transaction has been broadcast to other nodes.
    Broadcasted {
        /// Number of peers it's been broadcast to.
        num_peers: u32,
    },
    /// Transaction is no longer in a best block.
    NoLongerInBestBlock,
    /// Transaction has been included in block with given hash.
    InBestBlock {
        /// Block hash the transaction is in.
        hash: BlockRef<Hash>,
    },
    /// Transaction has been finalized by a finality-gadget, e.g GRANDPA
    InFinalizedBlock {
        /// Block hash the transaction is in.
        hash: BlockRef<Hash>,
    },
    /// Something went wrong in the node.
    Error {
        /// Human readable message; what went wrong.
        message: String,
    },
    /// Transaction is invalid (bad nonce, signature etc).
    Invalid {
        /// Human readable message; why was it invalid.
        message: String,
    },
    /// The transaction was dropped.
    Dropped {
        /// Human readable message; why was it dropped.
        message: String,
    },
}

/// A response from calls like [`Backend::storage_fetch_values`] or
/// [`Backend::storage_fetch_descendant_values`].
#[cfg_attr(test, derive(serde::Serialize, Clone, PartialEq, Debug))]
pub struct StorageResponse {
    /// The key.
    pub key: Vec<u8>,
    /// The associated value.
    pub value: Vec<u8>,
}

#[cfg(test)]
mod test {
    use super::*;
    pub use crate::backend::rpc::{RawRpcFuture, RawRpcSubscription};
    pub use crate::{backend::StorageResponse, error::RpcError};
    pub use futures::StreamExt;
    pub use rpc::RpcClientT;
    pub use serde::Serialize;
    pub use serde_json::value::RawValue;
    pub use sp_core::H256;
    pub use std::collections::{HashMap, VecDeque};
    pub use subxt_core::{config::DefaultExtrinsicParams, Config};
    pub use tokio::sync::{mpsc, Mutex};

    pub type RpcResult<T> = Result<T, RpcError>;
    pub type Item = RpcResult<String>;

    fn random_hash() -> H256 {
        H256::random()
    }

    fn storage_response<K: Into<Vec<u8>>, V: Into<Vec<u8>>>(key: K, value: V) -> StorageResponse
    where
        Vec<u8>: From<K>,
    {
        StorageResponse {
            key: key.into(),
            value: value.into(),
        }
    }
    pub mod rpc_client {
        use super::*;
        use std::time::Duration;

        pub type SubscriptionHandler = Box<
            dyn for<'a> Fn(
                    &'a mut MockDataTable,
                    &'a mut Option<Subscription>,
                    Option<Box<serde_json::value::RawValue>>,
                ) -> RawRpcFuture<'a, RawRpcSubscription>
                + Send,
        >;

        pub type MethodHandler = Box<
            dyn for<'a> Fn(
                    &'a mut MockDataTable,
                    &'a mut Option<Subscription>,
                    Option<Box<serde_json::value::RawValue>>,
                ) -> RawRpcFuture<'a, Box<serde_json::value::RawValue>>
                + Send,
        >;

        pub enum Message<T> {
            Many(RpcResult<Vec<T>>),
            Single(T),
        }

        impl<T> Message<T> {
            pub fn unwrap_single(self) -> T {
                match self {
                    Self::Single(s) => s,
                    _ => panic!("cannot unwrap_single on Message::Many"),
                }
            }
            pub fn unwrap_many(self) -> RpcResult<Vec<T>> {
                match self {
                    Self::Many(s) => s,
                    _ => panic!("cannot unwrap_many on Message::Single"),
                }
            }
        }

        #[derive(Default)]
        pub struct MockDataTable {
            items: HashMap<Vec<u8>, VecDeque<Message<Item>>>,
        }

        impl MockDataTable {
            pub fn push<I: Serialize>(&mut self, key: Vec<u8>, item: Message<RpcResult<I>>) {
                let item = match item {
                    Message::Many(items) => Message::Many(items.map(|items| {
                        items
                            .into_iter()
                            .map(|item| item.map(|x| serde_json::to_string(&x).unwrap()))
                            .collect()
                    })),
                    Message::Single(item) => {
                        Message::Single(item.map(|x| serde_json::to_string(&x).unwrap()))
                    }
                };
                self.items.entry(key).or_default().push_back(item);
            }

            pub fn pop(&mut self, key: Vec<u8>) -> Message<Item> {
                self.items.get_mut(&key).unwrap().pop_front().unwrap()
            }
        }

        pub struct Subscription {
            sender: mpsc::Sender<Item>,
        }

        impl Subscription {
            pub fn new() -> (Self, mpsc::Receiver<Item>) {
                let (sender, receiver) = mpsc::channel(32);
                (Self { sender }, receiver)
            }

            pub async fn write(&self, items: Message<Item>) {
                match items {
                    Message::Many(items) => {
                        for i in items.unwrap() {
                            self.sender.send(i).await.unwrap()
                        }
                    }
                    Message::Single(item) => self.sender.send(item).await.unwrap(),
                };
            }

            pub async fn write_delayed(&self, items: Message<Item>) {
                let sender = self.sender.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(500)).await;

                    match items {
                        Message::Many(items) => {
                            for i in items.unwrap() {
                                let _ = sender.send(i).await;
                            }
                        }
                        Message::Single(item) => sender.send(item).await.unwrap(),
                    };
                });
            }
        }

        #[derive(Default)]
        struct InnerMockedRpcClient {
            data_table: MockDataTable,
            subscription_channel: Option<Subscription>,
            subscription_handlers: HashMap<String, SubscriptionHandler>,
            method_handlers: HashMap<String, MethodHandler>,
        }

        impl InnerMockedRpcClient {
            fn call<'a>(
                &'a mut self,
                method_handler: &str,
                params: Option<Box<serde_json::value::RawValue>>,
            ) -> RawRpcFuture<'a, Box<serde_json::value::RawValue>> {
                let method = self.method_handlers.get(method_handler).unwrap_or_else(|| {
                    panic!(
                        "no method named {} registered. Params: {:?}",
                        method_handler, params
                    )
                });

                (*method)(&mut self.data_table, &mut self.subscription_channel, params)
            }

            fn subscribe<'a>(
                &'a mut self,
                sub: &str,
                params: Option<Box<serde_json::value::RawValue>>,
            ) -> RawRpcFuture<'a, RawRpcSubscription> {
                let sub = self.subscription_handlers.get(sub).unwrap_or_else(|| {
                    panic!(
                        "no subscription named {} registered. Params: {:?}",
                        sub, params
                    )
                });

                (*sub)(&mut self.data_table, &mut self.subscription_channel, params)
            }
        }

        #[derive(Default)]
        pub struct MockRpcBuilder {
            data: InnerMockedRpcClient,
        }

        impl MockRpcBuilder {
            pub fn add_method<F>(mut self, method_name: &str, method_handler: F) -> Self
            where
                F: Send
                    + for<'a> Fn(
                        &'a mut MockDataTable,
                        &'a mut Option<Subscription>,
                        Option<Box<serde_json::value::RawValue>>,
                    )
                        -> RawRpcFuture<'a, Box<serde_json::value::RawValue>>
                    + 'static,
            {
                self.data
                    .method_handlers
                    .insert(method_name.into(), Box::new(method_handler));
                self
            }

            pub fn add_subscription<F>(
                mut self,
                subscription_name: &str,
                subscription_handler: F,
            ) -> Self
            where
                F: Send
                    + for<'a> Fn(
                        &'a mut MockDataTable,
                        &'a mut Option<Subscription>,
                        Option<Box<serde_json::value::RawValue>>,
                    ) -> RawRpcFuture<'a, RawRpcSubscription>
                    + 'static,
            {
                self.data
                    .subscription_handlers
                    .insert(subscription_name.into(), Box::new(subscription_handler));
                self
            }

            pub fn add_mock_data<
                'a,
                T: Serialize,
                I: IntoIterator<Item = (&'a str, Message<RpcResult<T>>)>,
            >(
                mut self,
                item: I,
            ) -> Self {
                let data = &mut self.data.data_table;
                for (key, item) in item.into_iter() {
                    data.push(key.into(), item);
                }
                self
            }

            pub fn build(self) -> MockRpcClient {
                MockRpcClient {
                    data: Arc::new(Mutex::new(self.data)),
                }
            }
        }

        pub struct MockRpcClient {
            data: Arc<Mutex<InnerMockedRpcClient>>,
        }

        impl RpcClientT for MockRpcClient {
            fn request_raw<'a>(
                &'a self,
                method: &'a str,
                params: Option<Box<serde_json::value::RawValue>>,
            ) -> RawRpcFuture<'a, Box<serde_json::value::RawValue>> {
                Box::pin(async {
                    let mut data = self.data.lock().await;
                    data.call(method, params).await
                })
            }

            fn subscribe_raw<'a>(
                &'a self,
                sub: &'a str,
                params: Option<Box<serde_json::value::RawValue>>,
                _unsub: &'a str,
            ) -> RawRpcFuture<'a, RawRpcSubscription> {
                Box::pin(async {
                    let mut data = self.data.lock().await;
                    data.subscribe(sub, params).await
                })
            }
        }
    }

    // Define dummy config
    enum Conf {}
    impl Config for Conf {
        type Hash = H256;
        type AccountId = crate::utils::AccountId32;
        type Address = crate::utils::MultiAddress<Self::AccountId, ()>;
        type Signature = crate::utils::MultiSignature;
        type Hasher = crate::config::substrate::BlakeTwo256;
        type Header = crate::config::substrate::SubstrateHeader<u32, Self::Hasher>;
        type ExtrinsicParams = DefaultExtrinsicParams<Self>;
        type AssetId = u32;
    }

    mod legacy {
        use super::*;
        use crate::backend::{
            legacy::rpc_methods::Bytes, legacy::rpc_methods::RuntimeVersion, legacy::LegacyBackend,
        };
        use rpc_client::*;

        pub fn setup_mock_rpc() -> MockRpcBuilder {
            MockRpcBuilder::default()
                .add_method("state_getStorage", |data, _sub, params| {
                    Box::pin(async move {
                        let params = params.map(|p| p.get().to_string());
                        let rpc_params = jsonrpsee::types::Params::new(params.as_deref());
                        let key: sp_core::Bytes = rpc_params.sequence().next().unwrap();
                        let value = data.pop(key.0).unwrap_single();
                        value.map(|v| serde_json::value::RawValue::from_string(v).unwrap())
                    })
                })
                .add_method("chain_getBlockHash", |data, _, _| {
                    Box::pin(async move {
                        let value = data.pop("chain_getBlockHash".into()).unwrap_single();
                        value.map(|v| serde_json::value::RawValue::from_string(v).unwrap())
                    })
                })
        }

        use crate::backend::Backend;

        fn client_runtime_version(num: u32) -> crate::client::RuntimeVersion {
            crate::client::RuntimeVersion {
                spec_version: num,
                transaction_version: num,
            }
        }

        fn runtime_version(num: u32) -> RuntimeVersion {
            RuntimeVersion {
                spec_version: num,
                transaction_version: num,
                other: HashMap::new(),
            }
        }

        fn bytes(str: &str) -> RpcResult<Option<Bytes>> {
            Ok(Some(Bytes(str.into())))
        }

        #[tokio::test]
        async fn storage_fetch_values() {
            let mock_data = vec![
                ("ID1", Message::Single(bytes("Data1"))),
                (
                    "ID2",
                    Message::Single(Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    ))),
                ),
                ("ID2", Message::Single(bytes("Data2"))),
                (
                    "ID3",
                    Message::Single(Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    ))),
                ),
                ("ID3", Message::Single(bytes("Data3"))),
            ];
            let rpc_client = setup_mock_rpc().add_mock_data(mock_data).build();
            let backend: LegacyBackend<Conf> = LegacyBackend::builder().build(rpc_client);

            // Test
            let response = backend
                .storage_fetch_values(
                    ["ID1".into(), "ID2".into(), "ID3".into()].into(),
                    random_hash(),
                )
                .await
                .unwrap();

            let response = response
                .map(|x| x.unwrap())
                .collect::<Vec<StorageResponse>>()
                .await;

            let expected = vec![
                storage_response("ID1", "Data1"),
                storage_response("ID2", "Data2"),
                storage_response("ID3", "Data3"),
            ];

            assert_eq!(expected, response)
        }

        #[tokio::test]
        async fn storage_fetch_value() {
            // Setup
            let mock_data = [
                (
                    "ID1",
                    Message::Single(Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    ))),
                ),
                ("ID1", Message::Single(bytes("Data1"))),
            ];
            let rpc_client = setup_mock_rpc().add_mock_data(mock_data).build();

            // Test
            let backend: LegacyBackend<Conf> = LegacyBackend::builder().build(rpc_client);
            let response = backend
                .storage_fetch_value("ID1".into(), random_hash())
                .await
                .unwrap();

            let response = response.unwrap();
            assert_eq!("Data1".to_owned(), String::from_utf8(response).unwrap())
        }

        #[tokio::test]
        /// This test should cover the logic of the following methods:
        /// - `genesis_hash`
        /// - `block_header`
        /// - `block_body`
        /// - `latest_finalized_block`
        /// - `current_runtime_version`
        /// - `current_runtime_version`
        /// - `call`
        /// The test covers them because they follow the simple pattern of:
        /// ```no_run
        ///  async fn THE_THING(&self) -> Result<T::Hash, Error> {
        ///    retry(|| <DO THE THING> ).await
        ///  }
        /// ```
        async fn simple_fetch() {
            let hash = random_hash();
            let mock_data = vec![
                (
                    "chain_getBlockHash",
                    Message::Single(Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    ))),
                ),
                ("chain_getBlockHash", Message::Single(Ok(Some(hash)))),
            ];
            let rpc_client = setup_mock_rpc().add_mock_data(mock_data).build();

            // Test
            let backend: LegacyBackend<Conf> = LegacyBackend::builder().build(rpc_client);
            let response = backend.genesis_hash().await.unwrap();

            assert_eq!(hash, response)
        }

        #[tokio::test]
        /// This test should cover the logic of the following methods:
        /// - `stream_runtime_version`
        /// - `stream_all_block_headers`
        /// - `stream_best_block_headers`
        /// The test covers them because they follow the simple pattern of:
        /// ```no_run
        /// async fn stream_the_thing(
        ///     &self,
        /// ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error> {
        ///     let methods = self.methods.clone();
        ///     let retry_sub = retry_stream(move || {
        ///         let methods = methods.clone();
        ///         Box::pin(async move {
        ///               methods.do_the_thing().await?
        ///             });
        ///             Ok(StreamOf(Box::pin(sub)))
        ///         })
        ///     })
        ///     .await?;
        ///     Ok(retry_sub)
        /// }
        /// ```
        async fn stream_simple() {
            let mock_subscription_data = vec![
                (
                    "state_subscribeRuntimeVersion",
                    Message::Many(Ok(vec![
                        Ok(runtime_version(0)),
                        Err(RpcError::DisconnectedWillReconnect(
                            "Reconnecting".to_string(),
                        )),
                        Ok(runtime_version(1)),
                    ])),
                ),
                (
                    "state_subscribeRuntimeVersion",
                    Message::Many(Ok(vec![
                        Err(RpcError::DisconnectedWillReconnect(
                            "Reconnecting".to_string(),
                        )),
                        Ok(runtime_version(2)),
                        Ok(runtime_version(3)),
                    ])),
                ),
                (
                    "state_subscribeRuntimeVersion",
                    Message::Many(Ok(vec![
                        Ok(runtime_version(4)),
                        Ok(runtime_version(5)),
                        Err(RpcError::RequestRejected("Reconnecting".to_string())),
                    ])),
                ),
            ];
            let rpc_client = setup_mock_rpc()
                .add_subscription("state_subscribeRuntimeVersion", |data, _, _| {
                    Box::pin(async move {
                        let values = data
                            .pop("state_subscribeRuntimeVersion".into())
                            .unwrap_many();
                        let values: RpcResult<Vec<RpcResult<Box<RawValue>>>> = values.map(|v| {
                            v.into_iter()
                                .map(|v| {
                                    v.map(|v| serde_json::value::RawValue::from_string(v).unwrap())
                                })
                                .collect::<Vec<RpcResult<Box<RawValue>>>>()
                        });
                        values.map(|v| RawRpcSubscription {
                            stream: futures::stream::iter(v).boxed(),
                            id: Some("ID".to_string()),
                        })
                    })
                })
                .add_mock_data(mock_subscription_data)
                .build();

            // Test
            let backend: LegacyBackend<Conf> = LegacyBackend::builder().build(rpc_client);

            let mut results = backend.stream_runtime_version().await.unwrap();
            let mut expected = VecDeque::from(vec![
                Ok::<crate::client::RuntimeVersion, crate::Error>(client_runtime_version(0)),
                Ok(client_runtime_version(4)),
                Ok(client_runtime_version(5)),
            ]);

            while let Some(res) = results.next().await {
                if res.is_ok() {
                    assert_eq!(expected.pop_front().unwrap().unwrap(), res.unwrap())
                } else {
                    assert!(matches!(
                        res,
                        Err(crate::Error::Rpc(RpcError::RequestRejected(_)))
                    ))
                }
            }
            assert!(expected.is_empty());
            assert!(results.next().await.is_none())
        }
    }

    mod unstable_backend {

        use std::sync::atomic::AtomicBool;

        use futures::task::Poll;
        use rpc_client::{Message, MockRpcBuilder, Subscription};
        use rpc_methods::{
            Bytes, Initialized, MethodResponse, MethodResponseStarted, OperationError, OperationId,
            OperationStorageItems, RuntimeSpec, RuntimeVersionEvent,
        };

        use super::unstable::*;
        use super::*;

        fn build_backend(
            rpc_client: impl RpcClientT,
        ) -> (UnstableBackend<Conf>, UnstableBackendDriver<Conf>) {
            let (backend, driver): (UnstableBackend<Conf>, _) =
                UnstableBackend::builder().build(rpc_client);
            (backend, driver)
        }

        fn build_backend_spawn_background(rpc_client: impl RpcClientT) -> UnstableBackend<Conf> {
            let (backend, mut driver) = build_backend(rpc_client);
            tokio::spawn(async move {
                while let Some(val) = driver.next().await {
                    if let Err(e) = val {
                        eprintln!("Error driving unstable backend: {e}; terminating client");
                    }
                }
            });
            backend
        }

        fn runtime_spec() -> RuntimeSpec {
            let spec = serde_json::json!({
                        "specName": "westend",
                        "implName": "parity-westend",
                        "specVersion": 9122,
                        "implVersion": 0,
                        "transactionVersion": 7,
                        "apis": {
                            "0xdf6acb689907609b": 3,
                            "0x37e397fc7c91f5e4": 1,
                            "0x40fe3ad401f8959a": 5,
                            "0xd2bc9897eed08f15": 3,
                            "0xf78b278be53f454c": 2,
                            "0xaf2c0297a23e6d3d": 1,
                            "0x49eaaf1b548a0cb0": 1,
                            "0x91d5df18b0d2cf58": 1,
                            "0xed99c5acb25eedf5": 3,
                            "0xcbca25e39f142387": 2,
                            "0x687ad44ad37f03c2": 1,
                            "0xab3c0572291feb8b": 1,
                            "0xbc9d89904f5b923f": 1,
                            "0x37c8bb1350a9a2a8": 1
                    }
            });
            serde_json::from_value(spec).unwrap()
        }

        type FollowEvent = unstable::rpc_methods::FollowEvent<<Conf as Config>::Hash>;

        fn setup_mock_rpc_client(cycle_ids: bool) -> MockRpcBuilder {
            let hash = random_hash();
            let mut id = 0;
            rpc_client::MockRpcBuilder::default().add_subscription(
                "chainHead_v1_follow",
                move |_, sub, _| {
                    Box::pin(async move {
                        if cycle_ids {
                            id += 1;
                        }
                        let follow_event =
                            FollowEvent::Initialized(Initialized::<<Conf as Config>::Hash> {
                                finalized_block_hashes: vec![hash],
                                finalized_block_runtime: Some(rpc_methods::RuntimeEvent::Valid(
                                    RuntimeVersionEvent {
                                        spec: runtime_spec(),
                                    },
                                )),
                            });
                        let (subscription, mut receiver) = Subscription::new();
                        subscription
                            .write(Message::Single(Ok(
                                serde_json::to_string(&follow_event).unwrap()
                            )))
                            .await;
                        sub.replace(subscription);
                        let read_stream =
                            futures::stream::poll_fn(move |cx| -> Poll<Option<Item>> {
                                receiver.poll_recv(cx)
                            })
                            .map(|item| item.map(|x| RawValue::from_string(x).unwrap()));
                        let stream = RawRpcSubscription {
                            stream: read_stream.boxed(),
                            id: Some(format!("ID{}", id)),
                        };
                        Ok(stream)
                    })
                },
            )
        }

        fn response_started(id: &str) -> MethodResponse {
            MethodResponse::Started(MethodResponseStarted {
                operation_id: id.to_owned(),
                discarded_items: None,
            })
        }

        fn operation_error(id: &str) -> FollowEvent {
            FollowEvent::OperationError(OperationError {
                operation_id: id.to_owned(),
                error: "error".to_owned(),
            })
        }

        fn storage_done(id: &str) -> FollowEvent {
            FollowEvent::OperationStorageDone(OperationId {
                operation_id: id.to_owned(),
            })
        }
        fn storage_result(key: &str, value: &str) -> unstable::rpc_methods::StorageResult {
            unstable::rpc_methods::StorageResult {
                key: Bytes(key.to_owned().into()),
                result: rpc_methods::StorageResultType::Value(Bytes(value.to_owned().into())),
            }
        }
        fn storage_items(id: &str, items: &[unstable::rpc_methods::StorageResult]) -> FollowEvent {
            FollowEvent::OperationStorageItems(OperationStorageItems {
                operation_id: id.to_owned(),
                items: VecDeque::from(items.to_owned()),
            })
        }

        fn operation_continue(id: &str) -> FollowEvent {
            FollowEvent::OperationWaitingForContinue(OperationId {
                operation_id: id.to_owned(),
            })
        }

        #[tokio::test]
        async fn storage_fetch_values_returns_stream_with_single_error() {
            let response_data = vec![(
                "method_response",
                Message::Single(Ok(response_started("Id1"))),
            )];
            let mock_subscription_data = vec![(
                "chainHead_v1_storage",
                Message::Many(Ok(vec![Ok(operation_error("Id1")), Ok(FollowEvent::Stop)])),
            )];
            let rpc_client = setup_mock_rpc_client(false)
                .add_method("chainHead_v1_storage", |data, sub, _| {
                    Box::pin(async move {
                        let response = data.pop("method_response".into()).unwrap_single();
                        if response.is_ok() {
                            let item = data.pop("chainHead_v1_storage".into());
                            if let Some(sub) = sub {
                                let item = item;
                                sub.write_delayed(item).await
                            }
                        }
                        response.map(|x| RawValue::from_string(x).unwrap())
                    })
                })
                .add_mock_data(mock_subscription_data)
                .add_mock_data(response_data)
                .build();

            let backend = build_backend_spawn_background(rpc_client);

            // Test
            // This request should encounter an error on `request` and do a retry.
            let response = backend
                .storage_fetch_values(
                    ["ID1".into(), "ID2".into(), "ID3".into()].into(),
                    random_hash(),
                )
                .await
                .unwrap();

            // operation returned FollowEvent::OperationError
            let response = response
                .collect::<Vec<Result<StorageResponse, Error>>>()
                .await;

            assert!(matches!(
                response.as_slice(),
                [Err(Error::Other(s) )] if s == "error"
            ));
        }

        #[tokio::test]
        /// Tests that the method will retry on failed query
        async fn storage_fetch_values_retry_query() {
            let response_data = vec![
                (
                    "method_response",
                    Message::Single(Err(RpcError::DisconnectedWillReconnect("Error".into()))),
                ),
                (
                    "method_response",
                    Message::Single(Ok(response_started("Id1"))),
                ),
            ];
            let mock_data = vec![(
                "chainHead_v1_storage",
                Message::Many(Ok(vec![
                    Ok(storage_items(
                        "Id1",
                        &[
                            storage_result("ID1", "Data1"),
                            storage_result("ID2", "Data2"),
                            storage_result("ID3", "Data3"),
                        ],
                    )),
                    Ok(storage_done("Id1")),
                ])),
            )];
            let rpc_client = setup_mock_rpc_client(false)
                .add_method("chainHead_v1_storage", |data, sub, _| {
                    Box::pin(async move {
                        let response = data.pop("method_response".into()).unwrap_single();
                        if response.is_ok() {
                            let item = data.pop("chainHead_v1_storage".into());
                            if let Some(sub) = sub {
                                let item = item;
                                sub.write_delayed(item).await
                            }
                        }
                        response.map(|x| RawValue::from_string(x).unwrap())
                    })
                })
                .add_mock_data(mock_data)
                .add_mock_data(response_data)
                .build();
            let backend = build_backend_spawn_background(rpc_client);

            // We try again and should succeed
            let response = backend
                .storage_fetch_values(
                    ["ID1".into(), "ID2".into(), "ID3".into()].into(),
                    random_hash(),
                )
                .await
                .unwrap();

            let response = response
                .map(|x| x.unwrap())
                .collect::<Vec<StorageResponse>>()
                .await;

            assert_eq!(
                vec![
                    storage_response("ID1", "Data1"),
                    storage_response("ID2", "Data2"),
                    storage_response("ID3", "Data3"),
                ],
                response
            )
        }
        #[tokio::test]
        async fn storage_fetch_values_retry_chainhead_continue() {
            fn compare_storage_responses(
                expected_response: &StorageResponse,
                received_response: &StorageResponse,
            ) -> bool {
                expected_response == received_response
            }

            let response_data = vec![
                (
                    "method_response",
                    Message::Single(Err::<MethodResponse, RpcError>(
                        RpcError::DisconnectedWillReconnect("Error".into()),
                    )),
                ),
                (
                    "method_response",
                    Message::Single(Ok(response_started("Id1"))),
                ),
                (
                    "method_response",
                    Message::Single(Ok(response_started("Id1"))),
                ),
            ];
            let continue_data = vec![
                ("continue_response", Message::Single(Ok(()))),
                ("continue_response", Message::Single(Ok(()))),
                (
                    "continue_response",
                    Message::Single(Err(RpcError::DisconnectedWillReconnect("Error".into()))),
                ),
                ("continue_response", Message::Single(Ok(()))),
                ("continue_response", Message::Single(Ok(()))),
            ];
            let mock_data = vec![
                (
                    "chainHead_v1_storage",
                    Message::Many(Ok(vec![
                        Ok(storage_items("Id1", &[storage_result("ID1", "Data1")])),
                        Ok(operation_continue("Id1")),
                    ])),
                ),
                (
                    "chainHead_v1_storage",
                    Message::Many(Ok(vec![
                        Ok(storage_items("Id1", &[storage_result("ID2", "Data2")])),
                        Ok(operation_continue("Id1")),
                    ])),
                ),
                (
                    "chainHead_v1_storage",
                    Message::Many(Ok(vec![Ok(operation_error("Id1")), Ok(FollowEvent::Stop)])),
                ),
                (
                    "chainHead_v1_storage",
                    Message::Many(Ok(vec![
                        Ok(storage_items("Id1", &[storage_result("ID1", "Data1")])),
                        Ok(operation_continue("Id1")),
                    ])),
                ),
                (
                    "chainHead_v1_storage",
                    Message::Many(Ok(vec![
                        Ok(storage_items("Id1", &[storage_result("ID2", "Data2")])),
                        Ok(operation_continue("Id1")),
                    ])),
                ),
                (
                    "chainHead_v1_storage",
                    Message::Many(Ok(vec![
                        Ok(storage_items("Id1", &[storage_result("ID3", "Data3")])),
                        Ok(storage_done("Id1")),
                    ])),
                ),
            ];
            let rpc_client = setup_mock_rpc_client(false)
                .add_method("chainHead_v1_storage", |data, sub, _| {
                    Box::pin(async move {
                        let response = data.pop("method_response".into()).unwrap_single();
                        if response.is_ok() {
                            let item = data.pop("chainHead_v1_storage".into());
                            if let Some(sub) = sub {
                                let item = item;
                                sub.write_delayed(item).await
                            }
                        }
                        response.map(|x| RawValue::from_string(x).unwrap())
                    })
                })
                .add_method("chainHead_v1_continue", |data, sub, _| {
                    Box::pin(async move {
                        let response = data.pop("continue_response".into()).unwrap_single();
                        if response.is_ok() {
                            let item = data.pop("chainHead_v1_storage".into());
                            if let Some(sub) = sub {
                                let item = item;
                                sub.write_delayed(item).await
                            }
                        }
                        response.map(|x| RawValue::from_string(x).unwrap())
                    })
                })
                .add_mock_data(mock_data)
                .add_mock_data(response_data)
                .add_mock_data(continue_data)
                .build();
            let backend = build_backend_spawn_background(rpc_client);

            // We try again and should fail mid way
            let response = backend
                .storage_fetch_values(
                    ["ID1".into(), "ID2".into(), "ID3".into()].into(),
                    random_hash(),
                )
                .await
                .unwrap();
            // operation returned FollowEvent::OperationError
            let response = response
                .collect::<Vec<Result<StorageResponse, Error>>>()
                .await;

            assert!(matches!(
                response.as_slice(),
                [
                    Ok(resp1 @ StorageResponse { .. }),
                    Ok(resp2 @ StorageResponse { .. }),
                    Err(Error::Other(s))
                ] if s == "error"
                  && compare_storage_responses(&storage_response("ID1", "Data1"), resp1)
                  && compare_storage_responses(&storage_response("ID2", "Data2"), resp2)
            ));

            // We try again and should succeed
            let response = backend
                .storage_fetch_values(
                    ["ID1".into(), "ID2".into(), "ID3".into()].into(),
                    random_hash(),
                )
                .await
                .unwrap();

            let response = response
                .map(|x| x.unwrap())
                .collect::<Vec<StorageResponse>>()
                .await;

            assert_eq!(
                vec![
                    storage_response("ID1", "Data1"),
                    storage_response("ID2", "Data2"),
                    storage_response("ID3", "Data3"),
                ],
                response
            )
        }

        #[tokio::test]
        async fn simple_fetch() {
            let hash = random_hash();

            let mock_data = vec![
                (
                    "chainSpec_v1_genesisHash",
                    Message::Single(Err::<H256, RpcError>(RpcError::RequestRejected(
                        "Error".to_owned(),
                    ))),
                ),
                (
                    "chainSpec_v1_genesisHash",
                    Message::Single(Err(RpcError::DisconnectedWillReconnect("Error".to_owned()))),
                ),
                ("chainSpec_v1_genesisHash", Message::Single(Ok(hash))),
            ];
            let rpc_client = setup_mock_rpc_client(false)
                .add_method("chainSpec_v1_genesisHash", |data, _, _| {
                    Box::pin(async move {
                        let response = data.pop("chainSpec_v1_genesisHash".into()).unwrap_single();
                        response.map(|x| RawValue::from_string(x).unwrap())
                    })
                })
                .add_mock_data(mock_data)
                .build();

            let backend = build_backend_spawn_background(rpc_client);

            // Test
            // This request should encounter an error on `request` and do a retry.
            let response_hash = backend.genesis_hash().await.unwrap();

            assert_eq!(hash, response_hash)
        }

        #[tokio::test]
        // Failure as we do not wait for subscription id to be updated.
        // see https://github.com/paritytech/subxt/issues/1567
        async fn stale_subscription_id_failure() {
            let response_data = vec![
                (
                    "method_response",
                    Message::Single(Ok(response_started("Id1"))),
                ),
                (
                    "method_response",
                    Message::Single(Err(RpcError::RequestRejected("stale id".into()))),
                ),
                (
                    "method_response",
                    Message::Single(Ok(response_started("Id1"))),
                ),
            ];

            let mock_data = vec![
                (
                    "chainHead_v1_storage",
                    Message::Many(Ok(vec![Ok(operation_error("Id1")), Ok(FollowEvent::Stop)])),
                ),
                (
                    "chainHead_v1_storage",
                    Message::Many(Ok(vec![
                        Ok(storage_items(
                            "Id1",
                            &[
                                storage_result("ID1", "Data1"),
                                storage_result("ID2", "Data2"),
                                storage_result("ID3", "Data3"),
                            ],
                        )),
                        Ok(storage_done("Id1")),
                    ])),
                ),
            ];
            let rpc_client = setup_mock_rpc_client(true)
                .add_method("chainHead_v1_storage", {
                    let subscription_expired = Arc::new(AtomicBool::new(false));
                    move |data, sub, params| {
                        let subscription_expired = subscription_expired.clone();
                        Box::pin(async move {
                            let subscription_expired = subscription_expired.clone();
                            if subscription_expired.load(std::sync::atomic::Ordering::SeqCst) {
                                let params = params.map(|p| p.get().to_string());
                                let rpc_params = jsonrpsee::types::Params::new(params.as_deref());
                                let key: String = rpc_params.sequence().next().unwrap();
                                if key == *"ID1" {
                                    return Err(RpcError::RequestRejected("stale id".into()));
                                } else {
                                    subscription_expired
                                        .swap(false, std::sync::atomic::Ordering::SeqCst);
                                }
                            }
                            let response = data.pop("method_response".into()).unwrap_single();
                            if response.is_ok() {
                                let item = data.pop("chainHead_v1_storage".into());
                                if let Some(sub) = sub {
                                    let item = item;
                                    sub.write_delayed(item).await
                                }
                            } else {
                                subscription_expired
                                    .swap(true, std::sync::atomic::Ordering::SeqCst);
                            }
                            response.map(|x| RawValue::from_string(x).unwrap())
                        })
                    }
                })
                .add_mock_data(mock_data)
                .add_mock_data(response_data)
                .build();
            let (backend, mut driver): (UnstableBackend<Conf>, _) = build_backend(rpc_client);

            let _ = driver.next().await.unwrap();
            let _ = driver.next().await.unwrap();

            // not getting new subscription id and hitting request rejected > 10 times
            let response = backend
                .storage_fetch_values(
                    ["ID1".into(), "ID2".into(), "ID3".into()].into(),
                    random_hash(),
                )
                .await;

            let _ = driver.next().await.unwrap();

            let binding = response
                .unwrap()
                .collect::<Vec<Result<StorageResponse, Error>>>()
                .await;
            let response = binding.last().unwrap();

            assert!(matches!(
                response,
                Err(Error::Other(reason)) if reason == "error"
            ));

            // not getting new subscription id and hitting request rejected > 10 times
            let response = backend
                .storage_fetch_values(
                    ["ID1".into(), "ID2".into(), "ID3".into()].into(),
                    random_hash(),
                )
                .await;

            assert!(matches!(
                response,
                Err(Error::Rpc(RpcError::RequestRejected(reason))) if reason == "stale id"
            ))
        }
    }
}
