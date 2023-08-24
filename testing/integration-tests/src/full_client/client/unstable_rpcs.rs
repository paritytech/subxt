// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Just sanity checking some of the new RPC methods to try and
//! catch differences as the implementations evolve.

use crate::{test_context, utils::node_runtime};
use assert_matches::assert_matches;
use codec::Encode;
use subxt::{
    backend::rpc::RpcSubscription,
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
            finalized_block_hash,
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
            assert_eq!(init.finalized_block_hash, finalized_block_hash);
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
        FollowEvent::Initialized(init) => init.finalized_block_hash,
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
        FollowEvent::Initialized(init) => init.finalized_block_hash,
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
        FollowEvent::Initialized(init) => init.finalized_block_hash,
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
            res.items[0].key == format!("0x{}", hex::encode(addr_bytes))
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
        FollowEvent::Initialized(init) => init.finalized_block_hash,
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
        FollowEvent::Initialized(init) => init.finalized_block_hash,
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap();

    assert!(rpc
        .chainhead_unstable_unpin(sub_id.clone(), hash)
        .await
        .is_ok());
    // The block was already unpinned.
    assert!(rpc.chainhead_unstable_unpin(sub_id, hash).await.is_err());
}

/// Ignore block related events and obtain the next event related to an operation.
async fn next_operation_event<T: serde::de::DeserializeOwned>(
    sub: &mut RpcSubscription<FollowEvent<T>>,
) -> FollowEvent<T> {
    // At most 5 retries.
    for _ in 0..5 {
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

    panic!("Cannot find operation related event after 5 produced events");
}
