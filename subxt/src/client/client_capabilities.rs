// use crate::{backend::Backend, Config};

// use super::OfflineClientT;
use crate::{Config, Error, OfflineClient, OnlineClient};
use core::future::Future;

use super::OfflineClientT;

pub type FetchBlockHeader<Header> =
    Box<dyn Future<Output = Result<Header, Error>> + Send + 'static>;

/// An object-safe trait abstracting over capabilities of offline and online-clients.
///
/// Implemented by [`subxt::OfflineClient`], [`subxt::OnlineClient`] and [`subxt::LightClient`] the like.
pub trait ClientCapabilities<T: Config> {
    fn latest_block_header(&self) -> Option<FetchBlockHeader<T::Header>>
    where
        T::Header: Clone;
}

impl<T: Config> ClientCapabilities<T> for OfflineClient<T> {
    fn latest_block_header(&self) -> Option<FetchBlockHeader<T::Header>>
    where
        T::Header: Clone,
    {
        None
    }
}

impl<T: Config> ClientCapabilities<T> for OnlineClient<T> {
    fn latest_block_header(&self) -> Option<FetchBlockHeader<T::Header>>
    where
        T::Header: Clone,
    {
        let client = self.clone();
        Some(Box::new(async move {
            let block = client.blocks().at_latest().await?;
            let header: T::Header = block.header().clone();
            Ok(header)
        }))
    }
}

crate::macros::cfg_unstable_light_client! {
    use super::LightClient;

    impl<T: Config> ClientCapabilities<T> for LightClient<T> {
        fn latest_block_header(&self) -> Option<FetchBlockHeader<T::Header>>
        where
            T::Header: Clone,
        {
            let client = self.clone();
            Some(Box::new(async move {
                let block = client.blocks().at_latest().await?;
                let header: T::Header = block.header().clone();
                Ok(header)
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClientCapabilities;
    use crate::PolkadotConfig;
    use core::panic;

    fn is_object_safe(client: Box<dyn ClientCapabilities<PolkadotConfig>>) {
        _ = client.latest_block_header();
        unreachable!("Do not call this function, it just needs to compile.")
    }
}
