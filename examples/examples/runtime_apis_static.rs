use sp_keyring::AccountKeyring;
use subxt::{config::PolkadotConfig, OnlineClient};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Create a runtime API payload that calls into
    // `AccountNonceApi_account_nonce` function.
    let account = AccountKeyring::Alice.to_account_id().into();
    let runtime_api_call = polkadot::apis().account_nonce_api().account_nonce(account);

    // Submit the call and get back a result.
    let nonce = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await;

    println!("AccountNonceApi_account_nonce for Alice: {:?}", nonce);
    Ok(())
}
