use std::process::Command;

fn main() {
    // Make git hash available via GIT_HASH build-time env var:
    output_git_short_hash();
}

fn output_git_short_hash() {
    let output = Command::new("git")
        .args(["rev-parse", "--short=11", "HEAD"])
        .output()
        .expect("'git' command should exist and run successfully");

    let git_hash =
        String::from_utf8(output.stdout).expect("git hash should be valid UTF8");

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-rerun-if-changed=../.git/HEAD");
    println!("cargo:rustc-rerun-if-changed=build.rs");
}
