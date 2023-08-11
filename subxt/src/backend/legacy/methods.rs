// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The raw legacy RPC methods.
//!
//! **Note:** These will eventually be removed in a future release.

use serde::{ Serialize, Deserialize };
use primitive_types::U256;
use crate::metadata::Metadata;
use crate::{ Error, Config };
use crate::backend::rpc::{ RpcClient, rpc_params };

/// Fetch the raw bytes for a given storage key
pub async fn state_get_storage<T: Config>(
    client: &RpcClient<T>,
    key: &[u8],
    hash: Option<T::Hash>,
) -> Result<Option<StorageKey>, Error> {
    let params = rpc_params![to_hex(key), hash];
    let data = client.request("state_getStorage", params).await?;
    Ok(data)
}

/// Storage key.
pub type StorageKey = Bytes;

/// Returns the keys with prefix with pagination support.
/// Up to `count` keys will be returned.
/// If `start_key` is passed, return next keys in storage in lexicographic order.
pub async fn state_get_keys_paged<T: Config>(
    client: &RpcClient<T>,
    key: &[u8],
    count: u32,
    start_key: Option<&[u8]>,
    hash: Option<T::Hash>,
) -> Result<Vec<StorageData>, Error> {
    let start_key = start_key.map(to_hex);
    let params = rpc_params![to_hex(key), count, start_key, hash];
    let data = client.request("state_getKeysPaged", params).await?;
    Ok(data)
}

/// Storage data.
pub type StorageData = Bytes;

/// Query storage entries
pub async fn state_query_storage<T: Config>(
    client: &RpcClient<T>,
    keys: impl IntoIterator<Item = &[u8]>,
    from: T::Hash,
    to: Option<T::Hash>,
) -> Result<Vec<StorageChangeSet<T::Hash>>, Error> {
    let keys: Vec<String> = keys.into_iter().map(to_hex).collect();
    let params = rpc_params![keys, from, to];
    client
        .request("state_queryStorage", params)
        .await
        .map_err(Into::into)
}

/// Storage change set
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorageChangeSet<Hash> {
    /// Block hash
    pub block: Hash,
    /// A list of changes
    pub changes: Vec<(StorageKey, Option<StorageData>)>,
}

/// Query historical storage entries
pub async fn state_query_storage_at<T: Config>(
    client: &RpcClient<T>,
    keys: impl IntoIterator<Item = &[u8]>,
    at: Option<T::Hash>,
) -> Result<Vec<StorageChangeSet<T::Hash>>, Error> {
    let keys: Vec<String> = keys.into_iter().map(to_hex).collect();
    let params = rpc_params![keys, at];
    client
        .request("state_queryStorageAt", params)
        .await
        .map_err(Into::into)
}

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

/// Fetch system properties
pub async fn system_properties<T: Config>(client: &RpcClient<T>) -> Result<SystemProperties, Error> {
    client
        .request("system_properties", rpc_params![])
        .await
}

/// System properties; an arbitrary JSON object.
pub type SystemProperties = serde_json::Map<String, serde_json::Value>;

/// Fetch system health
pub async fn system_health<T: Config>(client: &RpcClient<T>) -> Result<Health, Error> {
    client.request("system_health", rpc_params![]).await
}

/// Health struct returned by the RPC
#[derive(Deserialize, Clone, Debug)]
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

/// Fetch system chain
pub async fn system_chain<T: Config>(client: &RpcClient<T>) -> Result<String, Error> {
    client.request("system_chain", rpc_params![]).await
}

/// Fetch system name
pub async fn system_name<T: Config>(client: &RpcClient<T>) -> Result<String, Error> {
    client.request("system_name", rpc_params![]).await
}

/// Fetch system version
pub async fn system_version<T: Config>(client: &RpcClient<T>) -> Result<String, Error> {
    client.request("system_version", rpc_params![]).await
}

