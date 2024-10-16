// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A simple helper crate to fetch metadata from a file or URL.
//!
//! This crate has an optional feature `url` which allows fetching metadata from URL
//! which requires lot of dependencies and is disabled by default.

mod error;
pub mod file;
#[cfg(feature = "url")]
#[cfg_attr(docsrs, doc(cfg(feature = "url")))]
pub mod url;

pub use error::Error;
