// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use subxt::{client::OnlineClientT, Config};

/// Wait for blocks to be produced before running tests. Waiting for two blocks
/// (the genesis block and another one) seems to be enough to allow tests
/// like `dry_run_passes` to work properly.
///
/// If the "unstable-light-client" feature flag is enabled, this will wait for
/// 5 blocks instead of two. The light client needs the extra blocks to avoid
/// errors caused by loading information that is not available in the first 2 blocks
/// (`Failed to load the block weight for block`).
pub async fn wait_for_blocks<C: Config>(api: &impl OnlineClientT<C>) {
    let mut sub = api.rpc().subscribe_all_block_headers().await.unwrap();
    sub.next().await;
    sub.next().await;

    #[cfg(feature = "unstable-light-client")]
    {
        sub.next().await;
        sub.next().await;
        sub.next().await;
    }
}
