// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub(crate) use crate::{
    node_runtime,
    TestNodeProcess,
};

use sp_core::sr25519::Pair;
use sp_keyring::AccountKeyring;
use subxt::{
    tx::PairSigner,
    SubstrateConfig,
};

/// substrate node should be installed on the $PATH
const SUBSTRATE_NODE_PATH: &str = "substrate";

pub async fn test_context_with(key: AccountKeyring) -> TestContext {
    let path = std::env::var("SUBSTRATE_NODE_PATH").unwrap_or_else(|_| {
        if which::which(SUBSTRATE_NODE_PATH).is_err() {
            panic!("A substrate binary should be installed on your path for integration tests. \
            See https://github.com/paritytech/subxt/tree/master#integration-testing")
        }
        SUBSTRATE_NODE_PATH.to_string()
    });

    let proc = TestContext::build(path.as_str())
        .with_authority(key)
        .spawn::<SubstrateConfig>()
        .await;
    proc.unwrap()
}

pub type TestContext = TestNodeProcess<SubstrateConfig>;

pub async fn test_context() -> TestContext {
    test_context_with(AccountKeyring::Alice).await
}

pub fn pair_signer(pair: Pair) -> PairSigner<SubstrateConfig, Pair> {
    PairSigner::new(pair)
}
