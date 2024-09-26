// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub(crate) use crate::{node_runtime, utils::TestNodeProcess};

use subxt::client::OnlineClient;
use subxt::SubstrateConfig;

use super::node_proc::RpcClientKind;

/// `substrate-node` should be installed on the $PATH. We fall back
/// to also checking for an older `substrate` binary.
const SUBSTRATE_NODE_PATHS: &str = "substrate-node,substrate";

pub async fn test_context_with(authority: String, rpc_client_kind: RpcClientKind) -> TestContext {
    let paths =
        std::env::var("SUBSTRATE_NODE_PATH").unwrap_or_else(|_| SUBSTRATE_NODE_PATHS.to_string());
    let paths: Vec<_> = paths.split(',').map(|p| p.trim()).collect();

    let mut proc = TestContext::build(&paths);
    proc.with_authority(authority);
    proc.with_rpc_client_kind(rpc_client_kind);
    proc.spawn::<SubstrateConfig>().await.unwrap()
}

pub type TestConfig = SubstrateConfig;

pub type TestContext = TestNodeProcess<SubstrateConfig>;

pub type TestClient = OnlineClient<SubstrateConfig>;

pub async fn test_context() -> TestContext {
    test_context_with("alice".to_string(), RpcClientKind::Legacy).await
}

pub async fn test_context_reconnecting_rpc_client() -> TestContext {
    test_context_with("alice".to_string(), RpcClientKind::UnstableReconnecting).await
}
