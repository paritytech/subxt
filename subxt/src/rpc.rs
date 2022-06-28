// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! RPC types and client for interacting with a substrate node.

// jsonrpsee subscriptions are interminable.
// Allows `while let status = subscription.next().await {}`
// Related: https://github.com/paritytech/subxt/issues/66
#![allow(irrefutable_let_patterns)]

use std::{
    collections::HashMap,
    sync::Arc,
};

use crate::{
    error::BasicError,
    subscription::{
        EventStorageSubscription,
        FinalizedEventStorageSubscription,
    },
    Config,
};
use core::marker::PhantomData;
use jsonrpsee::core::RpcResult;
pub use jsonrpsee::{
    client_transport::ws::{
        InvalidUri,
        Receiver as WsReceiver,
        Sender as WsSender,
        Uri,
        WsTransportClientBuilder,
    },
    core::{
        client::{
            Client as RpcClient,
            ClientBuilder as RpcClientBuilder,
            ClientT,
            Subscription,
            SubscriptionClientT,
        },
        to_json_value,
        DeserializeOwned,
        Error as RpcError,
        JsonValue,
    },
    proc_macros::rpc,
    rpc_params,
};
use serde::{
    Deserialize,
    Serialize,
};
use sp_core::{
    storage::{
        StorageChangeSet,
        StorageData,
        StorageKey,
    },
    Bytes,
    U256,
};
use sp_runtime::generic::{
    Block,
    SignedBlock,
};

/// subxt RPC API.
#[rpc(
    client,
    client_bounds(
        C::Hash: DeserializeOwned + Serialize, C::Header: Serialize, C::Extrinsic: Serialize)
)]
pub trait SubxtRpcApi<C: Config> {
    /// Fetch a storage key
    #[method(name = "state_getStorage")]
    async fn storage(
        &self,
        key: &StorageKey,
        hash: Option<C::Hash>,
    ) -> RpcResult<Option<StorageData>>;

    /// Returns the keys with prefix with pagination support.
    /// Up to `count` keys will be returned.
    /// If `start_key` is passed, return next keys in storage in lexicographic order.
    #[method(name = "state_getKeysPaged")]
    async fn storage_keys_paged(
        &self,
        prefix: Option<StorageKey>,
        count: u32,
        start_key: Option<StorageKey>,
        hash: Option<C::Hash>,
    ) -> RpcResult<Vec<StorageKey>>;

    /// Query historical storage entries
    #[method(name = "state_queryStorage")]
    async fn query_storage(
        &self,
        keys: Vec<StorageKey>,
        from: C::Hash,
        to: Option<C::Hash>,
    ) -> RpcResult<Vec<StorageChangeSet<C::Hash>>>;

    /// Query historical storage entries
    #[method(name = "state_queryStorageAt")]
    async fn query_storage_at(
        &self,
        keys: &[StorageKey],
        at: Option<C::Hash>,
    ) -> RpcResult<Vec<StorageChangeSet<C::Hash>>>;

    /// Fetch the genesis hash
    #[method(name = "chain_getBlockHash")]
    async fn genesis_hash(&self) -> RpcResult<C::Hash>;

    /// Fetch the metadata as bytes.
    #[method(name = "state_getMetadata")]
    async fn metadata(&self) -> RpcResult<sp_core::Bytes>;

    /// Fetch system properties
    #[method(name = "system_properties")]
    async fn system_properties(&self) -> RpcResult<SystemProperties>;

    /// Fetch system chain
    #[method(name = "system_chain")]
    async fn system_chain(&self) -> RpcResult<String>;

    /// Fetch system name
    #[method(name = "system_name")]
    async fn system_name(&self) -> RpcResult<String>;

    /// Fetch system version
    #[method(name = "system_version")]
    async fn system_version(&self) -> RpcResult<String>;

    /// Fetch the runtime version
    #[method(name = "state_getRuntimeVersion")]
    async fn runtime_version(&self, at: Option<C::Hash>) -> RpcResult<RuntimeVersion>;

    /// Get a header
    #[method(name = "state_getRuntimeVersion")]
    async fn header(&self, hash: Option<C::Hash>) -> RpcResult<Option<C::Header>>;

    /// Get a block hash, returns hash of latest block by default
    #[method(name = "chain_getBlockHash")]
    async fn block_hash(
        &self,
        block_number: Option<BlockNumber>,
    ) -> RpcResult<Option<C::Hash>>;

    /// Get a block hash of the latest finalized block
    #[method(name = "chain_getFinalizedHead")]
    async fn finalized_head(&self) -> RpcResult<C::Hash>;

