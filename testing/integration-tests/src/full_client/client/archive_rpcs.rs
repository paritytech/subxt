// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Just sanity checking some of the new RPC methods to try and
//! catch differences as the implementations evolve.

use crate::{
    subxt_test, test_context,
    utils::{TestNodeProcess, node_runtime},
};
use codec::Encode;
use futures::{Stream, StreamExt};
use subxt::{
    blocks::Block,
    client::OnlineClient,
    config::{Config, Hasher},
    utils::AccountId32,
};
use subxt_rpcs::methods::chain_head::{
    ArchiveStorageEventItem, Bytes, StorageQuery, StorageQueryType,
};

use subxt_signer::sr25519::dev;

async fn fetch_finalized_blocks<T: Config>(
    ctx: &TestNodeProcess<T>,
    n: usize,
) -> impl Stream<Item = Block<T, OnlineClient<T>>> {
    ctx.client()
        .blocks()
        .subscribe_finalized()
        .await
        .expect("issue subscribing to finalized in fetch_finalized_blocks")
        .skip(1) // <- skip first block in case next is close to being ready already.
        .take(n)
        .map(|r| r.expect("issue fetching block in fetch_finalized_blocks"))
}

#[subxt_test]
async fn archive_v1_body() {
    let ctx = test_context().await;
    let rpc = ctx.chainhead_rpc_methods().await;
    let mut blocks = fetch_finalized_blocks(&ctx, 3).await;

    while let Some(block) = blocks.next().await {
        let subxt_block_bodies = block
            .extrinsics()
            .await
            .unwrap()
            .iter()
            .map(|e| e.bytes().to_vec());
        let archive_block_bodies = rpc
            .archive_v1_body(block.hash())
            .await
            .unwrap()
            .into_iter()
            .flatten()
            .map(|e| e.0);

        // chainHead and archive methods should return same block bodies
        for (a, b) in subxt_block_bodies.zip(archive_block_bodies) {
            assert_eq!(a, b);
        }
    }
}

#[subxt_test]
async fn archive_v1_call() {
    let ctx = test_context().await;
    let rpc = ctx.chainhead_rpc_methods().await;
    let mut blocks = fetch_finalized_blocks(&ctx, 3).await;

    while let Some(block) = blocks.next().await {
        let subxt_metadata_versions = block
            .runtime_api()
            .await
            .call(node_runtime::apis().metadata().metadata_versions())
            .await
            .unwrap()
            .encode();
        let archive_metadata_versions = rpc
            .archive_v1_call(block.hash(), "Metadata_metadata_versions", &[])
            .await
            .unwrap()
            .as_success()
            .unwrap()
            .0;

        assert_eq!(subxt_metadata_versions, archive_metadata_versions);
    }
}

