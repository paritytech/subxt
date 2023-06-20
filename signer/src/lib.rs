// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Subxt-signer
//!
//! The main output from this crate is the [`sr25519::Keypair`], which can
//! be constructed from a bip39 phrase, secret URI or raw seed, and used to
//! sign and verify arbitrary messages. This crate is aligned with how Substrate's
//! `sp_core` crate constructs and signs keypairs, but is lighter on dependencies
//! and can support compilation to WASM with the `web` feature.
//!
//! Enable the `subxt` feature to enable use of this [`sr25519::Keypair`] in signing
//! subxt transactions for chains supporting sr25519 signatures.

#![deny(missing_docs)]

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
