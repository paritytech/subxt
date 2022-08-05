// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_address::{
    StorageAddress,
    Yes,
};
use crate::{
    client::{
        OfflineClientT,
        OnlineClientT,
    },
    error::Error,
    metadata::{
        DecodeWithMetadata,
        Metadata,
    },
    Config,
};
use derivative::Derivative;
use frame_metadata::StorageEntryType;
use scale_info::form::PortableForm;
use sp_core::storage::{
    StorageData,
    StorageKey,
};
use std::{
    future::Future,
    marker::PhantomData,
};

/// Query the runtime storage.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
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
    pub fn validate<Address: StorageAddress>(
        &self,
        address: &Address,
    ) -> Result<(), Error> {
        if let Some(hash) = address.validation_hash() {
            validate_storage(
                address.pallet_name(),
                address.entry_name(),
                hash,
                &self.client.metadata(),
            )?;
        }
        Ok(())
    }
}

impl<T, Client> StorageClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Fetch the raw encoded value at the address/key given.
    pub fn fetch_raw<'a>(
        &self,
        key: &'a [u8],
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Option<Vec<u8>>, Error>> + 'a {
        let client = self.client.clone();
        // Ensure that the returned future doesn't have a lifetime tied to api.storage(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let data = client.rpc().storage(key, hash).await?;
            Ok(data.map(|d| d.0))
        }
    }

    /// Fetch a decoded value from storage at a given address and optional block hash.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use subxt::{ PolkadotConfig, OnlineClient };
    ///
    /// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
    /// pub mod polkadot {}
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    ///
    /// // Address to a storage entry we'd like to access.
    /// let address = polkadot::storage().xcm_pallet().queries(&12345);
    ///
    /// // Fetch just the keys, returning up to 10 keys.
    /// let value = api
    ///     .storage()
    ///     .fetch(&address, None)
    ///     .await
    ///     .unwrap();
    ///
    /// println!("Value: {:?}", value);
    /// # }
    /// ```
    pub fn fetch<'a, Address>(
        &self,
        address: &'a Address,
        hash: Option<T::Hash>,
    ) -> impl Future<
        Output = Result<Option<<Address::Target as DecodeWithMetadata>::Target>, Error>,
    > + 'a
    where
        Address: StorageAddress<IsFetchable = Yes> + 'a,
    {
        let client = self.clone();
        async move {
            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            client.validate(address)?;

            // Look up the return type ID to enable DecodeWithMetadata:
            let metadata = client.client.metadata();
            let lookup_bytes = super::utils::storage_address_bytes(address, &metadata)?;
            if let Some(data) = client
                .client
                .storage()
                .fetch_raw(&lookup_bytes, hash)
                .await?
            {
                let val = <Address::Target as DecodeWithMetadata>::decode_storage_with_metadata(
                    &mut &*data,
                    address.pallet_name(),
                    address.entry_name(),
                    &metadata,
                )?;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }
    }

    /// Fetch a StorageKey that has a default value with an optional block hash.
    pub fn fetch_or_default<'a, Address>(
        &self,
        address: &'a Address,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<<Address::Target as DecodeWithMetadata>::Target, Error>>
           + 'a
    where
        Address: StorageAddress<IsFetchable = Yes, IsDefaultable = Yes> + 'a,
    {
        let client = self.client.clone();
        async move {
            let pallet_name = address.pallet_name();
            let storage_name = address.entry_name();
            // Metadata validation happens via .fetch():
            if let Some(data) = client.storage().fetch(address, hash).await? {
                Ok(data)
            } else {
                let metadata = client.metadata();

                // We have to dig into metadata already, so no point using the optimised `decode_storage_with_metadata` call.
                let pallet_metadata = metadata.pallet(pallet_name)?;
                let storage_metadata = pallet_metadata.storage(storage_name)?;
                let return_ty_id =
                    return_type_from_storage_entry_type(&storage_metadata.ty);
                let bytes = &mut &storage_metadata.default[..];

                let val = <Address::Target as DecodeWithMetadata>::decode_with_metadata(
                    bytes,
                    return_ty_id,
                    &metadata,
                )?;
                Ok(val)
            }
        }
    }

    /// Fetch up to `count` keys for a storage map in lexicographic order.
    ///
    /// Supports pagination by passing a value to `start_key`.
    pub fn fetch_keys<'a>(
        &self,
        key: &'a [u8],
        count: u32,
        start_key: Option<&'a [u8]>,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Vec<StorageKey>, Error>> + 'a {
        let client = self.client.clone();
        async move {
            let keys = client
                .rpc()
                .storage_keys_paged(key, count, start_key, hash)
                .await?;
            Ok(keys)
        }
    }

    /// Returns an iterator of key value pairs.
    ///
    /// ```no_run
    /// use subxt::{ PolkadotConfig, OnlineClient };
    ///
    /// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
    /// pub mod polkadot {}
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    ///
    /// // Address to the root of a storage entry that we'd like to iterate over.
    /// let address = polkadot::storage().xcm_pallet().version_notifiers_root();
    ///
    /// // Iterate over keys and values at that address.
    /// let mut iter = api
    ///     .storage()
    ///     .iter(address, 10, None)
    ///     .await
    ///     .unwrap();
    ///
    /// while let Some((key, value)) = iter.next().await.unwrap() {
    ///     println!("Key: 0x{}", hex::encode(&key));
    ///     println!("Value: {}", value);
    /// }
    /// # }
    /// ```
    pub fn iter<Address>(
        &self,
        address: Address,
        page_size: u32,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<KeyIter<T, Client, Address::Target>, Error>> + 'static
    where
        Address: StorageAddress<IsIterable = Yes> + 'static,
    {
        let client = self.clone();
        async move {
            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            client.validate(&address)?;

            // Fetch a concrete block hash to iterate over. We do this so that if new blocks
            // are produced midway through iteration, we continue to iterate at the block
            // we started with and not the new block.
            let hash = if let Some(hash) = hash {
                hash
            } else {
                client
                    .client
                    .rpc()
                    .block_hash(None)
                    .await?
                    .expect("didn't pass a block number; qed")
            };

            let metadata = client.client.metadata();

            // Look up the return type for flexible decoding. Do this once here to avoid
            // potentially doing it every iteration if we used `decode_storage_with_metadata`
            // in the iterator.
            let return_type_id = lookup_storage_return_type(
                &metadata,
                address.pallet_name(),
                address.entry_name(),
            )?;

            // The root pallet/entry bytes for this storage entry:
            let address_root_bytes = super::utils::storage_address_root_bytes(&address);

            Ok(KeyIter {
                client,
                address_root_bytes,
                metadata,
                return_type_id,
                block_hash: hash,
                count: page_size,
                start_key: None,
                buffer: Default::default(),
                _marker: std::marker::PhantomData,
            })
        }
    }
}

/// Iterates over key value pairs in a map.
pub struct KeyIter<T: Config, Client, ReturnTy> {
    client: StorageClient<T, Client>,
    address_root_bytes: Vec<u8>,
    return_type_id: u32,
    metadata: Metadata,
    count: u32,
    block_hash: T::Hash,
    start_key: Option<StorageKey>,
    buffer: Vec<(StorageKey, StorageData)>,
    _marker: std::marker::PhantomData<ReturnTy>,
}

impl<'a, T: Config, Client: OnlineClientT<T>, ReturnTy> KeyIter<T, Client, ReturnTy>
where
    T: Config,
    Client: OnlineClientT<T>,
    ReturnTy: DecodeWithMetadata,
{
    /// Returns the next key value pair from a map.
    pub async fn next(
        &mut self,
    ) -> Result<Option<(StorageKey, ReturnTy::Target)>, Error> {
        loop {
            if let Some((k, v)) = self.buffer.pop() {
                let val = ReturnTy::decode_with_metadata(
                    &mut &v.0[..],
                    self.return_type_id,
                    &self.metadata,
                )?;
                return Ok(Some((k, val)))
            } else {
                let start_key = self.start_key.take();
                let keys = self
                    .client
                    .fetch_keys(
                        &self.address_root_bytes,
                        self.count,
                        start_key.as_ref().map(|k| &*k.0),
                        Some(self.block_hash),
                    )
                    .await?;

                if keys.is_empty() {
                    return Ok(None)
                }

                self.start_key = keys.last().cloned();

                let change_sets = self
                    .client
                    .client
                    .rpc()
                    .query_storage_at(keys.iter().map(|k| &*k.0), Some(self.block_hash))
                    .await?;
                for change_set in change_sets {
                    for (k, v) in change_set.changes {
                        if let Some(v) = v {
                            self.buffer.push((k, v));
                        }
                    }
                }
                debug_assert_eq!(self.buffer.len(), keys.len());
            }
        }
    }
}

/// Validate a storage entry against the metadata.
fn validate_storage(
    pallet_name: &str,
    storage_name: &str,
    hash: [u8; 32],
    metadata: &Metadata,
) -> Result<(), Error> {
    let expected_hash = match metadata.storage_hash(pallet_name, storage_name) {
        Ok(hash) => hash,
        Err(e) => return Err(e.into()),
    };
    match expected_hash == hash {
        true => Ok(()),
        false => Err(crate::error::MetadataError::IncompatibleMetadata.into()),
    }
}

/// look up a return type ID for some storage entry.
fn lookup_storage_return_type(
    metadata: &Metadata,
    pallet: &str,
    entry: &str,
) -> Result<u32, Error> {
    let storage_entry_type = &metadata.pallet(pallet)?.storage(entry)?.ty;

    Ok(return_type_from_storage_entry_type(storage_entry_type))
}

/// Fetch the return type out of a [`StorageEntryType`].
fn return_type_from_storage_entry_type(entry: &StorageEntryType<PortableForm>) -> u32 {
    match entry {
        StorageEntryType::Plain(ty) => ty.id(),
        StorageEntryType::Map { value, .. } => value.id(),
    }
}
