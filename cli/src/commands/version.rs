use clap::Parser as ClapParser;

/// Version subcommand
///
/// This subcommand is implemented for backward compatibility reasons.
/// Its behavior should be the same as that of the `--version` flag which
/// is handled internally by clap.
#[derive(Debug, ClapParser)]
pub struct Opts {}

pub fn run(_opts: Opts) {
    println!("subxt-cli {}", clap::crate_version!());
}
