use polkadot::multisig::events::NewMultisig;
use polkadot::runtime_types::{
    frame_system::pallet::Call, polkadot_runtime::RuntimeCall, sp_weights::weight_v2::Weight,
};
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::{dev, Keypair};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Prepare the chain to have 3 open multisig requests (2 of them are alice + bob):
    let alice_signer = dev::alice();
    let bob = AccountId32(dev::bob().public_key().0);
    let charlie = AccountId32(dev::charlie().public_key().0);

    let new_multisig_1 = submit_remark_as_multi(&alice_signer, &bob, b"Hello", &api).await?;
    let new_multisig_2 = submit_remark_as_multi(&alice_signer, &bob, b"Hi", &api).await?;
    let new_multisig_3 = submit_remark_as_multi(&alice_signer, &charlie, b"Hello", &api).await?;

    // Note: the NewMultisig event contains the multisig address we need to use for the storage queries:
    assert_eq!(new_multisig_1.multisig, new_multisig_2.multisig);
    assert_ne!(new_multisig_1.multisig, new_multisig_3.multisig);

    // Build a storage query to iterate over open multisig extrinsics from
    // new_multisig_1.multisig which is the AccountId of the alice + bob multisig account
    let alice_bob_account_id = &new_multisig_1.multisig;
    let storage_query = polkadot::storage()
        .multisig()
        .multisigs_iter1(alice_bob_account_id);

    // Get back an iterator of results.
    let mut results = api.storage().at_latest().await?.iter(storage_query).await?;

    while let Some(Ok((key, value))) = results.next().await {
        println!("Key: 0x{}", hex::encode(&key));
        println!("Value: {:?}", value);
    }

    Ok(())
}

async fn submit_remark_as_multi(
    signer: &Keypair,
    other: &AccountId32,
    remark: &[u8],
    api: &OnlineClient<PolkadotConfig>,
) -> Result<NewMultisig, Box<dyn std::error::Error>> {
    let multisig_remark_tx = polkadot::tx().multisig().as_multi(
        2,
        vec![other.clone()],
        None,
        RuntimeCall::System(Call::remark {
            remark: remark.to_vec(),
        }),
        Weight {
            ref_time: 0,
            proof_size: 0,
        },
    );
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&multisig_remark_tx, signer)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new_multisig = events
        .find_first::<polkadot::multisig::events::NewMultisig>()?
        .expect("should contain event");
    Ok(new_multisig)
}
