use subxt::backend::legacy::LegacyRpcMethods;
use subxt::backend::rpc::RpcClient;
use subxt::config::DefaultExtrinsicParamsBuilder;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected None")]
    UnexpectedNone,
}

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create both an OnlineClient and a LegacyRpcClient, configured to talk to Polkadot
    // nodes with the same RpcClient using the default URL
    let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client.clone()).await?;
    let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client);

    let alice = dev::alice().public_key().to_account_id();

    // Some example destinations accounts and amounts
    let transactions = vec![
        (dev::bob().public_key(), 10000000),
        (dev::charlie().public_key(), 12000000),
        (dev::ferdie().public_key(), 30000000),
        (dev::eve().public_key(), 40000000),
        (dev::dave().public_key(), 50000000),
        (dev::bob().public_key(), 1000000),
        (dev::charlie().public_key(), 1200000),
        (dev::ferdie().public_key(), 3000000),
        (dev::eve().public_key(), 4000000),
        (dev::dave().public_key(), 50000000),
        (dev::bob().public_key(), 50000000),
        (dev::charlie().public_key(), 22000000),
        (dev::ferdie().public_key(), 130000000),
        (dev::eve().public_key(), 40003000),
        (dev::dave().public_key(), 150000000),
    ];

    println!(
        "📛 System Name: {:?}\n🩺 Health: {:?}\n🖫 Properties: {:?}\n🔗 Chain: {:?}\n",
        rpc.system_name().await?,
        rpc.system_health().await?,
        rpc.system_properties().await?,
        rpc.system_chain().await?
    );

    for (dest, amount) in transactions {
        let nonce = rpc.system_account_next_index(&alice).await?;

        // Extrinsics are mortal, only valid for a limited number of blocks
        // from the current best
        let extrinsic_parameters = rpc
            .chain_get_header(None)
            .await?
            .map(|header| {
                DefaultExtrinsicParamsBuilder::default()
                    .mortal(&header, 8)
                    .build()
            })
            .ok_or(Error::UnexpectedNone)?;
        let balance_transfer = polkadot::tx()
            .balances()
            .transfer_allow_death(dest.into(), amount);
        let extrinsic = api.tx().create_signed_with_nonce(
            &balance_transfer,
            &dev::alice(),
            nonce,
            extrinsic_parameters,
        )?;
        println!("Submitting {:?} with Nonce:{nonce}", extrinsic.hash());

        // Submitting without tracking the progress of the extrinsics
        let _ = extrinsic.submit().await?;

        // Account nonce will return the next nonce based on finalized extrinsics but
        // RPC call System.account_next_index will adjust for transactions already in
        // the pool
        println!(
            "AccountNonce: {:?} System.account_next_index: {:?}",
            api.tx().account_nonce(&alice).await?,
            rpc.system_account_next_index(&alice).await?
        );
        // Sleep less than block time, but long enough to ensure
        // not all transactions end up in the same block.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    for _ in 0..20 {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        let account_nonce = api.tx().account_nonce(&alice).await?;
        let system_next = rpc.system_account_next_index(&alice).await?;

        println!(
            "AccountNonce: {account_nonce:?} System.account_next_index: {system_next:?} Finalized: {:?}",
            rpc.chain_get_finalized_head().await?
        );
        if account_nonce == system_next {
            return Ok(());
        }
    }
    Ok(())
}
