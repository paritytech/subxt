// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the metadata obtained from a node.

mod decode_encode_traits;
mod metadata_type;

pub use decode_encode_traits::{DecodeWithMetadata, EncodeWithMetadata};
pub use metadata_type::Metadata;

// Expose metadata types under a sub module in case somebody needs to reference them:
pub use subxt_metadata as types;
