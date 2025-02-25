// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![allow(clippy::too_many_arguments)]

/// The SCALE encoded metadata obtained from a local run of a substrate node.
pub static METADATA: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/test_node_runtime_metadata.scale"
));

include!(concat!(env!("OUT_DIR"), "/runtime.rs"));
