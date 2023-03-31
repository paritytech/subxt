// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utilities to help with fetching and decoding metadata.

mod fetch_metadata;

// easy access to this type needed for fetching metadata:
pub use jsonrpsee::client_transport::ws::Uri;

pub use fetch_metadata::{
    fetch_metadata_bytes, fetch_metadata_bytes_blocking, fetch_metadata_hex,
    fetch_metadata_hex_blocking,
};
