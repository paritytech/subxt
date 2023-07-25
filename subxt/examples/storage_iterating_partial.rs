use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale")]
pub mod polkadot {}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let alice: AccountId32 = dev::alice().public_key().into();

    // alice sends two note_preimage requests to the chain:
    let proposal_1 = polkadot::tx().preimage().note_preimage(vec![0,1,2]);
    let proposal_2 = polkadot::tx().preimage().note_preimage(vec![0,1,2,3]);
    for proposal in [proposal_1, proposal_2]{
        api
            .tx()
            .sign_and_submit_then_watch_default(&proposal, &dev::alice())
            .await?
            .wait_for_finalized_success()
            .await?;
    }


    // NOT WORKING YET! WORK IN PROGRESS!

    // check with partial key iteration that the proposals are saved:
    // let storage_query = polkadot::storage().preimage().preimage_for_iter();
    // let mut results = api
    //     .storage()
    //     .at_latest()
    //     .await?
    //     .iter(storage_query, 10)
    //     .await?;
    //
    // while let Some((key, value)) = results.next().await? {
    //     println!("Key: 0x{}", hex::encode(&key));
    //     println!("Value: {:?}", value);
    // }

    Ok(())
}
