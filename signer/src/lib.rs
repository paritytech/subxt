// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#[macro_use]
mod utils;
mod crypto;

// An sr25519 key pair implementation.
pub mod sr25519;

// Re-export useful bits and pieces for generating a Pair from a phrase,
// namely the Mnemonic struct.
pub use bip39;

// Used to hold strings in a more secure manner in memory for a little extra
// protection.
pub use secrecy::{ExposeSecret, SecretString};

// SecretUri's can be parsed from strings and used to generate key pairs.
// DeriveJunctions are the "path" part of these SecretUris.
pub use crypto::{DeriveJunction, SecretUri, SecretUriError, DEV_PHRASE};
