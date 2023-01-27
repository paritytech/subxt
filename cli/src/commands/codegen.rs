// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use std::{
    fs,
    io::Read,
    path::PathBuf,
};
use subxt_codegen::{
    DerivesRegistry,
    TypeSubstitutes,
};

use crate::CliOpts;

/// Generate runtime API client code from metadata.
///
/// # Example (with code formatting)
///
/// `subxt codegen | rustfmt --edition=2018 --emit=stdout`
#[derive(Debug, ClapParser)]
pub struct CodegenOpts {
    /// The path to the encoded metadata file.
    #[clap(short, long, value_parser, conflicts_with = "url")]
    file: Option<PathBuf>,

    /// Additional derives
    #[clap(long = "derive")]
    derives: Vec<String>,

    /// Additional derives for a given type.
    ///
    /// Example `--derive-for-type my_module::my_type=serde::Serialize`.
    #[clap(long = "derive-for-type", value_parser = derive_for_type_parser)]
    derives_for_type: Vec<(String, String)>,

    /// The `subxt` crate access path in the generated code.
    /// Defaults to `::subxt`.
    #[clap(long = "crate")]
    crate_path: Option<String>,
}

fn derive_for_type_parser(src: &str) -> Result<(String, String), String> {
    let (ty, derive) = src
        .split_once('=')
        .ok_or_else(|| String::from("Invalid pattern for `derive-for-type`. It should be `type=derive`, like `my_type=serde::Serialize`"))?;

    Ok((ty.to_string(), derive.to_string()))
}

pub async fn run(opts: &CliOpts, cmd_opts: &CodegenOpts) -> color_eyre::Result<()> {
    let bytes = if let Some(file) = cmd_opts.file.as_ref() {
        let mut file = fs::File::open(file)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        bytes
    } else {
        subxt_codegen::utils::fetch_metadata_bytes(&opts.url).await?
    };

    codegen(
        &bytes,
        cmd_opts.derives.clone(),
        cmd_opts.derives_for_type.clone(),
        cmd_opts.crate_path.clone(),
    )?;
    Ok(())
}

fn codegen(
    metadata_bytes: &[u8],
    raw_derives: Vec<String>,
    derives_for_type: Vec<(String, String)>,
    crate_path: Option<String>,
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

    for (ty, derive) in derives_for_type.into_iter() {
        let ty = syn::parse_str(&ty)?;
        let derive = syn::parse_str(&derive)?;
        derives.extend_for_type(ty, std::iter::once(derive), &crate_path)
    }

    let type_substitutes = TypeSubstitutes::new(&crate_path);

    let runtime_api = subxt_codegen::generate_runtime_api_from_bytes(
        item_mod,
        metadata_bytes,
        derives,
        type_substitutes,
        crate_path,
    );
    println!("{}", runtime_api);
    Ok(())
}
