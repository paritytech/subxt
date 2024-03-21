// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Just sanity checking some of the new RPC methods to try and
//! catch differences as the implementations evolve.

use crate::{test_context, utils::node_runtime};
use assert_matches::assert_matches;
use codec::Encode;
use futures::Stream;
use subxt::{
    backend::unstable::rpc_methods::{
        FollowEvent, Initialized, MethodResponse, RuntimeEvent, RuntimeVersionEvent, StorageQuery,
        StorageQueryType,
    },
    utils::AccountId32,
};
use subxt_signer::sr25519::dev;

#[tokio::test]
async fn chainhead_unstable_follow() {
    let ctx = test_context().await;
    let rpc = ctx.unstable_rpc_methods().await;
    let legacy_rpc = ctx.legacy_rpc_methods().await;

    // Check subscription with runtime updates set on false.
    let mut blocks = rpc.chainhead_unstable_follow(false).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    // The initialized event should contain the finalized block hash.
    let finalized_block_hash = legacy_rpc.chain_get_finalized_head().await.unwrap();
    assert_eq!(
        event,
        FollowEvent::Initialized(Initialized {
            finalized_block_hashes: vec![finalized_block_hash],
            finalized_block_runtime: None,
        })
    );

    // Expect subscription to produce runtime versions.
    let mut blocks = rpc.chainhead_unstable_follow(true).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    // The initialized event should contain the finalized block hash.
    let finalized_block_hash = legacy_rpc.chain_get_finalized_head().await.unwrap();
    let runtime_version = ctx.client().runtime_version();

    assert_matches!(
        event,
        FollowEvent::Initialized(init) => {
            assert_eq!(init.finalized_block_hashes, vec![finalized_block_hash]);
            if let Some(RuntimeEvent::Valid(RuntimeVersionEvent { spec })) = init.finalized_block_runtime {
                assert_eq!(spec.spec_version, runtime_version.spec_version);
                assert_eq!(spec.transaction_version, runtime_version.transaction_version);
            } else {
                panic!("runtime details not provided with init event, got {:?}", init.finalized_block_runtime);
            }
        }
    );
}

#[tokio::test]
async fn chainhead_unstable_body() {
    let ctx = test_context().await;
    let rpc = ctx.unstable_rpc_methods().await;

    let mut blocks = rpc.chainhead_unstable_follow(false).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hashes.last().unwrap().clone(),
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap();

    // Fetch the block's body.
    let response = rpc.chainhead_unstable_body(sub_id, hash).await.unwrap();
    let operation_id = match response {
        MethodResponse::Started(started) => started.operation_id,
        MethodResponse::LimitReached => panic!("Expected started response"),
    };

    // Response propagated to `chainHead_follow`.
    let event = next_operation_event(&mut blocks).await;
    assert_matches!(
        event,
        FollowEvent::OperationBodyDone(done) if done.operation_id == operation_id
    );
}

#[tokio::test]
async fn chainhead_unstable_header() {
    let ctx = test_context().await;
    let rpc = ctx.unstable_rpc_methods().await;
    let legacy_rpc = ctx.legacy_rpc_methods().await;

    let mut blocks = rpc.chainhead_unstable_follow(false).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hashes.last().unwrap().clone(),
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap();

    let new_header = legacy_rpc
        .chain_get_header(Some(hash))
        .await
        .unwrap()
        .unwrap();
    let old_header = rpc
        .chainhead_unstable_header(sub_id, hash)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(new_header, old_header);
}

#[tokio::test]
async fn chainhead_unstable_storage() {
    let ctx = test_context().await;
    let api = ctx.client();
    let rpc = ctx.unstable_rpc_methods().await;

    let mut blocks = rpc.chainhead_unstable_follow(false).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hashes.last().unwrap().clone(),
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap();

    let alice: AccountId32 = dev::alice().public_key().into();
    let addr = node_runtime::storage().system().account(alice);
    let addr_bytes = api.storage().address_bytes(&addr).unwrap();

    let items = vec![StorageQuery {
        key: addr_bytes.as_slice(),
        query_type: StorageQueryType::Value,
    }];

    // Fetch storage.
    let response = rpc
        .chainhead_unstable_storage(sub_id, hash, items, None)
        .await
        .unwrap();
    let operation_id = match response {
        MethodResponse::Started(started) => started.operation_id,
        MethodResponse::LimitReached => panic!("Expected started response"),
    };

    // Response propagated to `chainHead_follow`.
    let event = next_operation_event(&mut blocks).await;
    assert_matches!(
        event,
        FollowEvent::OperationStorageItems(res) if res.operation_id == operation_id &&
            res.items.len() == 1 &&
            res.items[0].key.0 == addr_bytes
    );

    let event = next_operation_event(&mut blocks).await;
    assert_matches!(event, FollowEvent::OperationStorageDone(res) if res.operation_id == operation_id);
}

#[tokio::test]
async fn chainhead_unstable_call() {
    let ctx = test_context().await;
    let rpc = ctx.unstable_rpc_methods().await;

    let mut blocks = rpc.chainhead_unstable_follow(true).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hashes.last().unwrap().clone(),
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap();

    let alice_id = dev::alice().public_key().to_account_id();
    // Runtime API call.
    let response = rpc
        .chainhead_unstable_call(
            sub_id,
            hash,
            "AccountNonceApi_account_nonce",
            &alice_id.encode(),
        )
        .await
        .unwrap();
    let operation_id = match response {
        MethodResponse::Started(started) => started.operation_id,
        MethodResponse::LimitReached => panic!("Expected started response"),
    };

    // Response propagated to `chainHead_follow`.
    let event = next_operation_event(&mut blocks).await;
    assert_matches!(
        event,
        FollowEvent::OperationCallDone(res) if res.operation_id == operation_id
    );
}

