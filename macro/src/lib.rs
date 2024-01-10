// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Subxt macro for generating Substrate runtime interfaces.

extern crate proc_macro;

use codec::Decode;
use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::ToTokens;
use scale_typegen::typegen::{
    settings::substitutes::path_segments,
    validation::{registry_contains_type_path, similar_type_paths_in_registry},
};
use subxt_codegen::{
    fetch_metadata::{
        fetch_metadata_from_file_blocking, fetch_metadata_from_url_blocking, MetadataVersion, Url,
    },
    CodegenBuilder, CodegenError, Metadata,
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
    runtime_metadata_insecure_url: Option<String>,
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
    #[darling(default)]
    recursive: bool,
}

#[derive(Debug, FromMeta)]
struct AttributesForType {
    path: syn::TypePath,
    attributes: Punctuated<OuterAttribute, syn::Token![,]>,
    #[darling(default)]
    recursive: bool,
}

#[derive(Debug, FromMeta)]
struct SubstituteType {
    path: syn::Path,
    with: syn::Path,
}

// Note: docs for this are in the subxt library; don't add further docs here as they will be appended.
#[proc_macro_attribute]
#[proc_macro_error]
pub fn subxt(args: TokenStream, input: TokenStream) -> TokenStream {
    match _subxt(args, parse_macro_input!(input as syn::ItemMod)) {
        Ok(e) => e,
        Err(e) => e,
    }
}

// Node: just an additonal function to make early returns easier.
fn _subxt(args: TokenStream, item_mod: syn::ItemMod) -> Result<TokenStream, TokenStream> {
    let attr_args = NestedMeta::parse_meta_list(args.into())
        .map_err(|e| TokenStream::from(darling::Error::from(e).write_errors()))?;
    let args = RuntimeMetadataArgs::from_list(&attr_args)
        .map_err(|e| TokenStream::from(e.write_errors()))?;

    // Fetch metadata first, because we need it to validate some of the chosen codegen options.
    let metadata = fetch_metadata(&args)?;

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
        validate_type_path(&d.path.path, &metadata);
        codegen.add_derives_for_type(d.path, d.derive.into_iter(), d.recursive);
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
        validate_type_path(&d.path.path, &metadata);
        codegen.add_attributes_for_type(d.path, d.attributes.into_iter().map(|a| a.0), d.recursive)
    }

    // Insert type substitutions:
    for sub in args.substitute_type.into_iter() {
        validate_type_path(&sub.path, &metadata);
        codegen.set_type_substitute(sub.path, sub.with);
    }

    let code = codegen
        .generate(metadata)
        .map_err(|e| e.into_compile_error())?;

    Ok(code.into())
}

/// Checks that a type is present in the type registry. If it is not found, abort with a
/// helpful error message, showing the user alternative types, that have the same name, but are at different locations in the metadata.
fn validate_type_path(path: &syn::Path, metadata: &Metadata) {
    let path_segments = path_segments(path);
    let ident = &path
        .segments
        .last()
        .expect("Empty path should be filtered out before already")
        .ident;
    if !registry_contains_type_path(metadata.types(), &path_segments) {
        let alternatives = similar_type_paths_in_registry(metadata.types(), path);
        let alternatives: String = if alternatives.is_empty() {
            format!("There is no Type with name `{ident}` in the provided metadata.")
        } else {
            let mut s = "A type with the same name is present at: ".to_owned();
            for p in alternatives {
                s.push('\n');
                s.push_str(&pretty_path(&p));
            }
            s
        };

        abort_call_site!(
            "Type `{}` does not exist at path `{}`\n\n{}",
            ident.to_string(),
            pretty_path(path),
            alternatives
        );
    }

    fn pretty_path(path: &syn::Path) -> String {
        path.to_token_stream().to_string().replace(' ', "")
    }
}

/// Fetches metadata in a blocking manner, either from a url (not recommended) or from a file path.
fn fetch_metadata(args: &RuntimeMetadataArgs) -> Result<subxt_codegen::Metadata, TokenStream> {
    // Do we want to fetch unstable metadata? This only works if fetching from a URL.
    let unstable_metadata = args.unstable_metadata.is_present();
    let metadata = match (
        &args.runtime_metadata_path,
        &args.runtime_metadata_insecure_url,
    ) {
        (Some(rest_of_path), None) => {
            if unstable_metadata {
                abort_call_site!(
                    "The 'unstable_metadata' attribute requires `runtime_metadata_insecure_url`"
                )
            }

            let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
            let root_path = std::path::Path::new(&root);
            let path = root_path.join(rest_of_path);
            fetch_metadata_from_file_blocking(&path)
                .and_then(|b| subxt_codegen::Metadata::decode(&mut &*b).map_err(Into::into))
                .map_err(|e| CodegenError::from(e).into_compile_error())?
        }
        (None, Some(url_string)) => {
            let url = Url::parse(url_string).unwrap_or_else(|_| {
                abort_call_site!("Cannot download metadata; invalid url: {}", url_string)
            });

            let version = match unstable_metadata {
                true => MetadataVersion::Unstable,
                false => MetadataVersion::Latest,
            };

            fetch_metadata_from_url_blocking(url, version)
                .map_err(CodegenError::from)
                .and_then(|b| subxt_codegen::Metadata::decode(&mut &*b).map_err(Into::into))
                .map_err(|e| e.into_compile_error())?
        }
        (None, None) => {
            abort_call_site!(
                "One of 'runtime_metadata_path' or 'runtime_metadata_insecure_url' must be provided"
            )
        }
        (Some(_), Some(_)) => {
            abort_call_site!(
                "Only one of 'runtime_metadata_path' or 'runtime_metadata_insecure_url' can be provided"
            )
        }
    };
    Ok(metadata)
}
