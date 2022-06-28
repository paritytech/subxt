// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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
//! use frame_metadata::RuntimeMetadataPrefixed;
//! use subxt_codegen::DerivesRegistry;
//!
//! let encoded = fs::read("../artifacts/polkadot_metadata.scale").unwrap();
//!
//! // Runtime metadata obtained from a node.
//! let metadata = <RuntimeMetadataPrefixed as Decode>::decode(&mut &*encoded).unwrap();
//! // Module under which the API is generated.
//! let item_mod = syn::parse_quote!(
//!     pub mod api {}
//! );
//! // Default module derivatives.
//! let mut derives = DerivesRegistry::default();
//! // Generate the Runtime API.
//! let generator = subxt_codegen::RuntimeGenerator::new(metadata);
//! let runtime_api = generator.generate_runtime(item_mod, derives);
//! println!("{}", runtime_api);
//! ```

#![deny(unused_crate_dependencies)]

mod api;
mod ir;
mod types;

pub use self::{
    api::{
        generate_runtime_api,
        RuntimeGenerator,
    },
    types::{
        Derives,
        DerivesRegistry,
        Module,
        TypeGenerator,
    },
};
