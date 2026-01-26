// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! An interface to call the  API methods. See
//! <https://github.com/paritytech/json-rpc-interface-spec/> for details of the API
//! methods exposed here.

use crate::Hash;
use crate::client::{RpcClient, RpcSubscription, rpc_params};
use crate::{Error, RpcConfig};
use derive_where::derive_where;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{HashMap, VecDeque};
use std::task::Poll;

/// An interface to call the new ["chainHead" RPC methods](https://paritytech.github.io/json-rpc-interface-spec/).
/// This interface is instantiated with some `T: RpcConfig` trait which determines some of the types that
/// the RPC methods will take or hand back.
#[derive_where(Clone, Debug)]
pub struct ChainHeadRpcMethods<T> {
    client: RpcClient,
    _marker: std::marker::PhantomData<T>,
}

impl<T: RpcConfig> ChainHeadRpcMethods<T> {
    /// Instantiate the legacy RPC method interface.
    pub fn new(client: RpcClient) -> Self {
        ChainHeadRpcMethods {
            client,
            _marker: std::marker::PhantomData,
        }
    }

    /// Subscribe to `chainHead_v1_follow` to obtain all reported blocks by the chain.
    ///
    /// The subscription ID can be used to make queries for the
    /// block's body ([`chainHead_v1_body`](ChainHeadRpcMethods::chainhead_v1_follow)),
    /// block's header ([`chainHead_v1_header`](ChainHeadRpcMethods::chainhead_v1_header)),
    /// block's storage ([`chainHead_v1_storage`](ChainHeadRpcMethods::chainhead_v1_storage)) and submitting
    /// runtime API calls at this block ([`chainHead_v1_call`](ChainHeadRpcMethods::chainhead_v1_call)).
    ///
    /// # Note
    ///
    /// When the user is no longer interested in a block, the user is responsible
    /// for calling the [`chainHead_v1_unpin`](ChainHeadRpcMethods::chainhead_v1_unpin) method.
    /// Failure to do so will result in the subscription being stopped by generating the `Stop` event.
    pub async fn chainhead_v1_follow(
        &self,
        with_runtime: bool,
    ) -> Result<FollowSubscription<T::Hash>, Error> {
        let sub = self
            .client
            .subscribe(
                "chainHead_v1_follow",
                rpc_params![with_runtime],
                "chainHead_v1_unfollow",
            )
            .await?;

        Ok(FollowSubscription { sub, done: false })
    }

    /// Resumes a storage fetch started with chainHead_v1_storage after it has generated an
    /// `operationWaitingForContinue` event.
    ///
    /// Has no effect if the operationId is invalid or refers to an operation that has emitted a
    /// `{"event": "operationInaccessible"` event, or if the followSubscription is invalid or stale.
    pub async fn chainhead_v1_continue(
        &self,
        follow_subscription: &str,
        operation_id: &str,
    ) -> Result<(), Error> {
        self.client
            .request(
                "chainHead_v1_continue",
                rpc_params![follow_subscription, operation_id],
            )
            .await
    }

    /// Stops an operation started with `chainHead_v1_body`, `chainHead_v1_call`, or
    /// `chainHead_v1_storage¦. If the operation was still in progress, this interrupts it.
    /// If the operation was already finished, this call has no effect.
    ///
    /// Has no effect if the `followSubscription` is invalid or stale.
    pub async fn chainhead_v1_stop_operation(
        &self,
        follow_subscription: &str,
        operation_id: &str,
    ) -> Result<(), Error> {
        self.client
            .request(
                "chainHead_v1_stopOperation",
                rpc_params![follow_subscription, operation_id],
            )
            .await
    }

    /// Call the `chainHead_v1_body` method and return an operation ID to obtain the block's body.
    ///
    /// The response events are provided on the `chainHead_follow` subscription and identified by
    /// the returned operation ID.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainHead_v1_follow`](ChainHeadRpcMethods::chainhead_v1_follow).
    pub async fn chainhead_v1_body(
        &self,
        subscription_id: &str,
        hash: T::Hash,
    ) -> Result<MethodResponse, Error> {
        let response = self
            .client
            .request("chainHead_v1_body", rpc_params![subscription_id, hash])
            .await?;

        Ok(response)
    }

    /// Get the block's header using the `chainHead_v1_header` method.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainHead_v1_follow`](ChainHeadRpcMethods::chainhead_v1_follow).
    pub async fn chainhead_v1_header(
        &self,
        subscription_id: &str,
        hash: T::Hash,
    ) -> Result<Option<T::Header>, Error> {
        // header returned as hex encoded SCALE encoded bytes.
        let header: Option<Bytes> = self
            .client
            .request("chainHead_v1_header", rpc_params![subscription_id, hash])
            .await?;

        let header = header
            .map(|h| codec::Decode::decode(&mut &*h.0))
            .transpose()
            .map_err(Error::Decode)?;
        Ok(header)
    }

