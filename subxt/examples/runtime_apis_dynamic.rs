#![allow(missing_docs)]
use subxt::{OnlineClient, config::PolkadotConfig};
use subxt_signer::sr25519::dev;
use subxt::utils::AccountId32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Create a "dynamic" runtime API payload that calls the
    // `AccountNonceApi_account_nonce` function. We could use the
    // `scale_value::Value` type as output, and a vec of those as inputs,
    // but since we know the input + return types we can pass them directly.
    // There is one input argument, so the inputs are a tuple of one element.
    let account: AccountId32 = dev::alice().public_key().into();
    let runtime_api_call = subxt::dynamic::runtime_api_call::<_, u64>(
        "AccountNonceApi",
        "account_nonce",
        (account,),
    );

    // Submit the call to get back a result.
    let nonce = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await?;

    println!("Account nonce: {:#?}", nonce);
    Ok(())
}
