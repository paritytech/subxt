// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#[cfg(all(legacy_backend, chainhead_backend))]
compile_error!("The features 'legacy-backend' and 'chainhead-backend' cannot be used together");
#[cfg(all(lightclient_rpc, reconnecting_rpc))]
compile_error!("The features 'light-client-rpc' and 'reconnecting-rpc' cannot be used together");

#[cfg(test)]
pub mod utils;

#[cfg(test)]
#[cfg_attr(test, allow(unused_imports))]
use utils::*;

#[cfg(all(test, not(lightclient_rpc)))]
mod full_client;

#[cfg(all(test, lightclient_rpc))]
mod light_client;

#[cfg(test)]
use test_runtime::node_runtime;

// We don't use this dependency, but it's here so that we
// can enable logging easily if need be. Add this to a test
// to enable tracing for it:
//
// tracing_subscriber::fmt::init();
#[cfg(test)]
use tracing_subscriber as _;
