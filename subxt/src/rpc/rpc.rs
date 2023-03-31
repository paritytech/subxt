// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! RPC types and client for interacting with a substrate node.
//!
//! This is used behind the scenes by various `subxt` APIs, but can
//! also be used directly.
//!
//! # Example
//!
//! Fetching the chain genesis hash.
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() {
//! use subxt::{ PolkadotConfig, OnlineClient, storage::StorageKey };
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
//!
//! let genesis_hash = api
//!     .rpc()
//!     .genesis_hash()
//!     .await
//!     .unwrap();
//!
//! println!("{genesis_hash}");
//! # }
//! ```

use super::{
    rpc_params,
    types::{self, ChainHeadEvent, FollowEvent},
    RpcClient, RpcClientT, Subscription,
};
use crate::{error::Error, utils::PhantomDataSendSync, Config, Metadata};
use codec::{Decode, Encode};
use frame_metadata::RuntimeMetadataPrefixed;
use serde::Serialize;
use std::sync::Arc;

/// Client for substrate rpc interfaces
pub struct Rpc<T: Config> {
    client: RpcClient,
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

// Expose subscribe/request, and also subscribe_raw/request_raw
// from the even-deeper `dyn RpcClientT` impl.
impl<T: Config> std::ops::Deref for Rpc<T> {
    type Target = RpcClient;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<T: Config> Rpc<T> {
    /// Create a new [`Rpc`]
    pub fn new<R: RpcClientT>(client: Arc<R>) -> Self {
        Self {
            client: RpcClient::new(client),
            _marker: PhantomDataSendSync::new(),
        }
    }

    /// Fetch the raw bytes for a given storage key
    pub async fn storage(
        &self,
        key: &[u8],
        hash: Option<T::Hash>,
    ) -> Result<Option<types::StorageData>, Error> {
        let params = rpc_params![to_hex(key), hash];
        let data = self.client.request("state_getStorage", params).await?;
        Ok(data)
    }

    /// Returns the keys with prefix with pagination support.
    /// Up to `count` keys will be returned.
    /// If `start_key` is passed, return next keys in storage in lexicographic order.
    pub async fn storage_keys_paged(
        &self,
        key: &[u8],
        count: u32,
        start_key: Option<&[u8]>,
        hash: Option<T::Hash>,
    ) -> Result<Vec<types::StorageKey>, Error> {
        let start_key = start_key.map(to_hex);
        let params = rpc_params![to_hex(key), count, start_key, hash];
        let data = self.client.request("state_getKeysPaged", params).await?;
        Ok(data)
    }

    /// Query historical storage entries
    pub async fn query_storage(
        &self,
        keys: impl IntoIterator<Item = &[u8]>,
        from: T::Hash,
        to: Option<T::Hash>,
    ) -> Result<Vec<types::StorageChangeSet<T::Hash>>, Error> {
        let keys: Vec<String> = keys.into_iter().map(to_hex).collect();
        let params = rpc_params![keys, from, to];
        self.client
            .request("state_queryStorage", params)
            .await
            .map_err(Into::into)
    }

    /// Query historical storage entries
    pub async fn query_storage_at(
        &self,
        keys: impl IntoIterator<Item = &[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<types::StorageChangeSet<T::Hash>>, Error> {
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

    /// Fetch the metadata
    pub async fn metadata(&self, at: Option<T::Hash>) -> Result<Metadata, Error> {
        let bytes: types::Bytes = self
            .client
            .request("state_getMetadata", rpc_params![at])
            .await?;
        let meta: RuntimeMetadataPrefixed = Decode::decode(&mut &bytes[..])?;
        let metadata: Metadata = meta.try_into()?;
        Ok(metadata)
    }

    /// Execute a runtime API call.
    pub async fn call(
        &self,
        function: String,
        call_parameters: Option<&[u8]>,
        at: Option<T::Hash>,
    ) -> Result<types::Bytes, Error> {
        let call_parameters = call_parameters.unwrap_or_default();

        let bytes: types::Bytes = self
            .client
            .request(
                "state_call",
                rpc_params![function, to_hex(call_parameters), at],
            )
            .await?;
        Ok(bytes)
    }

    /// Fetch system properties
    pub async fn system_properties(&self) -> Result<types::SystemProperties, Error> {
        self.client
            .request("system_properties", rpc_params![])
            .await
    }

    /// Fetch system health
    pub async fn system_health(&self) -> Result<types::Health, Error> {
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

    /// Fetch the current nonce for the given account ID.
    pub async fn system_account_next_index<AccountId: Serialize>(
        &self,
        account: &AccountId,
    ) -> Result<T::Index, Error> {
        self.client
            .request("system_accountNextIndex", rpc_params![account])
            .await
    }

    /// Get a header
    pub async fn header(&self, hash: Option<T::Hash>) -> Result<Option<T::Header>, Error> {
        let params = rpc_params![hash];
        let header = self.client.request("chain_getHeader", params).await?;
        Ok(header)
    }

    /// Get a block hash, returns hash of latest block by default
    pub async fn block_hash(
        &self,
        block_number: Option<types::BlockNumber>,
    ) -> Result<Option<T::Hash>, Error> {
        let params = rpc_params![block_number];
        let block_hash = self.client.request("chain_getBlockHash", params).await?;
        Ok(block_hash)
    }

    /// Get a block hash of the latest finalized block
    pub async fn finalized_head(&self) -> Result<T::Hash, Error> {
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
    ) -> Result<Option<types::ChainBlockResponse<T>>, Error> {
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
    ) -> Result<Option<types::BlockStats>, Error> {
        let params = rpc_params![block_hash];
        let stats = self.client.request("dev_getBlockStats", params).await?;
        Ok(stats)
    }

    /// Get proof of storage entries at a specific block's state.
    pub async fn read_proof(
        &self,
        keys: impl IntoIterator<Item = &[u8]>,
        hash: Option<T::Hash>,
    ) -> Result<types::ReadProof<T::Hash>, Error> {
        let keys: Vec<String> = keys.into_iter().map(to_hex).collect();
        let params = rpc_params![keys, hash];
        let proof = self.client.request("state_getReadProof", params).await?;
        Ok(proof)
    }

    /// Fetch the runtime version
    pub async fn runtime_version(
        &self,
        at: Option<T::Hash>,
    ) -> Result<types::RuntimeVersion, Error> {
        let params = rpc_params![at];
        let version = self
            .client
            .request("state_getRuntimeVersion", params)
            .await?;
        Ok(version)
    }

    /// Subscribe to all new best block headers.
    pub async fn subscribe_best_block_headers(&self) -> Result<Subscription<T::Header>, Error> {
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
    pub async fn subscribe_all_block_headers(&self) -> Result<Subscription<T::Header>, Error> {
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
    pub async fn subscribe_finalized_block_headers(
        &self,
    ) -> Result<Subscription<T::Header>, Error> {
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
    pub async fn subscribe_runtime_version(
        &self,
    ) -> Result<Subscription<types::RuntimeVersion>, Error> {
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
    pub async fn submit_extrinsic<X: Encode>(&self, extrinsic: X) -> Result<T::Hash, Error> {
        let bytes: types::Bytes = extrinsic.encode().into();
        let params = rpc_params![bytes];
        let xt_hash = self
            .client
            .request("author_submitExtrinsic", params)
            .await?;
        Ok(xt_hash)
    }

    /// Execute a runtime API call.
    pub async fn state_call(
        &self,
        function: &str,
        call_parameters: Option<&[u8]>,
        at: Option<T::Hash>,
    ) -> Result<types::Bytes, Error> {
        let call_parameters = call_parameters.unwrap_or_default();

        let bytes: types::Bytes = self
            .client
            .request(
                "state_call",
                rpc_params![function, to_hex(call_parameters), at],
            )
            .await?;
        Ok(bytes)
    }

    /// Create and submit an extrinsic and return a subscription to the events triggered.
    pub async fn watch_extrinsic<X: Encode>(
        &self,
        extrinsic: X,
    ) -> Result<Subscription<types::SubstrateTxStatus<T::Hash, T::Hash>>, Error> {
        let bytes: types::Bytes = extrinsic.encode().into();
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
        public: types::Bytes,
    ) -> Result<(), Error> {
        let params = rpc_params![key_type, suri, public];
        self.client.request("author_insertKey", params).await?;
        Ok(())
    }

    /// Generate new session keys and returns the corresponding public keys.
    pub async fn rotate_keys(&self) -> Result<types::Bytes, Error> {
        self.client
            .request("author_rotateKeys", rpc_params![])
            .await
    }

    /// Checks if the keystore has private keys for the given session public keys.
    ///
    /// `session_keys` is the SCALE encoded session keys object from the runtime.
    ///
    /// Returns `true` iff all private keys could be found.
    pub async fn has_session_keys(&self, session_keys: types::Bytes) -> Result<bool, Error> {
        let params = rpc_params![session_keys];
        self.client.request("author_hasSessionKeys", params).await
    }

    /// Checks if the keystore has private keys for the given public key and key type.
    ///
    /// Returns `true` if a private key could be found.
    pub async fn has_key(&self, public_key: types::Bytes, key_type: String) -> Result<bool, Error> {
        let params = rpc_params![public_key, key_type];
        self.client.request("author_hasKey", params).await
    }

    /// Submits the extrinsic to the dry_run RPC, to test if it would succeed.
    ///
    /// Returns a [`types::DryRunResult`], which is the result of performing the dry run.
    pub async fn dry_run(
        &self,
        encoded_signed: &[u8],
        at: Option<T::Hash>,
    ) -> Result<types::DryRunResultBytes, Error> {
        let params = rpc_params![to_hex(encoded_signed), at];
        let result_bytes: types::Bytes = self.client.request("system_dryRun", params).await?;
        Ok(types::DryRunResultBytes(result_bytes.0))
    }

    /// Subscribe to `chainHead_unstable_follow` to obtain all reported blocks by the chain.
    ///
    /// The subscription ID can be used to make queries for the
    /// block's body ([`chainhead_unstable_body`](Rpc::chainhead_unstable_follow)),
    /// block's header ([`chainhead_unstable_header`](Rpc::chainhead_unstable_header)),
    /// block's storage ([`chainhead_unstable_storage`](Rpc::chainhead_unstable_storage)) and submitting
    /// runtime API calls at this block ([`chainhead_unstable_call`](Rpc::chainhead_unstable_call)).
    ///
    /// # Note
    ///
    /// When the user is no longer interested in a block, the user is responsible
    /// for calling the [`chainhead_unstable_unpin`](Rpc::chainhead_unstable_unpin) method.
    /// Failure to do so will result in the subscription being stopped by generating the `Stop` event.
    pub async fn chainhead_unstable_follow(
        &self,
        runtime_updates: bool,
    ) -> Result<Subscription<FollowEvent<T::Hash>>, Error> {
        let subscription = self
            .client
            .subscribe(
                "chainHead_unstable_follow",
                rpc_params![runtime_updates],
                "chainHead_unstable_unfollow",
            )
            .await?;

        Ok(subscription)
    }

    /// Subscribe to `chainHead_unstable_body` to obtain events regarding the block's body.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainhead_unstable_follow`](Rpc::chainhead_unstable_follow).
    pub async fn chainhead_unstable_body(
        &self,
        subscription_id: String,
        hash: T::Hash,
    ) -> Result<Subscription<ChainHeadEvent<String>>, Error> {
        let subscription = self
            .client
            .subscribe(
                "chainHead_unstable_body",
                rpc_params![subscription_id, hash],
                "chainHead_unstable_stopBody",
            )
            .await?;

        Ok(subscription)
    }

    /// Get the block's body using the `chainHead_unstable_header` method.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainhead_unstable_follow`](Rpc::chainhead_unstable_follow).
    pub async fn chainhead_unstable_header(
        &self,
        subscription_id: String,
        hash: T::Hash,
    ) -> Result<Option<String>, Error> {
        let header = self
            .client
            .request(
                "chainHead_unstable_header",
                rpc_params![subscription_id, hash],
            )
            .await?;

        Ok(header)
    }

    /// Subscribe to `chainHead_storage` to obtain events regarding the
    /// block's storage.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainhead_unstable_follow`](Rpc::chainhead_unstable_follow).
    pub async fn chainhead_unstable_storage(
        &self,
        subscription_id: String,
        hash: T::Hash,
        key: &[u8],
        child_key: Option<&[u8]>,
    ) -> Result<Subscription<ChainHeadEvent<Option<String>>>, Error> {
        let subscription = self
            .client
            .subscribe(
                "chainHead_unstable_storage",
                rpc_params![subscription_id, hash, to_hex(key), child_key.map(to_hex)],
                "chainHead_unstable_stopStorage",
            )
            .await?;

        Ok(subscription)
    }

    /// Subscribe to `chainHead_call` to obtain events regarding the
    /// runtime API call.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainhead_unstable_follow`](Rpc::chainhead_unstable_follow).
    pub async fn chainhead_unstable_call(
        &self,
        subscription_id: String,
        hash: T::Hash,
        function: String,
        call_parameters: &[u8],
    ) -> Result<Subscription<ChainHeadEvent<String>>, Error> {
        let subscription = self
            .client
            .subscribe(
                "chainHead_unstable_call",
                rpc_params![subscription_id, hash, function, to_hex(call_parameters)],
                "chainHead_unstable_stopCall",
            )
            .await?;

        Ok(subscription)
    }

    /// Unpin a block reported by the `chainHead_follow` subscription.
    ///
    /// # Note
    ///
    /// The subscription ID is obtained from an open subscription created by
    /// [`chainhead_unstable_follow`](Rpc::chainhead_unstable_follow).
    pub async fn chainhead_unstable_unpin(
        &self,
        subscription_id: String,
        hash: T::Hash,
    ) -> Result<(), Error> {
        self.client
            .request(
                "chainHead_unstable_unpin",
                rpc_params![subscription_id, hash],
            )
            .await?;

        Ok(())
    }

    /// Get genesis hash obtained from the `chainHead_genesisHash` method.
    pub async fn chainhead_unstable_genesishash(&self) -> Result<T::Hash, Error> {
        let hash = self
            .client
            .request("chainHead_unstable_genesisHash", rpc_params![])
            .await?;

        Ok(hash)
    }
}

fn to_hex(bytes: impl AsRef<[u8]>) -> String {
    format!("0x{}", hex::encode(bytes.as_ref()))
}
