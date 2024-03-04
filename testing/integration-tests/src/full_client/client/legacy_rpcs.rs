// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Just sanity checking some of the legacy RPC methods to make
//! sure they don't error out and can decode their results OK.

use crate::{subxt_test, test_context};

#[subxt_test(timeout = 800)]
async fn chain_get_block_hash() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    rpc.chain_get_block_hash(None).await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn chain_get_block() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let hash = rpc.chain_get_block_hash(None).await.unwrap();
    rpc.chain_get_block(hash).await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn chain_get_finalized_head() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    rpc.chain_get_finalized_head().await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn chain_subscribe_all_heads() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let mut sub = rpc.chain_subscribe_all_heads().await.unwrap();
    let _block_header = sub.next().await.unwrap().unwrap();
}

#[subxt_test(timeout = 800)]
async fn chain_subscribe_finalized_heads() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let mut sub = rpc.chain_subscribe_finalized_heads().await.unwrap();
    let _block_header = sub.next().await.unwrap().unwrap();
}

#[subxt_test(timeout = 800)]
async fn chain_subscribe_new_heads() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let mut sub = rpc.chain_subscribe_new_heads().await.unwrap();
    let _block_header = sub.next().await.unwrap().unwrap();
}

#[subxt_test(timeout = 800)]
async fn genesis_hash() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let _genesis_hash = rpc.genesis_hash().await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn state_get_metadata() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let _metadata = rpc.state_get_metadata(None).await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn state_call() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let _metadata = rpc
        .state_call("Metadata_metadata", None, None)
        .await
        .unwrap();
}

#[subxt_test(timeout = 800)]
async fn system_health() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let _ = rpc.system_health().await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn system_chain() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let _ = rpc.system_chain().await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn system_name() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let _ = rpc.system_name().await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn system_version() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let _ = rpc.system_version().await.unwrap();
}

#[subxt_test(timeout = 800)]
async fn system_properties() {
    let ctx = test_context().await;
    let rpc = ctx.legacy_rpc_methods().await;

    let _ = rpc.system_properties().await.unwrap();
}
