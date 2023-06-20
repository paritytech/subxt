// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub(crate) use crate::{node_runtime, TestNodeProcess};

use subxt::SubstrateConfig;

/// substrate node should be installed on the $PATH
const SUBSTRATE_NODE_PATH: &str = "substrate";

pub async fn test_context_with(authority: String) -> TestContext {
    let path = std::env::var("SUBSTRATE_NODE_PATH").unwrap_or_else(|_| {
        if which::which(SUBSTRATE_NODE_PATH).is_err() {
            panic!(
                "A substrate binary should be installed on your path for integration tests. \
            See https://github.com/paritytech/subxt/tree/master#integration-testing"
            )
        }
        SUBSTRATE_NODE_PATH.to_string()
    });

    let mut proc = TestContext::build(path.as_str());
    proc.with_authority(authority);
    proc.spawn::<SubstrateConfig>().await.unwrap()
}

pub type TestContext = TestNodeProcess<SubstrateConfig>;

pub async fn test_context() -> TestContext {
    test_context_with("alice".to_string()).await
}
