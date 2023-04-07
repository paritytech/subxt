// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    pair_signer, test_context, test_context_with,
    utils::{node_runtime, wait_for_blocks},
};
use assert_matches::assert_matches;
use codec::{Compact, Decode, Encode};
use frame_metadata::RuntimeMetadataPrefixed;
use sp_core::storage::well_known_keys;
use sp_core::{sr25519::Pair as Sr25519Pair, Pair};
use sp_keyring::AccountKeyring;
use subxt::{
    error::{DispatchError, Error, TokenError},
    rpc::types::{
        ChainHeadEvent, DryRunResult, DryRunResultBytes, FollowEvent, Initialized, RuntimeEvent,
        RuntimeVersionEvent,
    },
    tx::Signer,
    utils::AccountId32,
};

#[tokio::test]
async fn insert_key() {
    let ctx = test_context_with(AccountKeyring::Bob).await;
    let api = ctx.client();

    let public = AccountKeyring::Alice.public().as_array_ref().to_vec();
    api.rpc()
        .insert_key(
            "aura".to_string(),
            "//Alice".to_string(),
            public.clone().into(),
        )
        .await
        .unwrap();
    assert!(api
        .rpc()
        .has_key(public.clone().into(), "aura".to_string())
        .await
        .unwrap());
}

#[tokio::test]
async fn fetch_block_hash() {
    let ctx = test_context().await;
    ctx.client().rpc().block_hash(None).await.unwrap();
}

#[tokio::test]
async fn fetch_block() {
    let ctx = test_context().await;
    let api = ctx.client();

    let block_hash = api.rpc().block_hash(None).await.unwrap();
    api.rpc().block(block_hash).await.unwrap();
}