    /// Get proof of storage entries at a specific block's state.
    #[method(name = "state_getReadProof")]
    async fn read_proof(
        &self,
        keys: Vec<StorageKey>,
        hash: Option<C::Hash>,
    ) -> RpcResult<ReadProof<C::Hash>>;

    /// Get a Block
    #[method(name = "chain_getBlock")]
    async fn block(
        &self,
        hash: Option<C::Hash>,
    ) -> RpcResult<Option<SignedBlock<Block<C::Header, C::Extrinsic>>>>;

    /// Insert a key into the keystore.
    #[method(name = "author_insertKey")]
    async fn insert_key(
        &self,
        key_type: String,
        suri: String,
        public: Bytes,
    ) -> RpcResult<()>;

    /// Generate new session keys and returns the corresponding public keys.
    #[method(name = "author_rotateKeys")]
    async fn rotate_keys(&self) -> RpcResult<Bytes>;

    /// Checks if the keystore has private keys for the given session public keys.
    ///
    /// `session_keys` is the SCALE encoded session keys object from the runtime.
    ///
    /// Returns `true` iff all private keys could be found.
    #[method(name = "author_hasSessionKeys")]
    async fn has_session_keys(&self, session_keys: Bytes) -> RpcResult<bool>;

    /// Checks if the keystore has private keys for the given public key and key type.
    ///
    /// Returns `true` if a private key could be found.
    #[method(name = "author_hasKey")]
    async fn has_key(&self, public_key: Bytes, key_type: String) -> RpcResult<bool>;

    /// Create and submit an extrinsic and return corresponding Hash if successful
    #[method(name = "author_submitExtrinsic")]
    async fn submit_extrinsic(&self, extrinsic: Bytes) -> RpcResult<C::Hash>;

    /// Subscribe to System Events that are imported into blocks.
    ///
    /// *WARNING* these may not be included in the finalized chain, use
    /// `subscribe_finalized_events` to ensure events are finalized.
    #[subscription(
        name = "state_subscribeStorage", 
        unsubscribe = "state_unsubscribeStorage",
        item = StorageChangeSet<C::Hash>
    )]
    fn subscribe_events(&self);

    /// Subscribe to blocks.
    #[subscription(
        name = "chain_subscribeNewHeads", 
        unsubscribe = "chain_unsubscribeNewHeads",
        item = C::Header)]
    fn subscribe_blocks(&self);

    /// Subscribe to finalized blocks.
    #[subscription(
        name = "chain_subscribeFinalizedHeads", 
        unsubscribe = "chain_unsubscribeFinalizedHeads",
        item = C::Header
    )]
    fn subscribe_finalized_blocks(&self);

    /// Create and submit an extrinsic and return a subscription to the events triggered.
    #[subscription(
        name = "author_submitAndWatchExtrinsic",
        unsubscribe = "author_unwatchExtrinsic",
        item = SubstrateTransactionStatus<C::Hash, C::Hash>
    )]
    fn watch_extrinsic<X: Encode>(&self, xt: Bytes);
}

/// A number type that can be serialized both as a number or a string that encodes a number in a
/// string.
///
/// We allow two representations of the block number as input. Either we deserialize to the type
/// that is specified in the block type or we attempt to parse given hex value.
///
/// The primary motivation for having this type is to avoid overflows when using big integers in
/// JavaScript (which we consider as an important RPC API consumer).
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum NumberOrHex {
    /// The number represented directly.
    Number(u64),
    /// Hex representation of the number.
    Hex(U256),
}

/// RPC list or value wrapper.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ListOrValue<T> {
    /// A list of values of given type.
    List(Vec<T>),
    /// A single value of given type.
    Value(T),
}

/// Alias for the type of a block returned by `chain_getBlock`
pub type ChainBlock<T> =
    SignedBlock<Block<<T as Config>::Header, <T as Config>::Extrinsic>>;

/// Wrapper for NumberOrHex to allow custom From impls
#[derive(Serialize)]
pub struct BlockNumber(NumberOrHex);

impl From<NumberOrHex> for BlockNumber {
    fn from(x: NumberOrHex) -> Self {
        BlockNumber(x)
    }
}

impl From<u32> for BlockNumber {
    fn from(x: u32) -> Self {
        NumberOrHex::Number(x.into()).into()
    }
}

/// Arbitrary properties defined in the chain spec as a JSON object.
pub type SystemProperties = serde_json::Map<String, serde_json::Value>;

