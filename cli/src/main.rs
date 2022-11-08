// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![deny(unused_crate_dependencies)]

mod commands;
use clap::Parser as ClapParser;

/// Subxt utilities for interacting with Substrate based nodes.
#[derive(Debug, ClapParser)]
enum Command {
    Metadata(commands::metadata::Opts),
    Codegen(commands::codegen::Opts),
    Compatibility(commands::compatibility::Opts),
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Command::parse();

    match args {
        Command::Metadata(opts) => commands::metadata::run(opts).await,
        Command::Codegen(opts) => commands::codegen::run(opts).await,
        Command::Compatibility(opts) => commands::compatibility::run(opts).await,
    }
}
