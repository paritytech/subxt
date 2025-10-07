// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    backend::{BackendExt, BlockRef},
    client::{OfflineClientT, OnlineClientT},
    config::{Config, HashFor},
    error::StorageError,
};
use derive_where::derive_where;
use futures::StreamExt;
use std::marker::PhantomData;
use subxt_core::Metadata;
use subxt_core::storage::{PrefixOf, address::Address};
use subxt_core::utils::{Maybe, Yes};

pub use subxt_core::storage::{StorageKeyValue, StorageValue};

/// Query the runtime storage.
#[derive_where(Clone; Client)]
pub struct StorageClientAt<T: Config, Client> {
    client: Client,
    metadata: Metadata,
    block_ref: BlockRef<HashFor<T>>,
    _marker: PhantomData<T>,
}

impl<T, Client> StorageClientAt<T, Client>
where
    T: Config,
    Client: OfflineClientT<T>,
{
    /// Create a new [`StorageClientAt`].
    pub(crate) fn new(client: Client, block_ref: BlockRef<HashFor<T>>) -> Self {
        // Retrieve and store metadata here so that we can borrow it in
        // subsequent structs, and thus also borrow storage info and
        // things that borrow from metadata.
        let metadata = client.metadata();

        Self {
            client,
            metadata,
            block_ref,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> StorageClientAt<T, Client>
where
    T: Config,
    Client: OfflineClientT<T>,
{
    /// This returns a [`StorageEntryClient`], which allows working with the storage entry at the provided address.
    pub fn entry<Addr: Address>(
        &'_ self,
        address: Addr,
    ) -> Result<StorageEntryClient<'_, T, Client, Addr, Addr::IsPlain>, StorageError> {
        let inner = subxt_core::storage::entry(address, &self.metadata)?;
        Ok(StorageEntryClient {
            inner,
            client: self.client.clone(),
            block_ref: self.block_ref.clone(),
            _marker: core::marker::PhantomData,
        })
    }
}

/// This represents a single storage entry (be it a plain value or map)
/// and the operations that can be performed on it.
pub struct StorageEntryClient<'atblock, T: Config, Client, Addr, IsPlain> {
    inner: subxt_core::storage::StorageEntry<'atblock, Addr, IsPlain>,
    client: Client,
    block_ref: BlockRef<HashFor<T>>,
    _marker: PhantomData<T>,
}

impl<'atblock, T, Client, Addr, IsPlain> StorageEntryClient<'atblock, T, Client, Addr, IsPlain>
where
    T: Config,
    Addr: Address,
{
    /// Name of the pallet containing this storage entry.
    pub fn pallet_name(&self) -> &str {
        self.inner.pallet_name()
    }

    /// Name of the storage entry.
    pub fn entry_name(&self) -> &str {
        self.inner.entry_name()
    }

    /// Is the storage entry a plain value?
    pub fn is_plain(&self) -> bool {
        self.inner.is_plain()
    }

    /// Is the storage entry a map?
    pub fn is_map(&self) -> bool {
        self.inner.is_map()
    }

    /// Return the default value for this storage entry, if there is one. Returns `None` if there
    /// is no default value.
    pub fn default_value(&self) -> Option<StorageValue<'_, 'atblock, Addr::Value>> {
        self.inner.default_value()
    }
}

// Plain values get a fetch method with no extra arguments.
impl<'atblock, T, Client, Addr> StorageEntryClient<'atblock, T, Client, Addr, Yes>
where
    T: Config,
    Addr: Address,
    Client: OnlineClientT<T>,
{
    pub async fn fetch(&'_ self) -> Result<StorageValue<'_, 'atblock, Addr::Value>, StorageError> {
        let value = self.try_fetch().await?.map_or_else(
            || self.inner.default_value().ok_or(StorageError::NoValueFound),
            Ok,
        )?;

        Ok(value)
    }

    pub async fn try_fetch(
        &self,
    ) -> Result<Option<StorageValue<'_, 'atblock, Addr::Value>>, StorageError> {
        let value = self
            .client
            .backend()
            .storage_fetch_value(self.key_prefix().to_vec(), self.block_ref.hash())
            .await
            .map_err(StorageError::CannotFetchValue)?
            .map(|bytes| self.inner.value(bytes));

        Ok(value)
    }

    /// The keys for plain storage values are always 32 byte hashes.
    pub fn key_prefix(&self) -> [u8; 32] {
        self.inner.key_prefix()
    }
}

// When HasDefaultValue = Yes, we expect there to exist a valid default value and will use that
// if we fetch an entry and get nothing back.
impl<'atblock, T, Client, Addr> StorageEntryClient<'atblock, T, Client, Addr, Maybe>
where
    T: Config,
    Addr: Address,
    Client: OnlineClientT<T>,
{
    pub async fn fetch(
        &'_ self,
        keys: Addr::KeyParts,
    ) -> Result<StorageValue<'_, 'atblock, Addr::Value>, StorageError> {
        let value = self
            .try_fetch(keys)
            .await?
            .or_else(|| self.default_value())
            .unwrap();

        Ok(value)
    }

    pub async fn try_fetch(
        &self,
        keys: Addr::KeyParts,
    ) -> Result<Option<StorageValue<'_, 'atblock, Addr::Value>>, StorageError> {
        let key = self.inner.fetch_key(keys)?;

        let value = self
            .client
            .backend()
            .storage_fetch_value(key, self.block_ref.hash())
            .await
            .map_err(StorageError::CannotFetchValue)?
            .map(|bytes| self.inner.value(bytes))
            .or_else(|| self.default_value());

        Ok(value)
    }

    pub async fn iter<Keys: PrefixOf<Addr::KeyParts>>(
        &self,
        keys: Keys,
    ) -> Result<
        impl futures::Stream<Item = Result<StorageKeyValue<'_, 'atblock, Addr>, StorageError>>,
        StorageError,
    > {
        let key_bytes = self.inner.iter_key(keys)?;
        let block_hash = self.block_ref.hash();

        let stream = self
            .client
            .backend()
            .storage_fetch_descendant_values(key_bytes, block_hash)
            .await
            .map_err(StorageError::CannotIterateValues)?
            .map(|kv| {
                let kv = match kv {
                    Ok(kv) => kv,
                    Err(e) => return Err(StorageError::StreamFailure(e)),
                };
                Ok(self.inner.key_value(kv.key, kv.value))
            });

        Ok(Box::pin(stream))
    }

    /// The first 32 bytes of the storage entry key, which points to the entry but not necessarily
    /// a single storage value (unless the entry is a plain value).
    pub fn key_prefix(&self) -> [u8; 32] {
        self.inner.key_prefix()
    }
}

/*
impl<T, Client> Storage<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Fetch the raw encoded value at the key given.
    pub fn fetch_raw(
        &self,
        key: impl Into<Vec<u8>>,
    ) -> impl Future<Output = Result<Option<Vec<u8>>, StorageError>> + 'static {
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
    ) -> impl Future<Output = Result<StreamOfResults<Vec<u8>>, StorageError>> + 'static {
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
    /// ```rust,no_run,standalone_crate
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
    /// let address = polkadot::storage().xcm_pallet().queries(12345);
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
    pub fn fetch<'address, Addr>(
        &self,
        address: &'address Addr,
    ) -> impl Future<Output = Result<Option<Addr::Target>, StorageError>> + use<'address, Addr, Client, T>
    where
        Addr: Address<IsFetchable = Yes> + 'address,
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
    pub fn fetch_or_default<'address, Addr>(
        &self,
        address: &'address Addr,
    ) -> impl Future<Output = Result<Addr::Target, StorageError>> + use<'address, Addr, Client, T>
    where
        Addr: Address<IsFetchable = Yes, IsDefaultable = Yes> + 'address,
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
    /// ```rust,no_run,standalone_crate
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
    pub fn iter<Addr>(
        &self,
        address: Addr,
    ) -> impl Future<Output = Result<StreamOfResults<StorageKeyValuePair<Addr>>, StorageError>> + 'static
    where
        Addr: Address<IsIterable = Yes> + 'static,
        Addr::Keys: 'static + Sized,
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
                    let value = Addr::Target::decode_with_metadata(
                        &mut &*kv.value,
                        return_type_id,
                        &metadata,
                    )?;

                    let key_bytes = kv.key;
                    let cursor = &mut &key_bytes[..];
                    strip_storage_address_root_bytes(cursor)?;

                    let keys = <Addr::Keys as StorageKey>::decode_storage_key(
                        cursor,
                        &mut hashers.iter(),
                        metadata.types(),
                    )?;

                    Ok(StorageKeyValuePair::<Addr> {
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
    pub async fn storage_version(&self, pallet_name: impl AsRef<str>) -> Result<u16, StorageError> {
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
    pub async fn runtime_wasm_code(&self) -> Result<Vec<u8>, StorageError> {
        // note: this should match the `CODE` constant in `sp_core::storage::well_known_keys`
        const CODE: &str = ":code";
        self.fetch_raw(CODE.as_bytes()).await?.ok_or_else(|| {
            format!("Unexpected: entry for well known key \"{CODE}\" not found").into()
        })
    }
}

/// Strips the first 32 bytes (16 for the pallet hash, 16 for the entry hash) off some storage address bytes.
fn strip_storage_address_root_bytes(address_bytes: &mut &[u8]) -> Result<(), StorageError> {
    if address_bytes.len() >= 32 {
        *address_bytes = &address_bytes[32..];
        Ok(())
    } else {
        Err(StorageError::UnexpectedAddressBytes)
    }
}

/// A pair of keys and values together with all the bytes that make up the storage address.
/// `keys` is `None` if non-concat hashers are used. In this case the keys could not be extracted back from the key_bytes.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StorageKeyValuePair<T: Address> {
    /// The bytes that make up the address of the storage entry.
    pub key_bytes: Vec<u8>,
    /// The keys that can be used to construct the address of this storage entry.
    pub keys: T::Keys,
    /// The value of the storage entry.
    pub value: T::Target,
}
*/