/// Get a header
pub async fn chain_get_header<T: Config>(client: &RpcClient<T>, hash: Option<T::Hash>) -> Result<Option<T::Header>, Error> {
    let params = rpc_params![hash];
    let header = client.request("chain_getHeader", params).await?;
    Ok(header)
}

/// Get a block hash, returns hash of latest block by default
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
pub async fn block<T: Config>(
    client: &RpcClient<T>,
    hash: Option<T::Hash>,
) -> Result<Option<types::ChainBlockResponse<T>>, Error> {
    let params = rpc_params![hash];
    let block = client.request("chain_getBlock", params).await?;
    Ok(block)
}

/// Reexecute the specified `block_hash` and gather statistics while doing so.
///
/// This function requires the specified block and its parent to be available
/// at the queried node. If either the specified block or the parent is pruned,
/// this function will return `None`.
pub async fn block_stats<T: Config>(
    client: &RpcClient<T>,
    block_hash: T::Hash,
) -> Result<Option<types::BlockStats>, Error> {
    let params = rpc_params![block_hash];
    let stats = client.request("dev_getBlockStats", params).await?;
    Ok(stats)
}

/// Get proof of storage entries at a specific block's state.
pub async fn read_proof<T: Config>(
    client: &RpcClient<T>,
    keys: impl IntoIterator<Item = &[u8]>,
    hash: Option<T::Hash>,
) -> Result<types::ReadProof<T::Hash>, Error> {
    let keys: Vec<String> = keys.into_iter().map(to_hex).collect();
    let params = rpc_params![keys, hash];
    let proof = client.request("state_getReadProof", params).await?;
    Ok(proof)
}

/// Fetch the runtime version
pub async fn runtime_version<T: Config>(
    client: &RpcClient<T>,
    at: Option<T::Hash>,
) -> Result<types::RuntimeVersion, Error> {
    let params = rpc_params![at];
    let version = client
        .client
        .request("state_getRuntimeVersion", params)
        .await?;
    Ok(version)
}

