//! Example for how to use the Light client and connect to a relay+parachain.
//!
//! Run: `cargo run --example light_client_parachains --features="unstable-light-client native"`.

#![allow(missing_docs)]
use sp_core::crypto::{AccountId32, Ss58Codec};
use sp_core::ByteArray;
use std::collections::BTreeSet;
use std::{iter, num::NonZeroU32};
use subxt::{
    client::{LightClient, RawLightClient},
    PolkadotConfig,
};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[subxt::subxt(runtime_metadata_path = "../artifacts/collectives-polkadot.scale")]
pub mod collectives {}

const POLKADOT_SPEC: &str = include_str!("../../artifacts/demo_chain_specs/polkadot.json");
const COLLECTIVES_SPEC: &str =
    include_str!("../../artifacts/demo_chain_specs/polkadot_collectives.json");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The smoldot logs are informative:
    tracing_subscriber::fmt::init();

    // Connecting to a parachain is a multi step process.

    // Step 1. Construct a new smoldot client.
    let mut client =
        subxt_lightclient::smoldot::Client::new(subxt_lightclient::smoldot::DefaultPlatform::new(
            "subxt-example-light-client".into(),
            "version-0".into(),
        ));

    // Step 2. Connect to the relay chain of the parachain. For this example, the Polkadot relay chain.
    let polkadot_connection = client
        .add_chain(subxt_lightclient::smoldot::AddChainConfig {
            specification: POLKADOT_SPEC,
            json_rpc: subxt_lightclient::smoldot::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: NonZeroU32::new(128).unwrap(),
                max_subscriptions: 1024,
            },
            potential_relay_chains: iter::empty(),
            database_content: "",
            user_data: (),
        })
        .expect("Light client chain added with valid spec; qed");
    let polkadot_json_rpc_responses = polkadot_connection
        .json_rpc_responses
        .expect("Light client configured with json rpc enabled; qed");
    let polkadot_chain_id = polkadot_connection.chain_id;

    // Step 3. Connect to the parachain. For this example, the Collectives parachain.
    let collectives_connection = client
        .add_chain(subxt_lightclient::smoldot::AddChainConfig {
            specification: COLLECTIVES_SPEC,
            json_rpc: subxt_lightclient::smoldot::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: NonZeroU32::new(128).unwrap(),
                max_subscriptions: 1024,
            },
            // The chain specification of the collectives parachain mentions that the identifier
            // of its relay chain is `polkadot`.
            potential_relay_chains: [polkadot_chain_id].into_iter(),
            database_content: "",
            user_data: (),
        })
        .expect("Light client chain added with valid spec; qed");
    let parachain_json_rpc_responses = collectives_connection
        .json_rpc_responses
        .expect("Light client configured with json rpc enabled; qed");
    let parachain_chain_id = collectives_connection.chain_id;

    // Step 4. Turn the smoldot client into a raw client.
    let raw_light_client = RawLightClient::builder()
        .add_chain(polkadot_chain_id, polkadot_json_rpc_responses)
        .add_chain(parachain_chain_id, parachain_json_rpc_responses)
        .build(client)
        .await?;

    // Step 5. Obtain a client to target the relay chain and the parachain.
    let _polkadot_api: LightClient<PolkadotConfig> =
        raw_light_client.for_chain(polkadot_chain_id).await?;
    let parachain_api: LightClient<PolkadotConfig> =
        raw_light_client.for_chain(parachain_chain_id).await?;

    // Step 6. Subscribe to the finalized blocks of the chains.
    {
        let key = collectives::storage()
            .fellowship_collective()
            .members_iter();
        let mut query = parachain_api
            .storage()
            .at_latest()
            .await
            .unwrap()
            .iter(key)
            .await?;
        let mut members = BTreeSet::new();
        let mut members_by_key = BTreeSet::new();

        while let Some(Ok((id, fellow))) = query.next().await {
            let account = AccountId32::from_slice(&id[id.len() - 32..]).unwrap();

            println!(
                "Fetched member: {} rank {}",
                account.to_ss58check(),
                fellow.rank
            );
            if members.contains(&account) {
                let cont = members_by_key.contains(&id);
                println!(
                    "ERROR: Fetched account twice. However is the key already inserted? {}",
                    cont
                );
            }
            members.insert(account);
            members_by_key.insert(id);
        }
    }

    Ok(())
}
