// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes the necessary functionality for working with events.

mod blocks_client;
mod block_types;

pub use blocks_client::{
    subscribe_to_block_headers_filling_in_gaps,
    BlocksClient,
};
pub use block_types::{
    Block,
    Extrinsic,
    ExtrinsicEvents,
};