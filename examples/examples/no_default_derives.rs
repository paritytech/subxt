// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This example demonstrates how to use custom derives with subxt.
//!
//! Example verified against polkadot polkadot 0.9.25-5174e9ae75b.

#[subxt::subxt(
    runtime_metadata_path = "../artifacts/polkadot_metadata.scale",

    // opt out from the default derives and attributes
    no_default_derives,

    derive_for_all_types = "\
        subxt::ext::scale_encode::EncodeAsType,\
        subxt::ext::scale_decode::DecodeAsType,\
        subxt::ext::codec::Encode,\
        subxt::ext::codec::Decode,\
        Debug",
    attribute_all_types = r#"
        #[codec(crate = subxt::ext::codec)],
        #[encode_as_type(crate_path = "subxt::ext::scale_encode")],
        #[decode_as_type(crate_path = "subxt::ext::scale_decode")]"#,

    derive_for_type(type = "sp_runtime::ModuleError", derive = "Hash"),
    attribute_type(type = "sp_runtime::ModuleError", attribute = "#[allow(clippy::all)]"),
)]
pub mod polkadot {}

use polkadot::runtime_types::frame_support::PalletId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = PalletId([1u8; 8]);
    Ok(())
}