    /// Call the `chainHead_v1_storage` method and return an operation ID to obtain the block's storage.
    ///
    /// The response events are provided on the `chainHead_follow` subscription and identified by
    /// the returned operation ID.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainHead_v1_follow`](ChainHeadRpcMethods::chainhead_v1_follow).
    pub async fn chainhead_v1_storage(
        &self,
        subscription_id: &str,
        hash: T::Hash,
        items: impl IntoIterator<Item = StorageQuery<&[u8]>>,
        child_key: Option<&[u8]>,
    ) -> Result<MethodResponse, Error> {
        let items: Vec<StorageQuery<String>> = items
            .into_iter()
            .map(|item| StorageQuery {
                key: to_hex(item.key),
                query_type: item.query_type,
            })
            .collect();

        let response = self
            .client
            .request(
                "chainHead_v1_storage",
                rpc_params![subscription_id, hash, items, child_key.map(to_hex)],
            )
            .await?;

        Ok(response)
    }

    /// Call the `chainHead_v1_call` method and return an operation ID to obtain the runtime API result.
    ///
    /// The response events are provided on the `chainHead_follow` subscription and identified by
    /// the returned operation ID.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainHead_v1_follow`](ChainHeadRpcMethods::chainhead_v1_follow).
    pub async fn chainhead_v1_call(
        &self,
        subscription_id: &str,
        hash: T::Hash,
        function: &str,
        call_parameters: &[u8],
    ) -> Result<MethodResponse, Error> {
        let response = self
            .client
            .request(
                "chainHead_v1_call",
                rpc_params![subscription_id, hash, function, to_hex(call_parameters)],
            )
            .await?;

        Ok(response)
    }

    /// Unpin a block reported by the `chainHead_follow` subscription.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainHead_v1_follow`](ChainHeadRpcMethods::chainhead_v1_follow).
    pub async fn chainhead_v1_unpin(
        &self,
        subscription_id: &str,
        hash: T::Hash,
    ) -> Result<(), Error> {
        self.client
            .request("chainHead_v1_unpin", rpc_params![subscription_id, hash])
            .await
    }

    /// Return the genesis hash.
    pub async fn chainspec_v1_genesis_hash(&self) -> Result<T::Hash, Error> {
        self.client
            .request("chainSpec_v1_genesisHash", rpc_params![])
            .await
    }

    /// Return a string containing the human-readable name of the chain.
    pub async fn chainspec_v1_chain_name(&self) -> Result<String, Error> {
        self.client
            .request("chainSpec_v1_chainName", rpc_params![])
            .await
    }

    /// Returns the JSON payload found in the chain specification under the key properties.
    /// No guarantee is offered about the content of this object, and so it's up to the caller
    /// to decide what to deserialize it into.
    pub async fn chainspec_v1_properties<Props: serde::de::DeserializeOwned>(
        &self,
    ) -> Result<Props, Error> {
        self.client
            .request("chainSpec_v1_properties", rpc_params![])
            .await
    }

    /// Returns an array of strings indicating the names of all the JSON-RPC functions supported by
    /// the JSON-RPC server.
    pub async fn rpc_methods(&self) -> Result<Vec<String>, Error> {
        self.client.request("rpc_methods", rpc_params![]).await
    }

    /// Attempt to submit a transaction, returning events about its progress.
    pub async fn transactionwatch_v1_submit_and_watch(
        &self,
        tx: &[u8],
    ) -> Result<TransactionSubscription<T::Hash>, Error> {
        let sub = self
            .client
            .subscribe(
                "transactionWatch_v1_submitAndWatch",
                rpc_params![to_hex(tx)],
                "transactionWatch_v1_unwatch",
            )
            .await?;

        Ok(TransactionSubscription { sub, done: false })
    }

    /// Broadcast the transaction on the p2p network until the
    /// [`Self::transaction_v1_stop`] is called.
    ///
    /// Returns an operation ID that can be used to stop the broadcasting process.
    /// Returns `None` if the server cannot handle the request at the moment.
    pub async fn transaction_v1_broadcast(&self, tx: &[u8]) -> Result<Option<String>, Error> {
        self.client
            .request("transaction_v1_broadcast", rpc_params![to_hex(tx)])
            .await
    }

    /// Stop the broadcasting process of the transaction.
    ///
    /// The operation ID is obtained from the [`Self::transaction_v1_broadcast`] method.
    ///
    /// Returns an error if the operation ID does not correspond to any active transaction for this connection.
    pub async fn transaction_v1_stop(&self, operation_id: &str) -> Result<(), Error> {
        self.client
            .request("transaction_v1_stop", rpc_params![operation_id])
            .await
    }

