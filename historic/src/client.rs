mod online_client;
mod offline_client;

use subxt_rpcs::RpcClient;
use crate::config::Config;

pub use online_client::{ OnlineClientT, OnlineClient };
pub use offline_client::{ OfflineClientT, OfflineClient };

/// A client which is ready to decode data at a specific block number.
pub struct ClientAtBlock<'a, T: Config + 'a, Client> {
    /// The client used to communicate with the node.
    client: Client,
    /// Historic types to use at this block number.
    historic_types: T::LegacyTypes<'a>,
    /// Metadata to use at this block number.
    metadata: &'a frame_metadata::RuntimeMetadata
}