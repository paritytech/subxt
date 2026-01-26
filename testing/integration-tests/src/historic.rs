// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::subxt_test;
use subxt::{OnlineClient, config::PolkadotConfig};

#[subxt_test]
async fn can_instantiate_client_across_historic_polkadot_runtimes() {
    let api = connect_to_rpc_node(&[
        "wss://rpc.polkadot.io",
        "wss://polkadot-public-rpc.blockops.network/ws",
        "wss://1rpc.io/dot",
    ])
    .await;

    let futs = POLKADOT_SPEC_VERSION_BLOCKS.into_iter().map(|block_num| {
        let api = api.clone();
        async move {
            tracing::info!("Instantiating client at block {block_num}");
            api.at_block(block_num)
                .await
                .unwrap_or_else(|e| panic!("Can't instantiate client at block {block_num}: {e}"));
            tracing::info!("   -> Success Instantiating client at block {block_num}");
        }
    });

    run_with_concurrency(5, futs).await;
}

#[subxt_test]
async fn can_instantiate_client_across_historic_kusama_runtimes() {
    let api = connect_to_rpc_node(&[
        "wss://kusama-public-rpc.blockops.network/ws",
        "wss://rpc.ibp.network/kusama",
    ])
    .await;

    let futs = KUSAMA_SPEC_VERSION_BLOCKS.into_iter().map(|block_num| {
        let api = api.clone();
        async move {
            tracing::info!("Instantiating client at block {block_num}");
            api.at_block(block_num)
                .await
                .unwrap_or_else(|e| panic!("Can't instantiate client at block {block_num}: {e}"));
            tracing::info!("   -> Success Instantiating client at block {block_num}");
        }
    });

    run_with_concurrency(5, futs).await;
}

/// Runs at most `num_tasks` at once, running the next tasks only when currently running ones finish and free up space.
async fn run_with_concurrency<F: Future<Output = ()>>(
    num_tasks: usize,
    all_tasks: impl IntoIterator<Item = F>,
) {
    let semaphore = tokio::sync::Semaphore::new(num_tasks);
    let futs = futures::stream::FuturesUnordered::new();

    for task in all_tasks {
        futs.push(async {
            let _permit = semaphore.acquire().await.unwrap();
            task.await;
        });
    }

    use futures::StreamExt;
    futs.collect::<()>().await;
}

async fn connect_to_rpc_node(urls: &[&'static str]) -> OnlineClient<PolkadotConfig> {
    for url in urls {
        let api = OnlineClient::<PolkadotConfig>::from_url(url).await;
        match api {
            Ok(api) => return api,
            Err(e) => tracing::warn!("Error connecting to RPC node {url}: {e}"),
        }
    }
    panic!("Could not connect to any RPC node")
}

/// A list of entries denoting the historic Polkadot RC blocks with spec and transaction versions changes.
/// This will change over time, but we only really care about historic blocks.
const POLKADOT_SPEC_VERSION_BLOCKS: [u64; 70] = [
    0, 29231, 188836, 199405, 214264, 244358, 303079, 314201, 342400, 443963, 528470, 687751,
    746085, 787923, 799302, 1205128, 1603423, 1733218, 2005673, 2436698, 3613564, 3899547, 4345767,
    4876134, 5661442, 6321619, 6713249, 7217907, 7229126, 7560558, 8115869, 8638103, 9280179,
    9738717, 10156856, 10458576, 10655116, 10879371, 11328884, 11532856, 11933818, 12217535,
    12245277, 12532644, 12876189, 13800015, 14188833, 14543918, 15978362, 16450000, 17840000,
    18407475, 19551000, 20181758, 20438530, 21169168, 21455374, 21558004, 21800141, 22572435,
    22975676, 23463101, 24899777, 25005483, 26170985, 26902698, 27707460, 27994522, 28476903,
    28524511,
];

/// A list of entries denoting the historic Kusama RC blocks with spec and transaction versions changes.
/// This will change over time, but we only really care about historic blocks.
const KUSAMA_SPEC_VERSION_BLOCKS: [u64; 68] = [
    26668, 38244, 54248, 59658, 67650, 82191, 83237, 101503, 203466, 295787, 461692, 504329,
    569326, 587686, 653183, 693487, 901442, 1375086, 1445458, 1472960, 1475648, 1491596, 1574408,
    2064961, 2201991, 2671528, 2704202, 2728002, 2832534, 2962294, 3240000, 3274408, 3323565,
    3534175, 3860281, 4143129, 4401242, 4841367, 5961600, 6137912, 6561855, 7100891, 7468792,
    7668600, 7812476, 8010981, 8073833, 8555825, 8945245, 9611377, 9625129, 9866422, 10403784,
    10960765, 11006614, 11404482, 11601803, 12008022, 12405451, 12665416, 12909508, 13109752,
    13555777, 13727747, 14248044, 14433840, 14645900, 15048375,
];
