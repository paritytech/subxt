// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::metadata::DecodeWithMetadata;
use codec::Encode;
pub use sp_runtime::traits::SignedExtension;

// We use this type a bunch, so export it from here.
pub use frame_metadata::StorageHasher;

/// A trait representing a storage address. Anything implementing this trait
/// can be used to fetch and iterate over storage entries.
pub trait StorageAddressT {
    /// Thye target type of the value that lives at this address?
    type Target: DecodeWithMetadata;
    /// Can an entry be fetched from this address?
    type IsFetchable;
    /// Can a default entry be obtained from this address?
    type IsDefaultable;
    /// Can this address be iterated over?
    type IsIterable;

    /// The name of the pallet that the entry lives under.
    fn pallet_name(&self) -> &str;

    /// The name of the entry in a given pallet that the item is at.
    fn entry_name(&self) -> &str;

    /// Output the non-prefix bytes; that is, any additional bytes that need
    /// to be appended to the key to dig into maps.
    fn append_entry_bytes(&self, bytes: &mut Vec<u8>);

    /// An optional hash which, if present, will be checked against
    /// the node metadata to confirm that the return type matches what
    /// we are expecting.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }

    /// Output the "prefix"; the bytes which encode the pallet
    /// name and entry name.
    ///
    /// There should be no need to override this.
    fn append_root_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.extend(&sp_core::twox_128(self.pallet_name().as_bytes()));
        bytes.extend(&sp_core::twox_128(self.entry_name().as_bytes()));
    }

    /// This is a helper which combines [`StorageAddressT::append_root_bytes()`]
    /// and [`StorageAddressT::append_entry_bytes`] and gives back all of the bytes
    /// that represent this storage address.
    ///
    /// There should be no need to override this.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.append_root_bytes(&mut bytes);
        self.append_entry_bytes(&mut bytes);
        bytes
    }

    /// This is a helper which returns bytes representing the root pallet/entry
    /// location of this address; useful for manually iterating over for instance.
    ///
    /// There should be no need to override this.
    fn to_root_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.append_root_bytes(&mut bytes);
        bytes
    }
}

/// Used to signal whether a [`StorageAddressT`] can be iterated,
/// fetched and returned with a default value in the type system.
pub struct Yes;

/// This is returned from storage accesses in the statically generated
/// code, and contains the information needed to find, validate and decode
/// the storage entry.
pub struct StaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable> {
    pallet_name: &'static str,
    entry_name: &'static str,
    // How to access the specific value at that storage address.
    storage_entry_key: Vec<StorageMapKey>,
    // Hash provided from static code for validation.
    validation_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

impl<ReturnTy, Fetchable, Defaultable, Iterable>
    StaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable>
where
    ReturnTy: DecodeWithMetadata,
{
    /// Create a new [`StaticStorageAddress`] that will be validated
    /// against node metadata using the hash given.
    pub fn new(
        pallet_name: &'static str,
        entry_name: &'static str,
        storage_entry_key: Vec<StorageMapKey>,
        hash: [u8; 32],
    ) -> Self {
        Self {
            pallet_name,
            entry_name,
            storage_entry_key,
            validation_hash: Some(hash),
            _marker: std::marker::PhantomData,
        }
    }

    /// Do not validate this storage entry prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            pallet_name: self.pallet_name,
            entry_name: self.entry_name,
            storage_entry_key: self.storage_entry_key,
            validation_hash: None,
            _marker: self._marker,
        }
    }

    // A common trait methods implemented to avoid needingto import the trait:

    /// Return bytes representing this storage entry.
    pub fn to_bytes(&self) -> Vec<u8> {
        StorageAddressT::to_bytes(self)
    }

    /// Return bytes representing the root of this storage entry (ie a hash of
    /// the pallet and entry name).
    pub fn to_root_bytes(&self) -> Vec<u8> {
        StorageAddressT::to_root_bytes(self)
    }
}

impl<ReturnTy, Fetchable, Defaultable, Iterable> StorageAddressT
    for StaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable>
where
    ReturnTy: DecodeWithMetadata,
{
    type Target = ReturnTy;
    type IsDefaultable = Defaultable;
    type IsIterable = Iterable;
    type IsFetchable = Fetchable;

    fn pallet_name(&self) -> &str {
        self.pallet_name
    }

    fn entry_name(&self) -> &str {
        self.entry_name
    }

    fn append_entry_bytes(&self, bytes: &mut Vec<u8>) {
        for entry in &self.storage_entry_key {
            entry.to_bytes(bytes);
        }
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

/// Storage key for a Map.
#[derive(Clone)]
pub struct StorageMapKey {
    value: Vec<u8>,
    hasher: StorageHasher,
}

impl StorageMapKey {
    /// Create a new [`StorageMapKey`] with the encoded data and the hasher.
    pub fn new<T: Encode>(value: T, hasher: StorageHasher) -> Self {
        Self {
            value: value.encode(),
            hasher,
        }
    }

    /// Convert this [`StorageMapKey`] into bytes and append them to some existing bytes.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) {
        match &self.hasher {
            StorageHasher::Identity => bytes.extend(&self.value),
            StorageHasher::Blake2_128 => bytes.extend(sp_core::blake2_128(bytes)),
            StorageHasher::Blake2_128Concat => {
                // adapted from substrate Blake2_128Concat::hash since StorageHasher is not public
                let v = sp_core::blake2_128(&self.value);
                let v = v.iter().chain(&self.value).cloned();
                bytes.extend(v);
            }
            StorageHasher::Blake2_256 => bytes.extend(sp_core::blake2_256(&self.value)),
            StorageHasher::Twox128 => bytes.extend(sp_core::twox_128(&self.value)),
            StorageHasher::Twox256 => bytes.extend(sp_core::twox_256(&self.value)),
            StorageHasher::Twox64Concat => {
                let v = sp_core::twox_64(&self.value);
                let v = v.iter().chain(&self.value).cloned();
                bytes.extend(v);
            }
        }
    }
}
