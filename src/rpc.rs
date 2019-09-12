// Copyright 2019 Parity Technologies (UK) Ltd.
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

use crate::{
    error::Error,
    metadata::Metadata,
    srml::system::System,
};
use futures::future::{
    self,
    Future,
};
use jsonrpc_core_client::RpcChannel;
use log;
use num_traits::bounds::Bounded;
use parity_scale_codec::{
    Decode,
    Encode,
};

use runtime_metadata::RuntimeMetadataPrefixed;
use runtime_primitives::{
    generic::{
        Block,
        SignedBlock,
    },
    OpaqueExtrinsic,
};
use sr_version::RuntimeVersion;
use std::convert::TryInto;
use substrate_primitives::storage::StorageKey;
use substrate_rpc_api::{
    author::AuthorClient,
    chain::{
        number::NumberOrHex,
        ChainClient,
    },
    state::StateClient,
};

pub type ChainBlock<T> = SignedBlock<Block<<T as System>::Header, OpaqueExtrinsic>>;

/// Client for substrate rpc interfaces
pub struct Rpc<T: System> {
    state: StateClient<T::Hash>,
    chain: ChainClient<T::BlockNumber, T::Hash, T::Header, ChainBlock<T>>,
    author: AuthorClient<T::Hash, T::Hash>,
}

/// Allows connecting to all inner interfaces on the same RpcChannel
impl<T: System> From<RpcChannel> for Rpc<T> {
    fn from(channel: RpcChannel) -> Self {
        Self {
            state: channel.clone().into(),
            chain: channel.clone().into(),
            author: channel.into(),
        }
    }
}

impl<T: System> Rpc<T> {
    /// Fetch a storage key
    pub fn storage<V: Decode>(
        &self,
        key: StorageKey,
    ) -> impl Future<Item = Option<V>, Error = Error> {
        self.state
            .storage(key, None)
            .map(|data| {
                data.map(|d| Decode::decode(&mut &d.0[..]).expect("Valid storage key"))
            })
            .map_err(Into::into)
    }

    /// Fetch the genesis hash
    pub fn genesis_hash(&self) -> impl Future<Item = T::Hash, Error = Error> {
        let block_zero = T::BlockNumber::min_value();
        self.chain
            .block_hash(Some(NumberOrHex::Number(block_zero)))
            .map_err(Into::into)
            .and_then(|genesis_hash| {
                future::result(genesis_hash.ok_or("Genesis hash not found".into()))
            })
    }

    /// Fetch the metadata
    pub fn metadata(&self) -> impl Future<Item = Metadata, Error = Error> {
        self.state
            .metadata(None)
            .map(|bytes| Decode::decode(&mut &bytes[..]).unwrap())
            .map_err(Into::into)
            .and_then(|meta: RuntimeMetadataPrefixed| {
                future::result(meta.try_into().map_err(|err| format!("{:?}", err).into()))
            })
    }

    /// Fetch a Block
    pub fn block(&self, hash: Option<T::Hash>) -> impl Future<Item = Option<ChainBlock<T>>, Error = Error> {
        self.chain.block(hash).map_err(Into::into)
    }

    /// Fetch the runtime version
    pub fn runtime_version(&self, at: Option<T::Hash>) -> impl Future<Item = RuntimeVersion, Error = Error> {
        self.state.runtime_version(at).map_err(Into::into)
    }
}

use crate::ExtrinsicSuccess;
use futures::{
    future::IntoFuture,
    stream::{
        self,
        Stream,
    },
};
use jsonrpc_core_client::TypedSubscriptionStream;
use runtime_primitives::traits::Hash;
use srml_system::EventRecord;
use substrate_primitives::{
    storage::StorageChangeSet,
    twox_128,
};
use transaction_pool::txpool::watcher::Status;

type MapClosure<T> = Box<dyn Fn(T) -> T + Send>;
pub type MapStream<T> = stream::Map<TypedSubscriptionStream<T>, MapClosure<T>>;

impl<T: System> Rpc<T> {
    /// Subscribe to substrate System Events
    pub fn subscribe_events(
        &self,
    ) -> impl Future<Item = MapStream<StorageChangeSet<<T as System>::Hash>>, Error = Error>
    {
        let events_key = b"System Events";
        let storage_key = twox_128(events_key);
        log::debug!("Events storage key {:?}", storage_key);

        let closure: MapClosure<StorageChangeSet<<T as System>::Hash>> =
            Box::new(|event| {
                log::info!("Event {:?}", event);
                event
            });
        self.state
            .subscribe_storage(Some(vec![StorageKey(storage_key.to_vec())]))
            .map(|stream: TypedSubscriptionStream<_>| stream.map(closure))
            .map_err(Into::into)
    }

    /// Subscribe to blocks.
    pub fn subscribe_blocks(
        &self,
    ) -> impl Future<Item = MapStream<T::Header>, Error = Error> {
        let closure: MapClosure<T::Header> = Box::new(|event| {
            log::info!("New block {:?}", event);
            event
        });
        self.chain
            .subscribe_new_heads()
            .map(|stream| stream.map(closure))
            .map_err(Into::into)
    }

