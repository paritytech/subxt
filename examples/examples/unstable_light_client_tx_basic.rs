use sp_keyring::AccountKeyring;
use std::sync::Arc;
use subxt::{rpc::LightClientBuilder, tx::PairSigner, OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a light client by fetching the chain spec of a local running node.
    // In this case, because we start one single node, the bootnodes must be overwritten
    // for the light client to connect to the local node.
    let light_client = LightClientBuilder::new()
        .trusted_url("ws://127.0.0.1:9944")
        .bootnodes(
            ["/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"]
                .into_iter(),
        )
        .build()
        .await?;
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(Arc::new(light_client)).await?;

    // Build a balance transfer extrinsic.
    let dest = AccountKeyring::Bob.to_account_id().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 10_000);

    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = PairSigner::new(AccountKeyring::Alice.pair());
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Find a Transfer event and print it.
    let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }

    Ok(())
}
