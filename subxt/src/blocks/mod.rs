// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes the necessary functionality for working with events.

mod block_types;
mod blocks_client;
mod extrinsic_types;

/// A reference to a block.
pub use crate::backend::BlockRef;

pub use block_types::Block;
pub use blocks_client::BlocksClient;
pub use extrinsic_types::{ExtrinsicDetails, ExtrinsicEvents, Extrinsics, StaticExtrinsic};

// We get account nonce info in tx_client, too, so re-use the logic:
pub(crate) use block_types::get_account_nonce;
