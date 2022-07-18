// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![deny(unused_crate_dependencies)]

#[cfg(test)]
mod codegen;
#[cfg(test)]
mod utils;

#[cfg(test)]
mod client;
#[cfg(test)]
mod events;
#[cfg(test)]
mod frame;
#[cfg(test)]
mod metadata;
#[cfg(test)]
mod storage;

#[cfg(test)]
use test_runtime::node_runtime;
#[cfg(test)]
use utils::*;

// We don't use this dependency, but it's here so that we
// can enable logging easily if need be. Add this to a test
// to enable tracing for it:
//
// tracing_subscriber::fmt::init();
#[cfg(test)]
use tracing_subscriber as _;
