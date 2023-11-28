use subxt::backend::{legacy::LegacyRpcMethods, rpc::RpcClient};
use subxt::config::DefaultExtrinsicParamsBuilder as Params;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First, create a raw RPC client:
    let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;

    // Use this to construct our RPC methods:
    let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());

    // We can use the same client to drive our full Subxt interface too:
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client.clone()).await?;

    // Now, we can make some RPC calls using some legacy RPC methods.
    println!(
        "📛 System Name: {:?}\n🩺 Health: {:?}\n🖫 Properties: {:?}\n🔗 Chain: {:?}\n",
        rpc.system_name().await?,
        rpc.system_health().await?,
        rpc.system_properties().await?,
        rpc.system_chain().await?
    );

    // We can also interleave RPC calls and using the full Subxt client, here to submit multiple
    // transactions using the legacy `system_account_next_index` RPC call, which returns a nonce
    // that is adjusted for any transactions already in the pool:

    let alice = dev::alice();
    let bob = dev::bob();

    loop {
        let current_nonce = rpc
            .system_account_next_index(&alice.public_key().into())
            .await?;
        let current_header = rpc.chain_get_header(None).await?.unwrap();

        let ext_params = Params::new().mortal(&current_header, 8).build();

        let balance_transfer = polkadot::tx()
            .balances()
            .transfer_allow_death(bob.public_key().into(), 1_000_000);

        let ext_hash = api
            .tx()
            .create_signed_with_nonce(&balance_transfer, &alice, current_nonce, ext_params)?
            .submit()
            .await?;

        println!("Submitted ext {ext_hash} with nonce {current_nonce}");

        // Sleep less than block time, but long enough to ensure
        // not all transactions end up in the same block.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
