// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

// The crypto module contains code adapted from sp_core::crypto.

mod derive_junction;
mod secret_uri;
mod seed_from_entropy;

pub use derive_junction::DeriveJunction;
pub use secret_uri::{SecretUri, SecretUriError, DEV_PHRASE};
pub use seed_from_entropy::seed_from_entropy;
