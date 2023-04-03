// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use subxt::{client::OnlineClientT, Config};

/// Wait for blocks to be produced before running tests. Waiting for two blocks
/// (the genesis block and another one) seems to be enough to allow tests
/// like `dry_run_passes` to work properly.
pub async fn wait_for_blocks<C: Config>(api: &impl OnlineClientT<C>) {
    let mut sub = api.rpc().subscribe_all_block_headers().await.unwrap();
    sub.next().await;
    sub.next().await;
}
