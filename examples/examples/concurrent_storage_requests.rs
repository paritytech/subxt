use futures::join;
use sp_keyring::AccountKeyring;
use subxt::{
    PolkadotExtrinsicParams,
    ClientBuilder,
    DefaultConfig,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let addr = AccountKeyring::Bob.to_account_id().into();

    // For storage requests, we can join futures together to
    // await multiple futures concurrently:
    let controller = api.storage().staking().bonded(&addr, None);
    let ledger = api.storage().staking().ledger(&addr, None);
    let (a, b) = join!(controller, ledger);

    println!("{a:?}, {b:?}");

    Ok(())
}