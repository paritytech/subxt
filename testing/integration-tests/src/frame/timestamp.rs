// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{node_runtime, test_context};

#[tokio::test]
async fn storage_get_current_timestamp() {
    let ctx = test_context().await;
    let api = ctx.client();

    let timestamp = api
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&node_runtime::storage().timestamp().now())
        .await;

    assert!(timestamp.is_ok())
}
