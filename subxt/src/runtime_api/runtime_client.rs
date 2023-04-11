// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::runtime_types::RuntimeApi;

use crate::{client::OnlineClientT, error::Error, Config};
use derivative::Derivative;
use std::{future::Future, marker::PhantomData};

/// Execute runtime API calls.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct RuntimeApiClient<T, Client> {
    client: Client,
    _marker: PhantomData<T>,
}

impl<T, Client> RuntimeApiClient<T, Client> {
    /// Create a new [`RuntimeApiClient`]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> RuntimeApiClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Obtain a runtime API interface at some block hash.
    pub fn at(&self, block_hash: T::Hash) -> RuntimeApi<T, Client> {
        RuntimeApi::new(self.client.clone(), block_hash)
    }

    /// Obtain a runtime API interface at the latest block hash.
    pub fn at_latest(
        &self,
    ) -> impl Future<Output = Result<RuntimeApi<T, Client>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // get the hash for the latest block and use that.
            let block_hash = client
                .rpc()
                .block_hash(None)
                .await?
                .expect("didn't pass a block number; qed");

            Ok(RuntimeApi::new(client, block_hash))
        }
    }
}
