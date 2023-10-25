// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

extern crate proc_macro;

use codec::Decode;
use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use subxt_codegen::{
    fetch_metadata::{
        fetch_metadata_from_file_blocking, fetch_metadata_from_url_blocking, MetadataVersion, Url,
    },
    CodegenBuilder, CodegenError,
};
use syn::{parse_macro_input, punctuated::Punctuated};

#[derive(Clone, Debug)]
struct OuterAttribute(syn::Attribute);

impl syn::parse::Parse for OuterAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.call(syn::Attribute::parse_outer)?[0].clone()))
    }
}

#[derive(Debug, FromMeta)]
struct RuntimeMetadataArgs {
    #[darling(default)]
    runtime_metadata_path: Option<String>,
    #[darling(default)]
    runtime_metadata_url: Option<String>,
    #[darling(default)]
    derive_for_all_types: Option<Punctuated<syn::Path, syn::Token![,]>>,
    #[darling(default)]
    attributes_for_all_types: Option<Punctuated<OuterAttribute, syn::Token![,]>>,
    #[darling(multiple)]
    derive_for_type: Vec<DeriveForType>,
    #[darling(multiple)]
    attributes_for_type: Vec<AttributesForType>,
    #[darling(multiple)]
    substitute_type: Vec<SubstituteType>,
    #[darling(default, rename = "crate")]
    crate_path: Option<syn::Path>,
    #[darling(default)]
    generate_docs: darling::util::Flag,
    #[darling(default)]
    runtime_types_only: bool,
    #[darling(default)]
    no_default_derives: bool,
    #[darling(default)]
    no_default_substitutions: bool,
    #[darling(default)]
    unstable_metadata: darling::util::Flag,
}

#[derive(Debug, FromMeta)]
struct DeriveForType {
    path: syn::TypePath,
    derive: Punctuated<syn::Path, syn::Token![,]>,
}

#[derive(Debug, FromMeta)]
struct AttributesForType {
    path: syn::TypePath,
    attributes: Punctuated<OuterAttribute, syn::Token![,]>,
}

#[derive(Debug, FromMeta)]
struct SubstituteType {
    path: syn::Path,
    with: syn::Path,
}

// Note: docs for this are in the subxt library; don't add any here as they will be appended.
#[proc_macro_attribute]
#[proc_macro_error]
pub fn subxt(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let item_mod = parse_macro_input!(input as syn::ItemMod);
    let args = match RuntimeMetadataArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let mut codegen = CodegenBuilder::new();

    // Use the item module that the macro is on:
    codegen.set_target_module(item_mod);

    // Use the provided crate path:
    if let Some(crate_path) = args.crate_path {
        codegen.set_subxt_crate_path(crate_path)
    }

    // Respect the boolean flags:
    if args.runtime_types_only {
        codegen.runtime_types_only();
    }
    if args.no_default_derives {
        codegen.disable_default_derives();
    }
    if args.no_default_substitutions {
        codegen.disable_default_substitutes();
    }
    if !args.generate_docs.is_present() {
        codegen.no_docs()
    }

    // Configure derives:
    codegen.set_additional_global_derives(
        args.derive_for_all_types
            .unwrap_or_default()
            .into_iter()
            .collect(),
    );
    for d in args.derive_for_type {
        codegen.add_derives_for_type(d.path, d.derive.into_iter());
    }

    // Configure attributes:
    codegen.set_additional_global_attributes(
        args.attributes_for_all_types
            .unwrap_or_default()
            .into_iter()
            .map(|a| a.0)
            .collect(),
    );
    for d in args.attributes_for_type {
        codegen.add_attributes_for_type(d.path, d.attributes.into_iter().map(|a| a.0))
    }

    // Insert type substitutions:
    for sub in args.substitute_type.into_iter() {
        codegen.set_type_substitute(sub.path, sub.with);
    }

    // Do we want to fetch unstable metadata? This only works if fetching from a URL.
    let unstable_metadata = args.unstable_metadata.is_present();

    match (args.runtime_metadata_path, args.runtime_metadata_url) {
        (Some(rest_of_path), None) => {
            if unstable_metadata {
                abort_call_site!(
                    "The 'unstable_metadata' attribute requires `runtime_metadata_url`"
                )
            }

            let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
            let root_path = std::path::Path::new(&root);
            let path = root_path.join(rest_of_path);
            let generated_code = fetch_metadata_from_file_blocking(&path)
                .map_err(|e| CodegenError::from(e))
                .and_then(|b| subxt_codegen::Metadata::decode(&mut &*b).map_err(Into::into))
                .and_then(|m| codegen.generate(m).map_err(Into::into))
                .unwrap_or_else(|e| abort_call_site!("{}", e));

            generated_code.into()
        }
        (None, Some(url_string)) => {
            let url = Url::parse(&url_string).unwrap_or_else(|_| {
                abort_call_site!("Cannot download metadata; invalid url: {}", url_string)
            });

            let version = match unstable_metadata {
                true => MetadataVersion::Unstable,
                false => MetadataVersion::Latest,
            };

            let generated_code = fetch_metadata_from_url_blocking(url, version)
                .map_err(|e| CodegenError::from(e))
                .and_then(|b| subxt_codegen::Metadata::decode(&mut &*b).map_err(Into::into))
                .and_then(|m| codegen.generate(m).map_err(Into::into))
                .unwrap_or_else(|e| abort_call_site!("{}", e));

            generated_code.into()
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
