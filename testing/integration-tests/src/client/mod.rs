// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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
