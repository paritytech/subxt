//! In this example we want to submit a transaction to the asset hub parachain.
//! Doing so on the official polkadot network, would require us to have an account with sufficient funds.
//! That is why we spawn a local asset hub parachain instead, that comes with pre-funded dev accounts.
//!
//! # Setting up the local parachain
//!
//! We use [zombienet](https://github.com/paritytech/zombienet) to start up a local asset hub.
//!
//! ## 1. Install necessary tools
//!
//! To setup the local parachain we need to have 3 binaries installed in our path:
//!
//! ### 1. `zombienet`
//!
//! Zombienet is a tool for quickly spinning up a (local) blockchain network. Please follow the install guide in the [zombienet github repo](https://github.com/paritytech/zombienet).
//!
//! ### 2. `polkadot`
//!
//! Build the polkadot binary from the [polkadot github repo](https://github.com/paritytech/polkadot) and install it in your path:
//! ```txt
//! git clone https://github.com/paritytech/polkadot.git
//! cd polkadot
//! cargo build --release
//! cargo install --path .
//! ```
//!
//! ### 3. `polkadot-parachain`
//!
//! The polkadot asset hub is part of the [cumulus github repo](https://github.com/paritytech/cumulus), an SDK for developing parachains.
//! Building the cumulus workspace produces a binary called `polkadot-parachain` that has the capability and configuration data to run the asset hub.
//! ```txt
//! git clone https://github.com/paritytech/cumulus.git
//! cd cumulus
//! cargo build --release
//! cargo install --path .
//! ```
//!
//! ## 2. Run the parachain locally
//!
//! Zombienet can now spawn the parachain locally from a configuration file, `asset-hub-zombienet.toml` in this case.
//! We need to have at least 2 validator nodes running (via the `polkadot` binary),
//! while the `polkadot-parachain` binary starts the asset hub parachain and gets it registered with the validator nodes.
//! To do that, run:
//! ```txt
//! zombienet -p native spawn asset-hub-zombienet.toml
//! ```
//! Zombienet uses Kubernetes by default, but we can use it without Kubernetes, by providing the `-p native` flag.
//!
//! You might have notices, that we use `chain = "rococo-local"` in the `asset-hub-zombienet.toml` file for the relay chain.
//! This is just to make the epoch time shorter and should have no effect on your interactions with the parachain.
//! Polkadot / Kusama / Rococo have different epoch times of `24h` / `2h` / `2min` respectively.
//! The parachain is only registered after the first epoch. So we need to wait 2 minutes, until the parachain becomes interactive and produces blocks.
//!
//!
//! # Running the example
//!
//! After you have the network running, you should see something like [https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:41955#/explorer]
//! as the polkadot.js link to the parachain (e.g. "collator01") in the zombienet output. Here `41955` refers to the port,
//! which we will need shortly.
//!
//! In this example we use the _uniques_ pallet of the polakdot asset hub parachain.
//! The dev account _alice_ creates an NFT collection, then mints an NFT that is part of the collection.
//! Beware that this example can take up to a minute to run. Specify the port like so:
//! ```txt
//! cargo run --bin submit_transaction <PORT>
//! ```
//!

use metadata::statemint;
use parachain_example::StatemintConfig3;
use subxt::{
    utils::{AccountId32, MultiAddress},
    OnlineClient,
};

use subxt_signer::sr25519::dev::{self};

/// cargo run --bin submit_transaction <PORT>
#[tokio::main]
pub async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::args().nth(1).expect("port must be specified");
    // Connecting to the local parachain. You might need to replace the port in `"ws://127.0.0.1:36317"`
    // with a different value, depending on what your zombienet cli outputs to you.
    let api =
        OnlineClient::<StatemintConfig3>::from_url(format!("ws://127.0.0.1:{}", port)).await?;
    println!("Connection with parachain established.");
    let alice: MultiAddress<AccountId32, ()> = dev::alice().public_key().into();
    let alice_pair_signer = dev::alice();

    const COLLECTION_ID: u32 = 12;
    const NTF_ID: u32 = 234;

    // create a collection with id `42`
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

    // create an nft in that collection with id `420`
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
    let nft_owner_storage_query = statemint::storage().uniques().asset(COLLECTION_ID, NTF_ID);
    let nft_storage_details = api
        .storage()
        .at_latest()
        .await?
        .fetch(&nft_owner_storage_query)
        .await?
        .ok_or("The NFT should have an owner (alice)")?;

    // make sure that alice is the owner of the NFT:
    assert_eq!(nft_storage_details.owner, dev::alice().public_key().into());
    println!("Storage Item Details: {:?}", nft_storage_details);

    Ok(())
}
