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

use sp_keyring::AccountKeyring;
use substrate_subxt::{
    balances::{
        BalancesEventsDecoder,
        TransferCallExt,
        TransferEvent,
    },
    sp_core::Decode,
    ClientBuilder,
    DefaultNodeRuntime,
    EventSubscription,
    EventsDecoder,
    PairSigner,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let client = ClientBuilder::<DefaultNodeRuntime>::new().build().await?;
    let sub = client.subscribe_events().await?;
    let mut decoder = EventsDecoder::<DefaultNodeRuntime>::new(client.metadata().clone());
    decoder.with_balances();
    let mut sub = EventSubscription::<DefaultNodeRuntime>::new(sub, decoder);
    sub.filter_event::<TransferEvent<_>>();
    client.transfer(&signer, &dest, 10_000).await?;
    let raw = sub.next().await.unwrap().unwrap();
    let event = TransferEvent::<DefaultNodeRuntime>::decode(&mut &raw.data[..]);
    if let Ok(e) = event {
        println!("Balance transfer success: value: {:?}", e.amount);
    } else {
        println!("Failed to subscribe to Balances::Transfer Event");
    }
    Ok(())
}
