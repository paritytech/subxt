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
    events::{
        EventsDecoder,
        RuntimeEvent,
    },
    metadata::Metadata,
    srml::{
        balances::Balances,
        system::System,
    },
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
    Error as CodecError,
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
    chain::ChainClient,
    state::StateClient,
};
use substrate_rpc_primitives::number::NumberOrHex;

pub type ChainBlock<T> = SignedBlock<Block<<T as System>::Header, OpaqueExtrinsic>>;
pub type BlockNumber<T> = NumberOrHex<<T as System>::BlockNumber>;

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

    /// Get a block hash, returns hash of latest block by default
    pub fn block_hash(
        &self,
        hash: Option<BlockNumber<T>>,
    ) -> impl Future<Item = Option<T::Hash>, Error = Error> {
        self.chain.block_hash(hash).map_err(Into::into)
    }

    /// Get a Block
    pub fn block(
        &self,
        hash: Option<T::Hash>,
    ) -> impl Future<Item = Option<ChainBlock<T>>, Error = Error> {
        self.chain.block(hash).map_err(Into::into)
    }

    /// Fetch the runtime version
    pub fn runtime_version(
        &self,
        at: Option<T::Hash>,
    ) -> impl Future<Item = RuntimeVersion, Error = Error> {
        self.state.runtime_version(at).map_err(Into::into)
    }
}
use futures::{
    future::IntoFuture,
    stream::{
        self,
        Stream,
    },
};
use jsonrpc_core_client::TypedSubscriptionStream;
use runtime_primitives::traits::Hash;
use substrate_primitives::{
    storage::StorageChangeSet,
    twox_128,
};
use txpool::watcher::Status;

use crate::{
    events::RawEvent,
    srml::system::SystemEvent,
};
use srml_system::Phase;

type MapClosure<T> = Box<dyn Fn(T) -> T + Send>;
pub type MapStream<T> = stream::Map<TypedSubscriptionStream<T>, MapClosure<T>>;

impl<T: System + Balances + 'static> Rpc<T> {
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
    pub fn submit_and_watch_extrinsic<E: 'static>(
        self,
        extrinsic: E,
        decoder: EventsDecoder<T>,
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

                    wait_for_block_events(decoder, ext_hash, sb, bh, events)
                })
        })
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
    pub events: Vec<RuntimeEvent>,
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
    pub fn system_events(&self) -> Vec<&SystemEvent> {
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
    /// Returns `Err` if the data fails to decode into the supplied type
    pub fn find_event<E: Decode>(
        &self,
        module: &str,
        variant: &str,
    ) -> Option<Result<E, CodecError>> {
        self.find_event_raw(module, variant)
            .map(|evt| E::decode(&mut &evt.data[..]))
    }
}

/// Waits for events for the block triggered by the extrinsic
pub fn wait_for_block_events<T: System + Balances + 'static>(
    decoder: EventsDecoder<T>,
    ext_hash: T::Hash,
    signed_block: ChainBlock<T>,
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
    events_stream
        .filter(move |event| event.block == block_hash)
        .into_future()
        .map_err(|(e, _)| e.into())
        .join(ext_index)
        .and_then(move |((change_set, _), ext_index)| {
            let events = match change_set {
                None => Vec::new(),
                Some(change_set) => {
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
                                Err(err) => return future::err(err.into()),
                            }
                        }
                    }
                    events
                }
            };
            future::ok(ExtrinsicSuccess {
                block: block_hash,
                extrinsic: ext_hash,
                events,
            })
        })
}
