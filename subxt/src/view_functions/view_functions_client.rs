// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::view_function_types::ViewFunctionsApi;

use crate::{
    backend::BlockRef,
    client::OnlineClientT,
    config::{Config, HashFor},
    error::Error,
};
use derive_where::derive_where;
use std::{future::Future, marker::PhantomData};

/// Make View Function calls at some block.
#[derive_where(Clone; Client)]
pub struct ViewFunctionsClient<T, Client> {
    client: Client,
    _marker: PhantomData<T>,
}

impl<T, Client> ViewFunctionsClient<T, Client> {
    /// Create a new [`ViewFunctionsClient`]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> ViewFunctionsClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Obtain an interface to call View Functions at some block hash.
    pub fn at(&self, block_ref: impl Into<BlockRef<HashFor<T>>>) -> ViewFunctionsApi<T, Client> {
        ViewFunctionsApi::new(self.client.clone(), block_ref.into())
    }

    /// Obtain an interface to call View Functions at the latest block hash.
    pub fn at_latest(
        &self,
    ) -> impl Future<Output = Result<ViewFunctionsApi<T, Client>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // get the ref for the latest finalized block and use that.
            let block_ref = client.backend().latest_finalized_block_ref().await?;

            Ok(ViewFunctionsApi::new(client, block_ref))
        }
    }
}