    /// Subscribe to finalized blocks.
    pub fn subscribe_finalized_blocks(
        &self,
    ) -> impl Future<Item = MapStream<T::Header>, Error = Error> {
        let closure: MapClosure<T::Header> = Box::new(|event| {
            log::info!("Finalized block {:?}", event);
            event
        });
        self.chain
            .subscribe_finalized_heads()
            .map(|stream| stream.map(closure))
            .map_err(Into::into)
    }

    /// Create and submit an extrinsic and return corresponding Hash if successful
    pub fn submit_extrinsic<E>(
        self,
        extrinsic: E,
    ) -> impl Future<Item = T::Hash, Error = Error>
    where
        E: Encode,
    {
        self.author
            .submit_extrinsic(extrinsic.encode().into())
            .map_err(Into::into)
    }

    /// Create and submit an extrinsic and return corresponding Event if successful
    pub fn submit_and_watch_extrinsic<E>(
        self,
        extrinsic: E,
    ) -> impl Future<Item = ExtrinsicSuccess<T>, Error = Error>
    where
        E: Encode,
    {
        let events = self.subscribe_events().map_err(Into::into);
        events.and_then(move |events| {
            let ext_hash = T::Hashing::hash_of(&extrinsic);
            log::info!("Submitting Extrinsic `{:?}`", ext_hash);

            let chain = self.chain.clone();
            self.author
                .watch_extrinsic(extrinsic.encode().into())
                .map_err(Into::into)
                .and_then(|stream| {
                    stream
                        .filter_map(|status| {
                            log::info!("received status {:?}", status);
                            match status {
                                // ignore in progress extrinsic for now
                                Status::Future | Status::Ready | Status::Broadcast(_) => {
                                    None
                                }
                                Status::Finalized(block_hash) => Some(Ok(block_hash)),
                                Status::Usurped(_) => {
                                    Some(Err("Extrinsic Usurped".into()))
                                }
                                Status::Dropped => Some(Err("Extrinsic Dropped".into())),
                                Status::Invalid => Some(Err("Extrinsic Invalid".into())),
                            }
                        })
                        .into_future()
                        .map_err(|(e, _)| e.into())
                        .and_then(|(result, _)| {
                            log::info!("received result {:?}", result);

                            result
                                .ok_or(Error::from("Stream terminated"))
                                .and_then(|r| r)
                                .into_future()
                        })
                })
                .and_then(move |bh| {
                    log::info!("Fetching block {:?}", bh);
                    chain
                        .block(Some(bh))
                        .map(move |b| (bh, b))
                        .map_err(Into::into)
                })
                .and_then(|(h, b)| {
                    b.ok_or(format!("Failed to find block {:?}", h).into())
                        .map(|b| (h, b))
                        .into_future()
                })
                .and_then(move |(bh, sb)| {
                    log::info!(
                        "Found block {:?}, with {} extrinsics",
                        bh,
                        sb.block.extrinsics.len()
                    );
                    wait_for_block_events::<T>(ext_hash, &sb, bh, events)
                })
        })
    }
}

/// Waits for events for the block triggered by the extrinsic
fn wait_for_block_events<T: System>(
    ext_hash: T::Hash,
    signed_block: &ChainBlock<T>,
    block_hash: T::Hash,
    events_stream: MapStream<StorageChangeSet<T::Hash>>,
) -> impl Future<Item = ExtrinsicSuccess<T>, Error = Error> {
    let ext_index = signed_block
        .block
        .extrinsics
        .iter()
        .position(|ext| {
            let hash = T::Hashing::hash_of(ext);
            hash == ext_hash
        })
        .ok_or(format!("Failed to find Extrinsic with hash {:?}", ext_hash).into())
        .into_future();

    let block_hash = block_hash.clone();
    let block_events = events_stream
        .filter(move |event| event.block == block_hash)
        .into_future()
        .map_err(|(e, _)| e.into())
        .and_then(|(change_set, _)| {
            match change_set {
                None => future::ok(Vec::new()),
                Some(change_set) => {
                    let events = change_set
                        .changes
                        .iter()
                        .filter_map(|(_key, data)| {
                            data.as_ref().map(|data| Decode::decode(&mut &data.0[..]))
                        })
                        .collect::<Result<Vec<Vec<EventRecord<T::Event, T::Hash>>>, _>>()
                        .map(|events| events.into_iter().flat_map(|es| es).collect())
                        .map_err(Into::into);
                    future::result(events)
                }
            }
        });

    block_events
        .join(ext_index)
        .map(move |(events, ext_index)| {
            let events: Vec<T::Event> = events
                .iter()
                .filter_map(|e| {
                    if let srml_system::Phase::ApplyExtrinsic(i) = e.phase {
                        if i as usize == ext_index {
                            Some(e.event.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<T::Event>>();
            ExtrinsicSuccess {
                block: block_hash,
                extrinsic: ext_hash,
                events,
            }
        })
}
