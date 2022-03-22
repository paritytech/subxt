// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use structopt::StructOpt;

/// Utilities for validating metadata compatibility between multiple nodes.
#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Verify metadata compatibility between substrate nodes.
    Compatibility {
        /// Urls of the substrate nodes to verify for metadata compatibility.
        #[structopt(name = "nodes", long, use_delimiter = true, parse(try_from_str))]
        nodes: Vec<url::Url>,
    },
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Opts::from_args();

    match args.command {
        Command::Compatibility { nodes } => {
            println!("Nodes: {:#?}", nodes);
        }
    }
    Ok(())
}