    /// Fetch the block body (ie the extrinsics in the block) given its hash.
    ///
    /// Returns an array of the hexadecimal-encoded scale-encoded extrinsics found in the block,
    /// or `None` if the block wasn't found.
    pub async fn archive_v1_body(&self, block_hash: T::Hash) -> Result<Option<Vec<Bytes>>, Error> {
        self.client
            .request("archive_v1_body", rpc_params![block_hash])
            .await
    }

    /// Call the `archive_v1_call` method and return the response.
    pub async fn archive_v1_call(
        &self,
        block_hash: T::Hash,
        function: &str,
        call_parameters: &[u8],
    ) -> Result<ArchiveCallResult, Error> {
        use serde::de::Error as _;

        // We deserialize to this intermediate shape, since
        // we can't have a boolean tag to denote variants.
        #[derive(Deserialize)]
        struct Response {
            success: bool,
            value: Option<Bytes>,
            error: Option<String>,
            // This was accidentally used instead of value in Substrate,
            // so to support those impls we try it here if needed:
            result: Option<Bytes>,
        }

        let res: Response = self
            .client
            .request(
                "archive_v1_call",
                rpc_params![block_hash, function, to_hex(call_parameters)],
            )
            .await?;

        let value = res.value.or(res.result);
        match (res.success, value, res.error) {
            (true, Some(value), _) => Ok(ArchiveCallResult::Success(value)),
            (false, _, err) => Ok(ArchiveCallResult::Error(err.unwrap_or(String::new()))),
            (true, None, _) => {
                let m = "archive_v1_call: 'success: true' response should have `value: 0x1234` alongside it";
                Err(Error::Deserialization(serde_json::Error::custom(m)))
            }
        }
    }

    /// Return the finalized block height of the chain.
    pub async fn archive_v1_finalized_height(&self) -> Result<usize, Error> {
        self.client
            .request("archive_v1_finalizedHeight", rpc_params![])
            .await
    }

    /// Return the genesis hash.
    pub async fn archive_v1_genesis_hash(&self) -> Result<T::Hash, Error> {
        self.client
            .request("archive_v1_genesisHash", rpc_params![])
            .await
    }

    /// Given a block height, return the hashes of the zero or more blocks at that height.
    /// For blocks older than the latest finalized block, only one entry will be returned. For blocks
    /// newer than the latest finalized block, it's possible to have 0, 1 or multiple blocks at
    /// that height given that forks could occur.
    pub async fn archive_v1_hash_by_height(&self, height: usize) -> Result<Vec<T::Hash>, Error> {
        self.client
            .request("archive_v1_hashByHeight", rpc_params![height])
            .await
    }

    /// Fetch the header for a block with the given hash, or `None` if no block with that hash exists.
    pub async fn archive_v1_header(&self, block_hash: T::Hash) -> Result<Option<T::Header>, Error> {
        let maybe_encoded_header: Option<Bytes> = self
            .client
            .request("archive_v1_header", rpc_params![block_hash])
            .await?;

        let Some(encoded_header) = maybe_encoded_header else {
            return Ok(None);
        };

        let header =
            <T::Header as codec::Decode>::decode(&mut &*encoded_header.0).map_err(Error::Decode)?;
        Ok(Some(header))
    }

    /// Query the node storage and return a subscription which streams corresponding storage events back.
    pub async fn archive_v1_storage(
        &self,
        block_hash: T::Hash,
        items: impl IntoIterator<Item = ArchiveStorageQuery<&[u8]>>,
        child_key: Option<&[u8]>,
    ) -> Result<ArchiveStorageSubscription<T::Hash>, Error> {
        let items: Vec<ArchiveStorageQuery<String>> = items
            .into_iter()
            .map(|item| ArchiveStorageQuery {
                key: to_hex(item.key),
                query_type: item.query_type,
                pagination_start_key: item.pagination_start_key.map(to_hex),
            })
            .collect();

        let sub = self
            .client
            .subscribe(
                "archive_v1_storage",
                rpc_params![block_hash, items, child_key.map(to_hex)],
                "archive_v1_stopStorage",
            )
            .await?;

        Ok(ArchiveStorageSubscription { sub, done: false })
    }
}

