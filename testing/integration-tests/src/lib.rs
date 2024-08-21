// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#[cfg(all(feature = "unstable-light-client", feature = "unstable-backend-client"))]
compile_error!(
    "The features 'unstable-light-client' and 'unstable-backend-client' cannot be used together"
);

#[cfg(test)]
pub mod utils;

#[cfg(test)]
#[cfg_attr(test, allow(unused_imports))]
use utils::*;

#[cfg(any(
    all(test, not(feature = "unstable-light-client")),
    all(test, feature = "unstable-light-client-long-running")
))]
mod full_client;

#[cfg(all(test, feature = "unstable-light-client"))]
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
