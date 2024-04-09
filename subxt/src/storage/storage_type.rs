// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    backend::{BackendExt, BlockRef},
    client::OnlineClientT,
    error::{Error, MetadataError, StorageAddressError},
    metadata::DecodeWithMetadata,
    Config,
};
use codec::Decode;
use derive_where::derive_where;
use futures::StreamExt;
use std::{future::Future, marker::PhantomData};
use subxt_core::storage::address::{AddressT, StorageHashers, StorageKey};
use subxt_core::utils::Yes;

/// This is returned from a couple of storage functions.
pub use crate::backend::StreamOfResults;

/// Query the runtime storage.
#[derive_where(Clone; Client)]
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
        Address: AddressT<IsFetchable = Yes> + 'address,
    {
        let client = self.clone();
        async move {
            let metadata = client.client.metadata();

            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            subxt_core::storage::validate(address, &metadata)?;

            // Look up the return type ID to enable DecodeWithMetadata:
            let lookup_bytes = subxt_core::storage::get_address_bytes(address, &metadata)?;
            if let Some(data) = client.fetch_raw(lookup_bytes).await? {
                let val = subxt_core::storage::decode_value(&mut &*data, address, &metadata)?;
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
        Address: AddressT<IsFetchable = Yes, IsDefaultable = Yes> + 'address,
    {
        let client = self.clone();
        async move {
            // Metadata validation happens via .fetch():
            if let Some(data) = client.fetch(address).await? {
                Ok(data)
            } else {
                let metadata = client.client.metadata();
                let val = subxt_core::storage::default_value(address, &metadata)?;
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
    /// while let Some(Ok(kv)) = iter.next().await {
    ///     println!("Key bytes: 0x{}", hex::encode(&kv.key_bytes));
    ///     println!("Value: {}", kv.value);
    /// }
    /// # }
    /// ```
    pub fn iter<Address>(
        &self,
        address: Address,
    ) -> impl Future<Output = Result<StreamOfResults<StorageKeyValuePair<Address>>, Error>> + 'static
    where
        Address: AddressT<IsIterable = Yes> + 'static,
        Address::Keys: 'static + Sized,
    {
        let client = self.client.clone();
        let block_ref = self.block_ref.clone();
        async move {
            let metadata = client.metadata();
            let (_pallet, entry) = subxt_core::storage::lookup_storage_entry_details(
                address.pallet_name(),
                address.entry_name(),
                &metadata,
            )?;

            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            subxt_core::storage::validate(&address, &metadata)?;

            // Look up the return type for flexible decoding. Do this once here to avoid
            // potentially doing it every iteration if we used `decode_storage_with_metadata`
            // in the iterator.
            let entry = entry.entry_type();

            let return_type_id = entry.value_ty();
            let hashers = StorageHashers::new(entry, metadata.types())?;

            // The address bytes of this entry:
            let address_bytes = subxt_core::storage::get_address_bytes(&address, &metadata)?;
            let s = client
                .backend()
                .storage_fetch_descendant_values(address_bytes, block_ref.hash())
                .await?
                .map(move |kv| {
                    let kv = match kv {
                        Ok(kv) => kv,
                        Err(e) => return Err(e),
                    };
                    let value = Address::Target::decode_with_metadata(
                        &mut &*kv.value,
                        return_type_id,
                        &metadata,
                    )?;

                    let key_bytes = kv.key;
                    let cursor = &mut &key_bytes[..];
                    strip_storage_address_root_bytes(cursor)?;

                    let keys = <Address::Keys as StorageKey>::decode_storage_key(
                        cursor,
                        &mut hashers.iter(),
                        metadata.types(),
                    )?;

                    Ok(StorageKeyValuePair::<Address> {
                        keys,
                        key_bytes,
                        value,
                    })
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
        key_bytes.extend(&sp_crypto_hashing::twox_128(
            pallet_name.as_ref().as_bytes(),
        ));
        key_bytes.extend(&sp_crypto_hashing::twox_128(
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

/// Strips the first 32 bytes (16 for the pallet hash, 16 for the entry hash) off some storage address bytes.
fn strip_storage_address_root_bytes(address_bytes: &mut &[u8]) -> Result<(), StorageAddressError> {
    if address_bytes.len() >= 32 {
        *address_bytes = &address_bytes[32..];
        Ok(())
    } else {
        Err(StorageAddressError::UnexpectedAddressBytes)
    }
}

/// A pair of keys and values together with all the bytes that make up the storage address.
/// `keys` is `None` if non-concat hashers are used. In this case the keys could not be extracted back from the key_bytes.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StorageKeyValuePair<T: AddressT> {
    /// The bytes that make up the address of the storage entry.
    pub key_bytes: Vec<u8>,
    /// The keys that can be used to construct the address of this storage entry.
    pub keys: T::Keys,
    /// The value of the storage entry.
    pub value: T::Target,
}
