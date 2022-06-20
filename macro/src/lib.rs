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

//! Generate a strongly typed API for interacting with a Substrate runtime from its metadata.
//!
//! Usage:
//!
//! Download metadata from a running Substrate node using `subxt-cli`:
//!
//! ```bash
//! subxt metadata > polkadot_metadata.scale
//! ```
//!
//! Annotate a Rust module with the `subxt` attribute referencing the aforementioned metadata file.
//!
//! ```ignore
//! #[subxt::subxt(
//!     runtime_metadata_path = "polkadot_metadata.scale",
//! )]
//! pub mod polkadot {}
//! ```
//!
//! The `subxt` macro will populate the annotated module with all of the methods and types required
//! for submitting extrinsics and reading from storage for the given runtime.
//!
//! ## Substituting types
//!
//! In order to replace a generated type by a user-defined type, use `substitute_type`:
//!
//! ```ignore
//! #[subxt::subxt(
//!     runtime_metadata_path = "polkadot_metadata.scale",
//! )]
//! pub mod polkadot {
//!     #[subxt(substitute_type = "sp_arithmetic::per_things::Perbill")]
//!     use sp_runtime::Perbill;
//! }
//! ```
//!
//! This will replace the generated type and any usages with the specified type at the `use` import.
//! It is useful for using custom decoding for specific types, or to provide a type with foreign
//! trait implementations, or other specialized functionality.

//! ## Custom Derives
//!
//! By default all generated types are annotated with `scale::Encode` and `scale::Decode` derives.
//! However when using the generated types in the client, they may require additional derives to be
//! useful.
//!
//! ### Adding derives for all types
//!
//! Add `derive_for_all_types` with a comma seperated list of the derives to apply to *all* types
//!
//! ```ignore
//! #[subxt::subxt(
//!     runtime_metadata_path = "polkadot_metadata.scale",
//!     derive_for_all_types = "Eq, PartialEq"
//! )]
//! pub mod polkadot {}
//! ```
//!
//! ### Adding derives for specific types
//!
//! Add `derive_for_type` for each specific type with a comma seperated list of the derives to
//! apply for that type only.
//!
//! ```ignore
//! #[subxt::subxt(
//!     runtime_metadata_path = "polkadot_metadata.scale",
//!     derive_for_all_types = "Eq, PartialEq",
//!     derive_for_type(type = "frame_support::PalletId", derive = "Ord, PartialOrd"),
//!     derive_for_type(type = "sp_runtime::ModuleError", derive = "Hash"),
//! )]
//! pub mod polkadot {}
//! ```

#![deny(unused_crate_dependencies)]

extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use subxt_codegen::DerivesRegistry;
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
};

#[derive(Debug, FromMeta)]
struct RuntimeMetadataArgs {
    runtime_metadata_path: String,
    #[darling(default)]
    derive_for_all_types: Option<Punctuated<syn::Path, syn::Token![,]>>,
    #[darling(multiple)]
    derive_for_type: Vec<DeriveForType>,
}

#[derive(Debug, FromMeta)]
struct DeriveForType {
    #[darling(rename = "type")]
    ty: syn::TypePath,
    derive: Punctuated<syn::Path, syn::Token![,]>,
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn subxt(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let item_mod = parse_macro_input!(input as syn::ItemMod);
    let args = match RuntimeMetadataArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let root_path = std::path::Path::new(&root);
    let path = root_path.join(args.runtime_metadata_path);

    let mut derives_registry = DerivesRegistry::default();
    if let Some(derive_for_all) = args.derive_for_all_types {
        derives_registry.extend_for_all(derive_for_all.iter().cloned());
    }
    for derives in &args.derive_for_type {
        derives_registry
            .extend_for_type(derives.ty.clone(), derives.derive.iter().cloned())
    }

    subxt_codegen::generate_runtime_api(item_mod, &path, derives_registry).into()
}
