// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![deny(unused_crate_dependencies)]

mod commands;
use clap::{
    crate_authors,
    crate_version,
    ColorChoice,
    Parser as ClapParser,
    Parser,
};
use subxt_codegen::utils::Uri;

/// Subxt utilities for interacting with Substrate based nodes.
#[derive(Debug, ClapParser)]
pub enum SubCommand {
    Metadata(commands::metadata::MetadataOpts),
    Codegen(commands::codegen::CodegenOpts),
    Compatibility(commands::compatibility::CompatOpts),
    Version(commands::version::Version),
}

#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = crate_authors!(), color=ColorChoice::Always)]
pub struct CliOpts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,

    /// The url of the substrate node connect to
    #[clap(
        name = "url",
        long,
        value_parser,
        default_value = "http://localhost:9933",
        env = "SUBXT_URL"
    )]
    url: Uri,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = CliOpts::parse();

    match &opts.subcmd {
        SubCommand::Metadata(cmd_opts) => commands::metadata::run(&opts, cmd_opts).await,
        SubCommand::Codegen(cmd_opts) => commands::codegen::run(&opts, cmd_opts).await,
        SubCommand::Compatibility(cmd_opts) => {
            commands::compatibility::run(&opts, cmd_opts).await
        }
        SubCommand::Version(cmd_opts) => commands::version::run(&opts, cmd_opts),
    }
}
