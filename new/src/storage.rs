mod prefix_of;
mod storage_entry;
mod storage_key;
mod storage_key_value;
mod storage_value;

use crate::backend::BackendExt;
use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::StorageError;
use address::Address;
use core::marker::PhantomData;
use frame_decode::helpers::Entry;
use frame_decode::storage::StorageEntryInfo;
use std::borrow::Cow;

pub use prefix_of::PrefixOf;
pub use storage_entry::StorageEntry;
pub use storage_key::{StorageKey, StorageKeyPart};
pub use storage_key_value::StorageKeyValue;
pub use storage_value::StorageValue;
pub mod address;

/// A client for working with transactions.
#[derive(Clone)]
pub struct StorageClient<T, Client> {
    client: Client,
    marker: PhantomData<T>,
}

impl<T, Client> StorageClient<T, Client> {
    pub(crate) fn new(client: Client) -> Self {
        StorageClient {
            client,
            marker: PhantomData,
        }
    }
}

impl<T: Config, Client: OfflineClientAtBlockT<T>> StorageClient<T, Client> {
    /// When the provided `address` is statically generated via the `#[subxt]` macro, this validates
    /// that the shape of the storage value is the same as the shape expected by the static address.
    ///
    /// When the provided `address` is dynamic (and thus does not come with any expectation of the
    /// shape of the constant value), this just returns `Ok(())`
    pub fn validate<Addr: Address>(&self, address: Addr) -> Result<(), StorageError> {
        let Some(hash) = address.validation_hash() else {
            return Ok(());
        };

        let pallet_name = address.pallet_name();
        let entry_name = address.entry_name();

        let pallet_metadata = self
            .client
            .metadata_ref()
            .pallet_by_name(pallet_name)
            .ok_or_else(|| StorageError::PalletNameNotFound(pallet_name.to_string()))?;
        let storage_hash = pallet_metadata.storage_hash(entry_name).ok_or_else(|| {
            StorageError::StorageEntryNotFound {
                pallet_name: pallet_name.to_string(),
                entry_name: entry_name.to_string(),
            }
        })?;

        if storage_hash != hash {
            Err(StorageError::IncompatibleCodegen)
        } else {
            Ok(())
        }
    }

    /// This returns a [`StorageEntry`], which allows working with the storage entry at the provided address.
    pub fn entry<Addr: Address>(
        &self,
        address: Addr,
    ) -> Result<StorageEntry<'_, T, Client, Addr, Addr::IsPlain>, StorageError> {
        self.validate(&address)?;
        StorageEntry::new(&self.client, address)
    }

    /// Iterate over all of the storage entries listed in the metadata for the current block. This does **not** include well known
    /// storage entries like `:code` which are not listed in the metadata.
    pub fn entries(&self) -> impl Iterator<Item = StorageEntries<'_, Client, T>> {
        let metadata = self.client.metadata_ref();
        Entry::tuples_of(metadata.storage_entries()).map(|(pallet_name, entry_name)| {
            StorageEntries {
                pallet_name: pallet_name.clone(),
                entry_name,
                client: &self.client,
                marker: std::marker::PhantomData,
            }
        })
    }
}

impl<T: Config, Client: OnlineClientAtBlockT<T>> StorageClient<T, Client> {
    /// This is essentially a shorthand for `client.entry(addr)?.fetch(key_parts)`. See [`StorageEntry::fetch()`].
    pub async fn fetch<Addr: Address>(
        &self,
        addr: Addr,
        key_parts: Addr::KeyParts,
    ) -> Result<StorageValue<'_, Addr::Value>, StorageError> {
        let entry = self.entry(addr)?;
        entry.internal_fetch(key_parts).await
    }

    /// This is essentially a shorthand for `client.entry(addr)?.try_fetch(key_parts)`. See [`StorageEntry::try_fetch()`].
    pub async fn try_fetch<Addr: Address>(
        &self,
        addr: Addr,
        key_parts: Addr::KeyParts,
    ) -> Result<Option<StorageValue<'_, Addr::Value>>, StorageError> {
        let entry = self.entry(addr)?;
        entry.internal_try_fetch(key_parts).await
    }

    /// This is essentially a shorthand for `client.entry(addr)?.iter(key_parts)`. See [`StorageEntry::iter()`].
    pub async fn iter<Addr: Address, KeyParts: PrefixOf<Addr::KeyParts>>(
        &self,
        addr: Addr,
        key_parts: KeyParts,
    ) -> Result<
        impl futures::Stream<Item = Result<StorageKeyValue<'_, Addr>, StorageError>>
        + use<'_, Addr, Client, T, KeyParts>,
        StorageError,
    > {
        let entry = self.entry(addr)?;
        entry.internal_iter(key_parts).await
    }

    /// In rare cases, you may wish to fetch a storage value that does not live at a typical address. This method
    /// is a fallback for those cases, and allows you to provide the raw storage key bytes corresponding to the
    /// entry you wish to obtain. The response will either be the bytes for the value found at that location, or
    /// otherwise an error. [`StorageError::NoValueFound`] will be returned in the event that the request was valid
    /// but no value lives at the given location).
    pub async fn fetch_raw(&self, key_bytes: Vec<u8>) -> Result<Vec<u8>, StorageError> {
        let block_hash = self.client.block_hash();
        let value = self
            .client
            .backend()
            .storage_fetch_value(key_bytes, block_hash)
            .await
            .map_err(StorageError::CannotFetchValue)?
            .ok_or(StorageError::NoValueFound)?;

        Ok(value)
    }

    /// The storage version of a pallet.
    /// The storage version refers to the `frame_support::traits::Metadata::StorageVersion` type.
    pub async fn storage_version(&self, pallet_name: impl AsRef<str>) -> Result<u16, StorageError> {
        // construct the storage key. This is done similarly in
        // `frame_support::traits::metadata::StorageVersion::storage_key()`:
        let mut key_bytes: Vec<u8> = vec![];
        key_bytes.extend(&sp_crypto_hashing::twox_128(
            pallet_name.as_ref().as_bytes(),
        ));
        key_bytes.extend(&sp_crypto_hashing::twox_128(b":__STORAGE_VERSION__:"));

        // fetch the raw bytes and decode them into the StorageVersion struct:
        let storage_version_bytes = self.fetch_raw(key_bytes).await?;

        <u16 as codec::Decode>::decode(&mut &storage_version_bytes[..])
            .map_err(StorageError::CannotDecodeStorageVersion)
    }

    /// Fetch the runtime WASM code.
    pub async fn runtime_wasm_code(&self) -> Result<Vec<u8>, StorageError> {
        // note: this should match the `CODE` constant in `sp_core::storage::well_known_keys`
        self.fetch_raw(b":code".to_vec()).await
    }
}

/// Working with a specific storage entry.
pub struct StorageEntries<'atblock, Client, T> {
    pallet_name: Cow<'atblock, str>,
    entry_name: Cow<'atblock, str>,
    client: &'atblock Client,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, Client, T> StorageEntries<'atblock, Client, T>
where
    T: Config,
    Client: OfflineClientAtBlockT<T>,
{
    /// The pallet name.
    pub fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    /// The storage entry name.
    pub fn entry_name(&self) -> &str {
        &self.entry_name
    }

    /// Extract the relevant storage information so that we can work with this entry.
    pub fn entry(
        &self,
    ) -> Result<
        StorageEntry<'_, T, Client, address::DynamicAddress, crate::utils::Maybe>,
        StorageError,
    > {
        let addr = address::dynamic(self.pallet_name.to_owned(), self.entry_name.to_owned());
        StorageEntry::new(self.client, addr)
    }
}
