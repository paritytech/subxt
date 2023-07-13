// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{test_context, utils::node_runtime};
use codec::{Compact, Encode};
use futures::StreamExt;
use subxt::blocks::BlocksClient;
use subxt_metadata::Metadata;
use subxt_signer::sr25519::dev;

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

    // get metadata via state_call.
    let (_, meta1) = rt
        .call_raw::<(Compact<u32>, Metadata)>("Metadata_metadata", None)
        .await?;

    // get metadata via `state_getMetadata`.
    let meta2 = api.rpc().metadata_legacy(None).await?;

    // They should be the same.
    assert_eq!(meta1.encode(), meta2.encode());

    Ok(())
}

#[tokio::test]
async fn decode_extrinsics() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob();

    // Generate a block that has unsigned and signed transactions.
    let tx = node_runtime::tx()
        .balances()
        .transfer(bob.public_key().into(), 10_000);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();

    let in_block = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_in_block()
        .await
        .unwrap();

    let block_hash = in_block.block_hash();

    let block = BlocksClient::new(api).at(block_hash).await.unwrap();
    let extrinsics = block.body().await.unwrap().extrinsics();
    assert_eq!(extrinsics.len(), 2);
    assert_eq!(extrinsics.block_hash(), block_hash);

    assert!(extrinsics
        .has::<node_runtime::balances::calls::types::Transfer>()
        .unwrap());

    assert!(extrinsics
        .find_first::<node_runtime::balances::calls::types::Transfer>()
        .unwrap()
        .is_some());

    let block_extrinsics = extrinsics
        .iter()
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();

    assert_eq!(block_extrinsics.len(), 2);
    let timestamp = block_extrinsics.get(0).unwrap();
    timestamp.as_root_extrinsic::<node_runtime::Call>().unwrap();
    timestamp
        .as_extrinsic::<node_runtime::timestamp::calls::types::Set>()
        .unwrap();
    assert!(!timestamp.is_signed());

    let tx = block_extrinsics.get(1).unwrap();
    tx.as_root_extrinsic::<node_runtime::Call>().unwrap();
    tx.as_extrinsic::<node_runtime::balances::calls::types::Transfer>()
        .unwrap();
    assert!(tx.is_signed());
}
