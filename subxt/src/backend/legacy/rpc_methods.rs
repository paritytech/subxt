// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The raw legacy RPC methods.
//!
//! **Note:** These will eventually be removed in a future release.

use serde::{ Serialize, Deserialize };
use primitive_types::U256;
use codec::Decode;
use crate::metadata::Metadata;
use crate::{ Error, Config };
use crate::backend::rpc::{ RpcClient, Subscription, rpc_params };

/// Fetch the raw bytes for a given storage key
pub async fn state_get_storage<T: Config>(
    client: &RpcClient<T>,
    key: &[u8],
    hash: Option<T::Hash>,
) -> Result<Option<StorageKey>, Error> {
    let params = rpc_params![to_hex(key), hash];
    let data: Option<Bytes> = client.request("state_getStorage", params).await?;
    Ok(data.map(|b| b.0))
}

/// Storage key.
pub type StorageKey = Vec<u8>;

/// Returns the keys with prefix with pagination support.
/// Up to `count` keys will be returned.
/// If `start_key` is passed, return next keys in storage in lexicographic order.
pub async fn state_get_keys_paged<T: Config>(
    client: &RpcClient<T>,
    key: &[u8],
    count: u32,
    start_key: Option<&[u8]>,
    at: Option<T::Hash>,
) -> Result<Vec<StorageData>, Error> {
    let start_key = start_key.map(to_hex);
    let params = rpc_params![to_hex(key), count, start_key, at];
    let data: Vec<Bytes> = client.request("state_getKeysPaged", params).await?;
    Ok(data.into_iter().map(|b| b.0).collect())
}

/// Storage data.
pub type StorageData = Vec<u8>;

// /// Query storage entries
// pub async fn state_query_storage<T: Config>(
//     client: &RpcClient<T>,
//     keys: &mut dyn Iterator<Item = &[u8]>,
//     from: T::Hash,
//     to: Option<T::Hash>,
// ) -> Result<Vec<StorageChangeSet<T::Hash>>, Error> {
//     let keys: Vec<String> = keys.map(to_hex).collect();
//     let params = rpc_params![keys, from, to];
//     client
//         .request("state_queryStorage", params)
//         .await
//         .map_err(Into::into)
// }

// /// Storage change set
// #[derive(Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct StorageChangeSet<Hash> {
//     /// Block hash
//     pub block: Hash,
//     /// A list of changes
//     pub changes: Vec<(StorageKey, Option<StorageData>)>,
// }

// /// Return the storage entries starting with the given keys.
// pub async fn state_query_storage_at<T: Config>(
//     client: &RpcClient<T>,
//     keys: &mut dyn Iterator<Item = &[u8]>,
//     at: Option<T::Hash>,
// ) -> Result<Vec<StorageChangeSet<T::Hash>>, Error> {
//     #[derive(Deserialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct StorageChangeSet<Hash> {
//         /// A list of changes
//         changes: Vec<(Bytes, Option<Bytes>)>,
//     }


//     let keys: Vec<String> = keys.map(to_hex).collect();
//     let params = rpc_params![keys, at];
//     let res: Vec<StorageChangeSet<T::Hash>> = client
//         .request("state_queryStorageAt", params)
//         .await
//         .map_err(Into::into)?;

//     Ok(res)
// }

/// Fetch the genesis hash
pub async fn genesis_hash<T: Config>(client: &RpcClient<T>) -> Result<T::Hash, Error> {
    let block_zero = 0u32;
    let params = rpc_params![block_zero];
    let genesis_hash: Option<T::Hash> =
        client.request("chain_getBlockHash", params).await?;
    genesis_hash.ok_or_else(|| "Genesis hash not found".into())
}

/// Fetch the metadata via the legacy `state_getMetadata` RPC method.
pub async fn state_get_metadata<T: Config>(client: &RpcClient<T>, at: Option<T::Hash>) -> Result<Metadata, Error> {
    let bytes: Bytes = client
        .request("state_getMetadata", rpc_params![at])
        .await?;
    let metadata = Metadata::decode(&mut &bytes[..])?;
    Ok(metadata)
}

// /// Fetch system properties
// pub async fn system_properties<T: Config>(client: &RpcClient<T>) -> Result<SystemProperties, Error> {
//     client
//         .request("system_properties", rpc_params![])
//         .await
// }

// /// System properties; an arbitrary JSON object.
// pub type SystemProperties = serde_json::Map<String, serde_json::Value>;

// /// Fetch system health
// pub async fn system_health<T: Config>(client: &RpcClient<T>) -> Result<Health, Error> {
//     client.request("system_health", rpc_params![]).await
// }

// /// Health struct returned by the RPC
// #[derive(Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Health {
//     /// Number of connected peers
//     pub peers: usize,
//     /// Is the node syncing
//     pub is_syncing: bool,
//     /// Should this node have any peers
//     ///
//     /// Might be false for local chains or when running without discovery.
//     pub should_have_peers: bool,
// }

