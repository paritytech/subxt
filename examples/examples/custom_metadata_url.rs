// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

// If you'd like to use metadata directly from a running node, you
// can provide a URL to that node here. HTTP or WebSocket URLs can be
// provided. Note that if the metadata cannot be retrieved from this
// node URL at compile time, compilation will fail.
#[subxt::subxt(runtime_metadata_url = "wss://rpc.polkadot.io:443")]
pub mod polkadot {}

fn main() {}
