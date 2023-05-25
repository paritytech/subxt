// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::utils::FileOrUrl;
use clap::Parser as ClapParser;
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use subxt_codegen::{DerivesRegistry, TypeSubstitutes, TypeSubstitutionError};

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
    /// Example `--derive-for-type my_module::my_type=serde::Serialize`.
    #[clap(long = "derive-for-type", value_parser = derive_for_type_parser)]
    derives_for_type: Vec<(String, String)>,
    /// Additional attributes for a given type.
    ///
    /// Example `--attributes-for-type my_module::my_type=#[allow(clippy::all)]`.
    #[clap(long = "attributes-for-type", value_parser = attributes_for_type_parser)]
    attributes_for_type: Vec<(String, String)>,
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
}

fn derive_for_type_parser(src: &str) -> Result<(String, String), String> {
    let (ty, derive) = src
        .split_once('=')
        .ok_or_else(|| String::from("Invalid pattern for `derive-for-type`. It should be `type=derive`, like `my_type=serde::Serialize`"))?;

    Ok((ty.to_string(), derive.to_string()))
}

fn attributes_for_type_parser(src: &str) -> Result<(String, String), String> {
    let (ty, attribute) = src
        .split_once('=')
        .ok_or_else(|| String::from("Invalid pattern for `attribute-type`. It should be `type=attribute`, like `my_type=serde::#[allow(clippy::all)]`"))?;

    Ok((ty.to_string(), attribute.to_string()))
}

fn substitute_type_parser(src: &str) -> Result<(String, String), String> {
    let (from, to) = src
        .split_once('=')
        .ok_or_else(|| String::from("Invalid pattern for `substitute-type`. It should be something like `input::Type<A>=replacement::Type<A>`"))?;

    Ok((from.to_string(), to.to_string()))
}

pub async fn run(opts: Opts, output: &mut impl std::io::Write) -> color_eyre::Result<()> {
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
    derives_for_type: Vec<(String, String)>,
    attributes_for_type: Vec<(String, String)>,
    substitute_types: Vec<(String, String)>,
    crate_path: Option<String>,
    no_docs: bool,
    runtime_types_only: bool,
    no_default_derives: bool,
    no_default_substitutions: bool,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let item_mod = syn::parse_quote!(
        pub mod api {}
    );

    let universal_derives = raw_derives
        .iter()
        .map(|raw| syn::parse_str(raw))
        .collect::<Result<Vec<_>, _>>()?;
    let universal_attributes = raw_attributes
        .iter()
        .map(|raw| syn::parse_str(raw))
        .map(|attr: syn::Result<OuterAttribute>| attr.map(|attr| attr.0))
        .collect::<Result<Vec<_>, _>>()?;

    let crate_path = crate_path.map(Into::into).unwrap_or_default();
    let mut derives = if no_default_derives {
        DerivesRegistry::new()
    } else {
        DerivesRegistry::with_default_derives(&crate_path)
    };
    derives.extend_for_all(universal_derives, universal_attributes);

    for (ty, derive) in derives_for_type {
        let ty = syn::parse_str(&ty)?;
        let derive = syn::parse_str(&derive)?;
        derives.extend_for_type(ty, std::iter::once(derive), vec![]);
    }
    for (ty, attr) in attributes_for_type {
        let ty = syn::parse_str(&ty)?;
        let attribute: OuterAttribute = syn::parse_str(&attr)?;
        derives.extend_for_type(ty, vec![], std::iter::once(attribute.0));
    }

    let mut type_substitutes = if no_default_substitutions {
        TypeSubstitutes::new()
    } else {
        TypeSubstitutes::with_default_substitutes(&crate_path)
    };

    for (from_str, to_str) in substitute_types {
        let from: syn::Path = syn::parse_str(&from_str)?;
        let to: syn::Path = syn::parse_str(&to_str)?;
        let to = to.try_into().map_err(|e: TypeSubstitutionError| {
            eyre::eyre!("Cannot parse substitute '{from_str}={to_str}': {e}")
        })?;
        type_substitutes
            .insert(from, to)
            .map_err(|e: TypeSubstitutionError| {
                eyre::eyre!("Cannot parse substitute '{from_str}={to_str}': {e}")
            })?;
    }

    let should_gen_docs = !no_docs;
    let runtime_api = subxt_codegen::generate_runtime_api_from_bytes(
        item_mod,
        metadata_bytes,
        derives,
        type_substitutes,
        crate_path,
        should_gen_docs,
        runtime_types_only,
    )
    .map_err(|code_gen_err| eyre!("{code_gen_err}"))?;
    writeln!(output, "{runtime_api}")?;
    Ok(())
}