/// Get a header
pub async fn chain_get_header<T: Config>(client: &RpcClient<T>, hash: Option<T::Hash>) -> Result<Option<T::Header>, Error> {
    let params = rpc_params![hash];
    let header = client.request("chain_getHeader", params).await?;
    Ok(header)
}

/// Get a block hash, returns hash of latest _best_ block by default.
pub async fn chain_get_block_hash<T: Config>(
    client: &RpcClient<T>,
    block_number: Option<BlockNumber>,
) -> Result<Option<T::Hash>, Error> {
    let params = rpc_params![block_number];
    let block_hash = client.request("chain_getBlockHash", params).await?;
    Ok(block_hash)
}

/// A block number
pub type BlockNumber = NumberOrHex;

/// Get a block hash of the latest finalized block
pub async fn chain_get_finalized_head<T: Config>(client: &RpcClient<T>) -> Result<T::Hash, Error> {
    let hash = client
        .request("chain_getFinalizedHead", rpc_params![])
        .await?;
    Ok(hash)
}

/// Get a Block
pub async fn chain_get_block<T: Config>(
    client: &RpcClient<T>,
    hash: Option<T::Hash>,
) -> Result<Option<BlockDetails<T>>, Error> {
    let params = rpc_params![hash];
    let block = client.request("chain_getBlock", params).await?;
    Ok(block)
}

/// The response from `chain_getBlock`
#[derive(Debug, Deserialize)]
#[serde(bound = "T: Config")]
pub struct BlockDetails<T: Config> {
    /// The block itself.
    pub block: Block<T>,
    /// Block justification.
    pub justifications: Option<Vec<BlockJustification>>,
}

/// Block details in the [`BlockDetails`].
#[derive(Debug, Deserialize)]
pub struct Block<T: Config> {
    /// The block header.
    pub header: T::Header,
    /// The accompanying extrinsics.
    pub extrinsics: Vec<Bytes>,
}

/// An abstraction over justification for a block's validity under a consensus algorithm.
pub type BlockJustification = (ConsensusEngineId, EncodedJustification);
/// Consensus engine unique ID.
pub type ConsensusEngineId = [u8; 4];
/// The encoded justification specific to a consensus engine.
pub type EncodedJustification = Vec<u8>;

/// Fetch the runtime version
pub async fn state_get_runtime_version<T: Config>(
    client: &RpcClient<T>,
    at: Option<T::Hash>,
) -> Result<RuntimeVersion, Error> {
    let params = rpc_params![at];
    let version = client
        .request("state_getRuntimeVersion", params)
        .await?;
    Ok(version)
}

/// This contains the runtime version information necessary to make transactions, as obtained from
/// the RPC call `state_getRuntimeVersion`,
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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

    /// Fields unnecessary to Subxt are written out to this map.
    #[serde(flatten)]
    pub other: std::collections::HashMap<String, serde_json::Value>,
}

/// Subscribe to all new best block headers.
pub async fn chain_subscribe_new_heads<T: Config>(client: &RpcClient<T>) -> Result<Subscription<T::Header>, Error> {
    let subscription = client
        .subscribe(
            // Despite the name, this returns a stream of all new blocks
            // imported by the node that happen to be added to the current best chain
            // (ie all best blocks).
            "chain_subscribeNewHeads",
            rpc_params![],
            "chain_unsubscribeNewHeads",
        )
        .await?;

    Ok(subscription)
}

/// Subscribe to all new block headers.
pub async fn chain_subscribe_all_heads<T: Config>(client: &RpcClient<T>) -> Result<Subscription<T::Header>, Error> {
    let subscription = client
        .subscribe(
            // Despite the name, this returns a stream of all new blocks
            // imported by the node that happen to be added to the current best chain
            // (ie all best blocks).
            "chain_subscribeAllHeads",
            rpc_params![],
            "chain_unsubscribeAllHeads",
        )
        .await?;

    Ok(subscription)
}

/// Subscribe to finalized block headers.
///
/// Note: this may not produce _every_ block in the finalized chain;
/// sometimes multiple blocks are finalized at once, and in this case only the
/// latest one is returned. the higher level APIs that use this "fill in" the
/// gaps for us.
pub async fn chain_subscribe_finalized_heads<T: Config>(
    client: &RpcClient<T>,
) -> Result<Subscription<T::Header>, Error> {
    let subscription = client
        .subscribe(
            "chain_subscribeFinalizedHeads",
            rpc_params![],
            "chain_unsubscribeFinalizedHeads",
        )
        .await?;
    Ok(subscription)
}

/// Subscribe to runtime version updates that produce changes in the metadata.
/// The first item emitted by the stream is the current runtime version.
pub async fn state_subscribe_runtime_version<T: Config>(
    client: &RpcClient<T>,
) -> Result<Subscription<RuntimeVersion>, Error> {
    let subscription = client
        .subscribe(
            "state_subscribeRuntimeVersion",
            rpc_params![],
            "state_unsubscribeRuntimeVersion",
        )
        .await?;
    Ok(subscription)
}

