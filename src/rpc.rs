// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

// jsonrpsee subscriptions are interminable.
// Allows `while let status = subscription.next().await {}`
// Related: https://github.com/paritytech/substrate-subxt/issues/66
#![allow(irrefutable_let_patterns)]

use codec::{
    Decode,
    Encode,
    Error as CodecError,
};
use core::{
    convert::TryInto,
    marker::PhantomData,
};
use frame_metadata::RuntimeMetadataPrefixed;
use jsonrpsee::{
    client::Subscription,
    common::{
        to_value as to_json_value,
        Params,
    },
    Client,
};
use sc_rpc_api::state::ReadProof;
use serde::Serialize;
use sp_core::{
    storage::{
        StorageChangeSet,
        StorageData,
        StorageKey,
    },
    twox_128,
    Bytes,
};
use sp_rpc::{
    list::ListOrValue,
    number::NumberOrHex,
};
use sp_runtime::{
    generic::{
        Block,
        SignedBlock,
    },
    traits::Hash,
};
use sp_transaction_pool::TransactionStatus;
use sp_version::RuntimeVersion;

use crate::{
    error::Error,
    events::{
        EventsDecoder,
        RawEvent,
    },
    frame::{
        system::System,
        Event,
    },
    metadata::Metadata,
    runtimes::Runtime,
    subscription::EventSubscription,
};

pub type ChainBlock<T> =
    SignedBlock<Block<<T as System>::Header, <T as System>::Extrinsic>>;

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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
/// System properties for a Substrate-based runtime
pub struct SystemProperties {
    /// The address format
    pub ss58_format: u8,
    /// The number of digits after the decimal point in the native token
    pub token_decimals: u8,
    /// The symbol of the native token
    pub token_symbol: String,
}

/// Client for substrate rpc interfaces
pub struct Rpc<T: Runtime> {
    client: Client,
    marker: PhantomData<T>,
}

