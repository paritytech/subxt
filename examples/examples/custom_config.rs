// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This example should compile but should aos fail to work, since we've modified the
//! config to not align with a Polkadot node.

use sp_keyring::AccountKeyring;
use subxt::{
    config::{
        Config,
        SubstrateConfig,
    },
    tx::{
        PairSigner,
        SubstrateExtrinsicParams,
    },
    OnlineClient,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

/// Custom [`Config`] impl where the default types for the target chain differ from the
/// [`DefaultConfig`]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MyConfig;
impl Config for MyConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u64;
    type BlockNumber = <SubstrateConfig as Config>::BlockNumber;
    type Hash = <SubstrateConfig as Config>::Hash;
    type Hashing = <SubstrateConfig as Config>::Hashing;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Header = <SubstrateConfig as Config>::Header;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Extrinsic = <SubstrateConfig as Config>::Extrinsic;
    // ExtrinsicParams makes use of the index type, so we need to adjust it
    // too to align with our modified index type, above:
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    // Create a client to use:
    let api = OnlineClient::<MyConfig>::new().await?;

    // Create a transaction to submit:
    let tx = polkadot::tx()
        .balances()
        .transfer(dest, 123_456_789_012_345);

    // submit the transaction with default params:
    let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}
