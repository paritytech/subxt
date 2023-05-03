// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

extern crate proc_macro;

use std::str::FromStr;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use subxt_codegen::{utils::Uri, CodegenError, DerivesRegistry, TypeSubstitutes};
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
    crate_path: Option<String>,
    #[darling(default)]
    generate_docs: darling::util::Flag,
    #[darling(default)]
    runtime_types_only: bool,
    #[darling(default)]
    no_default_derives: bool,
    #[darling(default)]
    no_default_substitutions: bool,
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

    let crate_path = match args.crate_path {
        Some(crate_path) => crate_path.into(),
        None => subxt_codegen::CratePath::default(),
    };
    let mut derives_registry = if args.no_default_derives {
        DerivesRegistry::new()
    } else {
        DerivesRegistry::with_default_derives(&crate_path)
    };

    let universal_derives = args.derive_for_all_types.unwrap_or_default();
    let universal_attributes = args.attributes_for_all_types.unwrap_or_default();
    derives_registry.extend_for_all(
        universal_derives,
        universal_attributes.iter().map(|a| a.0.clone()),
    );

    for derives in &args.derive_for_type {
        derives_registry.extend_for_type(
            derives.path.clone(),
            derives.derive.iter().cloned(),
            vec![],
        )
    }
    for attributes in &args.attributes_for_type {
        derives_registry.extend_for_type(
            attributes.path.clone(),
            vec![],
            attributes.attributes.iter().map(|a| a.0.clone()),
        )
    }

    let mut type_substitutes = if args.no_default_substitutions {
        TypeSubstitutes::new()
    } else {
        TypeSubstitutes::with_default_substitutes(&crate_path)
    };
    let substitute_args_res: Result<(), _> = args.substitute_type.into_iter().try_for_each(|sub| {
        sub.with
            .try_into()
            .and_then(|with| type_substitutes.insert(sub.path, with))
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
