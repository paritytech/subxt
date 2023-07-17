// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes the necessary functionality for working with events.

mod block_types;
mod blocks_client;
mod extrinsic_types;

pub use block_types::{Block, BlockBody};
pub use blocks_client::{subscribe_to_block_headers_filling_in_gaps, BlocksClient};
pub use extrinsic_types::{ExtrinsicDetails, ExtrinsicEvents, Extrinsics, StaticExtrinsic};
