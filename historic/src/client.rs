mod online_client;
mod offline_client;

use subxt_rpcs::RpcClient;
use crate::config::Config;

pub use online_client::{ OnlineClientAtBlockT, OnlineClient, OnlineClientAtBlock };
pub use offline_client::{ OfflineClientAtBlockT, OfflineClient, OfflineClientAtBlock };

// Wrap both ClientAtBlock's into a struct like `Client` that exposes the methods?
// Don't expose the trait methods/traits at all so just internal details?