use crate::config::Config;
use crate::client::OfflineClientT;
use subxt_rpcs::RpcClient;

pub trait OnlineClientT<T: Config>: OfflineClientT<T> {
    /// Get the RPC client used to communicate with the node.
    fn rpc_client(&self) -> &RpcClient;
}

pub struct OnlineClient<T: Config> {
    /// The configuration for this client.
    config: T,
    /// The RPC client used to communicate with the node.
    rpc_client: RpcClient,
}

impl<T: Config> OnlineClientT<T> for OnlineClient<T> {
    fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }
}

impl<T: Config> OfflineClientT<T> for OnlineClient<T> {
    fn config(&self) -> &T {
        &self.config
    }

    fn spec_version_for_block_number(&self, block_number: u64) -> impl Future<Output = u64> + Send {
        async move { todo!("Implement this!") }
    }
}