/// This represents events generated by the `follow` method.
///
/// The block events are generated in the following order:
/// 1. Initialized - generated only once to signal the latest finalized block
/// 2. NewBlock - a new block was added.
/// 3. BestBlockChanged - indicate that the best block is now the one from this event. The block was
///    announced priorly with the `NewBlock` event.
/// 4. Finalized - State the finalized and pruned blocks.
///
/// The following events are related to operations:
/// - OperationBodyDone: The response of the `chainHead_body`
/// - OperationCallDone: The response of the `chainHead_call`
/// - OperationStorageItems: Items produced by the `chainHead_storage`
/// - OperationWaitingForContinue: Generated after OperationStorageItems and requires the user to
///   call `chainHead_continue`
/// - OperationStorageDone: The `chainHead_storage` method has produced all the results
/// - OperationInaccessible: The server was unable to provide the result, retries might succeed in
///   the future
/// - OperationError: The server encountered an error, retries will not succeed
///
/// The stop event indicates that the JSON-RPC server was unable to provide a consistent list of
/// the blocks at the head of the chain.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum FollowEvent<Hash> {
    /// The latest finalized block.
    ///
    /// This event is generated only once.
    Initialized(Initialized<Hash>),
    /// A new non-finalized block was added.
    NewBlock(NewBlock<Hash>),
    /// The best block of the chain.
    BestBlockChanged(BestBlockChanged<Hash>),
    /// A list of finalized and pruned blocks.
    Finalized(Finalized<Hash>),
    /// The response of the `chainHead_body` method.
    OperationBodyDone(OperationBodyDone),
    /// The response of the `chainHead_call` method.
    OperationCallDone(OperationCallDone),
    /// Yield one or more items found in the storage.
    OperationStorageItems(OperationStorageItems),
    /// Ask the user to call `chainHead_continue` to produce more events
    /// regarding the operation id.
    OperationWaitingForContinue(OperationId),
    /// The responses of the `chainHead_storage` method have been produced.
    OperationStorageDone(OperationId),
    /// The RPC server was unable to provide the response of the following operation id.
    ///
    /// Repeating the same operation in the future might succeed.
    OperationInaccessible(OperationId),
    /// The RPC server encountered an error while processing an operation id.
    ///
    /// Repeating the same operation in the future will not succeed.
    OperationError(OperationError),
    /// The subscription is dropped and no further events
    /// will be generated.
    Stop,
}

/// Contain information about the latest finalized block.
///
/// # Note
///
/// This is the first event generated by the `follow` subscription
/// and is submitted only once.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Initialized<Hash> {
    /// The hashes of the last finalized blocks.
    pub finalized_block_hashes: Vec<Hash>,
    /// The runtime version of the finalized block.
    ///
    /// # Note
    ///
    /// This is present only if the `with_runtime` flag is set for
    /// the `follow` subscription.
    pub finalized_block_runtime: Option<RuntimeEvent>,
}

impl<'de, Hash: Deserialize<'de>> Deserialize<'de> for Initialized<Hash> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Custom struct that can deserialize both `finalizedBlockHash` and `finalizedBlockHashes`.
        #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct InitializedIR<Hash> {
            finalized_block_hashes: Option<Vec<Hash>>,
            finalized_block_hash: Option<Hash>,
            finalized_block_runtime: Option<RuntimeEvent>,
        }

        let ir = InitializedIR::deserialize(deserializer)?;
        let finalized_block_hashes = ir
            .finalized_block_hashes
            .or_else(|| ir.finalized_block_hash.map(|hash| vec![hash]))
            .ok_or_else(|| serde::de::Error::custom("Missing finalized block hashes"))?;

        Ok(Initialized {
            finalized_block_hashes,
            finalized_block_runtime: ir.finalized_block_runtime,
        })
    }
}

/// The runtime event generated if the `follow` subscription
/// has set the `with_runtime` flag.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RuntimeEvent {
    /// The runtime version of this block.
    Valid(RuntimeVersionEvent),
    /// The runtime could not be obtained due to an error.
    Invalid(ErrorEvent),
}

/// The runtime specification of the current block.
///
/// This event is generated for:
///   - the first announced block by the follow subscription
///   - blocks that suffered a change in runtime compared with their parents
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersionEvent {
    /// Details about this runtime.
    pub spec: RuntimeSpec,
}

/// This contains the runtime version information necessary to make transactions, and is obtained from
/// the "initialized" event of `chainHead_follow` if the `withRuntime` flag is set.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeSpec {
    /// Opaque string indicating the name of the chain.
    pub spec_name: String,

    /// Opaque string indicating the name of the implementation of the chain.
    pub impl_name: String,

    /// Opaque integer. The JSON-RPC client can assume that the Runtime API call to `Metadata_metadata`
    /// will always produce the same output as long as the specVersion is the same.
    pub spec_version: u32,

    /// Opaque integer. Whenever the runtime code changes in a backwards-compatible way, the implVersion
    /// is modified while the specVersion is left untouched.
    pub impl_version: u32,

    /// Opaque integer. Necessary when building the bytes of a transaction. Transactions that have been
    /// generated with a different `transaction_version` are incompatible.
    pub transaction_version: u32,

    /// Object containing a list of "entry point APIs" supported by the runtime. Each key is an opaque string
    /// indicating the API, and each value is an integer version number. Before making a runtime call (using
    /// chainHead_call), you should make sure that this list contains the entry point API corresponding to the
    /// call and with a known version number.
    ///
    /// **Note:** In Substrate, the keys in the apis field consists of the hexadecimal-encoded 8-bytes blake2
    /// hash of the name of the API. For example, the `TaggedTransactionQueue` API is 0xd2bc9897eed08f15.
    #[serde(with = "hashmap_as_tuple_list")]
    pub apis: HashMap<String, u32>,
}

