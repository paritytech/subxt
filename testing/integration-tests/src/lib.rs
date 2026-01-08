// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#[cfg(all(legacy_backend, chainhead_backend))]
compile_error!("The features 'legacy-backend' and 'chainhead-backend' cannot be used together");
#[cfg(all(lightclient_rpc, reconnecting_rpc))]
compile_error!("The features 'light-client-rpc' and 'reconnecting-rpc' cannot be used together");

#[cfg(test)]
pub mod utils;

// Only run these against default RPC and backend.
#[cfg(all(test, default_rpc, default_backend))]
#[cfg_attr(test, allow(unused_imports))]
use utils::*;

// Run these against everything except lightclient RPC (it's too slow)
#[cfg(all(test, not(lightclient_rpc)))]
mod full_client;

// Run these against all backends but only with lightclient RPC feature
// (we explicitly create a light client to test here, so as long as the
// tests run once it doesn't matter under which feature, but this allows us
// to select light client RPC tests using the feature).
#[cfg(all(test, lightclient_rpc))]
mod light_client;

// We don't use this dependency, but it's here so that we
// can enable logging easily if need be. Add this to a test
// to enable tracing for it:
//
// tracing_subscriber::fmt::init();
#[cfg(test)]
use tracing_subscriber as _;

/// The test timeout is set to 1 second.
/// However, the test is sleeping for 5 seconds.
/// This must cause the test to panic.
#[cfg(test)]
#[utils::subxt_test(timeout = 1)]
#[should_panic]
async fn test_subxt_macro() {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}
