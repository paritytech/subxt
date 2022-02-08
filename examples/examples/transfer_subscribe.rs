// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
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
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! To run this example, a local polkadot node should be running.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.11/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
    EventSubscription,
    PairSigner,
};

#[subxt::subxt(runtime_metadata_path = "examples/polkadot_metadata.scale")]
pub mod polkadot {}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<DefaultConfig>::new(sub, decoder);
    sub.filter_event::<polkadot::balances::events::Transfer>();

    api.tx()
        .balances()
        .transfer(dest, 10_000)
        .sign_and_submit(&signer)
        .await?;

    let raw = sub.next().await.unwrap().unwrap();
    let event = <polkadot::balances::events::Transfer as subxt::codec::Decode>::decode(
        &mut &raw.data[..],
    );
    if let Ok(e) = event {
        println!("Balance transfer success: value: {:?}", e.2);
    } else {
        println!("Failed to subscribe to Balances::Transfer Event");
    }
    Ok(())
}

fn foo() {

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    struct Events<T: Config, E: Decode> {
        block_hash: T::Hash,
        bytes: Vec<u8>,
        _event_type: std::marker::PhantomData<E>
    }

    struct EventDetails<E> {
        phase: Phase,
        index: usize,
        event: E // polkadot::Event
    }

    let events_at_block: Events = api.events().at(block_hash);

    let event_subscription: EventSubscription<T> = api.events().subscribe();
    let event_subscription: EventSubscription<T> = api.events().subscribe_finalized();

    while let Some(events) = event_subscription.next().await? {
        // events is of type Events
    }

    // statically decoding is the default if we try iterating over events.
    for event_details /* EventDetails */ in events.iter() {

    }

    // Can we have a `find` that decodes statically?
    let event = events.find::<SomeEvent>();

    // but we need to be able to find events dynamically too, which is able to skip over unknown events:
    let event /* SomeEvent */ = events.find_ignoring_unknown::<SomeEvent>();
    let events /* Vec<SomeEvent> */ = events.find_all_ignoring_unknown::<SomeEvent>();
    // maybe EventDetails<SomeEvent> actually?

    // The transactions API returns RawEvents right now. Let's return Events, above, so that
    // we have the same event API in transactions and subscriptions.

    // Document that .find_events etc dynamically decodes unknown events based on metadata,
    // whereas iter statically decodes from codegen, so prefer `iter` in most cases.
}