/// The operation could not be processed due to an error.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
    /// Reason of the error.
    pub error: String,
}

/// Indicate a new non-finalized block.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBlock<Hash> {
    /// The hash of the new block.
    pub block_hash: Hash,
    /// The parent hash of the new block.
    pub parent_block_hash: Hash,
    /// The runtime version of the new block.
    ///
    /// # Note
    ///
    /// This is present only if the `with_runtime` flag is set for
    /// the `follow` subscription.
    pub new_runtime: Option<RuntimeEvent>,
}

/// Indicate the block hash of the new best block.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BestBlockChanged<Hash> {
    /// The block hash of the new best block.
    pub best_block_hash: Hash,
}

/// Indicate the finalized and pruned block hashes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Finalized<Hash> {
    /// Block hashes that are finalized.
    pub finalized_block_hashes: Vec<Hash>,
    /// Block hashes that are pruned (removed).
    pub pruned_block_hashes: Vec<Hash>,
}

/// Indicate the operation id of the event.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationId {
    /// The operation id of the event.
    pub operation_id: String,
}

/// The response of the `chainHead_body` method.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationBodyDone {
    /// The operation id of the event.
    pub operation_id: String,
    /// Array of hexadecimal-encoded scale-encoded extrinsics found in the block.
    pub value: Vec<Bytes>,
}

/// The response of the `chainHead_call` method.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationCallDone {
    /// The operation id of the event.
    pub operation_id: String,
    /// Hexadecimal-encoded output of the runtime function call.
    pub output: Bytes,
}

/// The response of the `chainHead_call` method.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationStorageItems {
    /// The operation id of the event.
    pub operation_id: String,
    /// The resulting items.
    pub items: VecDeque<StorageResult>,
}

/// Indicate a problem during the operation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationError {
    /// The operation id of the event.
    pub operation_id: String,
    /// The reason of the error.
    pub error: String,
}

/// The storage result.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageResult {
    /// The hex-encoded key of the result.
    pub key: Bytes,
    /// The result of the query.
    #[serde(flatten)]
    pub result: StorageResultType,
}

/// The type of the storage query.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StorageResultType {
    /// Fetch the value of the provided key.
    Value(Bytes),
    /// Fetch the hash of the value of the provided key.
    Hash(Bytes),
    /// Fetch the closest descendant merkle value.
    ClosestDescendantMerkleValue(Bytes),
}

/// The method response of `chainHead_body`, `chainHead_call` and `chainHead_storage`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "result")]
pub enum MethodResponse {
    /// The method has started.
    Started(MethodResponseStarted),
    /// The RPC server cannot handle the request at the moment.
    LimitReached,
}

/// The `started` result of a method.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodResponseStarted {
    /// The operation id of the response.
    pub operation_id: String,
    /// The number of items from the back of the `chainHead_storage` that have been discarded.
    pub discarded_items: Option<usize>,
}

/// The storage item received as parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageQuery<Key> {
    /// The provided key.
    pub key: Key,
    /// The type of the storage query.
    #[serde(rename = "type")]
    pub query_type: StorageQueryType,
}

/// The storage item received as parameter. This is used archive storage queries, and
/// unlike [`StorageQuery`] also contains `paginationStartKey` to define where iteration
/// should begin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveStorageQuery<Key> {
    /// The provided key.
    pub key: Key,
    /// The type of the storage query.
    #[serde(rename = "type")]
    pub query_type: StorageQueryType,
    /// This parameter is optional and should be a string containing the hexadecimal-encoded key
    /// from which the storage iteration should resume. This parameter is only valid in the context
    /// of `descendantsValues` and `descendantsHashes`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_start_key: Option<Key>,
}

/// The type of the storage query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StorageQueryType {
    /// Fetch the value of the provided key.
    Value,
    /// Fetch the hash of the value of the provided key.
    Hash,
    /// Fetch the closest descendant merkle value.
    ClosestDescendantMerkleValue,
    /// Fetch the values of all descendants of they provided key.
    DescendantsValues,
    /// Fetch the hashes of the values of all descendants of they provided key.
    DescendantsHashes,
}

/// A subscription which returns follow events, and ends when a Stop event occurs.
pub struct FollowSubscription<Hash> {
    sub: RpcSubscription<FollowEvent<Hash>>,
    done: bool,
}

impl<H: Hash> FollowSubscription<H> {
    /// Fetch the next item in the stream.
    pub async fn next(&mut self) -> Option<<Self as Stream>::Item> {
        <Self as StreamExt>::next(self).await
    }
    /// Fetch the subscription ID for the stream.
    pub fn subscription_id(&self) -> Option<&str> {
        self.sub.subscription_id()
    }
}

impl<H: Hash> Stream for FollowSubscription<H> {
    type Item = <RpcSubscription<FollowEvent<H>> as Stream>::Item;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        let res = self.sub.poll_next_unpin(cx);

