// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
    test_node_process,
    test_node_process_with,
    utils::{
        node_runtime::system,
        pair_signer,
        test_context,
    },
};

use sp_core::{
    sr25519::Pair,
    storage::{
        well_known_keys,
        StorageKey,
    },
    Pair as _,
};
use sp_keyring::AccountKeyring;
use sp_runtime::DispatchOutcome;
use subxt::Error;

#[tokio::test]
async fn insert_key() {
    let test_node_process = test_node_process_with(AccountKeyring::Bob).await;
    let client = test_node_process.client();
    let public = AccountKeyring::Alice.public().as_array_ref().to_vec();
    client
        .rpc()
        .insert_key(
            "aura".to_string(),
            "//Alice".to_string(),
            public.clone().into(),
        )
        .await
        .unwrap();
    assert!(client
        .rpc()
        .has_key(public.clone().into(), "aura".to_string())
        .await
        .unwrap());
}

#[tokio::test]
async fn fetch_block_hash() {
    let node_process = test_node_process().await;
    node_process.client().rpc().block_hash(None).await.unwrap();
}

#[tokio::test]
async fn fetch_block() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let block_hash = client.rpc().block_hash(None).await.unwrap();
    client.rpc().block(block_hash).await.unwrap();
}

#[tokio::test]
async fn fetch_read_proof() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let block_hash = client.rpc().block_hash(None).await.unwrap();
    client
        .rpc()
        .read_proof(
            vec![
                StorageKey(well_known_keys::HEAP_PAGES.to_vec()),
                StorageKey(well_known_keys::EXTRINSIC_INDEX.to_vec()),
            ],
            block_hash,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn chain_subscribe_blocks() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let mut blocks = client.rpc().subscribe_blocks().await.unwrap();
    blocks.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn chain_subscribe_finalized_blocks() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let mut blocks = client.rpc().subscribe_finalized_blocks().await.unwrap();
    blocks.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn fetch_keys() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let keys = client
        .storage()
        .fetch_keys::<system::storage::Account>(4, None, None)
        .await
        .unwrap();
    assert_eq!(keys.len(), 4)
}

#[tokio::test]
async fn test_iter() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let mut iter = client
        .storage()
        .iter::<system::storage::Account>(None)
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
    let node_process = test_node_process().await;
    let client = node_process.client();
    assert_eq!(client.rpc().system_chain().await.unwrap(), "Development");
    assert_eq!(client.rpc().system_name().await.unwrap(), "Substrate Node");
    assert!(!client.rpc().system_version().await.unwrap().is_empty());
}

#[tokio::test]
async fn dry_run_passes() {
    let node_process = test_node_process().await;
    let client = node_process.client();

    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = pair_signer(AccountKeyring::Bob.pair());
    let bob_address = bob.account_id().clone().into();
    let cxt = test_context().await;
    let api = &cxt.api;
    let signed_extrinsic = api
        .tx()
        .balances()
        .transfer(bob_address, 10_000)
        .unwrap()
        .create_signed(&alice, Default::default())
        .await
        .unwrap();

    client
        .rpc()
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
    let node_process = test_node_process().await;
    let client = node_process.client();

    let alice = pair_signer(AccountKeyring::Alice.pair());
    let hans = pair_signer(Pair::generate().0);
    let hans_address = hans.account_id().clone().into();
    let cxt = test_context().await;
    let api = &cxt.api;
    let signed_extrinsic = api
        .tx()
        .balances()
        .transfer(
            hans_address,
            100_000_000_000_000_000_000_000_000_000_000_000,
        )
        .unwrap()
        .create_signed(&alice, Default::default())
        .await
        .unwrap();

    let dry_run_res: DispatchOutcome = client
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
    if let Err(Error::Module(err)) = res {
        assert_eq!(err.pallet, "Balances");
        assert_eq!(err.error, "InsufficientBalance");
    } else {
        panic!("expected a runtime module error");
    }
}