//! Example for how to use the Light client and connect to a relay+parachain.
//!
//! Run: `cargo run --example light_client_parachains --features="unstable-light-client native"`.

#![allow(missing_docs)]
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
    let key = collectives::storage()
        .fellowship_collective()
        .members_iter();

    let parachain_rpc = raw_light_client
        .rpc_client_for_chain(parachain_chain_id)
        .await?;

    use subxt::backend::legacy::LegacyRpcMethods;
    let legacy_rpc: LegacyRpcMethods<PolkadotConfig> = LegacyRpcMethods::new(parachain_rpc);

    use subxt::client::OfflineClientT;
    let metadata = parachain_api.metadata();

    let key_bytes = {
        use subxt::storage::StorageAddress;
        pub(crate) fn write_storage_address_root_bytes<Address: subxt::storage::StorageAddress>(
            addr: &Address,
            out: &mut Vec<u8>,
        ) {
            out.extend(sp_core_hashing::twox_128(addr.pallet_name().as_bytes()));
            out.extend(sp_core_hashing::twox_128(addr.entry_name().as_bytes()));
        }

        let mut bytes = Vec::new();
        write_storage_address_root_bytes(&key, &mut bytes);
        key.append_entry_bytes(&metadata, &mut bytes)?;
        bytes
    };

    println!(" Smoldot result: ");
    let mut set = std::collections::HashSet::new();
    let mut last_key = None;
    loop {
        let param_start_key = last_key.as_deref();
        println!("New query with {:?}\n", param_start_key);

        let result = legacy_rpc
            .state_get_keys_paged(key_bytes.as_slice(), 10, param_start_key, None)
            .await?;
        for res in result.clone() {
            println!("Fetched member: {:?}", res);

            if set.contains(&res) {
                println!("ERROR: Fetched account twice");
            }
            set.insert(res);
        }

        if result.len() < 10 {
            break;
        }

        last_key = Some(result.last().unwrap().clone());
    }

    println!();
    // println!(" RPC node result: ");
    // {
    //     use subxt::backend::rpc::RpcClient;
    //     // First, create a raw RPC client:
    //     let rpc_client = RpcClient::from_url("wss://polkadot-collectives-rpc.dwellir.com").await?;
    //     // Use this to construct our RPC methods:
    //     let legacy_rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());

    //     let mut set = std::collections::HashSet::new();
    //     let mut last_key = None;
    //     loop {
    //         let param_start_key = last_key.as_deref();
    //         println!("New query with {:?}\n", param_start_key);

    //         let result = legacy_rpc
    //             .state_get_keys_paged(key_bytes.as_slice(), 10, param_start_key, None)
    //             .await?;
    //         for res in result.clone() {
    //             println!("Fetched member: {:?}", res);

    //             if set.contains(&res) {
    //                 println!("ERROR: Fetched account twice");
    //             }
    //             set.insert(res);
    //         }

    //         if result.is_empty() {
    //             break;
    //         }

    //         println!();

    //         last_key = Some(result.last().unwrap().clone());
    //     }
    // }

    Ok(())
}
