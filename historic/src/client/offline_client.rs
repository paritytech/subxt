use crate::config::Config;

pub trait OfflineClientT<T: Config> {
    /// Get the configuration for this client.
    fn config(&self) -> &T;
    /// Get the spec version for some block number.
    fn spec_version_for_block_number(&self, block_number: u64) -> impl Future<Output = u64> + Send;
}

pub struct OfflineClient<T: Config> {
    /// The configuration for this client.
    config: T,
}

impl <T: Config> OfflineClientT<T> for OfflineClient<T> {
    fn config(&self) -> &T {
        &self.config
    }
    fn spec_version_for_block_number(&self, block_number: u64) -> impl Future<Output = u64> + Send {
        async move { todo!("Implement this!") }
    }
}