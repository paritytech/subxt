use std::{borrow::Cow, process::Command};

fn main() {
    // Make git hash available via GIT_HASH build-time env var:
    output_git_short_hash();
}

fn output_git_short_hash() {
    let output = Command::new("git")
        .args(["rev-parse", "--short=11", "HEAD"])
        .output();

    let git_hash = match output {
        Ok(o) if o.status.success() => {
            let sha = String::from_utf8_lossy(&o.stdout).trim().to_owned();
            Cow::from(sha)
        }
        Ok(o) => {
            println!("cargo:warning=Git command failed with status: {}", o.status);
            Cow::from("unknown")
        }
        Err(err) => {
            println!("cargo:warning=Failed to execute git command: {err}");
            Cow::from("unknown")
        }
    };

    println!("cargo:rustc-env=GIT_HASH={git_hash}");
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/refs");
    println!("cargo:rerun-if-changed=build.rs");
}
