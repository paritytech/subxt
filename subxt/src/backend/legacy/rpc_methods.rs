// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! An interface to call the raw legacy RPC methods.

use crate::backend::rpc::{rpc_params, RpcClient, RpcSubscription};
use crate::metadata::Metadata;
use crate::{Config, Error};
use codec::Decode;
use derivative::Derivative;
use primitive_types::U256;
use serde::{Deserialize, Serialize};

/// An interface to call the legacy RPC methods. This interface is instantiated with
/// some `T: Config` trait which determines some of the types that the RPC methods will
/// take or hand back.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct LegacyRpcMethods<T> {
    client: RpcClient,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Config> LegacyRpcMethods<T> {
    /// Instantiate the legacy RPC method interface.
    pub fn new(client: RpcClient) -> Self {
        LegacyRpcMethods {
            client,
            _marker: std::marker::PhantomData,
        }
    }

    /// Fetch the raw bytes for a given storage key
    pub async fn state_get_storage(
        &self,
        key: &[u8],
        hash: Option<T::Hash>,
    ) -> Result<Option<StorageKey>, Error> {
        let params = rpc_params![to_hex(key), hash];
        let data: Option<Bytes> = self.client.request("state_getStorage", params).await?;
        Ok(data.map(|b| b.0))
    }

    /// Returns the keys with prefix with pagination support.
    /// Up to `count` keys will be returned.
    /// If `start_key` is passed, return next keys in storage in lexicographic order.
    pub async fn state_get_keys_paged(
        &self,
        key: &[u8],
        count: u32,
        start_key: Option<&[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<StorageData>, Error> {
        let start_key = start_key.map(to_hex);
        let params = rpc_params![to_hex(key), count, start_key, at];
        let data: Vec<Bytes> = self.client.request("state_getKeysPaged", params).await?;
        Ok(data.into_iter().map(|b| b.0).collect())
    }

    /// Query historical storage entries in the range from the start block to the end block,
    /// defaulting the end block to the current best block if it's not given. The first
    /// [`StorageChangeSet`] returned has all of the values for each key, and subsequent ones
    /// only contain values for any keys which have changed since the last.
    pub async fn state_query_storage(
        &self,
        keys: impl IntoIterator<Item = &[u8]>,
        from: T::Hash,
        to: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<T::Hash>>, Error> {
        let keys: Vec<String> = keys.into_iter().map(to_hex).collect();
        let params = rpc_params![keys, from, to];
        self.client
            .request("state_queryStorage", params)
            .await
            .map_err(Into::into)
    }

    /// Query storage entries at some block, using the best block if none is given.
    /// This essentially provides a way to ask for a batch of values given a batch of keys,
    /// despite the name of the [`StorageChangeSet`] type.
    pub async fn state_query_storage_at(
        &self,
        keys: impl IntoIterator<Item = &[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<T::Hash>>, Error> {
        let keys: Vec<String> = keys.into_iter().map(to_hex).collect();
        let params = rpc_params![keys, at];
        self.client
            .request("state_queryStorageAt", params)
            .await
            .map_err(Into::into)
    }

    /// Fetch the genesis hash
    pub async fn genesis_hash(&self) -> Result<T::Hash, Error> {
        let block_zero = 0u32;
        let params = rpc_params![block_zero];
        let genesis_hash: Option<T::Hash> =
            self.client.request("chain_getBlockHash", params).await?;
        genesis_hash.ok_or_else(|| "Genesis hash not found".into())
    }

    /// Fetch the metadata via the legacy `state_getMetadata` RPC method.
    pub async fn state_get_metadata(&self, at: Option<T::Hash>) -> Result<Metadata, Error> {
        let bytes: Bytes = self
            .client
            .request("state_getMetadata", rpc_params![at])
            .await?;
        let metadata = Metadata::decode(&mut &bytes[..])?;
        Ok(metadata)
    }

    /// Fetch system health
    pub async fn system_health(&self) -> Result<SystemHealth, Error> {
        self.client.request("system_health", rpc_params![]).await
    }

    /// Fetch system chain
    pub async fn system_chain(&self) -> Result<String, Error> {
        self.client.request("system_chain", rpc_params![]).await
    }

    /// Fetch system name
    pub async fn system_name(&self) -> Result<String, Error> {
        self.client.request("system_name", rpc_params![]).await
    }

    /// Fetch system version
    pub async fn system_version(&self) -> Result<String, Error> {
        self.client.request("system_version", rpc_params![]).await
    }

    /// Fetch system properties
    pub async fn system_properties(&self) -> Result<SystemProperties, Error> {
        self.client
            .request("system_properties", rpc_params![])
            .await
    }

    /// Get a header
    pub async fn chain_get_header(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<Option<T::Header>, Error> {
        let params = rpc_params![hash];
        let header = self.client.request("chain_getHeader", params).await?;
        Ok(header)
    }

    /// Get a block hash, returns hash of latest _best_ block by default.
    pub async fn chain_get_block_hash(
        &self,
        block_number: Option<BlockNumber>,
    ) -> Result<Option<T::Hash>, Error> {
        let params = rpc_params![block_number];
        let block_hash = self.client.request("chain_getBlockHash", params).await?;
        Ok(block_hash)
    }

    /// Get a block hash of the latest finalized block
    pub async fn chain_get_finalized_head(&self) -> Result<T::Hash, Error> {
        let hash = self
            .client
            .request("chain_getFinalizedHead", rpc_params![])
            .await?;
        Ok(hash)
    }

    /// Get a Block
    pub async fn chain_get_block(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<Option<BlockDetails<T>>, Error> {
        let params = rpc_params![hash];
        let block = self.client.request("chain_getBlock", params).await?;
        Ok(block)
    }

    /// Reexecute the specified `block_hash` and gather statistics while doing so.
    ///
    /// This function requires the specified block and its parent to be available
    /// at the queried node. If either the specified block or the parent is pruned,
    /// this function will return `None`.
    pub async fn dev_get_block_stats(
        &self,
        block_hash: T::Hash,
    ) -> Result<Option<BlockStats>, Error> {
        let params = rpc_params![block_hash];
        let stats = self.client.request("dev_getBlockStats", params).await?;
        Ok(stats)
    }

    /// Get proof of storage entries at a specific block's state.
    pub async fn state_get_read_proof(
        &self,
        keys: impl IntoIterator<Item = &[u8]>,
        hash: Option<T::Hash>,
    ) -> Result<ReadProof<T::Hash>, Error> {
        let keys: Vec<String> = keys.into_iter().map(to_hex).collect();
        let params = rpc_params![keys, hash];
        let proof = self.client.request("state_getReadProof", params).await?;
        Ok(proof)
    }

    /// Fetch the runtime version
    pub async fn state_get_runtime_version(
        &self,
        at: Option<T::Hash>,
    ) -> Result<RuntimeVersion, Error> {
        let params = rpc_params![at];
        let version = self
            .client
            .request("state_getRuntimeVersion", params)
            .await?;
        Ok(version)
    }

    /// Subscribe to all new best block headers.
    pub async fn chain_subscribe_new_heads(&self) -> Result<RpcSubscription<T::Header>, Error> {
        let subscription = self
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
    pub async fn chain_subscribe_all_heads(&self) -> Result<RpcSubscription<T::Header>, Error> {
        let subscription = self
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
    pub async fn chain_subscribe_finalized_heads(
        &self,
    ) -> Result<RpcSubscription<T::Header>, Error> {
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
    /// The first item emitted by the stream is the current runtime version.
    pub async fn state_subscribe_runtime_version(
        &self,
    ) -> Result<RpcSubscription<RuntimeVersion>, Error> {
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
    pub async fn author_submit_extrinsic(&self, extrinsic: &[u8]) -> Result<T::Hash, Error> {
        let params = rpc_params![to_hex(extrinsic)];
        let xt_hash = self
            .client
            .request("author_submitExtrinsic", params)
            .await?;
        Ok(xt_hash)
    }

    /// Create and submit an extrinsic and return a subscription to the events triggered.
    pub async fn author_submit_and_watch_extrinsic(
        &self,
        extrinsic: &[u8],
    ) -> Result<RpcSubscription<TransactionStatus<T::Hash>>, Error> {
        let params = rpc_params![to_hex(extrinsic)];
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
    pub async fn author_insert_key(
        &self,
        key_type: String,
        suri: String,
        public: Vec<u8>,
    ) -> Result<(), Error> {
        let params = rpc_params![key_type, suri, Bytes(public)];
        self.client.request("author_insertKey", params).await?;
        Ok(())
    }

    /// Generate new session keys and returns the corresponding public keys.
    pub async fn author_rotate_keys(&self) -> Result<Vec<u8>, Error> {
        let bytes: Bytes = self
            .client
            .request("author_rotateKeys", rpc_params![])
            .await?;
        Ok(bytes.0)
    }

    /// Checks if the keystore has private keys for the given session public keys.
    ///
    /// `session_keys` is the SCALE encoded session keys object from the runtime.
    ///
    /// Returns `true` if all private keys could be found.
    pub async fn author_has_session_keys(&self, session_keys: Vec<u8>) -> Result<bool, Error> {
        let params = rpc_params![Bytes(session_keys)];
        self.client.request("author_hasSessionKeys", params).await
    }

    /// Checks if the keystore has private keys for the given public key and key type.
    ///
    /// Returns `true` if a private key could be found.
    pub async fn author_has_key(
        &self,
        public_key: Vec<u8>,
        key_type: String,
    ) -> Result<bool, Error> {
        let params = rpc_params![Bytes(public_key), key_type];
        self.client.request("author_hasKey", params).await
    }

    /// Execute a runtime API call via `state_call` RPC method.
    pub async fn state_call(
        &self,
        function: &str,
        call_parameters: Option<&[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<u8>, Error> {
        let call_parameters = call_parameters.unwrap_or_default();
        let bytes: Bytes = self
            .client
            .request(
                "state_call",
                rpc_params![function, to_hex(call_parameters), at],
            )
            .await?;
        Ok(bytes.0)
    }

    /// Submits the extrinsic to the dry_run RPC, to test if it would succeed.
    ///
    /// Returns a [`DryRunResult`], which is the result of performing the dry run.
    pub async fn dry_run(
        &self,
        encoded_signed: &[u8],
        at: Option<T::Hash>,
    ) -> Result<DryRunResultBytes, Error> {
        let params = rpc_params![to_hex(encoded_signed), at];
        let result_bytes: Bytes = self.client.request("system_dryRun", params).await?;
        Ok(DryRunResultBytes(result_bytes.0))
    }
}

/// Storage key.
pub type StorageKey = Vec<u8>;

/// Storage data.
pub type StorageData = Vec<u8>;

/// Health struct returned by the RPC
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemHealth {
    /// Number of connected peers
    pub peers: usize,
    /// Is the node syncing
    pub is_syncing: bool,
    /// Should this node have any peers
    ///
    /// Might be false for local chains or when running without discovery.
    pub should_have_peers: bool,
}

/// System properties; an arbitrary JSON object.
pub type SystemProperties = serde_json::Map<String, serde_json::Value>;

/// A block number
pub type BlockNumber = NumberOrHex;

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

/// The decoded result returned from calling `system_dryRun` on some extrinsic.
#[derive(Debug, PartialEq, Eq)]
pub enum DryRunResult {
    /// The transaction could be included in the block and executed.
    Success,
    /// The transaction could be included in the block, but the call failed to dispatch.
    DispatchError(crate::error::DispatchError),
    /// The transaction could not be included in the block.
    TransactionValidityError,
}

/// The bytes representing an error dry running an extrinsic. call [`DryRunResultBytes::into_dry_run_result`]
/// to attempt to decode this into something more meaningful.
pub struct DryRunResultBytes(pub Vec<u8>);

impl DryRunResultBytes {
    /// Attempt to decode the error bytes into a [`DryRunResult`] using the provided [`Metadata`].
    pub fn into_dry_run_result(
        self,
        metadata: &crate::metadata::Metadata,
    ) -> Result<DryRunResult, crate::Error> {
        // dryRun returns an ApplyExtrinsicResult, which is basically a
        // `Result<Result<(), DispatchError>, TransactionValidityError>`.
        let bytes = self.0;
        if bytes[0] == 0 && bytes[1] == 0 {
            // Ok(Ok(())); transaction is valid and executed ok
            Ok(DryRunResult::Success)
        } else if bytes[0] == 0 && bytes[1] == 1 {
            // Ok(Err(dispatch_error)); transaction is valid but execution failed
            let dispatch_error =
                crate::error::DispatchError::decode_from(&bytes[2..], metadata.clone())?;
            Ok(DryRunResult::DispatchError(dispatch_error))
        } else if bytes[0] == 1 {
            // Err(transaction_error); some transaction validity error (we ignore the details at the moment)
            Ok(DryRunResult::TransactionValidityError)
        } else {
            // unable to decode the bytes; they aren't what we expect.
            Err(crate::Error::Unknown(bytes))
        }
    }
}

/// Storage change set
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorageChangeSet<Hash> {
    /// Block hash
    pub block: Hash,
    /// A list of changes; tuples of storage key and optional storage data.
    pub changes: Vec<(Bytes, Option<Bytes>)>,
}

/// Statistics of a block returned by the `dev_getBlockStats` RPC.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

/// ReadProof struct returned by the RPC
///
/// # Note
///
/// This is copied from `sc-rpc-api` to avoid a dependency on that crate. Therefore it
/// must be kept compatible with that type from the target substrate version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadProof<Hash> {
    /// Block hash used to generate the proof
    pub at: Hash,
    /// A proof used to prove that storage entries are included in the storage trie
    pub proof: Vec<Bytes>,
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
