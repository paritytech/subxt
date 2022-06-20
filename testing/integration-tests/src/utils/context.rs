// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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
