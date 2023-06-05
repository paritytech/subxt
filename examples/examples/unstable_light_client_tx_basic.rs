use futures::StreamExt;
use sp_keyring::AccountKeyring;
use std::sync::Arc;
use subxt::{rpc::LightClient, tx::PairSigner, OnlineClient, PolkadotConfig};

// // Generate an interface that we can use from the node's metadata.
// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
// pub mod polkadot {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a light client from the provided chain spec.
    //
    // # Note
    //
    // This example uses a local running polkadot node and the
    // provided spec might differ depending on the version used.
    //
    // The spec can be generated in the following manner:
    // - start the polkadot node:
    //
    // `./polkadot --dev --node-key 0000000000000000000000000000000000000000000000000000000000000001  --alice --validator`
    //
    // - fetch the spec
    //
    // ```bash
    // curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "sync_state_genSyncSpec", "params":[true]}' http://localhost:9944/ | jq .result > res.spec
    // ```
    //
    // - remove the `lightSyncState` entry from the spec
    //
    // - add the boot nodes entry to the spec
    //
    // ```json
    //   "bootNodes": [
    //       "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
    //    ],
    //
    // Where `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp` should be replaced with the
    // polkadot nodes identity extracted from the process's logs:
    //
    // ```bash
    // 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
    // 👶 Creating empty BABE epoch changes on what appears to be first startup.
    // 🏷 Local node identity is: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
    // ```
    let light_client = LightClient::new(include_str!("../../artifacts/dev_spec.json"))?;
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(Arc::new(light_client)).await?;

    {
        println!("Subscribe to latest finalized blocks: ");

        let mut blocks_sub = api.blocks().subscribe_finalized().await?.take(3);
        // For each block, print a bunch of information about it:
        while let Some(block) = blocks_sub.next().await {
            let block = block?;

            let block_number = block.header().number;
            let block_hash = block.hash();

            println!("Block #{block_number}:");
            println!("  Hash: {block_hash}");
        }
    }

    // Build a balance transfer extrinsic.
    // let dest = AccountKeyring::Bob.to_account_id().into();
    // let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 10_000);

    // // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // // and in a finalized block. We get back the extrinsic events if all is well.
    // let from = PairSigner::new(AccountKeyring::Alice.pair());
    // let events = api
    //     .tx()
    //     .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
    //     .await?
    //     .wait_for_finalized_success()
    //     .await?;

    // // Find a Transfer event and print it.
    // let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    // if let Some(event) = transfer_event {
    //     println!("Balance transfer success: {event:?}");
    // }

    Ok(())
}
