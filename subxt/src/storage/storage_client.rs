// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_address::{
    StorageAddressT,
    Yes,
};
use crate::{
    client::{
        OfflineClientT,
        OnlineClientT,
    },
    error::BasicError,
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
    pub fn validate<Address: StorageAddressT>(
        &self,
        address: &Address,
    ) -> Result<(), BasicError> {
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
    ) -> impl Future<Output = Result<Option<Vec<u8>>, BasicError>> + 'a {
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
        Output = Result<
            Option<<Address::Target as DecodeWithMetadata>::Target>,
            BasicError,
        >,
    > + 'a
    where
        Address: StorageAddressT<IsFetchable = Yes> + 'a,
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
            let return_ty_id = lookup_storage_return_type(
                &metadata,
                address.pallet_name(),
                address.entry_name(),
            )?;

            let lookup_bytes = address.to_bytes();
            if let Some(data) = client
                .client
                .storage()
                .fetch_raw(&lookup_bytes, hash)
                .await?
            {
                let val = <Address::Target as DecodeWithMetadata>::decode_with_metadata(
                    &mut &*data,
                    return_ty_id,
                    &metadata,
                )?;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }
    }

    /// Fetch a StorageKey that has a default value with an optional block hash.
    ///
    /// Note: The [`StorageAddress`] provided must be tagged with [`AddressHasDefaultValue`]
    /// in order to use this function. Statically generated storage addresses will be
    /// tagged appropriately.
    pub fn fetch_or_default<'a, Address>(
        &self,
        address: &'a Address,
        hash: Option<T::Hash>,
    ) -> impl Future<
        Output = Result<<Address::Target as DecodeWithMetadata>::Target, BasicError>,
    > + 'a
    where
        Address: StorageAddressT<IsFetchable = Yes, IsDefaultable = Yes> + 'a,
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
    ) -> impl Future<Output = Result<Vec<StorageKey>, BasicError>> + 'a {
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
    /// Note: The [`StorageAddress`] provided must be tagged with [`AddressIsIterable`]
    /// in order to use this function. Statically generated storage addresses will be
    /// tagged appropriately.
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
    ) -> impl Future<Output = Result<KeyIter<T, Client, Address::Target>, BasicError>> + 'static
    where
        Address: StorageAddressT<IsIterable = Yes> + 'static,
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

            // Look up the return type for flexible decoding:
            let return_type_id = lookup_storage_return_type(
                &metadata,
                address.pallet_name(),
                address.entry_name(),
            )?;

            // The root pallet/entry bytes for this storage entry:
            let mut address_root_bytes = Vec::new();
            address.append_root_bytes(&mut address_root_bytes);

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
    ) -> Result<Option<(StorageKey, ReturnTy::Target)>, BasicError> {
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
) -> Result<(), BasicError> {
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
) -> Result<u32, BasicError> {
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
