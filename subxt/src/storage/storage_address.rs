// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    dynamic::DecodedValueThunk,
    error::{Error, MetadataError, StorageAddressError},
    metadata::{DecodeWithMetadata, EncodeWithMetadata, Metadata},
    utils::{Encoded, Static},
};
use derivative::Derivative;
use scale_encode::EncodeAsType;
use scale_info::TypeDef;
use std::borrow::Cow;
use subxt_metadata::{StorageEntryMetadata, StorageEntryType, StorageHasher};

/// This represents a storage address. Anything implementing this trait
/// can be used to fetch and iterate over storage entries.
pub trait StorageAddress {
    /// The target type of the value that lives at this address.
    type Target: DecodeWithMetadata;
    /// Can an entry be fetched from this address?
    /// Set this type to [`Yes`] to enable the corresponding calls to be made.
    type IsFetchable;
    /// Can a default entry be obtained from this address?
    /// Set this type to [`Yes`] to enable the corresponding calls to be made.
    type IsDefaultable;
    /// Can this address be iterated over?
    /// Set this type to [`Yes`] to enable the corresponding calls to be made.
    type IsIterable;

    /// The name of the pallet that the entry lives under.
    fn pallet_name(&self) -> &str;

    /// The name of the entry in a given pallet that the item is at.
    fn entry_name(&self) -> &str;

