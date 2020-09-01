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

use jsonrpsee::client::Subscription;
use sp_core::storage::StorageChangeSet;
use std::collections::VecDeque;

use crate::{
    error::Error,
    events::{
        EventsDecoder,
        Raw,
        RawEvent,
    },
    frame::{
        system::Phase,
        Event,
    },
    runtimes::Runtime,
};

/// Event subscription simplifies filtering a storage change set stream for
/// events of interest.
pub struct EventSubscription<T: Runtime> {
    subscription: Subscription<StorageChangeSet<T::Hash>>,
    decoder: EventsDecoder<T>,
    block: Option<T::Hash>,
    extrinsic: Option<usize>,
    event: Option<(&'static str, &'static str)>,
    events: VecDeque<RawEvent>,
    finished: bool,
}

impl<T: Runtime> EventSubscription<T> {
    /// Creates a new event subscription.
    pub fn new(
        subscription: Subscription<StorageChangeSet<T::Hash>>,
        decoder: EventsDecoder<T>,
    ) -> Self {
        Self {
            subscription,
            decoder,
            block: None,
            extrinsic: None,
            event: None,
            events: Default::default(),
            finished: false,
        }
    }

    /// Only returns events contained in the block with the given hash.
    pub fn filter_block(&mut self, block: T::Hash) {
        self.block = Some(block);
    }

    /// Only returns events from block emitted by extrinsic with index.
    pub fn filter_extrinsic(&mut self, block: T::Hash, ext_index: usize) {
        self.block = Some(block);
        self.extrinsic = Some(ext_index);
    }

    /// Filters events by type.
    pub fn filter_event<E: Event<T>>(&mut self) {
        self.event = Some((E::MODULE, E::EVENT));
    }

    /// Gets the next event.
    pub async fn next(&mut self) -> Option<Result<RawEvent, Error>> {
        loop {
            if let Some(event) = self.events.pop_front() {
                return Some(Ok(event))
            }
            if self.finished {
                return None
            }
            let change_set = self.subscription.next().await;
            if let Some(hash) = self.block.as_ref() {
                if &change_set.block == hash {
                    self.finished = true;
                } else {
                    continue
                }
            }
            for (_key, data) in change_set.changes {
                if let Some(data) = data {
                    let raw_events = match self.decoder.decode_events(&mut &data.0[..]) {
                        Ok(events) => events,
                        Err(error) => return Some(Err(error)),
                    };
                    for (phase, raw) in raw_events {
                        if let Phase::ApplyExtrinsic(i) = phase {
                            if let Some(ext_index) = self.extrinsic {
                                if i as usize != ext_index {
                                    continue
                                }
                            }
                            let event = match raw {
                                Raw::Event(event) => event,
                                Raw::Error(err) => return Some(Err(err.into())),
                            };
                            if let Some((module, variant)) = self.event {
                                if event.module != module || event.variant != variant {
                                    continue
                                }
                            }
                            self.events.push_back(event);
                        }
                    }
                }
            }
        }
    }
}
