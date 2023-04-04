// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::utils::FileOrUrl;
use clap::Parser as ClapParser;
use color_eyre::eyre;
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
    /// Additional derives for a given type.
    ///
    /// Example `--derive-for-type my_module::my_type=serde::Serialize`.
    #[clap(long = "derive-for-type", value_parser = derive_for_type_parser)]
    derives_for_type: Vec<(String, String)>,
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
    #[clap(long)]
    runtime_types_only: bool,
}

fn derive_for_type_parser(src: &str) -> Result<(String, String), String> {
    let (ty, derive) = src
        .split_once('=')
        .ok_or_else(|| String::from("Invalid pattern for `derive-for-type`. It should be `type=derive`, like `my_type=serde::Serialize`"))?;

    Ok((ty.to_string(), derive.to_string()))
}

fn substitute_type_parser(src: &str) -> Result<(String, String), String> {
    let (from, to) = src
        .split_once('=')
        .ok_or_else(|| String::from("Invalid pattern for `substitute-type`. It should be something like `input::Type<A>=replacement::Type<A>`"))?;

    Ok((from.to_string(), to.to_string()))
}

pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    let bytes = opts.file_or_url.fetch().await?;

    codegen(
        &bytes,
        opts.derives,
        opts.derives_for_type,
        opts.substitute_types,
        opts.crate_path,
        opts.no_docs,
        opts.runtime_types_only,
    )?;
    Ok(())
}

fn codegen(
    metadata_bytes: &[u8],
    raw_derives: Vec<String>,
    derives_for_type: Vec<(String, String)>,
    substitute_types: Vec<(String, String)>,
    crate_path: Option<String>,
    no_docs: bool,
    runtime_types_only: bool,
) -> color_eyre::Result<()> {
    let item_mod = syn::parse_quote!(
        pub mod api {}
    );

    let p = raw_derives
        .iter()
        .map(|raw| syn::parse_str(raw))
        .collect::<Result<Vec<_>, _>>()?;

    let crate_path = crate_path.map(Into::into).unwrap_or_default();
    let mut derives = DerivesRegistry::new(&crate_path);
    derives.extend_for_all(p.into_iter());

    for (ty, derive) in derives_for_type {
        let ty = syn::parse_str(&ty)?;
        let derive = syn::parse_str(&derive)?;
        derives.extend_for_type(ty, std::iter::once(derive), &crate_path);
    }

    let mut type_substitutes = TypeSubstitutes::new(&crate_path);
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
    );
    match runtime_api {
        Ok(runtime_api) => println!("{runtime_api}"),
        Err(e) => {
            // Print the error directly to avoid implementing `Send + Sync` on `CodegenError`.
            use color_eyre::owo_colors::OwoColorize;
            println!("{}", e.to_string().red())
        }
    };

    Ok(())
}
