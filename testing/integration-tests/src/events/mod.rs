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

// Check that we can subscribe to non-finalized block events.
#[tokio::test]
async fn non_finalized_block_subscription() -> Result<(), subxt::BasicError> {
    tracing_subscriber::fmt::try_init().ok();
    let ctx = test_context().await;

    let mut event_sub = ctx.api.events().subscribe().await?;

    // Wait for the next set of events, and check that the
    // associated block hash is not finalized yet.
    let events = event_sub.next().await.unwrap()?;
    let event_block_hash = events.block_hash();
    let current_block_hash = ctx.api.client.rpc().block_hash(None).await?.unwrap();

    assert_eq!(event_block_hash, current_block_hash);
    Ok(())
}

// Check that we can subscribe to finalized block events.
#[tokio::test]
async fn finalized_block_subscription() -> Result<(), subxt::BasicError> {
    tracing_subscriber::fmt::try_init().ok();
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
#[tokio::test]
async fn subscription_produces_events_each_block() -> Result<(), subxt::BasicError> {
    tracing_subscriber::fmt::try_init().ok();
    let ctx = test_context().await;

    let mut event_sub = ctx.api.events().subscribe().await?;

    for i in 0..3 {
        let events = event_sub
            .next()
            .await
            .expect("events expected each block")?;
        let success_event = events
            .find_first::<system::events::ExtrinsicSuccess>()
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
#[tokio::test]
async fn balance_transfer_subscription() -> Result<(), subxt::BasicError> {
    tracing_subscriber::fmt::try_init().ok();
    let ctx = test_context().await;

    // Subscribe to balance transfer events, ignoring all else.
    let event_sub = ctx
        .api
        .events()
        .subscribe()
        .await?
        .filter_events::<(balances::events::Transfer,)>();

    // Calling `.next()` on the above borrows it, and the `filter_map`
    // means it's no longer `Unpin`, so we pin it on the stack:
    futures::pin_mut!(event_sub);

    // Make a transfer:
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id();
    ctx.api
        .tx()
        .balances()
        .transfer(bob.clone().into(), 10_000)?
        .sign_and_submit_then_watch_default(&alice)
        .await?;

    // Wait for the next balance transfer event in our subscription stream
    // and check that it lines up:
    let event = event_sub.next().await.unwrap().unwrap().event;
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

#[tokio::test]
async fn missing_block_headers_will_be_filled_in() -> Result<(), subxt::BasicError> {
    // This function is not publically available to use, but contains
    // the key logic for filling in missing blocks, so we want to test it.
    // This is used in `subscribe_finalized` to ensure no block headers are
    // missed.
    use subxt::events::subscribe_to_block_headers_filling_in_gaps;

    let ctx = test_context().await;

    // Manually subscribe to the next 6 finalized block headers, but deliberately
    // filter out some in the middle so we get back b _ _ b _ b. This guarantees
    // that there will be some gaps, even if there aren't any from the subscription.
    let some_finalized_blocks = ctx
        .api
        .client
        .rpc()
        .subscribe_finalized_blocks()
        .await?
        .enumerate()
        .take(6)
        .filter(|(n, _)| {
            let n = *n;
            async move { n == 0 || n == 3 || n == 5 }
        })
        .map(|(_, h)| h);

    // This should spot any gaps in the middle and fill them back in.
    let all_finalized_blocks = subscribe_to_block_headers_filling_in_gaps(
        &ctx.api.client,
        None,
        some_finalized_blocks,
    );
    futures::pin_mut!(all_finalized_blocks);

    // Iterate the block headers, making sure we get them all in order.
    let mut last_block_number = None;
    while let Some(header) = all_finalized_blocks.next().await {
        let header = header?;

        use sp_runtime::traits::Header;
        let block_number: u128 = (*header.number()).into();

        if let Some(last) = last_block_number {
            assert_eq!(last + 1, block_number);
        }
        last_block_number = Some(block_number);
    }

    Ok(())
}

// This is just a compile-time check that we can subscribe to events in
// a context that requires the event subscription/filtering to be Send-able.
// We test a typical use of EventSubscription and FilterEvents. We don't need
// to run this code; just check that it compiles.
#[allow(unused)]
async fn check_events_are_sendable() {
    // check that EventSubscription can be used across await points.
    tokio::task::spawn(async {
        let ctx = test_context().await;

        let mut event_sub = ctx.api.events().subscribe().await?;

        while let Some(ev) = event_sub.next().await {
            // if `event_sub` doesn't implement Send, we can't hold
            // it across an await point inside of a tokio::spawn, which
            // requires Send. This will lead to a compile error.
        }

        Ok::<_, subxt::BasicError>(())
    });

    // Check that FilterEvents can be used across await points.
    tokio::task::spawn(async {
        let ctx = test_context().await;

        let mut event_sub = ctx
            .api
            .events()
            .subscribe()
            .await?
            .filter_events::<(balances::events::Transfer,)>();

        while let Some(ev) = event_sub.next().await {
            // if `event_sub` doesn't implement Send, we can't hold
            // it across an await point inside of a tokio::spawn, which
            // requires Send; This will lead to a compile error.
        }

        Ok::<_, subxt::BasicError>(())
    });
}
