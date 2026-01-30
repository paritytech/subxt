//! Calling Runtime APIs
use subxt::{Error, OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let at_block = api.at_current_block().await?;

    // Runtime version:
    let version_payload = polkadot::runtime_apis().core().version();
    let version = at_block.runtime_apis().call(version_payload).await?;
    println!("Version: {version:?}");

    // Account nonce for some account:
    let account_nonce_payload = polkadot::runtime_apis()
        .account_nonce_api()
        .account_nonce(dev::alice().public_key().to_account_id());
    let account_nonce = at_block.runtime_apis().call(account_nonce_payload).await?;
    println!("Account nonce: {account_nonce}");

    Ok(())
}