/// Create and submit an extrinsic and return corresponding Hash if successful
pub async fn author_submit_extrinsic<T: Config>(client: &RpcClient<T>, extrinsic: &[u8]) -> Result<T::Hash, Error> {
    let params = rpc_params![to_hex(extrinsic)];
    let xt_hash = client
        .request("author_submitExtrinsic", params)
        .await?;
    Ok(xt_hash)
}

/// Create and submit an extrinsic and return a subscription to the events triggered.
pub async fn author_submit_and_watch_extrinsic<T: Config>(
    client: &RpcClient<T>,
    extrinsic: &[u8],
) -> Result<Subscription<TransactionStatus<T::Hash>>, Error> {
    let params = rpc_params![to_hex(extrinsic)];
    let subscription = client
        .subscribe(
            "author_submitAndWatchExtrinsic",
            params,
            "author_unwatchExtrinsic",
        )
        .await?;
    Ok(subscription)
}

/// Possible transaction status events.
///
/// # Note
///
/// This is copied from `sp-transaction-pool` to avoid a dependency on that crate. Therefore it
/// must be kept compatible with that type from the target substrate version.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus<Hash> {
    /// Transaction is part of the future queue.
    Future,
    /// Transaction is part of the ready queue.
    Ready,
    /// The transaction has been broadcast to the given peers.
    Broadcast(Vec<String>),
    /// Transaction has been included in block with given hash.
    InBlock(Hash),
    /// The block this transaction was included in has been retracted.
    Retracted(Hash),
    /// Maximum number of finality watchers has been reached,
    /// old watchers are being removed.
    FinalityTimeout(Hash),
    /// Transaction has been finalized by a finality-gadget, e.g GRANDPA
    Finalized(Hash),
    /// Transaction has been replaced in the pool, by another transaction
    /// that provides the same tags. (e.g. same (sender, nonce)).
    Usurped(Hash),
    /// Transaction has been dropped from the pool because of the limit.
    Dropped,
    /// Transaction is no longer valid in the current state.
    Invalid,
}

/// Execute a runtime API call via `state_call` RPC method.
pub async fn state_call<T: Config>(
    client: &RpcClient<T>,
    function: &str,
    call_parameters: Option<&[u8]>,
    at: Option<T::Hash>,
) -> Result<Vec<u8>, Error> {
    let call_parameters = call_parameters.unwrap_or_default();
    let bytes: Bytes = client
        .request(
            "state_call",
            rpc_params![function, to_hex(call_parameters), at],
        )
        .await?;
    Ok(bytes.0)
}

/// Submits the extrinsic to the dry_run RPC, to test if it would succeed.
///
/// Returns a [`types::DryRunResult`], which is the result of performing the dry run.
pub async fn system_dry_run<T: Config>(
    client: &RpcClient<T>,
    encoded_signed: &[u8],
    at: Option<T::Hash>,
) -> Result<Vec<u8>, Error> {
    let params = rpc_params![to_hex(encoded_signed), at];
    let result_bytes: Bytes = client.request("system_dryRun", params).await?;
    Ok(result_bytes.0)
}

/// A quick helper to encode some bytes to hex.
fn to_hex(bytes: impl AsRef<[u8]>) -> String {
    format!("0x{}", hex::encode(bytes.as_ref()))
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

/// A number type that can be serialized both as a number or a string that encodes a number in a
/// string.
///
/// We allow two representations of the block number as input. Either we deserialize to the type
/// that is specified in the block type or we attempt to parse given hex value.
///
/// The primary motivation for having this type is to avoid overflows when using big integers in
/// JavaScript (which we consider as an important RPC API consumer).
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum NumberOrHex {
    /// The number represented directly.
    Number(u64),
    /// Hex representation of the number.
    Hex(U256),
}

impl NumberOrHex {
    /// Converts this number into an U256.
    pub fn into_u256(self) -> U256 {
        match self {
            NumberOrHex::Number(n) => n.into(),
            NumberOrHex::Hex(h) => h,
        }
    }
}

impl From<NumberOrHex> for U256 {
    fn from(num_or_hex: NumberOrHex) -> U256 {
        num_or_hex.into_u256()
    }
}

macro_rules! into_number_or_hex {
    ($($t: ty)+) => {
        $(
            impl From<$t> for NumberOrHex {
                fn from(x: $t) -> Self {
                    NumberOrHex::Number(x.into())
                }
            }
        )+
    }
}
into_number_or_hex!(u8 u16 u32 u64);

impl From<u128> for NumberOrHex {
    fn from(n: u128) -> Self {
        NumberOrHex::Hex(n.into())
    }
}

impl From<U256> for NumberOrHex {
    fn from(n: U256) -> Self {
        NumberOrHex::Hex(n)
    }
}