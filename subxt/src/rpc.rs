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
//!
//! This is used behind the scenes by various `subxt` APIs, but can
//! also be used directly.
//!
//! # Examples
//!
//! ## Fetch Storage
//!
//! ```no_run
//! # use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
//! # use subxt::storage::StorageKeyPrefix;
//! # use subxt::rpc::Rpc;
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! # #[tokio::main]
//! # async fn main() {
//! # let api = ClientBuilder::new()
//! #     .build()
//! #     .await
//! #     .unwrap()
//! #     .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
//! // Storage prefix is `twox_128("System") ++ twox_128("ExtrinsicCount")`.
//! let key = StorageKeyPrefix::new::<polkadot::system::storage::ExtrinsicCount>()
//!     .to_storage_key();
//!
//! // Obtain the RPC from a generated API
//! let rpc: &Rpc<_> = api
//!     .client
//!     .rpc();
//!
//! let result = rpc.storage(&key, None).await.unwrap();
//! println!("Storage result: {:?}", result);
//! # }
//! ```
//!
//! ## Fetch Keys
//!
//! ```no_run
//! # use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
//! # use subxt::storage::StorageKeyPrefix;
//! # use subxt::rpc::Rpc;
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! # #[tokio::main]
//! # async fn main() {
//! # let api = ClientBuilder::new()
//! #     .build()
//! #     .await
//! #     .unwrap()
//! #     .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
//! let key = StorageKeyPrefix::new::<polkadot::xcm_pallet::storage::VersionNotifiers>()
//!     .to_storage_key();
//!
//! // Obtain the RPC from a generated API
//! let rpc: &Rpc<_> = api
//!     .client
//!     .rpc();
//!
//! // Fetch up to 10 keys.
//! let keys = rpc
//!     .storage_keys_paged(Some(key), 10, None, None)
//!     .await
//!     .unwrap();
//!
//! for key in keys.iter() {
//!     println!("Key: 0x{}", hex::encode(&key));
//! }
//! # }
//! ```

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
    Config,
    Metadata,
    PhantomDataSendSync,
};
use codec::{
    Decode,
    Encode,
};
use frame_metadata::RuntimeMetadataPrefixed;
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
use sp_runtime::{
    generic::{
        Block,
        SignedBlock,
    },
    ApplyExtrinsicResult,
};

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

// All unsigned ints can be converted into a BlockNumber:
macro_rules! into_block_number {
    ($($t: ty)+) => {
        $(
            impl From<$t> for BlockNumber {
                fn from(x: $t) -> Self {
                    NumberOrHex::Number(x.into()).into()
                }
            }
        )+
    }
}
into_block_number!(u8 u16 u32 u64);

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

/// Statistics of a block returned by the `dev_getBlockStats` RPC.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStats {
    /// The length in bytes of the storage proof produced by executing the block.
    pub witness_len: u64,
    /// The length in bytes of the storage proof after compaction.
    pub witness_compact_len: u64,
    /// Length of the block in bytes.
    ///
    /// This information can also be acquired by downloading the whole block. This merely
    /// saves some complexity on the client side.
    pub block_len: u64,
    /// Number of extrinsics in the block.
    ///
    /// This information can also be acquired by downloading the whole block. This merely
    /// saves some complexity on the client side.
    pub num_extrinsics: u64,
}

/// Health struct returned by the RPC
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    /// Number of connected peers
    pub peers: usize,
    /// Is the node syncing
    pub is_syncing: bool,
    /// Should this node have any peers
    ///
    /// Might be false for local chains or when running without discovery.
    pub should_have_peers: bool,
}

/// Client for substrate rpc interfaces
pub struct Rpc<T: Config> {
    /// Rpc client for sending requests.
    pub client: Arc<RpcClient>,
    _marker: PhantomDataSendSync<T>,
}

impl<T: Config> Clone for Rpc<T> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            _marker: PhantomDataSendSync::new(),
        }
    }
}

impl<T: Config> Rpc<T> {
    /// Create a new [`Rpc`]
    pub fn new(client: RpcClient) -> Self {
        Self {
            client: Arc::new(client),
            _marker: PhantomDataSendSync::new(),
        }
    }

    /// Fetch a storage key
    pub async fn storage(
        &self,
        key: &StorageKey,
        hash: Option<T::Hash>,
    ) -> Result<Option<StorageData>, BasicError> {
        let params = rpc_params![key, hash];
        let data = self.client.request("state_getStorage", params).await?;
        Ok(data)
    }

