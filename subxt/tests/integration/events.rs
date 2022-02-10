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

use crate::{
    node_runtime::{
        balances,
        system,
    },
    pair_signer,
    test_context,
};
use futures::StreamExt;
use sp_keyring::AccountKeyring;
use subxt::Signer;

// Check that we can subscribe to non-finalized block events.
#[async_std::test]
async fn non_finalized_block_subscription() -> Result<(), subxt::BasicError> {
    env_logger::try_init().ok();
    let ctx = test_context().await;

    let mut event_sub = ctx.api.events().subscribe().await?;

    // Wait for the next set of events, and check that the
    // associated block hash is not finalized yet.
    let events = event_sub.next().await.unwrap()?;
    let event_block_hash = events.block_hash();
    let finalized_hash = ctx.api.client.rpc().finalized_head().await?;

    assert_ne!(event_block_hash, finalized_hash);
    Ok(())
}

// Check that we can subscribe to finalized block events.
#[async_std::test]
async fn finalized_block_subscription() -> Result<(), subxt::BasicError> {
    env_logger::try_init().ok();
    let ctx = test_context().await;

    let mut event_sub = ctx.api.events().subscribe_finalized().await?;

    // Wait for the next set of events, and check that the
    // associated block hash is the one we just finalized.
    // (this can be a bit slow as we have to wait for finalization)
    let events = event_sub.next().await.unwrap()?;
    let event_block_hash = events.block_hash();
    let finalized_hash = ctx.api.client.rpc().finalized_head().await?;

    assert_eq!(event_block_hash, finalized_hash);
    Ok(())
}

// Check that our subscription actually keeps producing events for
// a few blocks.
#[async_std::test]
async fn subscription_produces_events_each_block() -> Result<(), subxt::BasicError> {
    env_logger::try_init().ok();
    let ctx = test_context().await;

    let mut event_sub = ctx.api.events().subscribe().await?;

    for i in 0..3 {
        let events = event_sub
            .next()
            .await
            .expect("events expected each block")?;
        let success_event = events
            .find_first_event::<system::events::ExtrinsicSuccess>()
            .expect("decode error");
        // Every now and then I get no bytes back for the first block events;
        // I assume that this might be the case for the genesis block, so don't
        // worry if no event found (but we should have no decode errors etc either way).
        if i > 0 && success_event.is_none() {
            let n = events.len();
            panic!("Expected an extrinsic success event on iteration {i} (saw {n} other events)")
        }
    }

    Ok(())
}

// Check that our subscription receives events, and we can filter them based on
// it's Stream impl, and ultimately see the event we expect.
#[async_std::test]
async fn balance_transfer_subscription() -> Result<(), subxt::BasicError> {
    env_logger::try_init().ok();
    let ctx = test_context().await;

    // Subscribe to balance transfer events, ignoring all else.
    let event_sub = ctx.api.events().subscribe().await?.filter_map(|events| {
        async move {
            let events = events.ok()?;
            events
                .find_first_event::<balances::events::Transfer>()
                .ok()?
        }
    });

    // Calling `.next()` on the above borrows it, and the `filter_map`
    // means it's no longer `Unpin`, so we pin it on the stack:
    futures::pin_mut!(event_sub);

    // Make a transfer:
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id();
    ctx.api
        .tx()
        .balances()
        .transfer(bob.clone().into(), 10_000)
        .sign_and_submit_then_watch(&alice)
        .await?;

    // Wait for the next balance transfer event in our subscription stream
    // and check that it lines up:
    let event = event_sub.next().await.unwrap();
    assert_eq!(
        event,
        balances::events::Transfer {
            from: alice.account_id().clone(),
            to: bob.clone(),
            amount: 10_000
        }
    );

    Ok(())
}
