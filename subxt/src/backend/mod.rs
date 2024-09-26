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

    mod legacy {
        use super::rpc::{RpcClient, RpcClientT};
        use crate::backend::rpc::RawRpcSubscription;
        use crate::backend::BackendExt;
        use crate::{
            backend::{
                legacy::rpc_methods::Bytes, legacy::rpc_methods::RuntimeVersion,
                legacy::LegacyBackend, StorageResponse,
            },
            error::RpcError,
        };
        use futures::StreamExt;
        use polkadot_sdk::sp_core;
        use serde::Serialize;
        use serde_json::value::RawValue;
        use std::{
            collections::{HashMap, VecDeque},
            sync::Arc,
        };
        use subxt_core::{config::DefaultExtrinsicParams, Config};
        use tokio::sync::{mpsc, Mutex};

        type RpcResult<T> = Result<T, RpcError>;
        type Item = RpcResult<String>;

        struct MockDataTable {
            items: HashMap<Vec<u8>, VecDeque<Item>>,
        }

        impl MockDataTable {
            fn new() -> Self {
                MockDataTable {
                    items: HashMap::new(),
                }
            }

            fn from_iter<'a, T: Serialize, I: IntoIterator<Item = (&'a str, RpcResult<T>)>>(
                item: I,
            ) -> Self {
                let mut data = Self::new();
                for (key, item) in item.into_iter() {
                    data.push(key.into(), item);
                }
                data
            }

            fn push<I: Serialize>(&mut self, key: Vec<u8>, item: RpcResult<I>) {
                let item = item.map(|x| serde_json::to_string(&x).unwrap());
                match self.items.entry(key) {
                    std::collections::hash_map::Entry::Occupied(v) => v.into_mut().push_back(item),
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(VecDeque::from([item]));
                    }
                }
            }

            fn pop(&mut self, key: Vec<u8>) -> Item {
                self.items.get_mut(&key).unwrap().pop_front().unwrap()
            }
        }

        struct Subscription {
            sender: mpsc::Sender<RpcResult<Vec<Item>>>,
            receiver: mpsc::Receiver<RpcResult<Vec<Item>>>,
        }

        impl Subscription {
            fn new() -> Self {
                let (sender, receiver) = mpsc::channel(32);
                Self { sender, receiver }
            }

            async fn from_iter<
                T: Serialize,
                S: IntoIterator<Item = RpcResult<Vec<RpcResult<T>>>>,
            >(
                items: S,
            ) -> Self {
                let sub = Self::new();
                for i in items {
                    let i: RpcResult<Vec<Item>> = i.map(|items| {
                        items
                            .into_iter()
                            .map(|item| item.map(|i| serde_json::to_string(&i).unwrap()))
                            .collect()
                    });
                    sub.write(i).await
                }
                sub
            }

            async fn read(&mut self) -> RpcResult<Vec<Item>> {
                self.receiver.recv().await.unwrap()
            }

            async fn write(&self, items: RpcResult<Vec<Item>>) {
                self.sender.send(items).await.unwrap()
            }
        }

        struct Data {
            request: MockDataTable,
            subscription: Subscription,
        }

        struct MockRpcClientStorage {
            data: Arc<Mutex<Data>>,
        }

        impl RpcClientT for MockRpcClientStorage {
            fn request_raw<'a>(
                &'a self,
                method: &'a str,
                params: Option<Box<serde_json::value::RawValue>>,
            ) -> super::rpc::RawRpcFuture<'a, Box<serde_json::value::RawValue>> {
                Box::pin(async move {
                    match method {
                        "state_getStorage" => {
                            let mut data = self.data.lock().await;
                            let params = params.map(|p| p.get().to_string());
                            let rpc_params = jsonrpsee::types::Params::new(params.as_deref());
                            let key: sp_core::Bytes = rpc_params.sequence().next().unwrap();
                            let value = data.request.pop(key.0);
                            value.map(|v| serde_json::value::RawValue::from_string(v).unwrap())
                        }
                        "chain_getBlockHash" => {
                            let mut data = self.data.lock().await;
                            let value = data.request.pop("chain_getBlockHash".into());
                            value.map(|v| serde_json::value::RawValue::from_string(v).unwrap())
                        }
                        _ => todo!(),
                    }
                })
            }

            fn subscribe_raw<'a>(
                &'a self,
                _sub: &'a str,
                _params: Option<Box<serde_json::value::RawValue>>,
                _unsub: &'a str,
            ) -> super::rpc::RawRpcFuture<'a, super::rpc::RawRpcSubscription> {
                Box::pin(async {
                    let mut data = self.data.lock().await;
                    let values: RpcResult<Vec<RpcResult<Box<RawValue>>>> =
                        data.subscription.read().await.map(|v| {
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
            }
        }

        // Define dummy config
        enum Conf {}
        impl Config for Conf {
            type Hash = crate::utils::H256;
            type AccountId = crate::utils::AccountId32;
            type Address = crate::utils::MultiAddress<Self::AccountId, ()>;
            type Signature = crate::utils::MultiSignature;
            type Hasher = crate::config::substrate::BlakeTwo256;
            type Header = crate::config::substrate::SubstrateHeader<u32, Self::Hasher>;
            type ExtrinsicParams = DefaultExtrinsicParams<Self>;

            type AssetId = u32;
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

        fn storage_response<K: Into<Vec<u8>>, V: Into<Vec<u8>>>(key: K, value: V) -> StorageResponse
        where
            Vec<u8>: From<K>,
        {
            StorageResponse {
                key: key.into(),
                value: value.into(),
            }
        }

        async fn build_mock_client<
            'a,
            T: Serialize,
            D: IntoIterator<Item = (&'a str, RpcResult<T>)>,
            S: IntoIterator<Item = RpcResult<Vec<RpcResult<T>>>>,
        >(
            table_data: D,
            subscription_data: S,
        ) -> RpcClient {
            let data = Data {
                request: MockDataTable::from_iter(table_data),
                subscription: Subscription::from_iter(subscription_data).await,
            };
            RpcClient::new(MockRpcClientStorage {
                data: Arc::new(Mutex::new(data)),
            })
        }

        #[tokio::test]
        async fn storage_fetch_values() {
            let mock_data = vec![
                ("ID1", bytes("Data1")),
                (
                    "ID2",
                    Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    )),
                ),
                ("ID2", bytes("Data2")),
                (
                    "ID3",
                    Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    )),
                ),
                ("ID3", bytes("Data3")),
            ];
            let rpc_client = build_mock_client(mock_data, vec![]).await;
            let backend: LegacyBackend<Conf> = LegacyBackend::builder().build(rpc_client);

            // Test
            let response = backend
                .storage_fetch_values(
                    ["ID1".into(), "ID2".into(), "ID3".into()].into(),
                    crate::utils::H256::random(),
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
                    Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    )),
                ),
                ("ID1", bytes("Data1")),
            ];
            let rpc_client = build_mock_client(mock_data, vec![]).await;

            // Test
            let backend: LegacyBackend<Conf> = LegacyBackend::builder().build(rpc_client);
            let response = backend
                .storage_fetch_value("ID1".into(), crate::utils::H256::random())
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
            let hash = crate::utils::H256::random();
            let mock_data = vec![
                (
                    "chain_getBlockHash",
                    Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    )),
                ),
                ("chain_getBlockHash", Ok(Some(hash))),
            ];
            let rpc_client = build_mock_client(mock_data, vec![]).await;

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
                Ok(vec![
                    Ok(runtime_version(0)),
                    Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    )),
                    Ok(runtime_version(1)),
                ]),
                Ok(vec![
                    Err(RpcError::DisconnectedWillReconnect(
                        "Reconnecting".to_string(),
                    )),
                    Ok(runtime_version(2)),
                    Ok(runtime_version(3)),
                ]),
                Ok(vec![
                    Ok(runtime_version(4)),
                    Ok(runtime_version(5)),
                    Err(RpcError::RequestRejected("Reconnecting".to_string())),
                ]),
            ];
            let rpc_client = build_mock_client(vec![], mock_subscription_data).await;

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
}
