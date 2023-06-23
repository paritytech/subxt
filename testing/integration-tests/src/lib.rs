// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![deny(unused_crate_dependencies)]

#[cfg(test)]
pub mod utils;

// We expose the utils crate's contents as public to avoid
// a few unused warnings when running the tests under the
// "unstable-light-client" flag.
#[cfg(all(test, feature = "unstable-light-client"))]
pub use utils::*;
#[cfg(all(test, not(feature = "unstable-light-client")))]
use utils::*;

#[cfg(all(test, not(feature = "unstable-light-client")))]
mod full_client;

#[cfg(all(test, feature = "unstable-light-client"))]
mod light_client;

#[cfg(test)]
use test_runtime::node_runtime;

// These dependencies are used for the full client.
#[cfg(all(test, not(feature = "unstable-light-client")))]
use regex as _;
#[cfg(all(test, not(feature = "unstable-light-client")))]
use subxt_codegen as _;
#[cfg(all(test, not(feature = "unstable-light-client")))]
use syn as _;

// We don't use this dependency, but it's here so that we
// can enable logging easily if need be. Add this to a test
// to enable tracing for it:
//
// tracing_subscriber::fmt::init();
#[cfg(test)]
use tracing_subscriber as _;
