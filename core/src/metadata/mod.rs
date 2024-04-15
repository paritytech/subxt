// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A [`Metadata`] type, which is used through this crate.
//!
//! This can be decoded from the bytes handed back from a node when asking for metadata.
//!
//! # Examples
//!
//! ```rust
//! use subxt_core::metadata;
//!
//! // We need to fetch the bytes from somewhere, and then we can decode them:
//! let metadata_bytes = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
//! let metadata = metadata::decode_from(&metadata_bytes[..]).unwrap();
//! ```

mod decode_encode_traits;
mod metadata_type;

use codec::Decode;

pub use decode_encode_traits::{DecodeWithMetadata, EncodeWithMetadata};
pub use metadata_type::Metadata;

/// Attempt to decode some bytes into [`Metadata`], returning an error
/// if decoding fails.
///
/// This is a shortcut for importing [`codec::Decode`] and using the
/// implementation of that on [`Metadata`].
pub fn decode_from(bytes: &[u8]) -> Result<Metadata, codec::Error> {
    Metadata::decode(&mut &*bytes)
}
