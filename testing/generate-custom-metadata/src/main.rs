// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::Encode;
use std::io::{self, Write};

/// Creates some scale encoded metadata with custom values and writes it out to stdout (as raw bytes)
/// 
/// Can be called from the root of the project with: `cargo run --bin generate-custom-metadata > output.scale`.
fn main() -> io::Result<()> {
    let metadata_prefixed = generate_custom_metadata::metadata_custom_values_foo();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&metadata_prefixed.encode())?;
    Ok(())
}
