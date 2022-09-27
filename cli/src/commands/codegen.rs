// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use color_eyre::eyre;
use frame_metadata::RuntimeMetadataPrefixed;
use jsonrpsee::client_transport::ws::Uri;
use scale::{
    Decode,
    Input,
};
use std::{
    fs,
    io::Read,
    path::PathBuf,
};
use subxt_codegen::DerivesRegistry;

/// Generate runtime API client code from metadata.
///
/// # Example (with code formatting)
///
/// `subxt codegen | rustfmt --edition=2018 --emit=stdout`
#[derive(Debug, ClapParser)]
pub struct Opts {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(name = "url", long, parse(try_from_str))]
    url: Option<Uri>,
    /// The path to the encoded metadata file.
    #[clap(short, long, parse(from_os_str))]
    file: Option<PathBuf>,
    /// Additional derives
    #[clap(long = "derive")]
    derives: Vec<String>,
    /// The `subxt` crate access path in the generated code.
    /// Defaults to `::subxt`.
    #[clap(long = "crate")]
    crate_path: Option<String>,
}

pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    if let Some(file) = opts.file.as_ref() {
        if opts.url.is_some() {
            eyre::bail!("specify one of `--url` or `--file` but not both")
        };

        let mut file = fs::File::open(file)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        codegen(&mut &bytes[..], opts.derives, opts.crate_path)?;
        return Ok(())
    }

    let url = opts.url.unwrap_or_else(|| {
        "http://localhost:9933"
            .parse::<Uri>()
            .expect("default url is valid")
    });
    let (_, bytes) = super::metadata::fetch_metadata(&url).await?;
    codegen(&mut &bytes[..], opts.derives, opts.crate_path)?;
    Ok(())
}

fn codegen<I: Input>(
    encoded: &mut I,
    raw_derives: Vec<String>,
    crate_path: Option<String>,
) -> color_eyre::Result<()> {
    let metadata = <RuntimeMetadataPrefixed as Decode>::decode(encoded)?;
    let generator = subxt_codegen::RuntimeGenerator::new(metadata);
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

    let runtime_api = generator.generate_runtime(item_mod, derives, crate_path);
    println!("{}", runtime_api);
    Ok(())
}
