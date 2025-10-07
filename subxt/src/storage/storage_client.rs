// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_client_at::StorageClientAt;
use crate::{
    error::StorageError,
    backend::BlockRef,
    client::{OfflineClientT, OnlineClientT},
    config::{Config, HashFor},
};
use derive_where::derive_where;
use std::{future::Future, marker::PhantomData};
use subxt_core::storage::address::Address;

/// Query the runtime storage.
#[derive_where(Clone; Client)]
pub struct StorageClient<T, Client> {
    client: Client,
    _marker: PhantomData<T>,
}

impl<T, Client> StorageClient<T, Client> {
    /// Create a new [`StorageClient`]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> StorageClient<T, Client>
where
    T: Config,
    Client: OfflineClientT<T>,
{
    /// Run the validation logic against some storage address you'd like to access. Returns `Ok(())`
    /// if the address is valid (or if it's not possible to check since the address has no validation hash).
    /// Return an error if the address was not valid or something went wrong trying to validate it (ie
    /// the pallet or storage entry in question do not exist at all).
    pub fn validate<Addr: Address>(&self, address: &Addr) -> Result<(), StorageError> {
        subxt_core::storage::validate(address, &self.client.metadata()).map_err(Into::into)
    }
}

impl<T, Client> StorageClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Obtain storage at some block hash.
    pub fn at(&self, block_ref: impl Into<BlockRef<HashFor<T>>>) -> StorageClientAt<T, Client> {
        StorageClientAt::new(self.client.clone(), block_ref.into())
    }

    /// Obtain storage at the latest finalized block.
    pub fn at_latest(
        &self,
    ) -> impl Future<Output = Result<StorageClientAt<T, Client>, StorageError>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // get the ref for the latest finalized block and use that.
            let block_ref = client
                .backend()
                .latest_finalized_block_ref()
                .await
                .map_err(StorageError::CannotGetLatestFinalizedBlock)?;

            Ok(StorageClientAt::new(client, block_ref))
        }
    }
}
