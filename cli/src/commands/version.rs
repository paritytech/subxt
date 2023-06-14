use clap::Parser as ClapParser;

/// Prints version information
#[derive(Debug, ClapParser)]
pub struct Opts {}

pub fn run(_opts: Opts, output: &mut impl std::io::Write) -> color_eyre::Result<()> {
    let git_hash = env!("GIT_HASH");
    writeln!(
        output,
        "{} {}-{}",
        clap::crate_name!(),
        clap::crate_version!(),
        git_hash
    )?;
    Ok(())
}
