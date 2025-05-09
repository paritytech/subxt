//! Example to use subxt to talk to substrate-based nodes with ethereum accounts
//! which is not the default for subxt which is why we need to provide a custom config.
//!
//! This example requires to run a local frontier/moonbeam node to work.

#![allow(missing_docs)]

use subxt::OnlineClient;
use subxt_core::utils::AccountId20;
use subxt_signer::eth::{dev, Signature};

#[subxt::subxt(runtime_metadata_path = "../artifacts/frontier_metadata_small.scale")]
mod eth_runtime {}

enum EthRuntimeConfig {}

impl subxt::Config for EthRuntimeConfig {
    type AccountId = AccountId20;
    type Address = AccountId20;
    type Signature = Signature;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type Header =
        subxt::config::substrate::SubstrateHeader<u32, subxt::config::substrate::BlakeTwo256>;
    type ExtrinsicParams = subxt::config::SubstrateExtrinsicParams<Self>;
    type AssetId = u32;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<EthRuntimeConfig>::from_insecure_url("ws://127.0.0.1:9944").await?;

    let alith = dev::alith();
    let baltathar = dev::baltathar();
    let dest = baltathar.public_key().to_account_id();

    println!("baltathar pub:  {}", hex::encode(baltathar.public_key().0));
    println!("baltathar addr: {}", hex::encode(dest));

    let balance_transfer_tx = eth_runtime::tx()
        .balances()
        .transfer_allow_death(dest, 10_001);

    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &alith)
        .await?
        .wait_for_finalized_success()
        .await?;

    let transfer_event = events.find_first::<eth_runtime::balances::events::Transfer>()?;
    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }

    Ok(())
}