/// Subscribe to all new best block headers.
pub async fn subscribe_best_block_headers<T: Config>(client: &RpcClient<T>) -> Result<Subscription<T::Header>, Error> {
    let subscription = client
        .client
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
pub async fn subscribe_all_block_headers<T: Config>(client: &RpcClient<T>) -> Result<Subscription<T::Header>, Error> {
    let subscription = client
        .client
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
pub async fn subscribe_finalized_block_headers<T: Config>(
    client: &RpcClient<T>,
) -> Result<Subscription<T::Header>, Error> {
    let subscription = client
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
/// The first item emitted by the stream is the current runtime version.
pub async fn subscribe_runtime_version<T: Config>(
    client: &RpcClient<T>,
) -> Result<Subscription<types::RuntimeVersion>, Error> {
    let subscription = client
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
pub async fn submit_extrinsic<T: Config, X: Encode>(client: &RpcClient<T>, extrinsic: X) -> Result<T::Hash, Error> {
    let bytes: types::Bytes = extrinsic.encode().into();
    let params = rpc_params![bytes];
    let xt_hash = client
        .client
        .request("author_submitExtrinsic", params)
        .await?;
    Ok(xt_hash)
}

/// Execute a runtime API call via `state_call` RPC method.
pub async fn state_call_raw<T: Config>(
    client: &RpcClient<T>,
    function: &str,
    call_parameters: Option<&[u8]>,
    at: Option<T::Hash>,
) -> Result<types::Bytes, Error> {
    let call_parameters = call_parameters.unwrap_or_default();
    let bytes: types::Bytes = client
        .client
        .request(
            "state_call",
            rpc_params![function, to_hex(call_parameters), at],
        )
        .await?;
    Ok(bytes)
}

/// Execute a runtime API call and decode the result.
pub async fn state_call<T: Config, Res: Decode>(
    client: &RpcClient<T>,
    function: &str,
    call_parameters: Option<&[u8]>,
    at: Option<T::Hash>,
) -> Result<Res, Error> {
    let bytes = client.state_call_raw(function, call_parameters, at).await?;
    let cursor = &mut &bytes[..];
    let res: Res = Decode::decode(cursor)?;
    Ok(res)
}

/// Provide a list of the supported metadata versions of the node.
pub async fn metadata_versions<T: Config>(client: &RpcClient<T>) -> Result<Vec<u32>, Error> {
    let versions = client
        .state_call("Metadata_metadata_versions", None, None)
        .await?;

    Ok(versions)
}

/// Execute runtime API call and return the specified runtime metadata version.
pub async fn metadata_at_version<T: Config>(client: &RpcClient<T>, version: u32) -> Result<Metadata, Error> {
    let param = version.encode();
    let opaque: Option<frame_metadata::OpaqueMetadata> = client
        .state_call("Metadata_metadata_at_version", Some(&param), None)
        .await?;

    let bytes = opaque.ok_or(Error::Other("Metadata version not found".into()))?;

    let metadata: Metadata = Decode::decode(&mut &bytes.0[..])?;
    Ok(metadata)
}

/// Execute a runtime API call into `Metadata_metadata` method
/// to fetch the latest available metadata.
///
/// # Note
///
/// This returns the same output as [`client::metadata`], but calls directly
/// into the runtime.
pub async fn metadata<T: Config>(client: &RpcClient<T>) -> Result<Metadata, Error> {
    let bytes: frame_metadata::OpaqueMetadata =
        client.state_call("Metadata_metadata", None, None).await?;

    let metadata: Metadata = Decode::decode(&mut &bytes.0[..])?;
    Ok(metadata)
}

/// Create and submit an extrinsic and return a subscription to the events triggered.
pub async fn watch_extrinsic<T: Config, X: Encode>(
    client: &RpcClient<T>,
    extrinsic: X,
) -> Result<Subscription<types::SubstrateTxStatus<T::Hash, T::Hash>>, Error> {
    let bytes: types::Bytes = extrinsic.encode().into();
    let params = rpc_params![bytes];
    let subscription = client
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
pub async fn insert_key<T: Config>(
    client: &RpcClient<T>,
    key_type: String,
    suri: String,
    public: types::Bytes,
) -> Result<(), Error> {
    let params = rpc_params![key_type, suri, public];
    client.request("author_insertKey", params).await?;
    Ok(())
}

/// Generate new session keys and returns the corresponding public keys.
pub async fn rotate_keys<T: Config>(client: &RpcClient<T>) -> Result<types::Bytes, Error> {
    client
        .request("author_rotateKeys", rpc_params![])
        .await
}

/// Checks if the keystore has private keys for the given session public keys.
///
/// `session_keys` is the SCALE encoded session keys object from the runtime.
///
/// Returns `true` iff all private keys could be found.
pub async fn has_session_keys<T: Config>(client: &RpcClient<T>, session_keys: types::Bytes) -> Result<bool, Error> {
    let params = rpc_params![session_keys];
    client.request("author_hasSessionKeys", params).await
}

/// Checks if the keystore has private keys for the given public key and key type.
///
/// Returns `true` if a private key could be found.
pub async fn has_key<T: Config>(client: &RpcClient<T>, public_key: types::Bytes, key_type: String) -> Result<bool, Error> {
    let params = rpc_params![public_key, key_type];
    client.request("author_hasKey", params).await
}

/// Submits the extrinsic to the dry_run RPC, to test if it would succeed.
///
/// Returns a [`types::DryRunResult`], which is the result of performing the dry run.
pub async fn dry_run<T: Config>(
    client: &RpcClient<T>,
    encoded_signed: &[u8],
    at: Option<T::Hash>,
) -> Result<types::DryRunResultBytes, Error> {
    let params = rpc_params![to_hex(encoded_signed), at];
    let result_bytes: types::Bytes = client.request("system_dryRun", params).await?;
    Ok(types::DryRunResultBytes(result_bytes.0))
}

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