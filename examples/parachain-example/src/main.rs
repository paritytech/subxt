use subxt::{
    PolkadotConfig,
    utils::{AccountId32, MultiAddress},
    OnlineClient,
};
use subxt_signer::sr25519::dev::{self};

#[subxt::subxt(runtime_metadata_path = "statemint_metadata.scale")]
pub mod statemint {}

// PolkadotConfig or SubstrateConfig will suffice for this example at the moment,
// but PolkadotConfig is a little more correct, having the right `Address` type.
type StatemintConfig = PolkadotConfig;

#[tokio::main]
pub async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // (the port 42069 is specified in the asset-hub-zombienet.toml)
    let api = OnlineClient::<StatemintConfig>::from_url("ws://127.0.0.1:42069").await?;
    println!("Connection with parachain established.");

    let alice: MultiAddress<AccountId32, ()> = dev::alice().public_key().into();
    let alice_pair_signer = dev::alice();

    const COLLECTION_ID: u32 = 12;
    const NTF_ID: u32 = 234;

    // create a collection with id `12`
    let collection_creation_tx = statemint::tx()
        .uniques()
        .create(COLLECTION_ID, alice.clone());
    let _collection_creation_events = api
        .tx()
        .sign_and_submit_then_watch_default(&collection_creation_tx, &alice_pair_signer)
        .await
        .map(|e| {
            println!("Collection creation submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;
    println!("Collection created.");

    // create an nft in that collection with id `234`
    let nft_creation_tx = statemint::tx()
        .uniques()
        .mint(COLLECTION_ID, NTF_ID, alice.clone());
    let _nft_creation_events = api
        .tx()
        .sign_and_submit_then_watch_default(&nft_creation_tx, &alice_pair_signer)
        .await
        .map(|e| {
            println!("NFT creation submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;
    println!("NFT created.");

    // check in storage, that alice is the official owner of the NFT:
    let nft_owner_storage_query = statemint::storage().uniques().asset();
    let nft_storage_details = api
        .storage()
        .at_latest()
        .await?
        .fetch(nft_owner_storage_query, (COLLECTION_ID, NTF_ID))
        .await?
        .decode()?;

    // make sure that alice is the owner of the NFT:
    assert_eq!(nft_storage_details.owner, dev::alice().public_key().into());
    println!("Storage Item Details: {:?}", nft_storage_details);

    Ok(())
}