#[tokio::test]
async fn fetch_read_proof() {
    let ctx = test_context().await;
    let api = ctx.client();

    let block_hash = api.rpc().block_hash(None).await.unwrap();
    api.rpc()
        .read_proof(
            vec![
                well_known_keys::HEAP_PAGES,
                well_known_keys::EXTRINSIC_INDEX,
            ],
            block_hash,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn chain_subscribe_all_blocks() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().subscribe_all_block_headers().await.unwrap();
    blocks.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn chain_subscribe_best_blocks() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().subscribe_best_block_headers().await.unwrap();
    blocks.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn chain_subscribe_finalized_blocks() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().subscribe_finalized_block_headers().await.unwrap();
    blocks.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn fetch_keys() {
    let ctx = test_context().await;
    let api = ctx.client();

    let addr = node_runtime::storage().system().account_root();
    let keys = api
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch_keys(&addr.to_root_bytes(), 4, None)
        .await
        .unwrap();
    assert_eq!(keys.len(), 4)
}

#[tokio::test]
async fn test_iter() {
    let ctx = test_context().await;
    let api = ctx.client();

    let addr = node_runtime::storage().system().account_root();
    let mut iter = api
        .storage()
        .at_latest()
        .await
        .unwrap()
        .iter(addr, 10)
        .await
        .unwrap();
    let mut i = 0;
    while iter.next().await.unwrap().is_some() {
        i += 1;
    }
    assert_eq!(i, 13);
}

#[tokio::test]
async fn fetch_system_info() {
    let ctx = test_context().await;
    let api = ctx.client();

    assert_eq!(api.rpc().system_chain().await.unwrap(), "Development");
    assert_eq!(api.rpc().system_name().await.unwrap(), "Substrate Node");
    assert!(!api.rpc().system_version().await.unwrap().is_empty());
}

#[tokio::test]
async fn dry_run_passes() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = pair_signer(AccountKeyring::Bob.pair());

    wait_for_blocks(&api).await;

    let tx = node_runtime::tx()
        .balances()
        .transfer(bob.account_id().clone().into(), 10_000);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();

    signed_extrinsic
        .dry_run(None)
        .await
        .expect("dryrunning failed");

    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
}

#[tokio::test]
async fn dry_run_fails() {
    let ctx = test_context().await;
    let api = ctx.client();

    wait_for_blocks(&api).await;

    let alice = pair_signer(AccountKeyring::Alice.pair());
    let hans = pair_signer(Sr25519Pair::generate().0);

    let tx = node_runtime::tx().balances().transfer(
        hans.account_id().clone().into(),
        // 7 more than the default amount Alice has, so this should fail; insufficient funds:
        1_000_000_000_000_000_000_007,
    );

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();

    let dry_run_res = signed_extrinsic
        .dry_run(None)
        .await
        .expect("dryrunning failed");

    assert_eq!(
        dry_run_res,
        DryRunResult::DispatchError(DispatchError::Token(TokenError::FundsUnavailable))
    );

    let res = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await;

    assert!(
        matches!(
            res,
            Err(Error::Runtime(DispatchError::Token(
                TokenError::FundsUnavailable
            )))
        ),
        "Expected an insufficient balance, got {res:?}"
    );
}

#[tokio::test]
async fn dry_run_result_is_substrate_compatible() {
    use sp_runtime::{
        transaction_validity::{
            InvalidTransaction as SpInvalidTransaction,
            TransactionValidityError as SpTransactionValidityError,
        },
        ApplyExtrinsicResult as SpApplyExtrinsicResult, DispatchError as SpDispatchError,
        TokenError as SpTokenError,
    };

    // We really just connect to a node to get some valid metadata to help us
    // decode Dispatch Errors.
    let ctx = test_context().await;
    let api = ctx.client();

    let pairs = vec![
        // All ok
        (SpApplyExtrinsicResult::Ok(Ok(())), DryRunResult::Success),
        // Some transaction error
        (
            SpApplyExtrinsicResult::Err(SpTransactionValidityError::Invalid(
                SpInvalidTransaction::BadProof,
            )),
            DryRunResult::TransactionValidityError,
        ),
        // Some dispatch errors to check that they decode OK. We've tested module errors
        // "in situ" in other places so avoid the complexity of testing them properly here.
        (
            SpApplyExtrinsicResult::Ok(Err(SpDispatchError::Other("hi"))),
            DryRunResult::DispatchError(DispatchError::Other),
        ),
        (
            SpApplyExtrinsicResult::Ok(Err(SpDispatchError::CannotLookup)),
            DryRunResult::DispatchError(DispatchError::CannotLookup),
        ),
        (
            SpApplyExtrinsicResult::Ok(Err(SpDispatchError::BadOrigin)),
            DryRunResult::DispatchError(DispatchError::BadOrigin),
        ),
        (
            SpApplyExtrinsicResult::Ok(Err(SpDispatchError::Token(SpTokenError::CannotCreate))),
            DryRunResult::DispatchError(DispatchError::Token(TokenError::CannotCreate)),
        ),
    ];

    for (actual, expected) in pairs {
        let encoded = actual.encode();
        let res = DryRunResultBytes(encoded)
            .into_dry_run_result(&api.metadata())
            .unwrap();

        assert_eq!(res, expected);
    }
}

#[tokio::test]
async fn external_signing() {
    let ctx = test_context().await;
    let api = ctx.client();
    let alice = pair_signer(AccountKeyring::Alice.pair());

    // Create a partial extrinsic. We can get the signer payload at this point, to be
    // signed externally.
    let tx = node_runtime::tx().preimage().note_preimage(vec![0u8]);
    let partial_extrinsic = api
        .tx()
        .create_partial_signed(&tx, alice.account_id(), Default::default())
        .await
        .unwrap();

    // Get the signer payload.
    let signer_payload = partial_extrinsic.signer_payload();
    // Sign it (possibly externally).
    let signature = alice.sign(&signer_payload);
    // Use this to build a signed extrinsic.
    let extrinsic = partial_extrinsic.sign_with_address_and_signature(&alice.address(), &signature);

    // And now submit it.
    extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
}

#[tokio::test]
async fn submit_large_extrinsic() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = pair_signer(AccountKeyring::Alice.pair());

    // 2 MiB blob of data.
    let bytes = vec![0_u8; 2 * 1024 * 1024];
    // The preimage pallet allows storing and managing large byte-blobs.
    let tx = node_runtime::tx().preimage().note_preimage(bytes);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();

    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
}

#[tokio::test]
async fn unsigned_extrinsic_is_same_shape_as_polkadotjs() {
    let ctx = test_context().await;
    let api = ctx.client();

    let tx = node_runtime::tx().balances().transfer(
        pair_signer(AccountKeyring::Alice.pair())
            .account_id()
            .clone()
            .into(),
        12345000000000000,
    );

    let actual_tx = api.tx().create_unsigned(&tx).unwrap();

    let actual_tx_bytes = actual_tx.encoded();

    // How these were obtained:
    // - start local substrate node.
    // - open polkadot.js UI in browser and point at local node.
    // - open dev console (may need to refresh page now) and find the WS connection.
    // - create a balances.transfer to ALICE with 12345 and "submit unsigned".
    // - find the submitAndWatchExtrinsic call in the WS connection to get these bytes:
    let expected_tx_bytes = hex::decode(
        "b004060700d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0f0090c04bb6db2b"
    )
    .unwrap();

    // Make sure our encoding is the same as the encoding polkadot UI created.
    assert_eq!(actual_tx_bytes, expected_tx_bytes);
}

