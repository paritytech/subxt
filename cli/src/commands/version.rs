use clap::Parser as ClapParser;

/// Prints version information
#[derive(Debug, ClapParser)]
pub struct Opts {}

pub fn run(_opts: Opts) -> color_eyre::Result<()> {
    let git_hash = env!("GIT_HASH");
    println!(
        "{} {}-{}",
        clap::crate_name!(),
        clap::crate_version!(),
        git_hash
    );
    Ok(())
}