#[tokio::test]
async fn chainhead_unstable_unpin() {
    let ctx = test_context().await;
    let rpc = ctx.unstable_rpc_methods().await;

    let mut blocks = rpc.chainhead_unstable_follow(true).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hashes.last().unwrap().clone(),
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap();

    assert!(rpc.chainhead_unstable_unpin(sub_id, hash).await.is_ok());
    // The block was already unpinned.
    assert!(rpc.chainhead_unstable_unpin(sub_id, hash).await.is_err());
}

#[tokio::test]
async fn chainspec_v1_genesishash() {
    let ctx = test_context().await;
    let old_rpc = ctx.legacy_rpc_methods().await;
    let rpc = ctx.unstable_rpc_methods().await;

    let a = old_rpc.genesis_hash().await.unwrap();
    let b = rpc.chainspec_v1_genesis_hash().await.unwrap();

    assert_eq!(a, b);
}

#[tokio::test]
async fn chainspec_v1_chainname() {
    let ctx = test_context().await;
    let old_rpc = ctx.legacy_rpc_methods().await;
    let rpc = ctx.unstable_rpc_methods().await;

    let a = old_rpc.system_chain().await.unwrap();
    let b = rpc.chainspec_v1_chain_name().await.unwrap();

    assert_eq!(a, b);
}

#[tokio::test]
async fn chainspec_v1_properties() {
    let ctx = test_context().await;
    let old_rpc = ctx.legacy_rpc_methods().await;
    let rpc = ctx.unstable_rpc_methods().await;

    let a = old_rpc.system_properties().await.unwrap();
    let b = rpc.chainspec_v1_properties().await.unwrap();

    assert_eq!(a, b);
}

#[tokio::test]
async fn transaction_unstable_submit_and_watch() {
    let ctx = test_context().await;
    let rpc = ctx.unstable_rpc_methods().await;

    // Build and sign some random tx, just to get some appropriate bytes:
    let payload = node_runtime::tx().system().remark(b"hello".to_vec());
    let tx_bytes = ctx
        .client()
        .tx()
        .create_signed_offline(&payload, &dev::alice(), Default::default())
        .unwrap()
        .into_encoded();

    // Test submitting it:
    let mut sub = rpc
        .transaction_unstable_submit_and_watch(&tx_bytes)
        .await
        .unwrap();

    // Check that the messages we get back on the way to it finishing deserialize ok
    // (this will miss some cases).
    while let Some(_ev) = sub.next().await.transpose().unwrap() {
        // This stream should end when it hits the relevant stopping event.
        // If the test continues forever then something isn't working.
        // If we hit an error then that's also an issue!
    }
}

/// Ignore block related events and obtain the next event related to an operation.
async fn next_operation_event<
    T: serde::de::DeserializeOwned,
    S: Unpin + Stream<Item = Result<FollowEvent<T>, subxt::Error>>,
>(
    sub: &mut S,
) -> FollowEvent<T> {
    use futures::StreamExt;

    // Number of events to wait for the next operation event.
    const NUM_EVENTS: usize = 10;

    for _ in 0..NUM_EVENTS {
        let event = sub.next().await.unwrap().unwrap();

        match event {
            // Can also return the `Stop` event for better debugging.
            FollowEvent::Initialized(_)
            | FollowEvent::NewBlock(_)
            | FollowEvent::BestBlockChanged(_)
            | FollowEvent::Finalized(_) => continue,
            _ => (),
        };

        return event;
    }

    panic!("Cannot find operation related event after {NUM_EVENTS} produced events");
}

#[tokio::test]
async fn transaction_unstable_broadcast() {
    let alice = dev::alice();
    let bob = dev::bob();
    let bob_address: MultiAddress<AccountId32, u32> = bob.public_key().into();

    let ctx = test_context().await;
    let api = ctx.client();
    let rpc = ctx.unstable_rpc_methods().await;

    let tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(bob_address.clone(), 10_001);

    let tx_bytes = ctx
        .client()
        .tx()
        .create_signed_offline(&payload, &dev::alice(), Default::default())
        .unwrap()
        .into_encoded();

    // Subscribe to finalized blocks.
    let mut finalized_sub = api.blocks().subscribe_finalized().await.unwrap();
    // Expect the tx to be encountered in a maximum number of blocks.
    let mut num_blocks: usize = 5;

    // Submit the transaction.
    let operation_id = rpc
        .transaction_unstable_broadcast(&tx_bytes)
        .await
        .unwrap()
        .expect("Server is not overloaded by 1 tx; qed");

    while let Some(finalized) = finalized_sub.next().await {
        let finalized = finalized.unwrap();

        // Started with positive, should not overflow.
        num_blocks.saturating_sub(1);
        if num_blocks == 0 {
            panic!("Did not find the tx in due time");
        }

        let extrinsics = block.extrinsics().await.unwrap();
        let block_extrinsics = extrinsics
            .iter()
            .map(|res| res.unwrap())
            .collect::<Vec<_>>();

        // All blocks must contain the timestamp, we expect the next extrinsics to be our own.
        let Some(tx) = block_extrinsics.get(1) else {
            continue;
        };

        let ext = tx
            .as_extrinsic::<node_runtime::balances::calls::types::TransferAllowDeath>()
            .unwrap()
            .unwrap();
        assert_eq!(ext.value, 10_001);
        assert!(tx.is_signed());
        return;
    }
}
