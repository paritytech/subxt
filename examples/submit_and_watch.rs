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

use futures::StreamExt;
use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
    PairSigner,
};

#[subxt::subxt(runtime_metadata_path = "examples/polkadot_metadata.scale")]
pub mod polkadot {}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    simple_transfer().await?;
    simple_transfer_separate_events().await?;
    handle_transfer_events().await?;

    Ok(())
}

/// This is the highest level approach to using this API. We use `wait_for_finalized_success`
/// to wait for the transaction to make it into a finalized block, and also ensure that the
/// transaction was successful according to the associated events.
async fn simple_transfer() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<_>>>();

    let balance_transfer = api
        .tx()
        .balances()
        .transfer(dest, 10_000)
        .sign_and_submit_then_watch(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    let transfer_event =
        balance_transfer.find_first_event::<polkadot::balances::events::Transfer>()?;

    if let Some(event) = transfer_event {
        println!("Balance transfer success: value: {:?}", event.2);
    } else {
        println!("Failed to find Balances::Transfer Event");
    }
    Ok(())
}

/// This is very similar to `simple_transfer`, except to show that we can handle
/// waiting for the transaction to be finalized separately from obtaining and checking
/// for success on the events.
async fn simple_transfer_separate_events() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<_>>>();

    let balance_transfer = api
        .tx()
        .balances()
        .transfer(dest, 10_000)
        .sign_and_submit_then_watch(&signer)
        .await?
        .wait_for_finalized()
        .await?;

    // Now we know it's been finalized, we can get hold of a couple of
    // details, including events. Calling `wait_for_finalized_success` is
    // equivalent to calling `wait_for_finalized` and then `wait_for_success`:
    let _events = balance_transfer.wait_for_success().await?;

    // Alternately, we could just `fetch_events`, which grabs all of the events like
    // the above, but does not check for success, and leaves it up to you:
    let events = balance_transfer.fetch_events().await?;

    let failed_event =
        events.find_first_event::<polkadot::system::events::ExtrinsicFailed>()?;

    if let Some(_ev) = failed_event {
        // We found a failed event; the transfer didn't succeed.
        println!("Balance transfer failed");
    } else {
        // We didn't find a failed event; the transfer succeeded. Find
        // more details about it to report..
        let transfer_event =
            events.find_first_event::<polkadot::balances::events::Transfer>()?;
        if let Some(event) = transfer_event {
            println!("Balance transfer success: value: {:?}", event.2);
        } else {
            println!("Failed to find Balances::Transfer Event");
        }
    }

    Ok(())
}

/// If we need more visibility into the state of the transaction, we can also ditch
/// `wait_for_finalized` entirely and stream the transaction progress events, handling
/// them more manually.
async fn handle_transfer_events() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<_>>>();

    let mut balance_transfer_progress = api
        .tx()
        .balances()
        .transfer(dest, 10_000)
        .sign_and_submit_then_watch(&signer)
        .await?;

    while let Some(ev) = balance_transfer_progress.next().await {
        let ev = ev?;
        use subxt::TransactionStatus::*;

        // Made it into a block, but not finalized.
        if let InBlock(details) = ev {
            println!(
                "Transaction {:?} made it into block {:?}",
                details.extrinsic_hash(),
                details.block_hash()
            );

            let events = details.wait_for_success().await?;
            let transfer_event =
                events.find_first_event::<polkadot::balances::events::Transfer>()?;

            if let Some(event) = transfer_event {
                println!(
                    "Balance transfer is now in block (but not finalized): value: {:?}",
                    event.2
                );
            } else {
                println!("Failed to find Balances::Transfer Event");
            }
        }
        // Finalized!
        else if let Finalized(details) = ev {
            println!(
                "Transaction {:?} is finalized in block {:?}",
                details.extrinsic_hash(),
                details.block_hash()
            );

            let events = details.wait_for_success().await?;
            let transfer_event =
                events.find_first_event::<polkadot::balances::events::Transfer>()?;

            if let Some(event) = transfer_event {
                println!("Balance transfer success: value: {:?}", event.2);
            } else {
                println!("Failed to find Balances::Transfer Event");
            }
        }
        // Report other statuses we see.
        else {
            println!("Current transaction status: {:?}", ev);
        }
    }

    Ok(())
}
