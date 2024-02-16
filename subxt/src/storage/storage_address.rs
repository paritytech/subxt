// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    dynamic::DecodedValueThunk,
    error::{Error, MetadataError, StorageAddressError},
    metadata::{DecodeWithMetadata, EncodeWithMetadata, Metadata},
    storage::utils::{strip_concat_hash_bytes, strip_storage_addess_root_bytes},
    utils::{Encoded, Static},
};
use derivative::Derivative;
use scale_encode::EncodeAsType;
use scale_info::TypeDef;
use std::borrow::Cow;
use subxt_metadata::{StorageEntryType, StorageHasher};

/// This represents a storage address. Anything implementing this trait
/// can be used to fetch and iterate over storage entries.
pub trait StorageAddress {
    /// The target type of the value that lives at this address.
    type Target: DecodeWithMetadata;
    /// The keys type used to construc this address.
    type Keys: StorageMultiKey;
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
#[derivative(
    Clone(bound = "Keys: Clone"),
    Debug(bound = "Keys: std::fmt::Debug"),
    Eq(bound = "Keys: std::cmp::Eq"),
    Ord(bound = "Keys: std::cmp::Ord"),
    PartialEq(bound = "Keys: std::cmp::PartialEq"),
    PartialOrd(bound = "Keys: std::cmp::PartialOrd")
)]
pub struct Address<Keys: StorageMultiKey, ReturnTy, Fetchable, Defaultable, Iterable> {
    pallet_name: Cow<'static, str>,
    entry_name: Cow<'static, str>,
    keys: Keys,
    validation_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

/// A storage key, mostly used for static encoded values.
/// The original value is only given during construction, but can be
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct StorageKey<K: ?Sized> {
    bytes: Static<Encoded>,
    _marker: std::marker::PhantomData<K>,
}

impl<K: codec::Encode + ?Sized> StorageKey<K> {
    /// Creates a new static storage key
    pub fn new(key: &K) -> Self {
        StorageKey {
            bytes: Static(Encoded(key.encode())),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the scale-encoded bytes that make up this key
    pub fn bytes(&self) -> &[u8] {
        &self.bytes.0 .0
    }
}

/// This trait should be implemented by anything that can be used as one or multiple storage keys.
pub trait StorageMultiKey {
    /// Iterator over the storage keys, each key implements EncodeAsType to
    /// give the corresponding bytes to a `StorageHasher`.
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType>;

    /// Attempts to decode the StorageMultiKey from a whole storage address.
    /// The key/keys can only be recovered if all hashers are concat-style hashers.
    /// Example: Imagine The `StorageMultiKey` is a tuple (A,B) and the hashers are: [Blake2_128Concat, Twox64Concat].
    /// Then the memory layout of the storage key is:
    /// ```txt
    /// | 8 bytes pallet hash | 8 bytes entry hash | 16 bytes hash of A | ... bytes of A | 8 bytes hash of B | ... bytes of B |
    /// ```
    /// Returns None, if any of the hashers is not a concat-style hasher.
    fn decode_from_address_bytes(
        address_bytes: &[u8],
        hashers_and_ty_ids: &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Option<Result<Self, Error>>
    where
        Self: Sized + 'static;
}

/// Implement `StorageMultiKey` for `()` which can be used for keyless storage entries
impl StorageMultiKey for () {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageMultiKey`.
        std::iter::empty()
    }

    fn decode_from_address_bytes(
        address_bytes: &[u8],
        _hashers_and_ty_ids: &[(StorageHasher, u32)],
        _metadata: &Metadata,
    ) -> Option<Result<Self, Error>> {
        if address_bytes.len() < 16 {
            return Some(Err(StorageAddressError::UnexpectedAddressBytes.into()));
        }
        Some(Ok(()))
    }
}

// Note: The ?Sized bound is necessary to support e.g. `StorageKey<[u8]>`.
impl<K: EncodeAsType + codec::Decode + ?Sized> StorageMultiKey for StorageKey<K> {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageMultiKey`.
        std::iter::once(&self.bytes as &dyn EncodeAsType)
    }

    fn decode_from_address_bytes(
        address_bytes: &[u8],
        hashers_and_ty_ids: &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Option<Result<Self, Error>>
    where
        Self: Sized + 'static,
    {
        let cursor = &mut &*address_bytes;
        if let Err(err) = strip_storage_addess_root_bytes(cursor) {
            return Some(Err(err.into()));
        }

        let Some((hasher, ty_id)) = hashers_and_ty_ids.first() else {
            return Some(Err(StorageAddressError::WrongNumberOfHashers {
                hashers: 0,
                fields: 1,
            }
            .into()));
        };
        decode_storage_key_from_hash(address_bytes, cursor, hasher, *ty_id, metadata)
    }
}

pub fn decode_storage_key_from_hash<K: ?Sized>(
    address_bytes: &[u8],
    cursor: &mut &[u8],
    hasher: &StorageHasher,
    ty_id: u32,
    metadata: &Metadata,
) -> Option<Result<StorageKey<K>, Error>> {
    if let Err(err) = strip_concat_hash_bytes(cursor, hasher)? {
        return Some(Err(err.into()));
    }
    let start_idx = address_bytes.len() - cursor.len();
    if let Err(err) = scale_decode::visitor::decode_with_visitor(
        cursor,
        ty_id,
        metadata.types(),
        scale_decode::visitor::IgnoreVisitor,
    ) {
        return Some(Err(scale_decode::Error::from(err).into()));
    }
    let end_idx = address_bytes.len() - cursor.len();
    let key_bytes = address_bytes[start_idx..end_idx].to_vec();
    let key = StorageKey {
        bytes: Static(Encoded(key_bytes)),
        _marker: std::marker::PhantomData::<K>,
    };
    Some(Ok(key))
}

impl StorageMultiKey for Vec<scale_value::Value> {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageMultiKey`.
        self.iter().map(|e| e as &dyn EncodeAsType)
    }

    fn decode_from_address_bytes(
        address_bytes: &[u8],
        hashers_and_ty_ids: &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Option<Result<Self, Error>>
    where
        Self: Sized + 'static,
    {
        let cursor = &mut &*address_bytes;
        if let Err(err) = strip_storage_addess_root_bytes(cursor) {
            return Some(Err(err.into()));
        }
        let mut hashers_and_ty_ids_iter = hashers_and_ty_ids.iter();

        let mut result: Vec<scale_value::Value> = vec![];
        while !cursor.is_empty() {
            let Some((hasher, ty_id)) = hashers_and_ty_ids_iter.next() else {
                // Still bytes left, but no hashers and type ids anymore to pull from: this is an unexpected error.
                return Some(Err(StorageAddressError::UnexpectedAddressBytes.into()));
            };
            if let Err(err) = strip_concat_hash_bytes(cursor, hasher)? {
                return Some(Err(err.into()));
            }
            match DecodedValueThunk::decode_with_metadata(cursor, *ty_id, metadata) {
                Ok(decoded) => {
                    match decoded.to_value() {
                        Ok(value) => result.push(value.remove_context()),
                        Err(err) => return Some(Err(err)),
                    };
                }
                Err(err) => return Some(Err(err)),
            }
        }
        Some(Ok(result))
    }
}

/// Generates StorageMultiKey implementations for tuples, e.g.
/// ```rs,norun
/// impl<A: EncodeAsType, B: EncodeAsType> StorageMultiKey for (StorageKey<A>, StorageKey<B>) {
///     fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
///         let arr = [&self.0 as &dyn EncodeAsType, &self.1 as &dyn EncodeAsType];
///         arr.into_iter()
///     }
/// }
/// ```
macro_rules! impl_tuples {
    ($($ty:ident $n:tt),+) => {{
        impl<$($ty: EncodeAsType + ?Sized),+> StorageMultiKey for ($( StorageKey<$ty >),+) {
            fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
                let arr = [$(
                    &self.$n.bytes as &dyn EncodeAsType
                ),+];
                arr.into_iter()
            }

            fn decode_from_address_bytes(
                address_bytes: &[u8],
                hashers_and_ty_ids: &[(StorageHasher, u32)],
                metadata: &Metadata,
            ) -> Option<Result<Self, Error>>
            where
                Self: Sized + 'static,
            {
                let cursor = &mut &*address_bytes;
                if let Err(err) = strip_storage_addess_root_bytes(cursor) {
                    return Some(Err(err.into()));
                }

                // The number of elements in this tuple.
                const LEN: usize = $((0*$n + 1)+)+0;

                // It is an error to not provide a hasher and type id for each element in this tuple.
                if hashers_and_ty_ids.len() < LEN {
                    return Some(Err(StorageAddressError::WrongNumberOfKeys{actual: hashers_and_ty_ids.len(), expected: LEN}.into()))
                }

                // Construct the tuple as a series of expressions.
                let tuple : Self = ( $(
                    {
                        // index is available, because of bounds check above; qed
                        let (hasher, ty_id) = &hashers_and_ty_ids[$n];
                        let key = match decode_storage_key_from_hash::<$ty>(address_bytes, cursor, hasher, *ty_id, metadata)? {
                            Ok(key) => key,
                            Err(err) => {
                                return Some(Err(err));
                            }
                        };
                        key
                    },
                )+);
                return Some(Ok(tuple))
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
    type Keys = Keys;
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

        if hashers.len() == 1 {
            // One hasher; hash a tuple of all SCALE encoded bytes with the one hash function.
            let mut input = Vec::new();
            let iter = keys_iter.zip(type_ids);
            for (key, type_id) in iter {
                key.encode_with_metadata(type_id, metadata, &mut input)?;
            }
            hash_bytes(&input, &hashers[0], bytes);
        } else if hashers.len() >= type_ids.len() {
            // A hasher per field; encode and hash each field independently.
            let iter = keys_iter.zip(type_ids).zip(hashers);
            for ((key, type_id), hasher) in iter {
                let mut input = Vec::new();
                key.encode_with_metadata(type_id, metadata, &mut input)?;
                hash_bytes(&input, hasher, bytes);
            }
        } else {
            // Provided more fields than hashers.
            return Err(StorageAddressError::WrongNumberOfHashers {
                hashers: hashers.len(),
                fields: type_ids.len(),
            }
            .into());
        }

        Ok(())
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
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