/// Possible transaction status events.
///
/// # Note
///
/// This is copied from `sp-transaction-pool` to avoid a dependency on that crate. Therefore it
/// must be kept compatible with that type from the target substrate version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubstrateTransactionStatus<Hash, BlockHash> {
    /// Transaction is part of the future queue.
    Future,
    /// Transaction is part of the ready queue.
    Ready,
    /// The transaction has been broadcast to the given peers.
    Broadcast(Vec<String>),
    /// Transaction has been included in block with given hash.
    InBlock(BlockHash),
    /// The block this transaction was included in has been retracted.
    Retracted(BlockHash),
    /// Maximum number of finality watchers has been reached,
    /// old watchers are being removed.
    FinalityTimeout(BlockHash),
    /// Transaction has been finalized by a finality-gadget, e.g GRANDPA
    Finalized(BlockHash),
    /// Transaction has been replaced in the pool, by another transaction
    /// that provides the same tags. (e.g. same (sender, nonce)).
    Usurped(Hash),
    /// Transaction has been dropped from the pool because of the limit.
    Dropped,
    /// Transaction is no longer valid in the current state.
    Invalid,
}

/// This contains the runtime version information necessary to make transactions, as obtained from
/// the RPC call `state_getRuntimeVersion`,
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
    /// Version of the runtime specification. A full-node will not attempt to use its native
    /// runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    /// `spec_version` and `authoring_version` are the same between Wasm and native.
    pub spec_version: u32,

    /// All existing dispatches are fully compatible when this number doesn't change. If this
    /// number changes, then `spec_version` must change, also.
    ///
    /// This number must change when an existing dispatchable (module ID, dispatch ID) is changed,
    /// either through an alteration in its user-level semantics, a parameter
    /// added/removed/changed, a dispatchable being removed, a module being removed, or a
    /// dispatchable/module changing its index.
    ///
    /// It need *not* change when a new module is added or when a dispatchable is added.
    pub transaction_version: u32,

    /// The other fields present may vary and aren't necessary for `subxt`; they are preserved in
    /// this map.
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

/// ReadProof struct returned by the RPC
///
/// # Note
///
/// This is copied from `sc-rpc-api` to avoid a dependency on that crate. Therefore it
/// must be kept compatible with that type from the target substrate version.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadProof<Hash> {
    /// Block hash used to generate the proof
    pub at: Hash,
    /// A proof used to prove that storage entries are included in the storage trie
    pub proof: Vec<Bytes>,
}

/// Client for substrate rpc interfaces
pub struct Rpc<T: Config> {
    /// Rpc client for sending requests.
    pub client: Arc<RpcClient>,
    marker: PhantomData<T>,
}

impl<T: Config> Clone for Rpc<T> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Config> Rpc<T> {
    /// Create a new [`Rpc`]
    pub fn new(client: RpcClient) -> Self {
        Self {
            client: Arc::new(client),
            marker: PhantomData,
        }
    }

    /// Get a reference to the client make calls.
    pub fn inner(&self) -> &RpcClient {
        &*self.client
    }

    /// Subscribe to finalized events.
    pub async fn subscribe_finalized_events(
        &self,
    ) -> Result<EventStorageSubscription<T>, BasicError> {
        Ok(EventStorageSubscription::Finalized(
            FinalizedEventStorageSubscription::new(
                self.clone(),
                SubxtRpcApiClient::<T>::subscribe_finalized_blocks(&*self.client).await?,
            ),
        ))
    }
}

/// Build WS RPC client from URL
pub async fn ws_client(url: &str) -> Result<RpcClient, RpcError> {
    let (sender, receiver) = ws_transport(url).await?;
    Ok(RpcClientBuilder::default()
        .max_notifs_per_subscription(4096)
        .build_with_tokio(sender, receiver))
}

async fn ws_transport(url: &str) -> Result<(WsSender, WsReceiver), RpcError> {
    let url: Uri = url
        .parse()
        .map_err(|e: InvalidUri| RpcError::Transport(e.into()))?;
    WsTransportClientBuilder::default()
        .build(url)
        .await
        .map_err(|e| RpcError::Transport(e.into()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deser_runtime_version() {
        let val: RuntimeVersion = serde_json::from_str(
            r#"{
            "specVersion": 123,
            "transactionVersion": 456,
            "foo": true,
            "wibble": [1,2,3]
        }"#,
        )
        .expect("deserializing failed");

        let mut m = std::collections::HashMap::new();
        m.insert("foo".to_owned(), serde_json::json!(true));
        m.insert("wibble".to_owned(), serde_json::json!([1, 2, 3]));

        assert_eq!(
            val,
            RuntimeVersion {
                spec_version: 123,
                transaction_version: 456,
                other: m
            }
        );
    }
}
