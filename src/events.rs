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

//! A library to **sub**mit e**xt**rinsics to a
//! [substrate](https://github.com/paritytech/substrate) node via RPC.

#![deny(missing_docs)]
#![deny(warnings)]

use parity_scale_codec::{Decode, Compact, Error as CodecError};
use crate::{System, Error, metadata::Metadata};

use runtime_primitives::traits::Hash;
use substrate_primitives::storage::StorageChangeSet;
use std::collections::HashMap;
use crate::rpc::{ChainBlock, MapStream};
use futures::future::{
    self,
    Future,
    IntoFuture,
};
use futures::stream::Stream;
use srml_system::{Phase};
use std::any::Any;
use std::marker::Send;

/// Captures data for when an extrinsic is successfully included in a block
#[derive(Debug)]
pub struct ExtrinsicSuccess<T: System> {
    /// Block hash.
    pub block: T::Hash,
    /// Extrinsic hash.
    pub extrinsic: T::Hash,
    /// Events by module and variant.
    events: HashMap<String, Vec<Box<dyn Any + Send>>>,
}

impl<T: System> ExtrinsicSuccess<T> {
    /// Get all events for a module, attempting to decode them.
    /// Returns an empty iterator if no events for that module.
    /// Returns an error if any of the events fail to downcast to the given Event type.
    pub fn module_events<E>(&self, module: &str) -> Result<Vec<E>, Error> where E: Decode + Clone + 'static {
        self.events
            .get(&module.to_string())
            .map_or(Ok(Vec::new()), |events| {
                events
                    .into_iter()
                    .map(|event| {
                        event.downcast_ref::<E>()
                            .map(Clone::clone)
                            .ok_or("Wrong Event type for module".into()) // todo: [AJ] err
                    })
                    .collect::<Result<Vec<E>, _>>()
            })
    }
}

pub struct EventListener {
    metadata: Metadata, // todo: borrow?
    decoders: HashMap<String, fn(&mut &[u8]) -> Result<Box<dyn Any + Send + 'static>, CodecError>>,
}

impl EventListener {
    pub fn new(metadata: Metadata) -> Self {
        Self { metadata, decoders: HashMap::new(), }
    }

    pub fn register_module_event_decoder<Event>(&mut self, module: &str) where Event: Decode + Send + 'static {
        self.decoders.insert(module.to_string(), |input| {
            Decode::decode(input).map(|evt: Event| Box::new(evt) as Box<dyn Any + Send>)
        });
    }

    fn decode_events(&self, input: &mut &[u8]) -> Result<Vec<(Phase, String, Box<dyn Any + Send + 'static>)>, Error> {
        <Compact<u32>>::decode(input)
            .map_err(Into::into)
            .and_then(move |Compact(len)| {
                let len = len as usize;
                let mut r = Vec::new();
                for _ in 0..len {
                    let phase = Phase::decode(input)?;
                    let module_variant = u8::decode(input)?;
                    let module = self.metadata.module_name(module_variant)?;
                    let event_decoder = self.decoders.get(&module).unwrap(); // todo: [AJ]
//                        .ok_or(format!("No Event Decoder registered for '{}'", module).into())?;
                    let event = event_decoder(input)?;
                    r.push((phase, module, event));
                }
                Ok(r)
            })
    }

    /// Waits for events for the block triggered by the extrinsic
    pub fn wait_for_block_events<T: System + 'static>(
        self,
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
                let events =
                    match change_set {
                        None => HashMap::new(),
                        Some(change_set) => {
                            let mut events = HashMap::new();
                            for (_key, data) in change_set.changes {
                                if let Some(data) = data {
                                    if let Ok(any_events) = self.decode_events(&mut &data.0[..]) {
                                        for (phase, module, event) in any_events {
                                            if let Phase::ApplyExtrinsic(i) = phase {
                                                if i as usize == ext_index {
                                                    let events = events.entry(module).or_insert(Vec::new());
                                                    events.push(event)
                                                }
                                            }
                                        }
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
}
