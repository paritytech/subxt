use polkadot::multisig::events::NewMultisig;
use polkadot::runtime_types::{frame_system::pallet::Call, sp_weights::weight_v2::Weight};
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::{dev, Keypair};

#[subxt::subxt(runtime_metadata_path = "../cli/enjin2.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api =
        OnlineClient::<PolkadotConfig>::from_url("wss://rpc.relay.blockchain.enjin.io:443").await?;

    let unbonding_members = {
        let query = polkadot::storage()
            .nomination_pools()
            .unbonding_members_iter1(6u32);

        println!("Query {:?}", query);

        let static_query = "0x7a6d38deaa01cb6e76ee69889f1696270a0047e8c76616ede28bd21ecd3c8acbb61f803a716bd3b906000000";

        let mut results = api.storage().at_latest().await?.iter(query).await?;

        let mut unbonding_members = std::collections::HashMap::new();
        while let Some(Ok((key, prefs))) = results.next().await {
            unbonding_members.insert(key, prefs);
        }
        unbonding_members
    };

    println!("Unbonding members {:?}", unbonding_members.len());

    Ok(())
}
