#![allow(missing_docs)]
use subxt::utils::fetch_chainspec_from_rpc_node;
use subxt::{
    client::OnlineClient,
    lightclient::{ChainConfig, LightClient},
    PolkadotConfig,
};
use subxt_signer::sr25519::dev;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The smoldot logs are informative:
    tracing_subscriber::fmt::init();

    // Use a utility function to obtain a chain spec from a locally running node:
    let chain_spec = fetch_chainspec_from_rpc_node("ws://127.0.0.1:9944").await?;

    // Configure the bootnodes of this chain spec. In this case, because we start one
    // single node, the bootnodes must be overwritten for the light client to connect
    // to the local node.
    //
    // The `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp` is the P2P address
    // from a local polkadot node starting with
    // `--node-key 0000000000000000000000000000000000000000000000000000000000000001`
    let chain_config = ChainConfig::chain_spec(chain_spec.get()).set_bootnodes([
        "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
    ])?;

    // Start the light client up, establishing a connection to the local node.
    let (_light_client, chain_rpc) = LightClient::relay_chain(chain_config)?;
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(chain_rpc).await?;

    // Build a balance transfer extrinsic.
    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer_allow_death(dest, 10_000);

    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = dev::alice();
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
