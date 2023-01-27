use clap::Parser as ClapParser;

use crate::CliOpts;

/// Prints version information
#[derive(Debug, ClapParser)]
pub struct Version {}

pub fn run(_opts: &CliOpts, _cmd_opts: &Version) -> color_eyre::Result<()> {
    let git_hash = env!("GIT_HASH");
    let name = clap::crate_name!();
    let version = clap::crate_version!();
    println!("{name} {version}-{git_hash}");
    Ok(())
}
