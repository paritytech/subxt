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
use num_traits::bounds::Bounded;
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
        RuntimeEvent,
    },
    frame::{
        system::{
            Phase,
            System,
            SystemEvent,
        },
        Event,
    },
    metadata::Metadata,
};

pub type ChainBlock<T> =
    SignedBlock<Block<<T as System>::Header, <T as System>::Extrinsic>>;

/// Wrapper for NumberOrHex to allow custom From impls
#[derive(Serialize)]
#[serde(bound = "<T as System>::BlockNumber: Serialize")]
pub struct BlockNumber<T: System>(NumberOrHex<<T as System>::BlockNumber>);

impl<T> From<NumberOrHex<<T as System>::BlockNumber>> for BlockNumber<T>
where
    T: System,
{
    fn from(x: NumberOrHex<<T as System>::BlockNumber>) -> Self {
        BlockNumber(x)
    }
}

impl<T> From<u32> for BlockNumber<T>
where
    T: System,
    <T as System>::BlockNumber: From<u32>,
{
    fn from(x: u32) -> Self {
        NumberOrHex::Number(x.into()).into()
    }
}

/// Client for substrate rpc interfaces
pub struct Rpc<T: System> {
    client: Client,
    marker: PhantomData<T>,
}

impl<T: System> Clone for Rpc<T> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: System> Rpc<T> {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            marker: PhantomData,
        }
    }

    /// Fetch a storage key
    pub async fn storage<V: Decode>(
        &self,
        key: StorageKey,
        hash: Option<T::Hash>,
    ) -> Result<Option<V>, Error> {
        let params = Params::Array(vec![to_json_value(key)?, to_json_value(hash)?]);
        let data: Option<StorageData> =
            self.client.request("state_getStorage", params).await?;
        match data {
            Some(data) => {
                log::debug!("state_getStorage {:?}", data.0);
                let value = Decode::decode(&mut &data.0[..])?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
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

    /// Fetch the genesis hash
    pub async fn genesis_hash(&self) -> Result<T::Hash, Error> {
        let block_zero = Some(ListOrValue::Value(NumberOrHex::Number(
            T::BlockNumber::min_value(),
        )));
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
        block_number: Option<BlockNumber<T>>,
    ) -> Result<Option<T::Hash>, Error> {
        let block_number = block_number.map(|bn| ListOrValue::Value(bn));
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
    ) -> Result<Subscription<StorageChangeSet<<T as System>::Hash>>, Error> {
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
        self,
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
                            wait_for_block_events(
                                decoder,
                                ext_hash,
                                signed_block,
                                block_hash,
                                events_sub,
                            )
                            .await
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
}

/// Captures data for when an extrinsic is successfully included in a block
#[derive(Debug)]
pub struct ExtrinsicSuccess<T: System> {
    /// Block hash.
    pub block: T::Hash,
    /// Extrinsic hash.
    pub extrinsic: T::Hash,
    /// Raw runtime events, can be decoded by the caller.
    pub events: Vec<RuntimeEvent<T>>,
}

impl<T: System> ExtrinsicSuccess<T> {
    /// Find the Event for the given module/variant, with raw encoded event data.
    /// Returns `None` if the Event is not found.
    pub fn find_event_raw(&self, module: &str, variant: &str) -> Option<&RawEvent> {
        self.events.iter().find_map(|evt| {
            match evt {
                RuntimeEvent::Raw(ref raw)
                    if raw.module == module && raw.variant == variant =>
                {
                    Some(raw)
                }
                _ => None,
            }
        })
    }

    /// Returns all System Events
    pub fn system_events(&self) -> Vec<&SystemEvent<T>> {
        self.events
            .iter()
            .filter_map(|evt| {
                match evt {
                    RuntimeEvent::System(evt) => Some(evt),
                    _ => None,
                }
            })
            .collect()
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

/// Waits for events for the block triggered by the extrinsic
pub async fn wait_for_block_events<T: System>(
    decoder: EventsDecoder<T>,
    ext_hash: T::Hash,
    signed_block: ChainBlock<T>,
    block_hash: T::Hash,
    events_subscription: Subscription<StorageChangeSet<T::Hash>>,
) -> Result<ExtrinsicSuccess<T>, Error> {
    let ext_index = signed_block
        .block
        .extrinsics
        .iter()
        .position(|ext| {
            let hash = T::Hashing::hash_of(ext);
            hash == ext_hash
        })
        .ok_or_else(|| {
            Error::Other(format!("Failed to find Extrinsic with hash {:?}", ext_hash))
        })?;

    let mut subscription = events_subscription;
    while let change_set = subscription.next().await {
        // only interested in events for the given block
        if change_set.block != block_hash {
            continue
        }
        let mut events = Vec::new();
        for (_key, data) in change_set.changes {
            if let Some(data) = data {
                match decoder.decode_events(&mut &data.0[..]) {
                    Ok(raw_events) => {
                        for (phase, event) in raw_events {
                            if let Phase::ApplyExtrinsic(i) = phase {
                                if i as usize == ext_index {
                                    events.push(event)
                                }
                            }
                        }
                    }
                    Err(err) => return Err(err.into()),
                }
            }
        }
        return if events.len() > 0 {
            Ok(ExtrinsicSuccess {
                block: block_hash,
                extrinsic: ext_hash,
                events,
            })
        } else {
            Err(format!("No events found for block {}", block_hash).into())
        }
    }
    unreachable!()
}