    /// Output the non-prefix bytes; that is, any additional bytes that need
    /// to be appended to the key to dig into maps.
    fn append_entry_bytes(&self, metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error>;

    /// An optional hash which, if present, will be checked against
    /// the node metadata to confirm that the return type matches what
    /// we are expecting.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

/// Used to signal whether a [`StorageAddress`] can be iterated,
/// fetched and returned with a default value in the type system.
pub struct Yes;

/// A concrete storage address. This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`].
#[derive(Derivative)]
#[derivative(Clone(bound = "Keys: Clone"), Debug(bound = "Keys: std::fmt::Debug"))]
pub struct Address<Keys: StorageMultiKey, ReturnTy, Fetchable, Defaultable, Iterable> {
    pallet_name: Cow<'static, str>,
    entry_name: Cow<'static, str>,
    keys: Keys,
    validation_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

// #[derive(Derivative)]
// #[derivative(Clone(bound = "K: Clone"), Debug(bound = "K: std::fmt::Debug"))]
// pub enum StorageKey<K: EncodeAsType> {
//     Encoded(Static<Encoded>),
//     Bare(K),
// }

// impl<K: codec::Encode + EncodeAsType> StorageKey<K> {
//     pub fn new_encoded(key_ty: &K) -> StorageKey<K> {
//         StorageKey::Encoded(Static(Encoded(key_ty.encode())))
//     }
// }

// impl<K: EncodeAsType> EncodeAsType for StorageKey<K> {
//     fn encode_as_type_to(
//         &self,
//         type_id: u32,
//         types: &scale_info::PortableRegistry,
//         out: &mut Vec<u8>,
//     ) -> Result<(), scale_encode::Error> {
//         match self {
//             StorageKey::Encoded(e) => e.encode_as_type_to(type_id, types, out),
//             StorageKey::Bare(e) => e.encode_as_type_to(type_id, types, out),
//         }
//     }
// }

/// A storage key, mostly used for static encoded values.
/// The original value is only given during construction, but can be
pub struct StorageKey<K> {
    bytes: Static<Encoded>,
    _marker: std::marker::PhantomData<K>,
}

impl<K: EncodeAsType + codec::Encode + codec::Decode> StorageKey<K> {
    pub fn new(key: &K) -> Self {
        StorageKey {
            bytes: Static(Encoded(key.encode())),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes.0 .0
    }
}

// impl<K: codec::Encode> EncodeAsType for StorageKey<K> {
//     fn encode_as_type_to(
//         &self,
//         type_id: u32,
//         types: &scale_info::PortableRegistry,
//         out: &mut Vec<u8>,
//     ) -> Result<(), scale_encode::Error> {
//         self.bytes.encode_as_type_to(type_id, types, out)
//     }
// }

pub trait StorageMultiKey {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType>;
}

/// Implement `StorageMultiKey` for `()` which can be used for keyless storage entries
impl StorageMultiKey for () {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageMultiKey`.
        std::iter::empty()
    }
}

impl<K: EncodeAsType> StorageMultiKey for StorageKey<K> {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageMultiKey`.
        std::iter::once(&self.bytes as &dyn EncodeAsType)
    }
}

impl<K: EncodeAsType> StorageMultiKey for Vec<K> {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageMultiKey`.
        self.iter().map(|e| e as &dyn EncodeAsType)
    }
}

// impl<A: EncodeAsType, B: EncodeAsType> StorageMultiKey for (StorageKey<A>, StorageKey<B>) {
//     fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
//         let arr = [&self.0 as &dyn EncodeAsType, &self.1 as &dyn EncodeAsType];
//         arr.into_iter()
//     }
// }

macro_rules! impl_tuples {
    ($($ty:ident $n:tt),+) => {{
        impl<$($ty: EncodeAsType),+> StorageMultiKey for ($( StorageKey<$ty >),+) {
            fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
                let arr = [$(
                    &self.$n.bytes as &dyn EncodeAsType
                ),+];
                arr.into_iter()
            }
        }
    }};
}

#[rustfmt::skip]
const _: () = {
    impl_tuples!(A 0, B 1);
    impl_tuples!(A 0, B 1, C 2);
    impl_tuples!(A 0, B 1, C 2, D 3);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
};

// todo! impl MultiStorageKey for Vec<StorageKey<K>> and for (StorageKey<K1>, StorageKey<K2>), ...
// impl MultiStorageKey

/// A typical storage address constructed at runtime rather than via the `subxt` macro; this
/// has no restriction on what it can be used for (since we don't statically know).
pub type DynamicAddress<Keys> = Address<Keys, DecodedValueThunk, Yes, Yes, Yes>;

impl<Keys: StorageMultiKey> DynamicAddress<Keys> {
    /// Creates a new dynamic address. As `Keys` you can use a `Vec<scale_value::Value>`
    pub fn new(pallet_name: impl Into<String>, entry_name: impl Into<String>, keys: Keys) -> Self {
        Self {
            pallet_name: Cow::Owned(pallet_name.into()),
            entry_name: Cow::Owned(entry_name.into()),
            keys,
            validation_hash: None,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
    Address<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
where
    Keys: StorageMultiKey,
    ReturnTy: DecodeWithMetadata,
{
    /// Create a new [`Address`] using static strings for the pallet and call name.
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        entry_name: &'static str,
        keys: Keys,
        hash: [u8; 32],
    ) -> Self {
        Self {
            pallet_name: Cow::Borrowed(pallet_name),
            entry_name: Cow::Borrowed(entry_name),
            keys,
            validation_hash: Some(hash),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
    Address<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
where
    Keys: StorageMultiKey,
    ReturnTy: DecodeWithMetadata,
{
    /// Do not validate this storage entry prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            validation_hash: None,
            ..self
        }
    }

    /// Return bytes representing the root of this storage entry (ie a hash of
    /// the pallet and entry name). Use [`crate::storage::StorageClient::address_bytes()`]
    /// to obtain the bytes representing the entire address.
    pub fn to_root_bytes(&self) -> Vec<u8> {
        super::utils::storage_address_root_bytes(self)
    }
}

impl<Keys, ReturnTy, Fetchable, Defaultable, Iterable> StorageAddress
    for Address<Keys, ReturnTy, Fetchable, Defaultable, Iterable>
where
    Keys: StorageMultiKey,
    ReturnTy: DecodeWithMetadata,
{
    type Target = ReturnTy;
    type IsFetchable = Fetchable;
    type IsDefaultable = Defaultable;
    type IsIterable = Iterable;

    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn entry_name(&self) -> &str {
        &self.entry_name
    }

    fn append_entry_bytes(&self, metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error> {
        let pallet = metadata.pallet_by_name_err(self.pallet_name())?;
        let storage = pallet
            .storage()
            .ok_or_else(|| MetadataError::StorageNotFoundInPallet(self.pallet_name().to_owned()))?;
        let entry = storage
            .entry_by_name(self.entry_name())
            .ok_or_else(|| MetadataError::StorageEntryNotFound(self.entry_name().to_owned()))?;

        let keys_iter = self.keys.keys_iter();
        let keys_len = keys_iter.len();

        if keys_len == 0 {
            return Ok(());
        }

        let StorageEntryType::Map {
            hashers, key_ty, ..
        } = entry.entry_type()
        else {
            // Plain entries are only okay, if keys_len == 0, see early return above.
            return Err(StorageAddressError::WrongNumberOfKeys {
                expected: 0,
                actual: keys_len,
            }
            .into());
        };

        let ty = metadata
            .types()
            .resolve(*key_ty)
            .ok_or(MetadataError::TypeNotFound(*key_ty))?;

        // If the key is a tuple, we encode each value to the corresponding tuple type.
        // If the key is not a tuple, encode a single value to the key type.
        let type_ids = match &ty.type_def {
            TypeDef::Tuple(tuple) => either::Either::Left(tuple.fields.iter().map(|f| f.id)),
            _other => either::Either::Right(std::iter::once(*key_ty)),
        };

        // Provided more fields than hashers: This is unacceptable (on the contrary, providing more hashers than keys is ok)
        if hashers.len() < type_ids.len() {
            return Err(StorageAddressError::WrongNumberOfHashers {
                hashers: hashers.len(),
                fields: type_ids.len(),
            }
            .into());
        }

        if hashers.len() == 1 {
            // One hasher; hash a tuple of all SCALE encoded bytes with the one hash function.
            let mut input = Vec::new();
            let iter = keys_iter.zip(type_ids);
            for (key, type_id) in iter {
                key.encode_with_metadata(type_id, metadata, &mut input)?;
            }
            hash_bytes(&input, &hashers[0], bytes);
        } else {
            // A hasher per field; encode and hash each field independently.
            let iter = keys_iter.zip(type_ids).zip(hashers);
            for ((key, type_id), hasher) in iter {
                let mut input = Vec::new();
                key.encode_with_metadata(type_id, metadata, &mut input)?;
                hash_bytes(&input, hasher, bytes);
            }
        }

        Ok(())
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

/// A static storage key; this is some pre-encoded bytes
/// likely provided by the generated interface.
pub type StaticStorageMapKey = Static<Encoded>;

// Used in codegen to construct the above.
#[doc(hidden)]
pub fn make_static_storage_map_key<T: codec::Encode>(t: T) -> StaticStorageMapKey {
    Static(Encoded(t.encode()))
}

/// Construct a new dynamic storage lookup.
pub fn dynamic<Keys: StorageMultiKey>(
    pallet_name: impl Into<String>,
    entry_name: impl Into<String>,
    storage_entry_keys: Keys,
) -> DynamicAddress<Keys> {
    DynamicAddress::new(pallet_name, entry_name, storage_entry_keys)
}

/// Take some SCALE encoded bytes and a [`StorageHasher`] and hash the bytes accordingly.
fn hash_bytes(input: &[u8], hasher: &StorageHasher, bytes: &mut Vec<u8>) {
    match hasher {
        StorageHasher::Identity => bytes.extend(input),
        StorageHasher::Blake2_128 => bytes.extend(sp_core_hashing::blake2_128(input)),
        StorageHasher::Blake2_128Concat => {
            bytes.extend(sp_core_hashing::blake2_128(input));
            bytes.extend(input);
        }
        StorageHasher::Blake2_256 => bytes.extend(sp_core_hashing::blake2_256(input)),
        StorageHasher::Twox128 => bytes.extend(sp_core_hashing::twox_128(input)),
        StorageHasher::Twox256 => bytes.extend(sp_core_hashing::twox_256(input)),
        StorageHasher::Twox64Concat => {
            bytes.extend(sp_core_hashing::twox_64(input));
            bytes.extend(input);
        }
    }
}

/// Tries to recover from the hash, the bytes of the value that was originially hashed.
/// Note: this only returns `Some(..)` for concat-style hashers.
pub fn value_bytes_from_hash_bytes<'a>(hash: &'a [u8], hasher: &StorageHasher) -> Option<&'a [u8]> {
    match hasher {
        StorageHasher::Blake2_128Concat => Some(&hash[16..0]),
        StorageHasher::Twox64Concat => Some(&hash[8..0]),
        StorageHasher::Blake2_128
        | StorageHasher::Blake2_256
        | StorageHasher::Twox128
        | StorageHasher::Twox256
        | StorageHasher::Identity => None,
    }
}

/// Tries to recover an encoded value from a concat-style hash.
pub fn value_from_hash_bytes<V: codec::Encode + codec::Decode>(
    hash: &[u8],
    hasher: &StorageHasher,
) -> Option<Result<V, Error>> {
    let value_bytes = value_bytes_from_hash_bytes(hash, hasher)?;
    let value = match V::decode(&mut &value_bytes[..]) {
        Ok(value) => value,
        Err(err) => return Some(Err(err.into())),
    };
    Some(Ok(value))
}
