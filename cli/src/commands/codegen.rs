// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::utils::{validate_url_security, FileOrUrl};
use clap::Parser as ClapParser;
use codec::Decode;
use color_eyre::eyre::eyre;
use subxt_codegen::CodegenBuilder;

/// Generate runtime API client code from metadata.
///
/// # Example (with code formatting)
///
/// `subxt codegen | rustfmt --edition=2018 --emit=stdout`
#[derive(Debug, ClapParser)]
pub struct Opts {
    #[command(flatten)]
    file_or_url: FileOrUrl,
    /// Additional derives
    #[clap(long = "derive")]
    derives: Vec<String>,
    /// Additional attributes
    #[clap(long = "attribute")]
    attributes: Vec<String>,
    /// Additional derives for a given type.
    ///
    /// Example 1: `--derive-for-type my_module::my_type=serde::Serialize`.
    /// Example 2: `--derive-for-type my_module::my_type=serde::Serialize,recursive`.
    #[clap(long = "derive-for-type", value_parser = derive_for_type_parser)]
    derives_for_type: Vec<DeriveForType>,
    /// Additional attributes for a given type.
    ///
    /// Example 1: `--attributes-for-type my_module::my_type=#[allow(clippy::all)]`.
    /// Example 2: `--attributes-for-type my_module::my_type=#[allow(clippy::all)],recursive`.
    #[clap(long = "attributes-for-type", value_parser = attributes_for_type_parser)]
    attributes_for_type: Vec<AttributeForType>,
    /// Substitute a type for another.
    ///
    /// Example `--substitute-type sp_runtime::MultiAddress<A,B>=subxt::utils::Static<::sp_runtime::MultiAddress<A,B>>`
    #[clap(long = "substitute-type", value_parser = substitute_type_parser)]
    substitute_types: Vec<(String, String)>,
    /// The `subxt` crate access path in the generated code.
    /// Defaults to `::subxt`.
    #[clap(long = "crate")]
    crate_path: Option<String>,
    /// Do not generate documentation for the runtime API code.
    ///
    /// Defaults to `false` (documentation is generated).
    #[clap(long, action)]
    no_docs: bool,
    /// Whether to limit code generation to only runtime types.
    ///
    /// Defaults to `false` (all types are generated).
    #[clap(long)]
    runtime_types_only: bool,
    /// Do not provide default trait derivations for the generated types.
    ///
    /// Defaults to `false` (default trait derivations are provided).
    #[clap(long)]
    no_default_derives: bool,
    /// Do not provide default substitutions for the generated types.
    ///
    /// Defaults to `false` (default substitutions are provided).
    #[clap(long)]
    no_default_substitutions: bool,
    /// Allow insecure URLs e.g. URLs starting with ws:// or http:// without SSL encryption
    #[clap(long, short)]
    allow_insecure: bool,
}

#[derive(Debug, Clone)]
struct DeriveForType {
    type_path: String,
    trait_path: String,
    recursive: bool,
}

#[derive(Debug, Clone)]
struct AttributeForType {
    type_path: String,
    attribute: String,
    recursive: bool,
}

fn derive_for_type_parser(src: &str) -> Result<DeriveForType, String> {
    let (type_path,trait_path , recursive) = str_equals_str_comma_maybe_recursive_parser(src)
    .ok_or_else(|| String::from("Invalid pattern for `derive-for-type`. It should be `type=derive` or `type=derive,recursive`, like `my_type=serde::Serialize` or `my_type=serde::Serialize,recursive`"))?;
    Ok(DeriveForType {
        type_path,
        trait_path,
        recursive,
    })
}

fn attributes_for_type_parser(src: &str) -> Result<AttributeForType, String> {
    let (type_path,attribute , recursive) = str_equals_str_comma_maybe_recursive_parser(src)
    .ok_or_else(|| String::from("Invalid pattern for `attributes-for-type`. It should be `type=attribute` like `my_type=serde::#[allow(clippy::all)]` or `type=attribute,recursive` like `my_type=serde::#[allow(clippy::all)], recursive`"))?;
    Ok(AttributeForType {
        type_path,
        attribute,
        recursive,
    })
}

/// Parses a `&str` of the form `str1=str2` into `(str1, str2, false)` or `str1=str2,recursive` into `(str1, str2, true)`.
fn str_equals_str_comma_maybe_recursive_parser(src: &str) -> Option<(String, String, bool)> {
    let (str1, rest) = src.split_once('=')?;

    let mut split_rest = rest.split(',');
    let str2 = split_rest
        .next()
        .expect("split iter always returns at least one element; qed");

    let rest_args = split_rest.collect::<Vec<_>>();
    let recursive = rest_args.contains(&"recursive");
    Some((str1.to_owned(), str2.to_owned(), recursive))
}

