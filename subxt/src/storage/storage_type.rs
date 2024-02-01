// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::storage_address::{StorageAddress, Yes};

use crate::{
    backend::{BackendExt, BlockRef},
    client::OnlineClientT,
    error::{Error, MetadataError},
    Config,
};
use codec::Decode;
use derivative::Derivative;
use futures::StreamExt;
use std::{future::Future, marker::PhantomData};
use subxt_core::metadata::{DecodeWithMetadata, Metadata};
use subxt_metadata::{PalletMetadata, StorageEntryMetadata, StorageEntryType};

/// This is returned from a couple of storage functions.
pub use crate::backend::StreamOfResults;

/// Query the runtime storage.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct Storage<T: Config, Client> {
    client: Client,
    block_ref: BlockRef<T::Hash>,
    _marker: PhantomData<T>,
}

impl<T: Config, Client> Storage<T, Client> {
    /// Create a new [`Storage`]
    pub(crate) fn new(client: Client, block_ref: BlockRef<T::Hash>) -> Self {
        Self {
            client,
            block_ref,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> Storage<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Fetch the raw encoded value at the key given.
    pub fn fetch_raw(
        &self,
        key: impl Into<Vec<u8>>,
    ) -> impl Future<Output = Result<Option<Vec<u8>>, Error>> + 'static {
        let client = self.client.clone();
        let key = key.into();
        // Keep this alive until the call is complete:
        let block_ref = self.block_ref.clone();
        // Manual future so lifetime not tied to api.storage().
        async move {
            let data = client
                .backend()
                .storage_fetch_value(key, block_ref.hash())
                .await?;
            Ok(data)
        }
    }

    /// Stream all of the raw keys underneath the key given
    pub fn fetch_raw_keys(
        &self,
        key: impl Into<Vec<u8>>,
    ) -> impl Future<Output = Result<StreamOfResults<Vec<u8>>, Error>> + 'static {
        let client = self.client.clone();
        let block_hash = self.block_ref.hash();
        let key = key.into();
        // Manual future so lifetime not tied to api.storage().
        async move {
            let keys = client
                .backend()
                .storage_fetch_descendant_keys(key, block_hash)
                .await?;
            Ok(keys)
        }
    }

    /// Fetch a decoded value from storage at a given address.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use subxt::{ PolkadotConfig, OnlineClient };
    ///
    /// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale")]
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
            let metadata = client.client.metadata();
            let (pallet, entry) =
                lookup_entry_details(address.pallet_name(), address.entry_name(), &metadata)?;

            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            validate_storage_address(address, pallet)?;

