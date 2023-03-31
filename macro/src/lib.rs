// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

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
//!     substitute_type(type = "sp_arithmetic::per_things::Perbill", with = "sp_runtime::Perbill")
//! )]
//! pub mod polkadot {}
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
//! Add `derive_for_all_types` with a comma separated list of the derives to apply to *all* types
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
//! Add `derive_for_type` for each specific type with a comma separated list of the derives to
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
//!
//! ### Custom crate path
//!
//! In order to specify a custom crate path to be used for the code generation:
//!
//! ```ignore
//! #[subxt::subxt(crate = "crate::path::to::subxt")]
//! pub mod polkadot {}
//! ```
//!
//! By default the path `::subxt` is used.
//!
//! ### Expose documentation
//!
//! In order to expose the documentation from the runtime metadata on the generated
//! code, users must specify the `generate_docs` flag:
//!
//! ```ignore
//! #[subxt::subxt(generate_docs)]
//! pub mod polkadot {}
//! ```
//!
//! By default the documentation is not generated.
//!
//! ### Runtime types generation
//!
//! In some cases, you may be interested only in the runtime types, like `RuntimeCall` enum. You can
//! limit code generation to just `runtime_types` module with `runtime_types_only` flag:
//!
//! ```ignore
//! #[subxt::subxt(runtime_types_only)]
//! // or equivalently
//! #[subxt::subxt(runtime_types_only = true)]
//! ```

#![deny(unused_crate_dependencies)]

extern crate proc_macro;

use std::str::FromStr;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use subxt_codegen::{utils::Uri, CodegenError, DerivesRegistry, TypeSubstitutes};
use syn::{parse_macro_input, punctuated::Punctuated};

#[derive(Debug, FromMeta)]
struct RuntimeMetadataArgs {
    #[darling(default)]
    runtime_metadata_path: Option<String>,
    #[darling(default)]
    runtime_metadata_url: Option<String>,
    #[darling(default)]
    derive_for_all_types: Option<Punctuated<syn::Path, syn::Token![,]>>,
    #[darling(multiple)]
    derive_for_type: Vec<DeriveForType>,
    #[darling(multiple)]
    substitute_type: Vec<SubstituteType>,
    #[darling(default, rename = "crate")]
    crate_path: Option<String>,
    #[darling(default)]
    generate_docs: darling::util::Flag,
    #[darling(default)]
    runtime_types_only: bool,
}

#[derive(Debug, FromMeta)]
struct DeriveForType {
    #[darling(rename = "type")]
    ty: syn::TypePath,
    derive: Punctuated<syn::Path, syn::Token![,]>,
}

#[derive(Debug, FromMeta)]
struct SubstituteType {
    #[darling(rename = "type")]
    ty: syn::Path,
    with: syn::Path,
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

    let crate_path = match args.crate_path {
        Some(crate_path) => crate_path.into(),
        None => subxt_codegen::CratePath::default(),
    };
    let mut derives_registry = DerivesRegistry::new(&crate_path);

    if let Some(derive_for_all) = args.derive_for_all_types {
        derives_registry.extend_for_all(derive_for_all.iter().cloned());
    }
    for derives in &args.derive_for_type {
        derives_registry.extend_for_type(
            derives.ty.clone(),
            derives.derive.iter().cloned(),
            &crate_path,
        )
    }

    let mut type_substitutes = TypeSubstitutes::new(&crate_path);
    let substitute_args_res: Result<(), _> = args.substitute_type.into_iter().try_for_each(|sub| {
        sub.with
            .try_into()
            .and_then(|with| type_substitutes.insert(sub.ty, with))
    });

    if let Err(err) = substitute_args_res {
        return CodegenError::from(err).into_compile_error().into();
    }

    let should_gen_docs = args.generate_docs.is_present();
    match (args.runtime_metadata_path, args.runtime_metadata_url) {
        (Some(rest_of_path), None) => {
            let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
            let root_path = std::path::Path::new(&root);
            let path = root_path.join(rest_of_path);
            subxt_codegen::generate_runtime_api_from_path(
                item_mod,
                path,
                derives_registry,
                type_substitutes,
                crate_path,
                should_gen_docs,
                args.runtime_types_only,
            )
            .map_or_else(|err| err.into_compile_error().into(), Into::into)
        }
        (None, Some(url_string)) => {
            let url = Uri::from_str(&url_string).unwrap_or_else(|_| {
                abort_call_site!("Cannot download metadata; invalid url: {}", url_string)
            });
            subxt_codegen::generate_runtime_api_from_url(
                item_mod,
                &url,
                derives_registry,
                type_substitutes,
                crate_path,
                should_gen_docs,
                args.runtime_types_only,
            )
            .map_or_else(|err| err.into_compile_error().into(), Into::into)
        }
        (None, None) => {
            abort_call_site!(
                "One of 'runtime_metadata_path' or 'runtime_metadata_url' must be provided"
            )
        }
        (Some(_), Some(_)) => {
            abort_call_site!(
                "Only one of 'runtime_metadata_path' or 'runtime_metadata_url' can be provided"
            )
        }
    }
}
