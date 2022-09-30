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