#[tokio::test]
async fn rpc_state_call() {
    let ctx = test_context().await;
    let api = ctx.client();

    // Call into the runtime of the chain to get the Metadata.
    let metadata_bytes = api
        .rpc()
        .state_call("Metadata_metadata", None, None)
        .await
        .unwrap();

    let cursor = &mut &*metadata_bytes;
    let _ = <Compact<u32>>::decode(cursor).unwrap();
    let meta: RuntimeMetadataPrefixed = Decode::decode(cursor).unwrap();
    let metadata_call = match meta.1 {
        frame_metadata::RuntimeMetadata::V14(metadata) => metadata,
        _ => panic!("Metadata V14 unavailable"),
    };

    // Compare the runtime API call against the `state_getMetadata`.
    let metadata = api.rpc().metadata(None).await.unwrap();
    let metadata = metadata.runtime_metadata();
    assert_eq!(&metadata_call, metadata);
}

#[tokio::test]
async fn chainhead_unstable_follow() {
    let ctx = test_context().await;
    let api = ctx.client();

    // Check subscription with runtime updates set on false.
    let mut blocks = api.rpc().chainhead_unstable_follow(false).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    // The initialized event should contain the finalized block hash.
    let finalized_block_hash = api.rpc().finalized_head().await.unwrap();
    assert_eq!(
        event,
        FollowEvent::Initialized(Initialized {
            finalized_block_hash,
            finalized_block_runtime: None,
        })
    );

    // Expect subscription to produce runtime versions.
    let mut blocks = api.rpc().chainhead_unstable_follow(true).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    // The initialized event should contain the finalized block hash.
    let finalized_block_hash = api.rpc().finalized_head().await.unwrap();
    let runtime_version = ctx.client().runtime_version();

    assert_matches!(
        event,
        FollowEvent::Initialized(init) => {
            assert_eq!(init.finalized_block_hash, finalized_block_hash);
            assert_eq!(init.finalized_block_runtime, Some(RuntimeEvent::Valid(RuntimeVersionEvent {
                spec: runtime_version,
            })));
        }
    );
}

#[tokio::test]
async fn chainhead_unstable_body() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().chainhead_unstable_follow(false).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hash,
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap().clone();

    // Subscribe to fetch the block's body.
    let mut sub = api
        .rpc()
        .chainhead_unstable_body(sub_id, hash)
        .await
        .unwrap();
    let event = sub.next().await.unwrap().unwrap();

    // Expected block's extrinsics scale encoded and hex encoded.
    let body = api.rpc().block(Some(hash)).await.unwrap().unwrap();
    let extrinsics: Vec<Vec<u8>> = body.block.extrinsics.into_iter().map(|ext| ext.0).collect();
    let expected = format!("0x{}", hex::encode(extrinsics.encode()));

    assert_matches!(event,
        ChainHeadEvent::Done(done) if done.result == expected
    );
}

#[tokio::test]
async fn chainhead_unstable_header() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().chainhead_unstable_follow(false).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hash,
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap().clone();

    let header = api.rpc().header(Some(hash)).await.unwrap().unwrap();
    let expected = format!("0x{}", hex::encode(header.encode()));

    let header = api
        .rpc()
        .chainhead_unstable_header(sub_id, hash)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(header, expected);
}

#[tokio::test]
async fn chainhead_unstable_storage() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().chainhead_unstable_follow(false).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hash,
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap().clone();

    let alice: AccountId32 = AccountKeyring::Alice.to_account_id().into();
    let addr = node_runtime::storage().system().account(alice);
    let addr_bytes = api.storage().address_bytes(&addr).unwrap();

    let mut sub = api
        .rpc()
        .chainhead_unstable_storage(sub_id, hash, &addr_bytes, None)
        .await
        .unwrap();
    let event = sub.next().await.unwrap().unwrap();

    assert_matches!(event, ChainHeadEvent::<Option<String>>::Done(done) if done.result.is_some());
}

#[tokio::test]
async fn chainhead_unstable_call() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().chainhead_unstable_follow(true).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hash,
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap().clone();

    let alice_id = AccountKeyring::Alice.to_account_id();
    let mut sub = api
        .rpc()
        .chainhead_unstable_call(
            sub_id,
            hash,
            "AccountNonceApi_account_nonce".into(),
            &alice_id.encode(),
        )
        .await
        .unwrap();
    let event = sub.next().await.unwrap().unwrap();

    assert_matches!(event, ChainHeadEvent::<String>::Done(_));
}

#[tokio::test]
async fn chainhead_unstable_unpin() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().chainhead_unstable_follow(true).await.unwrap();
    let event = blocks.next().await.unwrap().unwrap();
    let hash = match event {
        FollowEvent::Initialized(init) => init.finalized_block_hash,
        _ => panic!("Unexpected event"),
    };
    let sub_id = blocks.subscription_id().unwrap().clone();

    assert!(api
        .rpc()
        .chainhead_unstable_unpin(sub_id.clone(), hash)
        .await
        .is_ok());
    // The block was already unpinned.
    assert!(api
        .rpc()
        .chainhead_unstable_unpin(sub_id, hash)
        .await
        .is_err());
}
