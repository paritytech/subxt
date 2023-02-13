// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use color_eyre::eyre;
use jsonrpsee::client_transport::ws::Uri;
use std::{
    fs,
    io::Read,
    path::PathBuf,
};
use subxt_codegen::{
    DerivesRegistry,
    TypeSubstitutes,
};

/// Generate runtime API client code from metadata.
///
/// # Example (with code formatting)
///
/// `subxt codegen | rustfmt --edition=2018 --emit=stdout`
#[derive(Debug, ClapParser)]
pub struct Opts {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(flatten)]
    metadata_opts: MetadataOpts,
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

#[derive(Debug, Clone, clap::Args)]
pub struct MetadataOpts {
    /// The url of the substrate node to query for metadata.
    #[clap(name = "url", long, value_parser, env = "SUBXT_METADATA_URL", conflicts_with = "file")]
    url: Option<Uri>,
    /// The path to the encoded metadata file.
    #[clap(short, long, value_parser, env = "SUBXT_METADATA_PATH", conflicts_with = "url")]
    file: Option<PathBuf>,
}


fn derive_for_type_parser(src: &str) -> Result<(String, String), String> {
    let (ty, derive) = src
        .split_once('=')
        .ok_or_else(|| String::from("Invalid pattern for `derive-for-type`. It should be `type=derive`, like `my_type=serde::Serialize`"))?;

    Ok((ty.to_string(), derive.to_string()))
}

pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    print!("{opts:?}");
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
    println!("{runtime_api}");
    Ok(())
}