            // Look up the return type ID to enable DecodeWithMetadata:
            let lookup_bytes = super::utils::storage_address_bytes(address, &metadata)?;
            if let Some(data) = client.fetch_raw(lookup_bytes).await? {
                let val =
                    decode_storage_with_metadata::<Address::Target>(&mut &*data, &metadata, entry)?;
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
            let entry_name = address.entry_name();
            // Metadata validation happens via .fetch():
            if let Some(data) = client.fetch(address).await? {
                Ok(data)
            } else {
                let metadata = client.client.metadata();
                let (_pallet_metadata, storage_entry) =
                    lookup_entry_details(pallet_name, entry_name, &metadata)?;

                let return_ty_id = return_type_from_storage_entry_type(storage_entry.entry_type());
                let bytes = &mut storage_entry.default_bytes();

                let val = Address::Target::decode_with_metadata(bytes, return_ty_id, &metadata)?;
                Ok(val)
            }
        }
    }

    /// Returns an iterator of key value pairs.
    ///
    /// ```no_run
    /// use subxt::{ PolkadotConfig, OnlineClient };
    ///
    /// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale")]
    /// pub mod polkadot {}
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    ///
    /// // Address to the root of a storage entry that we'd like to iterate over.
    /// let address = polkadot::storage().xcm_pallet().version_notifiers_iter();
    ///
    /// // Iterate over keys and values at that address.
    /// let mut iter = api
    ///     .storage()
    ///     .at_latest()
    ///     .await
    ///     .unwrap()
    ///     .iter(address)
    ///     .await
    ///     .unwrap();
    ///
    /// while let Some(Ok((key, value))) = iter.next().await {
    ///     println!("Key: 0x{}", hex::encode(&key));
    ///     println!("Value: {}", value);
    /// }
    /// # }
    /// ```
    pub fn iter<Address>(
        &self,
        address: Address,
    ) -> impl Future<Output = Result<StreamOfResults<(Vec<u8>, Address::Target)>, Error>> + 'static
    where
        Address: StorageAddress<IsIterable = Yes> + 'static,
    {
        let client = self.client.clone();
        let block_ref = self.block_ref.clone();
        async move {
            let metadata = client.metadata();
            let (pallet, entry) =
                lookup_entry_details(address.pallet_name(), address.entry_name(), &metadata)?;

            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            validate_storage_address(&address, pallet)?;

            // Look up the return type for flexible decoding. Do this once here to avoid
            // potentially doing it every iteration if we used `decode_storage_with_metadata`
            // in the iterator.
            let return_type_id = return_type_from_storage_entry_type(entry.entry_type());

            // The address bytes of this entry:
            let address_bytes = super::utils::storage_address_bytes(&address, &metadata)?;

            let s = client
                .backend()
                .storage_fetch_descendant_values(address_bytes, block_ref.hash())
                .await?
                .map(move |kv| {
                    let kv = match kv {
                        Ok(kv) => kv,
                        Err(e) => return Err(e),
                    };
                    let val = Address::Target::decode_with_metadata(
                        &mut &*kv.value,
                        return_type_id,
                        &metadata,
                    )?;
                    Ok((kv.key, val))
                });

            let s = StreamOfResults::new(Box::pin(s));
            Ok(s)
        }
    }

    /// The storage version of a pallet.
    /// The storage version refers to the `frame_support::traits::Metadata::StorageVersion` type.
    pub async fn storage_version(&self, pallet_name: impl AsRef<str>) -> Result<u16, Error> {
        // check that the pallet exists in the metadata:
        self.client
            .metadata()
            .pallet_by_name(pallet_name.as_ref())
            .ok_or_else(|| MetadataError::PalletNameNotFound(pallet_name.as_ref().into()))?;

        // construct the storage key. This is done similarly in `frame_support::traits::metadata::StorageVersion::storage_key()`.
        pub const STORAGE_VERSION_STORAGE_KEY_POSTFIX: &[u8] = b":__STORAGE_VERSION__:";
        let mut key_bytes: Vec<u8> = vec![];
        key_bytes.extend(&sp_core_hashing::twox_128(pallet_name.as_ref().as_bytes()));
        key_bytes.extend(&sp_core_hashing::twox_128(
            STORAGE_VERSION_STORAGE_KEY_POSTFIX,
        ));

        // fetch the raw bytes and decode them into the StorageVersion struct:
        let storage_version_bytes = self.fetch_raw(key_bytes).await?.ok_or_else(|| {
            format!(
                "Unexpected: entry for storage version in pallet \"{}\" not found",
                pallet_name.as_ref()
            )
        })?;
        u16::decode(&mut &storage_version_bytes[..]).map_err(Into::into)
    }

    /// Fetch the runtime WASM code.
    pub async fn runtime_wasm_code(&self) -> Result<Vec<u8>, Error> {
        // note: this should match the `CODE` constant in `sp_core::storage::well_known_keys`
        const CODE: &str = ":code";
        self.fetch_raw(CODE.as_bytes()).await?.ok_or_else(|| {
            format!("Unexpected: entry for well known key \"{CODE}\" not found").into()
        })
    }
}

/// Validate a storage address against the metadata.
pub(crate) fn validate_storage_address<Address: StorageAddress>(
    address: &Address,
    pallet: PalletMetadata<'_>,
) -> Result<(), Error> {
    if let Some(hash) = address.validation_hash() {
        validate_storage(pallet, address.entry_name(), hash)?;
    }
    Ok(())
}

/// Return details about the given storage entry.
fn lookup_entry_details<'a>(
    pallet_name: &str,
    entry_name: &str,
    metadata: &'a Metadata,
) -> Result<(PalletMetadata<'a>, &'a StorageEntryMetadata), Error> {
    let pallet_metadata = metadata.pallet_by_name_err(pallet_name)?;
    let storage_metadata = pallet_metadata
        .storage()
        .ok_or_else(|| MetadataError::StorageNotFoundInPallet(pallet_name.to_owned()))?;
    let storage_entry = storage_metadata
        .entry_by_name(entry_name)
        .ok_or_else(|| MetadataError::StorageEntryNotFound(entry_name.to_owned()))?;
    Ok((pallet_metadata, storage_entry))
}

/// Validate a storage entry against the metadata.
fn validate_storage(
    pallet: PalletMetadata<'_>,
    storage_name: &str,
    hash: [u8; 32],
) -> Result<(), Error> {
    let Some(expected_hash) = pallet.storage_hash(storage_name) else {
        return Err(MetadataError::IncompatibleCodegen.into());
    };
    if expected_hash != hash {
        return Err(MetadataError::IncompatibleCodegen.into());
    }
    Ok(())
}

/// Fetch the return type out of a [`StorageEntryType`].
fn return_type_from_storage_entry_type(entry: &StorageEntryType) -> u32 {
    match entry {
        StorageEntryType::Plain(ty) => *ty,
        StorageEntryType::Map { value_ty, .. } => *value_ty,
    }
}

/// Given some bytes, a pallet and storage name, decode the response.
fn decode_storage_with_metadata<T: DecodeWithMetadata>(
    bytes: &mut &[u8],
    metadata: &Metadata,
    storage_metadata: &StorageEntryMetadata,
) -> Result<T, Error> {
    let ty = storage_metadata.entry_type();
    let return_ty = return_type_from_storage_entry_type(ty);
    let val = T::decode_with_metadata(bytes, return_ty, metadata)?;
    Ok(val)
}
