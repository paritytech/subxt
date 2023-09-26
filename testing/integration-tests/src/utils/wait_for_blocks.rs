// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use subxt::{client::OnlineClientT, Config};

/// Wait for blocks to be produced before running tests. Specifically, we
/// wait for one more finalized block to be produced, which is important because
/// the first finalized block doesn't have much state etc associated with it.
pub async fn wait_for_blocks<C: Config>(api: &impl OnlineClientT<C>) {
    let mut sub = api.blocks().subscribe_finalized().await.unwrap();
    // The current finalized block:
    sub.next().await;
    // The next one:
    sub.next().await;
}
