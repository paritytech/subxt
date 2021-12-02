// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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
    utils::node_runtime::system,
};

use sp_core::storage::{
    well_known_keys,
    StorageKey,
};
use sp_keyring::AccountKeyring;

#[async_std::test]
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

#[async_std::test]
async fn fetch_block_hash() {
    let node_process = test_node_process().await;
    node_process.client().rpc().block_hash(None).await.unwrap();
}

#[async_std::test]
async fn fetch_block() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let block_hash = client.rpc().block_hash(None).await.unwrap();
    client.rpc().block(block_hash).await.unwrap();
}

#[async_std::test]
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

#[async_std::test]
async fn chain_subscribe_blocks() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let mut blocks = client.rpc().subscribe_blocks().await.unwrap();
    blocks.next().await.unwrap();
}

#[async_std::test]
async fn chain_subscribe_finalized_blocks() {
    let node_process = test_node_process().await;
    let client = node_process.client();
    let mut blocks = client.rpc().subscribe_finalized_blocks().await.unwrap();
    blocks.next().await.unwrap();
}

#[async_std::test]
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

#[async_std::test]
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
