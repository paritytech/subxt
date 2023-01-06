// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    pair_signer,
    test_context,
    test_context_with,
    utils::{
        node_runtime,
        wait_for_blocks,
    },
};
use sp_core::{
    sr25519::Pair as Sr25519Pair,
    storage::well_known_keys,
    Pair,
};
use sp_keyring::AccountKeyring;
use subxt::{
    error::DispatchError,
    rpc::DryRunError,
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
        .fetch_keys(&addr.to_root_bytes(), 4, None, None)
        .await
        .unwrap();
    assert_eq!(keys.len(), 4)
}

#[tokio::test]
async fn test_iter() {
    let ctx = test_context().await;
    let api = ctx.client();

    let addr = node_runtime::storage().system().account_root();
    let mut iter = api.storage().iter(addr, 10, None).await.unwrap();
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
        .expect("dryrunning failed")
        .expect("dry run should be successful");

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
        100_000_000_000_000_000_000_000_000_000_000_000,
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

    assert_eq!(dry_run_res, Err(DryRunError::DispatchError));

    let res = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await;

    if let Err(subxt::error::Error::Runtime(DispatchError::Module(err))) = res {
        assert_eq!(err.pallet, "Balances");
        assert_eq!(err.error, "InsufficientBalance");
    } else {
        panic!("expected a runtime module error");
    }
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
        12345,
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
        "9804060000d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27de5c0",
    )
    .unwrap();

    // Make sure our encoding is the same as the encoding polkadot UI created.
    assert_eq!(actual_tx_bytes, expected_tx_bytes);
}
