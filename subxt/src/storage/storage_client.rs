// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_type::Storage;
use crate::{
    backend::BlockRef,
    client::{OfflineClientT, OnlineClientT},
    error::Error,
    Config,
};
use derive_where::derive_where;
use std::{future::Future, marker::PhantomData};
use subxt_core::storage::address::AddressT;

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
    pub fn validate<Address: AddressT>(&self, address: &Address) -> Result<(), Error> {
        subxt_core::storage::validate(&self.client.metadata(), address).map_err(Into::into)
    }

    /// Convert some storage address into the raw bytes that would be submitted to the node in order
    /// to retrieve the entries at the root of the associated address.
    pub fn address_root_bytes<Address: AddressT>(&self, address: &Address) -> Vec<u8> {
        subxt_core::storage::get_address_root_bytes(address)
    }

    /// Convert some storage address into the raw bytes that would be submitted to the node in order
    /// to retrieve an entry. This fails if [`AddressT::append_entry_bytes`] does; in the built-in
    /// implementation this would be if the pallet and storage entry being asked for is not available on the
    /// node you're communicating with, or if the metadata is missing some type information (which should not
    /// happen).
    pub fn address_bytes<Address: AddressT>(&self, address: &Address) -> Result<Vec<u8>, Error> {
        subxt_core::storage::get_address_bytes(&self.client.metadata(), address).map_err(Into::into)
    }
}

impl<T, Client> StorageClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Obtain storage at some block hash.
    pub fn at(&self, block_ref: impl Into<BlockRef<T::Hash>>) -> Storage<T, Client> {
        Storage::new(self.client.clone(), block_ref.into())
    }

    /// Obtain storage at the latest block hash.
    pub fn at_latest(
        &self,
    ) -> impl Future<Output = Result<Storage<T, Client>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // get the ref for the latest finalized block and use that.
            let block_ref = client.backend().latest_finalized_block_ref().await?;

            Ok(Storage::new(client, block_ref))
        }
    }
}
