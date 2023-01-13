use std::process::Command;

use clap::Parser as ClapParser;

/// Prints version information
///
/// This subcommand is implemented for backward compatibility reasons.
/// Its behavior should be the same as that of the `--version` flag which
/// is handled internally by clap.
#[derive(Debug, ClapParser)]
pub struct Opts {}

pub fn run(_opts: Opts) {
    let commit = match Command::new("git")
        .args(&["rev-parse", "--short=11", "HEAD"])
        .output()
    {
        Ok(o) if o.status.success() => {
            let sha = String::from_utf8_lossy(&o.stdout).trim().to_owned();
            sha
        }
        Ok(o) => {
            println!("cargo:warning=Git command failed with status: {}", o.status);
            "unknown".to_string()
        }
        Err(err) => {
            println!("cargo:warning=Failed to execute git command: {}", err);
            "unknown".to_string()
        }
    };

    println!("{} {}", clap::crate_name!(), get_version(&commit));
}

fn get_version(impl_commit: &str) -> String {
    let commit_dash = if impl_commit.is_empty() { "" } else { "-" };

    format!("{}{}{}", clap::crate_version!(), commit_dash, impl_commit)
}
