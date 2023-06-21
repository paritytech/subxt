// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![deny(unused_crate_dependencies)]

mod commands;
mod utils;

use clap::Parser as ClapParser;

/// Subxt utilities for interacting with Substrate based nodes.
#[derive(Debug, ClapParser)]
enum Command {
    Metadata(commands::metadata::Opts),
    Codegen(commands::codegen::Opts),
    Compatibility(commands::compatibility::Opts),
    Diff(commands::diff::Opts),
    Version(commands::version::Opts),
    Explore(commands::explore::Opts),
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Command::parse();
    let mut output = std::io::stdout();
    match args {
        Command::Metadata(opts) => commands::metadata::run(opts, &mut output).await,
        Command::Codegen(opts) => commands::codegen::run(opts, &mut output).await,
        Command::Compatibility(opts) => commands::compatibility::run(opts, &mut output).await,
        Command::Diff(opts) => commands::diff::run(opts, &mut output).await,
        Command::Version(opts) => commands::version::run(opts, &mut output),
        Command::Explore(opts) => commands::explore::run(opts, &mut output).await,
    }
}
