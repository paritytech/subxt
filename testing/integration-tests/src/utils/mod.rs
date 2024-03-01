// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

mod context;
mod node_proc;
mod tx_retries;
mod wait_for_blocks;

pub use context::*;
pub use node_proc::TestNodeProcess;
pub use tx_retries::*;
pub use wait_for_blocks::*;

pub use integration_tests_proc_macro::subxt_test;

/// The test timeout is set to 1 second.
/// However, the test is sleeping for 5 seconds.
/// This must cause the test to panic.
#[subxt_test(timeout = 1)]
#[should_panic]
async fn test_subxt_macro() {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}
