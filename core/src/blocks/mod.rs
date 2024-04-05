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
//! use subxt_core::metadata::Metadata;
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
//!     hex::decode("280402000bf18367a38e01").unwrap(),
//!     hex::decode("c10184008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4801f4de97941fcc3f95c761cd58d480bb41ce64836850f51b6fcc7542e809eb0a346fe95eb1b72de542273d4f1b00b636eb025e2b0e98cc498a095e7ce48f3d4f82b501040000001848656c6c6f21").unwrap(),
//!     hex::decode("5102840090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe2201ac0c06f55cf3461067bbe48da16efbb50dfad555e2821ce20d37b2e42d6dcb439acd40f742b12ef00f8889944060b04373dc4d34a1992042fd269e8ec1e64a848502000004000090b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe2217000010632d5ec76b05").unwrap()
//! ];
//!
//! // Given some chain config and metadata, we know how to decode the bytes.
//! let exts = blocks::decode_from::<PolkadotConfig>(ext_bytes, metadata).unwrap();
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

use crate::error::BlockError;
use crate::Metadata;
use crate::config::Config;

pub use static_extrinsic::StaticExtrinsic;
pub use extrinsic_signed_extensions::{ExtrinsicSignedExtension, ExtrinsicSignedExtensions};
pub use extrinsics::{
    ExtrinsicDetails, ExtrinsicMetadataDetails, Extrinsics, FoundExtrinsic, SignedExtrinsicDetails,
};

/// Instantiate a new [`Extrinsics`] object, given a vector containing each extrinsic hash (in the
/// form of bytes) and some metadata that we'll use to decode them.
///
/// This is a shortcut for [`Extrinsics::decode_from`].
pub fn decode_from<T: Config>(extrinsics: Vec<Vec<u8>>, metadata: Metadata) -> Result<Extrinsics<T>, BlockError> {
    Extrinsics::decode_from(extrinsics, metadata)
}