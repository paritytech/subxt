// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::subxt_test;
use subxt::{OnlineClient, config::PolkadotConfig};

#[subxt_test]
async fn can_instantiate_client_across_historic_polkadot_runtimes() {
    let api = connect_to_rpc_node(&[
        "wss://rpc.polkadot.io",
        "wss://1rpc.io/dot",
        "wss://polkadot-public-rpc.blockops.network/ws",
    ])
    .await;

    for block in POLKADOT_SPEC_VERSION_BLOCKS.iter() {
        api.at_block(*block)
            .await
            .expect(&format!("Can instantiate client at block {block}"));
    }
}

#[subxt_test]
async fn can_instantiate_client_across_historic_kusama_runtimes() {
    let api = connect_to_rpc_node(&[
        "wss://kusama-public-rpc.blockops.network/ws",
        "wss://rpc.ibp.network/kusama",
    ])
    .await;

    for block in KUSAMA_SPEC_VERSION_BLOCKS.iter() {
        api.at_block(*block)
            .await
            .expect(&format!("Can instantiate client at block {block}"));
    }
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
/// This will change over time, but we only really care about historic blocks, so it's not
/// super important to update this often.
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
/// This will change over time, but we only really care about historic blocks, so it's not
/// super important to update this often.
const KUSAMA_SPEC_VERSION_BLOCKS: [u64; 30] = [
    0, 26668, 38244, 54248, 59658, 67650, 82191, 83237, 101503, 203466, 295787, 461692, 504329,
    569326, 587686, 653183, 693487, 901442, 1375086, 1445458, 1472960, 1475648, 1491596, 1574408,
    2064961, 2201991, 2671528, 2704202, 2728002, 2832534,
];
