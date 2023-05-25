// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Library to generate an API for a Substrate runtime from its metadata.
//!
//! ## Generated Structure
//!
//! The API generator logic:
//! - At the root there is the `item_mod` provided (ie `pub mod api {}`)
//! - Pallets are represented by a child module (ie `pub mod PalletName {}`) of the root
//! - Each pallet exposes as child modules (if applicable):
//!   - Calls (`pub mod calls {}`)
//!   - Events (`pub mod events {}`)
//!   - Storage (`pub mod storage {}`)
//!   - Constants (`pub mod constants {}`)
//!
//! ## Example
//!
//! ```no_run
//! use std::fs;
//! use codec::Decode;
//! use subxt_metadata::Metadata;
//! use subxt_codegen::{CratePath, DerivesRegistry, TypeSubstitutes};
//!
//! let encoded = fs::read("../artifacts/polkadot_metadata_full.scale").unwrap();
//!
//! // Runtime metadata obtained from a node.
//! let metadata = Metadata::decode(&mut &*encoded).unwrap();
//! // Module under which the API is generated.
//! let item_mod = syn::parse_quote!(
//!     pub mod api {}
//! );
//! // Default module derivatives.
//! let mut derives = DerivesRegistry::with_default_derives(&CratePath::default());
//! // Default type substitutes.
//! let substs = TypeSubstitutes::with_default_substitutes(&CratePath::default());
//! // Generate the Runtime API.
//! let generator = subxt_codegen::RuntimeGenerator::new(metadata);
//! // Include metadata documentation in the Runtime API.
//! let generate_docs = true;
//! let runtime_api = generator.generate_runtime(item_mod, derives, substs, CratePath::default(), generate_docs).unwrap();
//! println!("{}", runtime_api);
//! ```

#![deny(unused_crate_dependencies, missing_docs)]

mod api;
mod error;
mod ir;
mod types;

pub mod utils;

pub use self::{
    api::{
        generate_runtime_api_from_bytes, generate_runtime_api_from_path,
        generate_runtime_api_from_url, RuntimeGenerator,
    },
    error::{CodegenError, TypeSubstitutionError},
    types::{CratePath, Derives, DerivesRegistry, Module, TypeGenerator, TypeSubstitutes},
};