#[subxt_test]
async fn archive_v1_finalized_height() {
    let ctx = test_context().await;
    let rpc = ctx.chainhead_rpc_methods().await;

    // This test is quite ugly. Originally, we asked for finalized blocks from subxt and
    // asserted that the archive height we then get back matches, but that is subject to
    // races between subxt's stream and reality (and failed surprisingly often). To try
    // to avoid this, we weaken the test to just check that the height increments over time.
    let mut last_block_height = None;
    loop {
        // Fetch archive block height.
        let archive_block_height = rpc.archive_v1_finalized_height().await.unwrap();

        // On a dev node we expect blocks to be finalized 1 by 1, so panic
        // if the height we fetch has grown by more than 1.
        if let Some(last) = last_block_height {
            if archive_block_height != last && archive_block_height != last + 1 {
                panic!(
                    "Archive block height should increase 1 at a time, but jumped from {last} to {archive_block_height}"
                );
            }
        }

        last_block_height = Some(archive_block_height);
        if archive_block_height > 5 {
            break;
        }

        // Wait a little before looping
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

#[subxt_test]
async fn archive_v1_genesis_hash() {
    let ctx = test_context().await;
    let rpc = ctx.chainhead_rpc_methods().await;

    let chain_head_genesis_hash = rpc.chainspec_v1_genesis_hash().await.unwrap();
    let archive_genesis_hash = rpc.archive_v1_genesis_hash().await.unwrap();

    assert_eq!(chain_head_genesis_hash, archive_genesis_hash);
}

#[subxt_test]
async fn archive_v1_hash_by_height() {
    let ctx = test_context().await;
    let rpc = ctx.chainhead_rpc_methods().await;
    let mut blocks = fetch_finalized_blocks(&ctx, 3).await;

    while let Some(block) = blocks.next().await {
        let subxt_block_height = block.number() as usize;
        let subxt_block_hash = block.hash();

        let archive_block_hash = rpc
            .archive_v1_hash_by_height(subxt_block_height)
            .await
            .unwrap();

        // Should only ever be 1 finalized block hash.
        assert_eq!(archive_block_hash.len(), 1);
        assert_eq!(subxt_block_hash, archive_block_hash[0]);
    }
}

#[subxt_test]
async fn archive_v1_header() {
    let ctx = test_context().await;
    let rpc = ctx.chainhead_rpc_methods().await;
    let mut blocks = fetch_finalized_blocks(&ctx, 3).await;

    while let Some(block) = blocks.next().await {
        let block_hash = block.hash();

        let subxt_block_header = block.header();
        let archive_block_header = rpc.archive_v1_header(block_hash).await.unwrap().unwrap();

        assert_eq!(subxt_block_header, &archive_block_header);
    }
}

#[subxt_test]
async fn archive_v1_storage() {
    let ctx = test_context().await;
    let rpc = ctx.chainhead_rpc_methods().await;
    let api = ctx.client();
    let hasher = api.hasher();
    let mut blocks = fetch_finalized_blocks(&ctx, 3).await;

    while let Some(block) = blocks.next().await {
        let block_hash = block.hash();
        let alice: AccountId32 = dev::alice().public_key().into();
        let addr = node_runtime::storage().system().account();

        // Fetch value using Subxt to compare against
        let storage_at = api.storage().at(block.reference());
        let storage_entry = storage_at.entry(addr).unwrap();
        let subxt_account_info = storage_entry.fetch((alice.clone(),)).await.unwrap();
        let subxt_account_info_bytes = subxt_account_info.bytes();
        let account_info_addr = storage_entry.key((alice,)).unwrap();

        // Construct archive query; ask for item then hash of item.
        let storage_query = vec![
            StorageQuery {
                key: account_info_addr.as_slice(),
                query_type: StorageQueryType::Value,
            },
            StorageQuery {
                key: account_info_addr.as_slice(),
                query_type: StorageQueryType::Hash,
            },
        ];

        let mut res = rpc
            .archive_v1_storage(block_hash, storage_query, None)
            .await
            .unwrap();

        // Expect item back first in archive response
        let query_item = res.next().await.unwrap().unwrap().as_item().unwrap();

        assert_eq!(
            query_item,
            ArchiveStorageEventItem {
                key: Bytes(account_info_addr.clone()),
                value: Some(Bytes(subxt_account_info_bytes.to_vec())),
                hash: None,
                closest_descendant_merkle_value: None,
                child_trie_key: None
            }
        );

        // Expect item hash back next
        let query_item_hash = res.next().await.unwrap().unwrap().as_item().unwrap();

        assert_eq!(
            query_item_hash,
            ArchiveStorageEventItem {
                key: Bytes(account_info_addr),
                value: None,
                hash: Some(hasher.hash(subxt_account_info_bytes)),
                closest_descendant_merkle_value: None,
                child_trie_key: None
            }
        );

        // Expect nothing else back after
        assert!(res.next().await.unwrap().unwrap().is_done());
        assert!(res.next().await.is_none());
    }
}
