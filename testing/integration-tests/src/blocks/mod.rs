// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::test_context;
use futures::StreamExt;

// Check that we can subscribe to non-finalized blocks.
#[tokio::test]
async fn non_finalized_headers_subscription() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut sub = api.blocks().subscribe_headers().await?;

    // Wait for the next set of headers, and check that the
    // associated block hash is the one we just finalized.
    // (this can be a bit slow as we have to wait for finalization)
    let header = sub.next().await.unwrap()?;
    let block_hash = header.hash();
    let current_block_hash = api.rpc().block_hash(None).await?.unwrap();

    assert_eq!(block_hash, current_block_hash);
    Ok(())
}

// Check that we can subscribe to finalized blocks.
#[tokio::test]
async fn finalized_headers_subscription() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut sub = api.blocks().subscribe_finalized_headers().await?;

    // Wait for the next set of headers, and check that the
    // associated block hash is the one we just finalized.
    // (this can be a bit slow as we have to wait for finalization)
    let header = sub.next().await.unwrap()?;
    let finalized_hash = api.rpc().finalized_head().await?;

    assert_eq!(header.hash(), finalized_hash);
    Ok(())
}

#[tokio::test]
async fn missing_block_headers_will_be_filled_in() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    // Manually subscribe to the next 6 finalized block headers, but deliberately
    // filter out some in the middle so we get back b _ _ b _ b. This guarantees
    // that there will be some gaps, even if there aren't any from the subscription.
    let some_finalized_blocks = api
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
    let all_finalized_blocks = subxt::blocks::subscribe_to_block_headers_filling_in_gaps(
        ctx.client().rpc().clone(),
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
