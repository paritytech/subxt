// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{node_runtime, subxt_test, test_context};

#[subxt_test]
async fn storage_get_current_timestamp() {
    let ctx = test_context().await;
    let api = ctx.client();
    let at_block = api.at_current_block().await.unwrap();

    let timestamp_value = at_block
        .storage()
        .entry(node_runtime::storage().timestamp().now())
        .unwrap()
        .fetch(())
        .await
        .unwrap();

    assert!(timestamp_value.decode().is_ok())
}
