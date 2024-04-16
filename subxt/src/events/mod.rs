// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes the types and such necessary for working with events.
//! The two main entry points into events are [`crate::OnlineClient::events()`]
//! and calls like [crate::tx::TxProgress::wait_for_finalized_success()].
use crate::client::OnlineClientT;
use crate::Error;
use subxt_core::{Config, Metadata};

mod events_client;
pub use events_client::EventsClient;
pub use subxt_core::events::{EventDetails, Events, Phase, StaticEvent};

/// Creates a new [`Events`] instance by fetching the corresponding bytes at `block_hash` from the client.
pub async fn new_events_from_client<T, C>(
    metadata: Metadata,
    block_hash: T::Hash,
    client: C,
) -> Result<Events<T>, Error>
where
    T: Config,
    C: OnlineClientT<T>,
{
    let event_bytes = events_client::get_event_bytes(client.backend(), block_hash).await?;
    Ok(Events::<T>::decode_from(event_bytes, metadata))
}
