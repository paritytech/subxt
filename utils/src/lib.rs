// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Subxt utils.

// Internal helper macros
#[macro_use]
mod macros;

macros::cfg_fetch_metadata! {
    pub mod fetch_metadata;
}
