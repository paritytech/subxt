//! Build

use std::process::Command;

fn main() {
    // We want to be able to link to examples on GitHub specific to the current version.
    // So, output an env var with either the git tag if one points to the current code, or
    // the git hash pointing to the current code if no tag, or master if we
    // encounter some error and can't be more specific.
    if let Ok(tag) = Command::new("git").args(["tag", "--points-at"]).output()
        && tag.status.success()
        && !tag.stdout.is_empty()
    {
        write_to_out(&tag.stdout)
    } else if let Ok(commit_hash) = Command::new("git").args(["rev-parse", "HEAD"]).output()
        && commit_hash.status.success()
        && !commit_hash.stdout.is_empty()
    {
        write_to_out(&commit_hash.stdout)
    } else {
        write_to_out(b"master")
    }
}

fn write_to_out(v: &[u8]) {
    let out = String::from_utf8_lossy(v);
    println!("cargo::rustc-env=SUBXT_REF={out}");
    println!("cargo::rerun-if-env-changed=SUBXT_REF");
}