    /// Returns the keys with prefix with pagination support.
    /// Up to `count` keys will be returned.
    /// If `start_key` is passed, return next keys in storage in lexicographic order.
    pub async fn storage_keys_paged(
        &self,
        key: Option<StorageKey>,
        count: u32,
        start_key: Option<StorageKey>,
        hash: Option<T::Hash>,
    ) -> Result<Vec<StorageKey>, BasicError> {
        let params = rpc_params![key, count, start_key, hash];
        let data = self.client.request("state_getKeysPaged", params).await?;
        Ok(data)
    }

    /// Query historical storage entries
    pub async fn query_storage(
        &self,
        keys: Vec<StorageKey>,
        from: T::Hash,
        to: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<T::Hash>>, BasicError> {
        let params = rpc_params![keys, from, to];
        self.client
            .request("state_queryStorage", params)
            .await
            .map_err(Into::into)
    }

    /// Query historical storage entries
    pub async fn query_storage_at(
        &self,
        keys: &[StorageKey],
        at: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<T::Hash>>, BasicError> {
        let params = rpc_params![keys, at];
        self.client
            .request("state_queryStorageAt", params)
            .await
            .map_err(Into::into)
    }

    /// Fetch the genesis hash
    pub async fn genesis_hash(&self) -> Result<T::Hash, BasicError> {
        let block_zero = 0u32;
        let params = rpc_params![block_zero];
        let genesis_hash: Option<T::Hash> =
            self.client.request("chain_getBlockHash", params).await?;
        genesis_hash.ok_or_else(|| "Genesis hash not found".into())
    }

    /// Fetch the metadata
    pub async fn metadata(&self) -> Result<Metadata, BasicError> {
        let bytes: Bytes = self
            .client
            .request("state_getMetadata", rpc_params![])
            .await?;
        let meta: RuntimeMetadataPrefixed = Decode::decode(&mut &bytes[..])?;
        let metadata: Metadata = meta.try_into()?;
        Ok(metadata)
    }

    /// Fetch system properties
    pub async fn system_properties(&self) -> Result<SystemProperties, BasicError> {
        Ok(self
            .client
            .request("system_properties", rpc_params![])
            .await?)
    }

    /// Fetch system health
    pub async fn system_health(&self) -> Result<Health, BasicError> {
        Ok(self.client.request("system_health", rpc_params![]).await?)
    }

    /// Fetch system chain
    pub async fn system_chain(&self) -> Result<String, BasicError> {
        Ok(self.client.request("system_chain", rpc_params![]).await?)
    }

    /// Fetch system name
    pub async fn system_name(&self) -> Result<String, BasicError> {
        Ok(self.client.request("system_name", rpc_params![]).await?)
    }

    /// Fetch system version
    pub async fn system_version(&self) -> Result<String, BasicError> {
        Ok(self.client.request("system_version", rpc_params![]).await?)
    }

    /// Fetch the current nonce for the given account ID.
    pub async fn system_account_next_index(
        &self,
        account: &T::AccountId,
    ) -> Result<T::Index, BasicError> {
        Ok(self
            .client
            .request("system_accountNextIndex", rpc_params![account])
            .await?)
    }

    /// Get a header
    pub async fn header(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<Option<T::Header>, BasicError> {
        let params = rpc_params![hash];
        let header = self.client.request("chain_getHeader", params).await?;
        Ok(header)
    }

    /// Get a block hash, returns hash of latest block by default
    pub async fn block_hash(
        &self,
        block_number: Option<BlockNumber>,
    ) -> Result<Option<T::Hash>, BasicError> {
        let params = rpc_params![block_number];
        let block_hash = self.client.request("chain_getBlockHash", params).await?;
        Ok(block_hash)
    }

    /// Get a block hash of the latest finalized block
    pub async fn finalized_head(&self) -> Result<T::Hash, BasicError> {
        let hash = self
            .client
            .request("chain_getFinalizedHead", rpc_params![])
            .await?;
        Ok(hash)
    }

    /// Get a Block
    pub async fn block(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<Option<ChainBlock<T>>, BasicError> {
        let params = rpc_params![hash];
        let block = self.client.request("chain_getBlock", params).await?;
        Ok(block)
    }

    /// Reexecute the specified `block_hash` and gather statistics while doing so.
    ///
    /// This function requires the specified block and its parent to be available
    /// at the queried node. If either the specified block or the parent is pruned,
    /// this function will return `None`.
    pub async fn block_stats(
        &self,
        block_hash: T::Hash,
    ) -> Result<Option<BlockStats>, BasicError> {
        let params = rpc_params![block_hash];
        let stats = self.client.request("dev_getBlockStats", params).await?;
        Ok(stats)
    }

    /// Get proof of storage entries at a specific block's state.
    pub async fn read_proof(
        &self,
        keys: Vec<StorageKey>,
        hash: Option<T::Hash>,
    ) -> Result<ReadProof<T::Hash>, BasicError> {
        let params = rpc_params![keys, hash];
        let proof = self.client.request("state_getReadProof", params).await?;
        Ok(proof)
    }

    /// Fetch the runtime version
    pub async fn runtime_version(
        &self,
        at: Option<T::Hash>,
    ) -> Result<RuntimeVersion, BasicError> {
        let params = rpc_params![at];
        let version = self
            .client
            .request("state_getRuntimeVersion", params)
            .await?;
        Ok(version)
    }

    /// Subscribe to blocks.
    pub async fn subscribe_blocks(&self) -> Result<Subscription<T::Header>, BasicError> {
        let subscription = self
            .client
            .subscribe(
                "chain_subscribeNewHeads",
                rpc_params![],
                "chain_unsubscribeNewHeads",
            )
            .await?;

        Ok(subscription)
    }

    /// Subscribe to finalized blocks.
    pub async fn subscribe_finalized_blocks(
        &self,
    ) -> Result<Subscription<T::Header>, BasicError> {
        let subscription = self
            .client
            .subscribe(
                "chain_subscribeFinalizedHeads",
                rpc_params![],
                "chain_unsubscribeFinalizedHeads",
            )
            .await?;
        Ok(subscription)
    }

    /// Subscribe to runtime version updates that produce changes in the metadata.
    pub async fn subscribe_runtime_version(
        &self,
    ) -> Result<Subscription<RuntimeVersion>, BasicError> {
        let subscription = self
            .client
            .subscribe(
                "state_subscribeRuntimeVersion",
                rpc_params![],
                "state_unsubscribeRuntimeVersion",
            )
            .await?;
        Ok(subscription)
    }

    /// Create and submit an extrinsic and return corresponding Hash if successful
    pub async fn submit_extrinsic<X: Encode>(
        &self,
        extrinsic: X,
    ) -> Result<T::Hash, BasicError> {
        let bytes: Bytes = extrinsic.encode().into();
        let params = rpc_params![bytes];
        let xt_hash = self
            .client
            .request("author_submitExtrinsic", params)
            .await?;
        Ok(xt_hash)
    }

    /// Create and submit an extrinsic and return a subscription to the events triggered.
    pub async fn watch_extrinsic<X: Encode>(
        &self,
        extrinsic: X,
    ) -> Result<Subscription<SubstrateTransactionStatus<T::Hash, T::Hash>>, BasicError>
    {
        let bytes: Bytes = extrinsic.encode().into();
        let params = rpc_params![bytes];
        let subscription = self
            .client
            .subscribe(
                "author_submitAndWatchExtrinsic",
                params,
                "author_unwatchExtrinsic",
            )
            .await?;
        Ok(subscription)
    }

    /// Insert a key into the keystore.
    pub async fn insert_key(
        &self,
        key_type: String,
        suri: String,
        public: Bytes,
    ) -> Result<(), BasicError> {
        let params = rpc_params![key_type, suri, public];
        self.client.request("author_insertKey", params).await?;
        Ok(())
    }

    /// Generate new session keys and returns the corresponding public keys.
    pub async fn rotate_keys(&self) -> Result<Bytes, BasicError> {
        Ok(self
            .client
            .request("author_rotateKeys", rpc_params![])
            .await?)
    }

    /// Checks if the keystore has private keys for the given session public keys.
    ///
    /// `session_keys` is the SCALE encoded session keys object from the runtime.
    ///
    /// Returns `true` iff all private keys could be found.
    pub async fn has_session_keys(
        &self,
        session_keys: Bytes,
    ) -> Result<bool, BasicError> {
        let params = rpc_params![session_keys];
        Ok(self.client.request("author_hasSessionKeys", params).await?)
    }

    /// Checks if the keystore has private keys for the given public key and key type.
    ///
    /// Returns `true` if a private key could be found.
    pub async fn has_key(
        &self,
        public_key: Bytes,
        key_type: String,
    ) -> Result<bool, BasicError> {
        let params = rpc_params![public_key, key_type];
        Ok(self.client.request("author_hasKey", params).await?)
    }

    /// Submits the extrinsic to the dry_run RPC, to test if it would succeed.
    ///
    /// Returns `Ok` with an [`ApplyExtrinsicResult`], which is the result of applying of an extrinsic.
    pub async fn dry_run(
        &self,
        encoded_signed: &[u8],
        at: Option<T::Hash>,
    ) -> Result<ApplyExtrinsicResult, BasicError> {
        let params = rpc_params![format!("0x{}", hex::encode(encoded_signed)), at];
        let result_bytes: Bytes = self.client.request("system_dryRun", params).await?;
        let data: ApplyExtrinsicResult =
            codec::Decode::decode(&mut result_bytes.0.as_slice())?;
        Ok(data)
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
