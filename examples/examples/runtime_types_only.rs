// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! In some cases we are interested only in the `RuntimeCall` enum (or more generally, only in some
//! runtime types). We can ask `subxt` to generate only runtime types by passing a corresponding
//! flag.
//!
//! Here we present how to correctly create `Block` type for the Polkadot chain.

use sp_core::H256;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, Block as _, Header as _},
    Digest,
};
use subxt::PolkadotConfig;

#[subxt::subxt(
    runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
    derive_for_all_types = "Clone, PartialEq, Eq",
    runtime_types_only
)]
pub mod polkadot {}

type RuntimeCall = polkadot::runtime_types::polkadot_runtime::RuntimeCall;

type UncheckedExtrinsic = generic::UncheckedExtrinsic<
    <PolkadotConfig as subxt::Config>::Address,
    RuntimeCall,
    <PolkadotConfig as subxt::Config>::Signature,
    // Usually we are not interested in `SignedExtra`.
    (),
>;

type Header = generic::Header<u32, BlakeTwo256>;
type Block = generic::Block<Header, UncheckedExtrinsic>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Although we could build an online client, we do not have access to the full runtime API. For
    // that, we would have to specify `runtime_types_only = false` (or just skipping it).
    //
    // let api = subxt::OnlineClient::<PolkadotConfig>::new().await?;
    // let address = polkadot::constants().balances().existential_deposit(); <- this won't compile!

    let polkadot_header = Header::new(
        41,
        H256::default(),
        H256::default(),
        H256::default(),
        Digest::default(),
    );

    let polkadot_block = Block::new(polkadot_header, vec![]);

    println!("{polkadot_block:?}");

    Ok(())
}