        if let Poll::Ready(Some(Ok(FollowEvent::Stop))) = &res {
            // No more events will occur after this one.
            self.done = true;
        }

        res
    }
}

/// A subscription which returns transaction status events, stopping
/// when no more events will be sent.
pub struct TransactionSubscription<Hash> {
    sub: RpcSubscription<TransactionStatus<Hash>>,
    done: bool,
}

impl<H: Hash> TransactionSubscription<H> {
    /// Fetch the next item in the stream.
    pub async fn next(&mut self) -> Option<<Self as Stream>::Item> {
        <Self as StreamExt>::next(self).await
    }
}

impl<H: Hash> Stream for TransactionSubscription<H> {
    type Item = <RpcSubscription<TransactionStatus<H>> as Stream>::Item;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        let res = self.sub.poll_next_unpin(cx);

        if let Poll::Ready(Some(Ok(res))) = &res {
            if matches!(
                res,
                TransactionStatus::Dropped { .. }
                    | TransactionStatus::Error { .. }
                    | TransactionStatus::Invalid { .. }
                    | TransactionStatus::Finalized { .. }
            ) {
                // No more events will occur after these ones.
                self.done = true
            }
        }

        res
    }
}

/// Transaction progress events
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum TransactionStatus<Hash> {
    /// Transaction is part of the future queue.
    Validated,
    /// The transaction has been broadcast to other nodes.
    ///
    /// Note: This event is no longer expected to be returned as of
    /// the chainHead_v1 spec, but we do so for compatibility with
    /// older versions of Smoldot, which do return it.
    Broadcasted,
    /// Transaction has been included in block with given details.
    /// Null is returned if the transaction is no longer in any block
    /// of the best chain.
    BestChainBlockIncluded {
        /// Details of the block it's been seen in.
        block: Option<TransactionBlockDetails<Hash>>,
    },
    /// The transaction is in a block that's been finalized.
    Finalized {
        /// Details of the block it's been seen in.
        block: TransactionBlockDetails<Hash>,
    },
    /// Something went wrong in the node.
    Error {
        /// Human readable message; what went wrong.
        error: String,
    },
    /// Transaction is invalid (bad nonce, signature etc).
    Invalid {
        /// Human readable message; why was it invalid.
        error: String,
    },
    /// The transaction was dropped.
    Dropped {
        /// Human readable message; why was it dropped.
        error: String,
    },
}

/// Details of a block that a transaction is seen in.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionBlockDetails<Hash> {
    /// The block hash.
    pub hash: Hash,
    /// The index of the transaction in the block.
    #[serde(with = "unsigned_number_as_string")]
    pub index: u64,
}

/// The response from calling `archive_call`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveCallResult {
    /// The bytes returned from successfully making a call
    Success(Bytes),
    /// An error returned if the call was not successful.
    Error(String),
}

impl ArchiveCallResult {
    /// Return the bytes on success, or `None` if not an [`ArchiveCallResult::Success`].
    pub fn as_success(self) -> Option<Bytes> {
        match self {
            ArchiveCallResult::Success(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Return the error message on call failure, or `None` if not an [`ArchiveCallResult::Error`].
    pub fn as_error(self) -> Option<String> {
        match self {
            ArchiveCallResult::Success(_) => None,
            ArchiveCallResult::Error(e) => Some(e),
        }
    }
}

/// A subscription which returns follow events, and ends when a Stop event occurs.
pub struct ArchiveStorageSubscription<Hash> {
    sub: RpcSubscription<ArchiveStorageEvent<Hash>>,
    done: bool,
}

impl<H: Hash> ArchiveStorageSubscription<H> {
    /// Fetch the next item in the stream.
    pub async fn next(&mut self) -> Option<<Self as Stream>::Item> {
        <Self as StreamExt>::next(self).await
    }
    /// Fetch the subscription ID for the stream.
    pub fn subscription_id(&self) -> Option<&str> {
        self.sub.subscription_id()
    }
}

impl<H: Hash> Stream for ArchiveStorageSubscription<H> {
    type Item = <RpcSubscription<ArchiveStorageEvent<H>> as Stream>::Item;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        let res = self.sub.poll_next_unpin(cx);

        if let Poll::Ready(Some(Ok(ArchiveStorageEvent::Done | ArchiveStorageEvent::Error(..)))) =
            &res
        {
            // No more events will occur after "done" or "error" events.
            self.done = true;
        }

        res
    }
}

/// Responses returned from [`ArchiveStorageSubscription`].
#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum ArchiveStorageEvent<Hash> {
    /// A storage response for one of the requested items.
    #[serde(rename = "storage")]
    Item(ArchiveStorageEventItem<Hash>),
    /// A human-readable error indicating what went wrong. No more storage events
    /// will be emitted after this.
    #[serde(rename = "storageError")]
    Error(ArchiveStorageEventError),
    /// No more storage events will be emitted after this.
    #[serde(rename = "storageDone")]
    Done,
}

impl<Hash> ArchiveStorageEvent<Hash> {
    /// Return a storage item or `None` if not an [`ArchiveStorageEvent::Item`].
    pub fn as_item(self) -> Option<ArchiveStorageEventItem<Hash>> {
        match self {
            ArchiveStorageEvent::Item(item) => Some(item),
            _ => None,
        }
    }

