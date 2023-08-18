// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Just sanity checking some of the legacy RPC methods to make
//! sure they don't error out and can decode their results OK.

use crate::test_context;
use subxt::backend::legacy::rpc_methods;

#[tokio::test]
async fn chain_get_block_hash() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    rpc_methods::chain_get_block_hash(&rpc, None).await.unwrap();
}

#[tokio::test]
async fn chain_get_block() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let hash = rpc_methods::chain_get_block_hash(&rpc, None).await.unwrap();
    rpc_methods::chain_get_block(&rpc, hash).await.unwrap();
}

#[tokio::test]
async fn chain_get_finalized_head() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    rpc_methods::chain_get_finalized_head(&rpc).await.unwrap();
}

#[tokio::test]
async fn chain_subscribe_all_heads() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let mut sub = rpc_methods::chain_subscribe_all_heads(&rpc).await.unwrap();
    let _block_header = sub.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn chain_subscribe_finalized_heads() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let mut sub = rpc_methods::chain_subscribe_finalized_heads(&rpc)
        .await
        .unwrap();
    let _block_header = sub.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn chain_subscribe_new_heads() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let mut sub = rpc_methods::chain_subscribe_new_heads(&rpc).await.unwrap();
    let _block_header = sub.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn genesis_hash() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let _genesis_hash = rpc_methods::genesis_hash(&rpc).await.unwrap();
}

#[tokio::test]
async fn state_get_metadata() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let _metadata = rpc_methods::state_get_metadata(&rpc, None).await.unwrap();
}

#[tokio::test]
async fn state_call() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let _metadata = rpc_methods::state_call(&rpc, "Metadata_metadata", None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn system_health() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let _ = rpc_methods::system_health(&rpc).await.unwrap();
}

#[tokio::test]
async fn system_chain() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let _ = rpc_methods::system_chain(&rpc).await.unwrap();
}

#[tokio::test]
async fn system_name() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let _ = rpc_methods::system_name(&rpc).await.unwrap();
}

#[tokio::test]
async fn system_version() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let _ = rpc_methods::system_version(&rpc).await.unwrap();
}

#[tokio::test]
async fn system_properties() {
    let ctx = test_context().await;
    let rpc = ctx.rpc().await;

    let _ = rpc_methods::system_properties(&rpc).await.unwrap();
}
