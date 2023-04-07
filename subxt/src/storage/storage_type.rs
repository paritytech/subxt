// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_address::{StorageAddress, Yes};
use crate::{
    client::OnlineClientT,
    error::Error,
    metadata::{DecodeWithMetadata, Metadata},
    rpc::types::{StorageData, StorageKey},
    Config,
};
use derivative::Derivative;
use frame_metadata::StorageEntryType;
use scale_info::form::PortableForm;
use std::{future::Future, marker::PhantomData};

/// Query the runtime storage.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct Storage<T: Config, Client> {
    client: Client,
    block_hash: T::Hash,
    _marker: PhantomData<T>,
}

impl<T: Config, Client> Storage<T, Client> {
    /// Create a new [`Storage`]
    pub(crate) fn new(client: Client, block_hash: T::Hash) -> Self {
        Self {
            client,
            block_hash,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> Storage<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Fetch the raw encoded value at the address/key given.
    pub fn fetch_raw<'address>(
        &self,
        key: &'address [u8],
    ) -> impl Future<Output = Result<Option<Vec<u8>>, Error>> + 'address {
        let client = self.client.clone();
        let block_hash = self.block_hash;
        // Ensure that the returned future doesn't have a lifetime tied to api.storage(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let data = client.rpc().storage(key, Some(block_hash)).await?;
            Ok(data.map(|d| d.0))
        }
    }

    /// Fetch a decoded value from storage at a given address.
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
    ///     .at_latest()
    ///     .await
    ///     .unwrap()
    ///     .fetch(&address)
    ///     .await
    ///     .unwrap();
    ///
    /// println!("Value: {:?}", value);
    /// # }
    /// ```
    pub fn fetch<'address, Address>(
        &self,
        address: &'address Address,
    ) -> impl Future<Output = Result<Option<Address::Target>, Error>> + 'address
    where
        Address: StorageAddress<IsFetchable = Yes> + 'address,
    {
        let client = self.clone();
        async move {
            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            validate_storage_address(address, &client.client.metadata())?;

            // Look up the return type ID to enable DecodeWithMetadata:
            let metadata = client.client.metadata();
            let lookup_bytes = super::utils::storage_address_bytes(address, &metadata)?;
            if let Some(data) = client.fetch_raw(&lookup_bytes).await? {
                let val = decode_storage_with_metadata::<Address::Target>(
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
    pub fn fetch_or_default<'address, Address>(
        &self,
        address: &'address Address,
    ) -> impl Future<Output = Result<Address::Target, Error>> + 'address
    where
        Address: StorageAddress<IsFetchable = Yes, IsDefaultable = Yes> + 'address,
    {
        let client = self.clone();
        async move {
            let pallet_name = address.pallet_name();
            let storage_name = address.entry_name();
            // Metadata validation happens via .fetch():
            if let Some(data) = client.fetch(address).await? {
                Ok(data)
            } else {
                let metadata = client.client.metadata();

                // We have to dig into metadata already, so no point using the optimised `decode_storage_with_metadata` call.
                let pallet_metadata = metadata.pallet(pallet_name)?;
                let storage_metadata = pallet_metadata.storage(storage_name)?;
                let return_ty_id = return_type_from_storage_entry_type(&storage_metadata.ty);
                let bytes = &mut &storage_metadata.default[..];

                let val = Address::Target::decode_with_metadata(bytes, return_ty_id, &metadata)?;
                Ok(val)
            }
        }
    }

    /// Fetch up to `count` keys for a storage map in lexicographic order.
    ///
    /// Supports pagination by passing a value to `start_key`.
    pub fn fetch_keys<'address>(
        &self,
        key: &'address [u8],
        count: u32,
        start_key: Option<&'address [u8]>,
    ) -> impl Future<Output = Result<Vec<StorageKey>, Error>> + 'address {
        let client = self.client.clone();
        let block_hash = self.block_hash;
        async move {
            let keys = client
                .rpc()
                .storage_keys_paged(key, count, start_key, Some(block_hash))
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
    ///     .at_latest()
    ///     .await
    ///     .unwrap()
    ///     .iter(address, 10)
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
    ) -> impl Future<Output = Result<KeyIter<T, Client, Address::Target>, Error>> + 'static
    where
        Address: StorageAddress<IsIterable = Yes> + 'static,
    {
        let client = self.clone();
        let block_hash = self.block_hash;
        async move {
            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            validate_storage_address(&address, &client.client.metadata())?;

            let metadata = client.client.metadata();

            // Look up the return type for flexible decoding. Do this once here to avoid
            // potentially doing it every iteration if we used `decode_storage_with_metadata`
            // in the iterator.
            let return_type_id =
                lookup_storage_return_type(&metadata, address.pallet_name(), address.entry_name())?;

            // The root pallet/entry bytes for this storage entry:
            let address_root_bytes = super::utils::storage_address_root_bytes(&address);

            Ok(KeyIter {
                client,
                address_root_bytes,
                metadata,
                return_type_id,
                block_hash,
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
    client: Storage<T, Client>,
    address_root_bytes: Vec<u8>,
    return_type_id: u32,
    metadata: Metadata,
    count: u32,
    block_hash: T::Hash,
    start_key: Option<StorageKey>,
    buffer: Vec<(StorageKey, StorageData)>,
    _marker: std::marker::PhantomData<ReturnTy>,
}

impl<'a, T, Client, ReturnTy> KeyIter<T, Client, ReturnTy>
where
    T: Config,
    Client: OnlineClientT<T>,
    ReturnTy: DecodeWithMetadata,
{
    /// Returns the next key value pair from a map.
    pub async fn next(&mut self) -> Result<Option<(StorageKey, ReturnTy)>, Error> {
        loop {
            if let Some((k, v)) = self.buffer.pop() {
                let val = ReturnTy::decode_with_metadata(
                    &mut &v.0[..],
                    self.return_type_id,
                    &self.metadata,
                )?;
                return Ok(Some((k, val)));
            } else {
                let start_key = self.start_key.take();
                let keys = self
                    .client
                    .fetch_keys(
                        &self.address_root_bytes,
                        self.count,
                        start_key.as_ref().map(|k| &*k.0),
                    )
                    .await?;

                if keys.is_empty() {
                    return Ok(None);
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

/// Validate a storage address against the metadata.
pub(crate) fn validate_storage_address<Address: StorageAddress>(
    address: &Address,
    metadata: &Metadata,
) -> Result<(), Error> {
    if let Some(hash) = address.validation_hash() {
        validate_storage(address.pallet_name(), address.entry_name(), hash, metadata)?;
    }
    Ok(())
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
        false => Err(crate::error::MetadataError::IncompatibleStorageMetadata(
            pallet_name.into(),
            storage_name.into(),
        )
        .into()),
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
        StorageEntryType::Plain(ty) => ty.id,
        StorageEntryType::Map { value, .. } => value.id,
    }
}

/// Given some bytes, a pallet and storage name, decode the response.
fn decode_storage_with_metadata<T: DecodeWithMetadata>(
    bytes: &mut &[u8],
    pallet_name: &str,
    storage_entry: &str,
    metadata: &Metadata,
) -> Result<T, Error> {
    let ty = &metadata.pallet(pallet_name)?.storage(storage_entry)?.ty;

    let id = match ty {
        StorageEntryType::Plain(ty) => ty.id,
        StorageEntryType::Map { value, .. } => value.id,
    };

    let val = T::decode_with_metadata(bytes, id, metadata)?;
    Ok(val)
}
