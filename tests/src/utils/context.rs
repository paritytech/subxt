// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

pub use crate::{
    node_runtime,
    TestNodeProcess,
    TestRuntime,
};

use sp_keyring::AccountKeyring;
use subxt::Client;

/// substrate node should be installed on the $PATH
const SUBSTRATE_NODE_PATH: &str = "substrate";

pub async fn test_node_process_with(key: AccountKeyring) -> TestNodeProcess<TestRuntime> {
    let path = std::env::var("SUBSTRATE_NODE_PATH").unwrap_or_else(|_| {
        if which::which(SUBSTRATE_NODE_PATH).is_err() {
            panic!("A substrate binary should be installed on your path for integration tests. \
            See https://github.com/paritytech/substrate-subxt/tree/master#integration-testing")
        }
        SUBSTRATE_NODE_PATH.to_string()
    });

    let proc = TestNodeProcess::<TestRuntime>::build(path.as_str())
        .with_authority(key)
        .scan_for_open_ports()
        .spawn::<TestRuntime>()
        .await;
    proc.unwrap()
}

pub async fn test_node_process() -> TestNodeProcess<TestRuntime> {
    test_node_process_with(AccountKeyring::Alice).await
}

pub struct TestContext {
    pub node_proc: TestNodeProcess<TestRuntime>,
    pub api: node_runtime::RuntimeApi<TestRuntime>,
    pub client: Client<TestRuntime>,
}

pub async fn test_context() -> TestContext {
    let node_proc = test_node_process_with(AccountKeyring::Alice).await;
    let client = node_proc.client().clone();
    let api = node_runtime::RuntimeApi::<TestRuntime>::new(client.clone());
    TestContext {
        node_proc,
        api,
        client,
    }
}