    /// Return a storage error or `None` if not an [`ArchiveStorageEvent::Error`].
    pub fn as_error(self) -> Option<ArchiveStorageEventError> {
        match self {
            ArchiveStorageEvent::Error(e) => Some(e),
            _ => None,
        }
    }

    /// Is this an [`ArchiveStorageEvent::Done`].
    pub fn is_done(self) -> bool {
        matches!(self, ArchiveStorageEvent::Done)
    }
}

/// Something went wrong during the [`ChainHeadRpcMethods::archive_v1_storage()`] subscription.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveStorageEventError {
    /// The human readable error message indicating what went wrong.
    pub error: String,
}

/// A storage item returned from the [`ChainHeadRpcMethods::archive_v1_storage()`] subscription.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveStorageEventItem<Hash> {
    /// String containing the hexadecimal-encoded key of the storage entry.
    pub key: Bytes,
    /// String containing the hexadecimal-encoded value of the storage entry.
    /// Returned when the request type is [`StorageQueryType::Value`] or [`StorageQueryType::DescendantsValues`].
    pub value: Option<Bytes>,
    /// String containing the hexadecimal-encoded hash of the storage entry.
    /// Returned when the request type is [`StorageQueryType::Hash`] or [`StorageQueryType::DescendantsHashes`].
    pub hash: Option<Hash>,
    /// String containing the hexadecimal-encoded Merkle value of the closest descendant of key (including branch nodes).
    /// Returned when the request type is [`StorageQueryType::ClosestDescendantMerkleValue`].
    pub closest_descendant_merkle_value: Option<Bytes>,
    /// String containing the hexadecimal-encoded key of the child trie of the "default" namespace if the storage entry
    /// is part of a child trie. If the storage entry is part of the main trie, this field is not present.
    pub child_trie_key: Option<Bytes>,
}

/// Hex-serialized shim for `Vec<u8>`.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Hash, PartialOrd, Ord, Debug)]
pub struct Bytes(#[serde(with = "impl_serde::serialize")] pub Vec<u8>);
impl std::ops::Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0[..]
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(s: Vec<u8>) -> Self {
        Bytes(s)
    }
}

fn to_hex(bytes: impl AsRef<[u8]>) -> String {
    format!("0x{}", hex::encode(bytes.as_ref()))
}

/// Attempt to deserialize either a string or integer into an integer.
/// See <https://github.com/paritytech/json-rpc-interface-spec/issues/83>
pub(crate) mod unsigned_number_as_string {
    use serde::de::{Deserializer, Visitor};
    use std::fmt;

    /// Deserialize a number from a string or number.
    pub fn deserialize<'de, N: From<u64>, D>(deserializer: D) -> Result<N, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberVisitor(std::marker::PhantomData))
    }

    struct NumberVisitor<N>(std::marker::PhantomData<N>);

    impl<N: From<u64>> Visitor<'_> for NumberVisitor<N> {
        type Value = N;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an unsigned integer or a string containing one")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            let n: u64 = v.parse().map_err(serde::de::Error::custom)?;
            Ok(n.into())
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v.into())
        }
    }

    use serde::ser::Serializer;

    /// Serialize a number as string
    pub fn serialize<S>(item: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&item.to_string())
    }
}

/// A temporary shim to decode "spec.apis" if it comes back as an array like:
///
/// ```text
/// [["0xABC", 1], ["0xCDE", 2]]
/// ```
///
/// The expected format (which this also supports deserializing from) is:
///
/// ```text
/// { "0xABC": 1, "0xCDE": 2 }
/// ```
///
/// We can delete this when the correct format is being returned.
///
/// Adapted from <https://tikv.github.io/doc/serde_with/rust/hashmap_as_tuple_list>
pub(crate) mod hashmap_as_tuple_list {
    use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
    use std::collections::HashMap;
    use std::fmt;
    use std::hash::{BuildHasher, Hash};
    use std::marker::PhantomData;

    /// Deserialize a [`HashMap`] from a list of tuples or object
    pub fn deserialize<'de, K, V, BH, D>(deserializer: D) -> Result<HashMap<K, V, BH>, D::Error>
    where
        D: Deserializer<'de>,
        K: Eq + Hash + Deserialize<'de>,
        V: Deserialize<'de>,
        BH: BuildHasher + Default,
    {
        deserializer.deserialize_any(HashMapVisitor(PhantomData))
    }

