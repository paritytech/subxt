// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::test_context;
use codec::{Compact, Decode};
use frame_metadata::RuntimeMetadataPrefixed;
use futures::StreamExt;

// Check that we can subscribe to non-finalized blocks.
#[tokio::test]
async fn non_finalized_headers_subscription() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut sub = api.blocks().subscribe_best().await?;

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

    let mut sub = api.blocks().subscribe_finalized().await?;

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
        .subscribe_finalized_block_headers()
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
        ctx.client(),
        None,
        some_finalized_blocks,
    );
    futures::pin_mut!(all_finalized_blocks);

    // Iterate the block headers, making sure we get them all in order.
    let mut last_block_number = None;
    while let Some(header) = all_finalized_blocks.next().await {
        let header = header?;

        use subxt::config::Header;
        let block_number: u128 = header.number().into();

        if let Some(last) = last_block_number {
            assert_eq!(last + 1, block_number);
        }
        last_block_number = Some(block_number);
    }

    assert!(last_block_number.is_some());
    Ok(())
}

// Check that we can subscribe to non-finalized blocks.
#[tokio::test]
async fn runtime_api_call() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut sub = api.blocks().subscribe_best().await?;

    let block = sub.next().await.unwrap()?;
    let rt = block.runtime_api().await?;

    let bytes = rt.call_raw("Metadata_metadata", None).await?;
    let cursor = &mut &*bytes;
    let _ = <Compact<u32>>::decode(cursor)?;
    let meta: RuntimeMetadataPrefixed = Decode::decode(cursor)?;
    let metadata_call = match meta.1 {
        frame_metadata::RuntimeMetadata::V14(metadata) => metadata,
        _ => panic!("Metadata V14 unavailable"),
    };

    // Compare the runtime API call against the `state_getMetadata`.
    let metadata = api.rpc().metadata(None).await?;
    let metadata = metadata.runtime_metadata();
    assert_eq!(&metadata_call, metadata);
    Ok(())
}
