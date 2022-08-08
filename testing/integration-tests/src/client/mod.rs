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
use subxt::error::DispatchError;

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
async fn chain_subscribe_blocks() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().subscribe_blocks().await.unwrap();
    blocks.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn chain_subscribe_finalized_blocks() {
    let ctx = test_context().await;
    let api = ctx.client();

    let mut blocks = api.rpc().subscribe_finalized_blocks().await.unwrap();
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

    api.rpc()
        .dry_run(signed_extrinsic.encoded(), None)
        .await
        .expect("dryrunning failed")
        .expect("expected dryrunning to be successful")
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

    let dry_run_res = api
        .rpc()
        .dry_run(signed_extrinsic.encoded(), None)
        .await
        .expect("dryrunning failed")
        .expect("expected dryrun transaction to be valid");

    if let Err(sp_runtime::DispatchError::Module(module_error)) = dry_run_res {
        assert_eq!(module_error.index, 6);
        assert_eq!(module_error.error, 2);
    } else {
        panic!("expected a module error when dryrunning");
    }

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