    #[allow(clippy::type_complexity)]
    struct HashMapVisitor<K, V, BH>(PhantomData<fn() -> HashMap<K, V, BH>>);

    impl<'de, K, V, BH> Visitor<'de> for HashMapVisitor<K, V, BH>
    where
        K: Deserialize<'de> + Eq + Hash,
        V: Deserialize<'de>,
        BH: BuildHasher + Default,
    {
        type Value = HashMap<K, V, BH>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list of key-value pairs")
        }

        // Work with maps too:
        fn visit_map<A>(self, mut m: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut map =
                HashMap::with_capacity_and_hasher(m.size_hint().unwrap_or(0), BH::default());
            while let Some((key, value)) = m.next_entry()? {
                map.insert(key, value);
            }
            Ok(map)
        }

        // The shim to also work with sequences of tuples.
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut map =
                HashMap::with_capacity_and_hasher(seq.size_hint().unwrap_or(0), BH::default());
            while let Some((key, value)) = seq.next_element()? {
                map.insert(key, value);
            }
            Ok(map)
        }
    }

    use serde::ser::{Serialize, SerializeSeq, Serializer};

    /// Serialize hashmap as list of tuples
    pub fn serialize<S, K: Eq + Hash + Serialize, V: Serialize>(
        item: &HashMap<K, V>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for i in item {
            seq.serialize_element(&i)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_deserialize_apis_from_tuple_or_object() {
        let old_response = serde_json::json!({
            "authoringVersion": 10,
            "specName": "westend",
            "implName": "parity-westend",
            "specVersion": 9122,
            "implVersion": 0,
            "stateVersion": 1,
            "transactionVersion": 7,
            "apis": [
                ["0xdf6acb689907609b", 3],
                ["0x37e397fc7c91f5e4", 1],
                ["0x40fe3ad401f8959a", 5],
                ["0xd2bc9897eed08f15", 3],
                ["0xf78b278be53f454c", 2],
                ["0xaf2c0297a23e6d3d", 1],
                ["0x49eaaf1b548a0cb0", 1],
                ["0x91d5df18b0d2cf58", 1],
                ["0xed99c5acb25eedf5", 3],
                ["0xcbca25e39f142387", 2],
                ["0x687ad44ad37f03c2", 1],
                ["0xab3c0572291feb8b", 1],
                ["0xbc9d89904f5b923f", 1],
                ["0x37c8bb1350a9a2a8", 1]
            ]
        });
        let old_spec: RuntimeSpec = serde_json::from_value(old_response).unwrap();

        let new_response = serde_json::json!({
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
        let new_spec: RuntimeSpec = serde_json::from_value(new_response).unwrap();

        assert_eq!(old_spec, new_spec);
    }

    #[test]
    fn can_deserialize_from_number_or_string() {
        #[derive(Debug, Deserialize)]
        struct Foo64 {
            #[serde(with = "super::unsigned_number_as_string")]
            num: u64,
        }
        #[derive(Debug, Deserialize)]
        struct Foo32 {
            #[serde(with = "super::unsigned_number_as_string")]
            num: u128,
        }

        let from_string = serde_json::json!({
            "num": "123"
        });
        let from_num = serde_json::json!({
            "num": 123
        });
        let from_err = serde_json::json!({
            "num": "123a"
        });

        let f1: Foo64 =
            serde_json::from_value(from_string.clone()).expect("can deser string into u64");
        let f2: Foo32 = serde_json::from_value(from_string).expect("can deser string into u32");
        let f3: Foo64 = serde_json::from_value(from_num.clone()).expect("can deser num into u64");
        let f4: Foo32 = serde_json::from_value(from_num).expect("can deser num into u32");

        assert_eq!(f1.num, 123);
        assert_eq!(f2.num, 123);
        assert_eq!(f3.num, 123);
        assert_eq!(f4.num, 123);

        // Invalid things should lead to an error:
        let _ = serde_json::from_value::<Foo32>(from_err)
            .expect_err("can't deser invalid num into u32");
    }

    #[test]
    fn chain_head_initialized() {
        // Latest format version.
        let event = serde_json::json!({
            "finalizedBlockHashes": ["0x1", "0x2"],
        });
        let decoded: Initialized<String> = serde_json::from_value(event).unwrap();
        assert_eq!(
            decoded.finalized_block_hashes,
            vec!["0x1".to_string(), "0x2".to_string()]
        );

        // Old format.
        let event = serde_json::json!({
            "finalizedBlockHash": "0x1",
        });
        let decoded: Initialized<String> = serde_json::from_value(event).unwrap();
        assert_eq!(decoded.finalized_block_hashes, vec!["0x1".to_string()]);

        // Wrong format.
        let event = serde_json::json!({
            "finalizedBlockHash": ["0x1"],
        });
        let _ = serde_json::from_value::<Initialized<String>>(event).unwrap_err();
    }
}