impl<T: Runtime> Clone for Rpc<T> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Runtime> Rpc<T> {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            marker: PhantomData,
        }
    }

    /// Fetch a storage key
    pub async fn storage(
        &self,
        key: &StorageKey,
        hash: Option<T::Hash>,
    ) -> Result<Option<StorageData>, Error> {
        let params = Params::Array(vec![to_json_value(key)?, to_json_value(hash)?]);
        let data = self.client.request("state_getStorage", params).await?;
        log::debug!("state_getStorage {:?}", data);
        Ok(data)
    }

    /// Returns the keys with prefix with pagination support.
    /// Up to `count` keys will be returned.
    /// If `start_key` is passed, return next keys in storage in lexicographic order.
    pub async fn storage_keys_paged(
        &self,
        prefix: Option<StorageKey>,
        count: u32,
        start_key: Option<StorageKey>,
        hash: Option<T::Hash>,
    ) -> Result<Vec<StorageKey>, Error> {
        let params = Params::Array(vec![
            to_json_value(prefix)?,
            to_json_value(count)?,
            to_json_value(start_key)?,
            to_json_value(hash)?,
        ]);
        let data = self.client.request("state_getKeysPaged", params).await?;
        log::debug!("state_getKeysPaged {:?}", data);
        Ok(data)
    }

    /// Query historical storage entries
    pub async fn query_storage(
        &self,
        keys: Vec<StorageKey>,
        from: T::Hash,
        to: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<<T as System>::Hash>>, Error> {
        let params = Params::Array(vec![
            to_json_value(keys)?,
            to_json_value(from)?,
            to_json_value(to)?,
        ]);
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
    ) -> Result<Vec<StorageChangeSet<<T as System>::Hash>>, Error> {
        let params = Params::Array(vec![to_json_value(keys)?, to_json_value(at)?]);
        self.client
            .request("state_queryStorage", params)
            .await
            .map_err(Into::into)
    }

    /// Fetch the genesis hash
    pub async fn genesis_hash(&self) -> Result<T::Hash, Error> {
        let block_zero = Some(ListOrValue::Value(NumberOrHex::Number(0)));
        let params = Params::Array(vec![to_json_value(block_zero)?]);
        let list_or_value: ListOrValue<Option<T::Hash>> =
            self.client.request("chain_getBlockHash", params).await?;
        match list_or_value {
            ListOrValue::Value(genesis_hash) => {
                genesis_hash.ok_or_else(|| "Genesis hash not found".into())
            }
            ListOrValue::List(_) => Err("Expected a Value, got a List".into()),
        }
    }

    /// Fetch the metadata
    pub async fn metadata(&self) -> Result<Metadata, Error> {
        let bytes: Bytes = self
            .client
            .request("state_getMetadata", Params::None)
            .await?;
        let meta: RuntimeMetadataPrefixed = Decode::decode(&mut &bytes[..])?;
        let metadata: Metadata = meta.try_into()?;
        Ok(metadata)
    }

    /// Fetch system properties
    pub async fn system_properties(&self) -> Result<SystemProperties, Error> {
        Ok(self
            .client
            .request("system_properties", Params::None)
            .await?)
    }

    /// Get a header
    pub async fn header(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<Option<T::Header>, Error> {
        let params = Params::Array(vec![to_json_value(hash)?]);
        let header = self.client.request("chain_getHeader", params).await?;
        Ok(header)
    }

    /// Get a block hash, returns hash of latest block by default
    pub async fn block_hash(
        &self,
        block_number: Option<BlockNumber>,
    ) -> Result<Option<T::Hash>, Error> {
        let block_number = block_number.map(ListOrValue::Value);
        let params = Params::Array(vec![to_json_value(block_number)?]);
        let list_or_value = self.client.request("chain_getBlockHash", params).await?;
        match list_or_value {
            ListOrValue::Value(hash) => Ok(hash),
            ListOrValue::List(_) => Err("Expected a Value, got a List".into()),
        }
    }

    /// Get a block hash of the latest finalized block
    pub async fn finalized_head(&self) -> Result<T::Hash, Error> {
        let hash = self
            .client
            .request("chain_getFinalizedHead", Params::None)
            .await?;
        Ok(hash)
    }

    /// Get a Block
    pub async fn block(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<Option<ChainBlock<T>>, Error> {
        let params = Params::Array(vec![to_json_value(hash)?]);
        let block = self.client.request("chain_getBlock", params).await?;
        Ok(block)
    }

    /// Get proof of storage entries at a specific block's state.
    pub async fn read_proof(
        &self,
        keys: Vec<StorageKey>,
        hash: Option<T::Hash>,
    ) -> Result<ReadProof<T::Hash>, Error> {
        let params = Params::Array(vec![to_json_value(keys)?, to_json_value(hash)?]);
        let proof = self.client.request("state_getReadProof", params).await?;
        Ok(proof)
    }

    /// Fetch the runtime version
    pub async fn runtime_version(
        &self,
        at: Option<T::Hash>,
    ) -> Result<RuntimeVersion, Error> {
        let params = Params::Array(vec![to_json_value(at)?]);
        let version = self
            .client
            .request("state_getRuntimeVersion", params)
            .await?;
        Ok(version)
    }

    /// Subscribe to substrate System Events
    pub async fn subscribe_events(
        &self,
    ) -> Result<Subscription<StorageChangeSet<T::Hash>>, Error> {
        let mut storage_key = twox_128(b"System").to_vec();
        storage_key.extend(twox_128(b"Events").to_vec());
        log::debug!("Events storage key {:?}", hex::encode(&storage_key));

        let keys = Some(vec![StorageKey(storage_key)]);
        let params = Params::Array(vec![to_json_value(keys)?]);

        let subscription = self
            .client
            .subscribe("state_subscribeStorage", params, "state_unsubscribeStorage")
            .await?;
        Ok(subscription)
    }

    /// Subscribe to blocks.
    pub async fn subscribe_blocks(&self) -> Result<Subscription<T::Header>, Error> {
        let subscription = self
            .client
            .subscribe(
                "chain_subscribeNewHeads",
                Params::None,
                "chain_subscribeNewHeads",
            )
            .await?;

        Ok(subscription)
    }

    /// Subscribe to finalized blocks.
    pub async fn subscribe_finalized_blocks(
        &self,
    ) -> Result<Subscription<T::Header>, Error> {
        let subscription = self
            .client
            .subscribe(
                "chain_subscribeFinalizedHeads",
                Params::None,
                "chain_subscribeFinalizedHeads",
            )
            .await?;
        Ok(subscription)
    }

    /// Create and submit an extrinsic and return corresponding Hash if successful
    pub async fn submit_extrinsic<E: Encode>(
        &self,
        extrinsic: E,
    ) -> Result<T::Hash, Error> {
        let bytes: Bytes = extrinsic.encode().into();
        let params = Params::Array(vec![to_json_value(bytes)?]);
        let xt_hash = self
            .client
            .request("author_submitExtrinsic", params)
            .await?;
        Ok(xt_hash)
    }

    pub async fn watch_extrinsic<E: Encode>(
        &self,
        extrinsic: E,
    ) -> Result<Subscription<TransactionStatus<T::Hash, T::Hash>>, Error> {
        let bytes: Bytes = extrinsic.encode().into();
        let params = Params::Array(vec![to_json_value(bytes)?]);
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

    /// Create and submit an extrinsic and return corresponding Event if successful
    pub async fn submit_and_watch_extrinsic<E: Encode + 'static>(
        &self,
        extrinsic: E,
        decoder: EventsDecoder<T>,
    ) -> Result<ExtrinsicSuccess<T>, Error> {
        let ext_hash = T::Hashing::hash_of(&extrinsic);
        log::info!("Submitting Extrinsic `{:?}`", ext_hash);

        let events_sub = self.subscribe_events().await?;
        let mut xt_sub = self.watch_extrinsic(extrinsic).await?;

        while let status = xt_sub.next().await {
            log::info!("received status {:?}", status);
            match status {
                // ignore in progress extrinsic for now
                TransactionStatus::Future
                | TransactionStatus::Ready
                | TransactionStatus::Broadcast(_) => continue,
                TransactionStatus::InBlock(block_hash) => {
                    log::info!("Fetching block {:?}", block_hash);
                    let block = self.block(Some(block_hash)).await?;
                    return match block {
                        Some(signed_block) => {
                            log::info!(
                                "Found block {:?}, with {} extrinsics",
                                block_hash,
                                signed_block.block.extrinsics.len()
                            );
                            let ext_index = signed_block
                                .block
                                .extrinsics
                                .iter()
                                .position(|ext| {
                                    let hash = T::Hashing::hash_of(ext);
                                    hash == ext_hash
                                })
                                .ok_or_else(|| {
                                    Error::Other(format!(
                                        "Failed to find Extrinsic with hash {:?}",
                                        ext_hash,
                                    ))
                                })?;
                            let mut sub = EventSubscription::new(events_sub, decoder);
                            sub.filter_extrinsic(block_hash, ext_index);
                            let mut events = vec![];
                            while let Some(event) = sub.next().await {
                                events.push(event?);
                            }
                            Ok(ExtrinsicSuccess {
                                block: block_hash,
                                extrinsic: ext_hash,
                                events,
                            })
                        }
                        None => {
                            Err(format!("Failed to find block {:?}", block_hash).into())
                        }
                    }
                }
                TransactionStatus::Invalid => return Err("Extrinsic Invalid".into()),
                TransactionStatus::Usurped(_) => return Err("Extrinsic Usurped".into()),
                TransactionStatus::Dropped => return Err("Extrinsic Dropped".into()),
                TransactionStatus::Retracted(_) => {
                    return Err("Extrinsic Retracted".into())
                }
                // should have made it `InBlock` before either of these
                TransactionStatus::Finalized(_) => {
                    return Err("Extrinsic Finalized".into())
                }
                TransactionStatus::FinalityTimeout(_) => {
                    return Err("Extrinsic FinalityTimeout".into())
                }
            }
        }
        unreachable!()
    }

    /// Insert a key into the keystore.
    pub async fn insert_key(
        &self,
        key_type: String,
        suri: String,
        public: Bytes,
    ) -> Result<(), Error> {
        let params = Params::Array(vec![
            to_json_value(key_type)?,
            to_json_value(suri)?,
            to_json_value(public)?,
        ]);
        self.client.request("author_insertKey", params).await?;
        Ok(())
    }

    /// Generate new session keys and returns the corresponding public keys.
    pub async fn rotate_keys(&self) -> Result<Bytes, Error> {
        Ok(self
            .client
            .request("author_rotateKeys", Params::None)
            .await?)
    }

    /// Checks if the keystore has private keys for the given session public keys.
    ///
    /// `session_keys` is the SCALE encoded session keys object from the runtime.
    ///
    /// Returns `true` iff all private keys could be found.
    pub async fn has_session_keys(&self, session_keys: Bytes) -> Result<bool, Error> {
        let params = Params::Array(vec![to_json_value(session_keys)?]);
        Ok(self.client.request("author_hasSessionKeys", params).await?)
    }

    /// Checks if the keystore has private keys for the given public key and key type.
    ///
    /// Returns `true` if a private key could be found.
    pub async fn has_key(
        &self,
        public_key: Bytes,
        key_type: String,
    ) -> Result<bool, Error> {
        let params =
            Params::Array(vec![to_json_value(public_key)?, to_json_value(key_type)?]);
        Ok(self.client.request("author_hasKey", params).await?)
    }
}

/// Captures data for when an extrinsic is successfully included in a block
#[derive(Debug)]
pub struct ExtrinsicSuccess<T: System> {
    /// Block hash.
    pub block: T::Hash,
    /// Extrinsic hash.
    pub extrinsic: T::Hash,
    /// Raw runtime events, can be decoded by the caller.
    pub events: Vec<RawEvent>,
}

impl<T: System> ExtrinsicSuccess<T> {
    /// Find the Event for the given module/variant, with raw encoded event data.
    /// Returns `None` if the Event is not found.
    pub fn find_event_raw(&self, module: &str, variant: &str) -> Option<&RawEvent> {
        self.events
            .iter()
            .find(|raw| raw.module == module && raw.variant == variant)
    }

    /// Find the Event for the given module/variant, attempting to decode the event data.
    /// Returns `None` if the Event is not found.
    /// Returns `Err` if the data fails to decode into the supplied type.
    pub fn find_event<E: Event<T>>(&self) -> Result<Option<E>, CodecError> {
        if let Some(event) = self.find_event_raw(E::MODULE, E::EVENT) {
            Ok(Some(E::decode(&mut &event.data[..])?))
        } else {
            Ok(None)
        }
    }
}
