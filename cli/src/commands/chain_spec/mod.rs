// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use std::{io::Write, path::PathBuf};
use subxt_codegen::fetch_metadata::Url;

mod fetch;

/// Download chainSpec from a substrate node.
#[derive(Debug, ClapParser)]
pub struct Opts {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(long)]
    url: Url,
    /// Write the output of the command to the provided file path.
    #[clap(long, short, value_parser)]
    pub output_file: Option<PathBuf>,
}

pub async fn run(opts: Opts, output: &mut impl Write) -> color_eyre::Result<()> {
    let url = opts.url;

    let spec = fetch::fetch_chain_spec(url).await?;

    let mut output: Box<dyn Write> = match opts.output_file {
        Some(path) => Box::new(std::fs::File::create(path)?),
        None => Box::new(output),
    };

    let json = serde_json::to_string_pretty(&spec)?;
    write!(output, "{json}")?;
    Ok(())
}
