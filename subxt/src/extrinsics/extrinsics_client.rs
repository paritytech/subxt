// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::config::Header;
use crate::{
    client::OnlineClientT,
    error::{BlockError, Error},
    extrinsics::Extrinsics,
    Config,
};
use derivative::Derivative;
use std::future::Future;

/// A client for working with extrinsics.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct ExtrinsicsClient<T, Client> {
    client: Client,
    _marker: std::marker::PhantomData<T>,
}

impl<T, Client> ExtrinsicsClient<T, Client> {
    /// Create a new [`ExtrinsicsClient`].
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, Client> ExtrinsicsClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Obtain extrinsics at some block hash.
    ///
    /// # Warning
    ///
    /// This call only supports blocks produced since the most recent
    /// runtime upgrade. You can attempt to retrieve extrinsics from older blocks,
    /// but may run into errors attempting to work with them.
    pub fn at(
        &self,
        block_hash: T::Hash,
    ) -> impl Future<Output = Result<Extrinsics<T>, Error>> + Send + 'static {
        self.at_or_latest(Some(block_hash))
    }

    /// Obtain extrinsics at the latest block hash.
    pub fn at_latest(&self) -> impl Future<Output = Result<Extrinsics<T>, Error>> + Send + 'static {
        self.at_or_latest(None)
    }

    /// Obtain extrinsics at some block hash.
    fn at_or_latest(
        &self,
        block_hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Extrinsics<T>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // If block hash is not provided, get the hash
            // for the latest block and use that to create an explicit error.
            let block_hash = match block_hash {
                Some(hash) => hash,
                None => client
                    .rpc()
                    .block_hash(None)
                    .await?
                    .expect("didn't pass a block number; qed"),
            };

            let result = client.rpc().block(Some(block_hash)).await?;
            let Some(block_details) = result else {
                return Err(BlockError::not_found(block_hash).into());
            };

            Extrinsics::new(client.metadata(), block_details.block)
        }
    }
}
