// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use subxt::{
    Config,
    client::{OnlineClient, Blocks},
};

/// Wait for blocks to be produced before running tests. Specifically, we
/// wait for one more finalized block to be produced, which is important because
/// the first finalized block doesn't have much state etc associated with it.
pub async fn wait_for_blocks<C: Config>(api: &OnlineClient<C>) {
    // The current finalized block and the next block.
    wait_for_number_of_blocks(api, 2).await;
}

/// Wait for a number of blocks to be produced.
pub async fn wait_for_number_of_blocks<C: Config>(
    api: &OnlineClient<C>,
    number_of_blocks: usize,
) {
    let mut sub = api.stream_blocks().await.unwrap();

    for _ in 0..number_of_blocks {
        sub.next().await;
    }
}

/// Consumes the initial blocks from the stream of blocks to ensure that the stream is up-to-date.
///
/// This may be useful on the unstable backend when the initial blocks may be large
/// and one relies on something to included in finalized block in ner future.
pub async fn consume_initial_blocks<C: Config>(
    blocks: &mut Blocks<C>,
) {
    use tokio::time::{Duration, Instant, interval_at};
    const MAX_DURATION: Duration = Duration::from_millis(200);

    let mut now = interval_at(Instant::now() + MAX_DURATION, MAX_DURATION);

    loop {
        tokio::select! {
            _ = now.tick() => {
                break;
            }
            _  = blocks.next() => {}
        }
    }
}
