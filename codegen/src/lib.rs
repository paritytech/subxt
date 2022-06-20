// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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