fn substitute_type_parser(src: &str) -> Result<(String, String), String> {
    let (from, to) = src
        .split_once('=')
        .ok_or_else(|| String::from("Invalid pattern for `substitute-type`. It should be something like `input::Type<A>=replacement::Type<A>`"))?;

    Ok((from.to_string(), to.to_string()))
}

pub async fn run(opts: Opts, output: &mut impl std::io::Write) -> color_eyre::Result<()> {
    validate_url_security(opts.file_or_url.url.as_ref(), opts.allow_insecure)?;

    let bytes = opts.file_or_url.fetch().await?;

    codegen(
        &bytes,
        opts.derives,
        opts.attributes,
        opts.derives_for_type,
        opts.attributes_for_type,
        opts.substitute_types,
        opts.crate_path,
        opts.no_docs,
        opts.runtime_types_only,
        opts.no_default_derives,
        opts.no_default_substitutions,
        output,
    )?;
    Ok(())
}

#[derive(Clone, Debug)]
struct OuterAttribute(syn::Attribute);

impl syn::parse::Parse for OuterAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.call(syn::Attribute::parse_outer)?[0].clone()))
    }
}

#[allow(clippy::too_many_arguments)]
fn codegen(
    metadata_bytes: &[u8],
    raw_derives: Vec<String>,
    raw_attributes: Vec<String>,
    derives_for_type: Vec<DeriveForType>,
    attributes_for_type: Vec<AttributeForType>,
    substitute_types: Vec<(String, String)>,
    crate_path: Option<String>,
    no_docs: bool,
    runtime_types_only: bool,
    no_default_derives: bool,
    no_default_substitutions: bool,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let mut codegen = CodegenBuilder::new();

    // Use the provided crate path:
    if let Some(crate_path) = crate_path {
        let crate_path =
            syn::parse_str(&crate_path).map_err(|e| eyre!("Cannot parse crate path: {e}"))?;
        codegen.set_subxt_crate_path(crate_path);
    }

    // Respect the boolean flags:
    if runtime_types_only {
        codegen.runtime_types_only()
    }
    if no_default_derives {
        codegen.disable_default_derives()
    }
    if no_default_substitutions {
        codegen.disable_default_substitutes()
    }
    if no_docs {
        codegen.no_docs()
    }

    // Configure derives:
    let global_derives = raw_derives
        .iter()
        .map(|raw| syn::parse_str(raw))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| eyre!("Cannot parse global derives: {e}"))?;
    codegen.set_additional_global_derives(global_derives);

    for d in derives_for_type {
        let ty_str = &d.type_path;
        let ty: syn::TypePath = syn::parse_str(ty_str)
            .map_err(|e| eyre!("Cannot parse derive for type {ty_str}: {e}"))?;
        let derive = syn::parse_str(&d.trait_path)
            .map_err(|e| eyre!("Cannot parse derive for type {ty_str}: {e}"))?;
        // Note: recursive derives and attributes not supported in the CLI => recursive: false
        codegen.add_derives_for_type(ty, std::iter::once(derive), d.recursive);
    }

    // Configure attribtues:
    let universal_attributes = raw_attributes
        .iter()
        .map(|raw| syn::parse_str(raw))
        .map(|attr: syn::Result<OuterAttribute>| attr.map(|attr| attr.0))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| eyre!("Cannot parse global attributes: {e}"))?;
    codegen.set_additional_global_attributes(universal_attributes);

    for a in attributes_for_type {
        let ty_str = &a.type_path;
        let ty = syn::parse_str(ty_str)
            .map_err(|e| eyre!("Cannot parse attribute for type {ty_str}: {e}"))?;
        let attribute: OuterAttribute = syn::parse_str(&a.attribute)
            .map_err(|e| eyre!("Cannot parse attribute for type {ty_str}: {e}"))?;
        // Note: recursive derives and attributes not supported in the CLI => recursive: false
        codegen.add_attributes_for_type(ty, std::iter::once(attribute.0), a.recursive);
    }

    // Insert type substitutions:
    for (from_str, to_str) in substitute_types {
        let from: syn::Path = syn::parse_str(&from_str)
            .map_err(|e| eyre!("Cannot parse type substitution for path {from_str}: {e}"))?;
        let to: syn::Path = syn::parse_str(&to_str)
            .map_err(|e| eyre!("Cannot parse type substitution for path {from_str}: {e}"))?;
        codegen.set_type_substitute(from, to);
    }

    let metadata = subxt_metadata::Metadata::decode(&mut &*metadata_bytes)
        .map_err(|e| eyre!("Cannot decode the provided metadata: {e}"))?;
    let code = codegen
        .generate(metadata)
        .map_err(|e| eyre!("Cannot generate code: {e}"))?;

    writeln!(output, "{code}")?;
    Ok(())
}
