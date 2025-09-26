// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_client_at::StorageClientAt;
use crate::{
    backend::BlockRef,
    client::{OfflineClientT, OnlineClientT},
    config::{Config, HashFor},
    error::Error,
};
use derive_where::derive_where;
use std::{future::Future, marker::PhantomData};
use subxt_core::storage::address::Address;
use subxt_core::storage::EqualOrPrefixOf;

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
    pub fn validate<Addr: Address>(&self, address: &Addr) -> Result<(), Error> {
        subxt_core::storage::validate(address, &self.client.metadata()).map_err(Into::into)
    }

    /// Convert some storage address into the raw bytes that would be submitted to the node in order
    /// to retrieve the entries at the root of the associated address.
    pub fn address_root_bytes<Addr: Address>(&self, address: &Addr) -> [u8; 32] {
        subxt_core::storage::get_address_root_bytes(address)
    }

    /// Convert some storage address into the raw bytes that would be submitted to the node in order
    /// to retrieve an entry. This fails if [`Address::append_entry_bytes`] does; in the built-in
    /// implementation this would be if the pallet and storage entry being asked for is not available on the
    /// node you're communicating with, or if the metadata is missing some type information (which should not
    /// happen).
    pub fn address_bytes<Addr: Address, Keys: EqualOrPrefixOf<Addr::KeyParts>>(&self, address: &Addr, keys: Keys) -> Result<Vec<u8>, Error> {
        subxt_core::storage::get_address_bytes(address, &self.client.metadata(), keys).map_err(Into::into)
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
    ) -> impl Future<Output = Result<StorageClientAt<T, Client>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // get the ref for the latest finalized block and use that.
            let block_ref = client.backend().latest_finalized_block_ref().await?;

            Ok(StorageClientAt::new(client, block_ref))
        }
    }
}
