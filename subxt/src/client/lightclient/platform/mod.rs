// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub mod default;

#[cfg(not(feature = "unstable-light-client-wasm"))]
mod native;

#[cfg(feature = "unstable-light-client-wasm")]
mod wasm;
#[cfg(feature = "unstable-light-client-wasm")]
mod wasm_socket;
