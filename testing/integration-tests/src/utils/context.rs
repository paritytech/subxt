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
    Client,
    DefaultConfig,
    PairSigner,
    SubstrateExtrinsicParams,
};

/// substrate node should be installed on the $PATH
const SUBSTRATE_NODE_PATH: &str = "substrate";

pub type NodeRuntimeParams = SubstrateExtrinsicParams<DefaultConfig>;

pub async fn test_node_process_with(
    key: AccountKeyring,
) -> TestNodeProcess<DefaultConfig> {
    let path = std::env::var("SUBSTRATE_NODE_PATH").unwrap_or_else(|_| {
        if which::which(SUBSTRATE_NODE_PATH).is_err() {
            panic!("A substrate binary should be installed on your path for integration tests. \
            See https://github.com/paritytech/subxt/tree/master#integration-testing")
        }
        SUBSTRATE_NODE_PATH.to_string()
    });

    let proc = TestNodeProcess::<DefaultConfig>::build(path.as_str())
        .with_authority(key)
        .spawn::<DefaultConfig>()
        .await;
    proc.unwrap()
}

pub async fn test_node_process() -> TestNodeProcess<DefaultConfig> {
    test_node_process_with(AccountKeyring::Alice).await
}

pub struct TestContext {
    pub node_proc: TestNodeProcess<DefaultConfig>,
    pub api: node_runtime::RuntimeApi<DefaultConfig, NodeRuntimeParams>,
}

impl TestContext {
    pub fn client(&self) -> &Client<DefaultConfig> {
        &self.api.client
    }
}

pub async fn test_context() -> TestContext {
    tracing_subscriber::fmt::try_init().ok();
    let node_proc = test_node_process_with(AccountKeyring::Alice).await;
    let api = node_proc.client().clone().to_runtime_api();
    TestContext { node_proc, api }
}

pub fn pair_signer(pair: Pair) -> PairSigner<DefaultConfig, Pair> {
    PairSigner::new(pair)
}
