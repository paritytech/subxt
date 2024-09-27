// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Decode and iterate over the extrinsics in block bodies.
//!
//! Use the [`decode_from`] function as an entry point to decoding extrinsics, and then
//! have a look at [`Extrinsics`] and [`ExtrinsicDetails`] to see which methods are available
//! to work with the extrinsics.
//!
//! # Example
//!
//! ```rust
//! extern crate alloc;
//!
//! use subxt_macro::subxt;
//! use subxt_core::blocks;
//! use subxt_core::metadata;
//! use subxt_core::config::PolkadotConfig;
//! use alloc::vec;
//!
//! // If we generate types without `subxt`, we need to point to `::subxt_core`:
//! #[subxt(
//!     crate = "::subxt_core",
//!     runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale",
//! )]
//! pub mod polkadot {}
//!
//! // Some metadata we'd like to use to help us decode extrinsics:
//! let metadata_bytes = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
//! let metadata = metadata::decode_from(&metadata_bytes[..]).unwrap();
//!
//! // Some extrinsics we'd like to decode:
//! let ext_bytes = vec![
//!     hex::decode("1004020000").unwrap(),
//!     hex::decode("c10184001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c01a27c400241aeafdea1871b32f1f01e92acd272ddfe6b2f8b73b64c606572a530c470a94ef654f7baa5828474754a1fe31b59f91f6bb5c2cd5a07c22d4b8b8387350100000000001448656c6c6f").unwrap(),
//!     hex::decode("550284001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0144bb92734447c893ab16d520fae0d455257550efa28ee66bf6dc942cb8b00d5d2799b98bc2865d21812278a9a266acd7352f40742ff11a6ce1f400013961598485010000000400008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a481700505a4f7e9f4eb106").unwrap()
//! ];
//!
//! // Given some chain config and metadata, we know how to decode the bytes.
//! let exts = blocks::decode_from::<PolkadotConfig>(ext_bytes, metadata);
//!
//! // We'll see 3 extrinsics:
//! assert_eq!(exts.len(), 3);
//!
//! // We can iterate over them and decode various details out of them.
//! for ext in exts.iter() {
//!     let ext = ext.unwrap();
//!     println!("Pallet: {}", ext.pallet_name().unwrap());
//!     println!("Call:   {}", ext.variant_name().unwrap());
//! }
//!
//! # let ext_details: Vec<_> = exts.iter()
//! #     .map(|ext| {
//! #         let ext = ext.unwrap();
//! #         let pallet = ext.pallet_name().unwrap().to_string();
//! #         let call = ext.variant_name().unwrap().to_string();
//! #         (pallet, call)
//! #     })
//! #     .collect();
//! #
//! # assert_eq!(ext_details, vec![
//! #     ("Timestamp".to_owned(), "set".to_owned()),
//! #     ("System".to_owned(), "remark".to_owned()),
//! #     ("Balances".to_owned(), "transfer_allow_death".to_owned()),
//! # ]);
//! ```

mod extrinsic_signed_extensions;
mod extrinsics;
mod static_extrinsic;

use crate::error::Error;
use crate::config::Config;
use crate::Metadata;
use alloc::vec::Vec;

pub use crate::error::BlockError;
pub use extrinsic_signed_extensions::{ExtrinsicSignedExtension, ExtrinsicSignedExtensions};
pub use extrinsics::{ExtrinsicDetails, ExtrinsicMetadataDetails, Extrinsics, FoundExtrinsic};
pub use static_extrinsic::StaticExtrinsic;

/// Instantiate a new [`Extrinsics`] object, given a vector containing each extrinsic hash (in the
/// form of bytes) and some metadata that we'll use to decode them.
///
/// This is a shortcut for [`Extrinsics::decode_from`].
pub fn decode_from<T: Config>(extrinsics: Vec<Vec<u8>>, metadata: Metadata) -> Result<Extrinsics<T>, Error> {
    Extrinsics::decode_from(extrinsics, metadata)
}
