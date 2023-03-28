// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub(crate) use crate::{node_runtime, TestNodeProcess};
use futures::lock::Mutex;
use lazy_static::lazy_static;
use sp_core::sr25519::Pair;
use sp_keyring::AccountKeyring;
use subxt::{tx::PairSigner, OnlineClient, SubstrateConfig};

/// substrate node should be installed on the $PATH
const SUBSTRATE_NODE_PATH: &str = "substrate";

/// Test context that spawns a dedicated substrate binary.
pub type TestContext = TestNodeProcess;

pub async fn test_context_with(key: AccountKeyring) -> TestContext {
    let path = std::env::var("SUBSTRATE_NODE_PATH").unwrap_or_else(|_| {
        if which::which(SUBSTRATE_NODE_PATH).is_err() {
            panic!(
                "A substrate binary should be installed on your path for integration tests. \
            See https://github.com/paritytech/subxt/tree/master#integration-testing"
            )
        }
        SUBSTRATE_NODE_PATH.to_string()
    });

    let proc = TestContext::build(path.as_str())
        .with_authority(key)
        .spawn()
        .await;
    proc.unwrap()
}

/// Create a test context that spawns a dedicated substrate binary.
pub async fn test_context() -> TestContext {
    test_context_with(AccountKeyring::Alice).await
}

/// Test context that shares a single substrate binary.
pub struct TestContextShared {
    client: OnlineClient<SubstrateConfig>,
}

impl TestContextShared {
    /// Returns the subxt client connected to the running node.
    pub fn client(&self) -> OnlineClient<SubstrateConfig> {
        self.client.clone()
    }
}

/// Create a test context that shares the substrate binary.
pub async fn test_context_shared() -> TestContextShared {
    lazy_static! {
        static ref CACHE: Mutex<Option<TestNodeProcess>> = Mutex::new(None);
    }

    let mut cache = CACHE.lock().await;
    match &mut *cache {
        Some(test_context) => TestContextShared {
            client: test_context.client(),
        },
        None => {
            let test_context = test_context_with(AccountKeyring::Alice).await;
            let shared = TestContextShared {
                client: test_context.client(),
            };

            *cache = Some(test_context);
            shared
        }
    }
}

pub fn pair_signer(pair: Pair) -> PairSigner<SubstrateConfig, Pair> {
    PairSigner::new(pair)
}
