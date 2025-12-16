//! We can provide a custom RPC client to Subxt to use, instead of the default.
use subxt::config::RpcConfigFor;
use subxt::{Error, OnlineClient, PolkadotConfig};
use subxt_rpcs::client::{ReconnectingRpcClient, RpcClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = PolkadotConfig::new();

    // Configure an RPC client. Here we use the reconnecting one, but several impls are
    // available, or you can implement the subxt_rpcs::client::RpcClientT trait yourself
    // to bring your own RPC client.
    let inner_rpc_client = ReconnectingRpcClient::builder()
        .build("wss://rpc.ibp.network/polkadot")
        .await
        .map_err(Error::other)?;

    let rpc_client = RpcClient::new(inner_rpc_client);

    // Pass it to Subxt to use.
    let client = OnlineClient::from_rpc_client(config, rpc_client.clone()).await?;

    // We can use the Subxt client:
    let at_block = client.at_current_block().await?;
    let header = at_block.block_header().await?;
    println!("Current block header via Subxt: {header:?}");

    // Since we cloned the RPC client above, we can use it ourselves too:
    let legacy_rpcs =
        subxt_rpcs::methods::LegacyRpcMethods::<RpcConfigFor<PolkadotConfig>>::new(rpc_client);
    let header = legacy_rpcs
        .chain_get_header(Some(at_block.block_hash()))
        .await?
        .unwrap();
    println!("Current block header via RPC call: {header:?}");

    Ok(())
}
