// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::utils::hash_bytes;
use crate::error::{Error, MetadataError, StorageAddressError};
use alloc::vec;
use alloc::vec::Vec;
use scale_decode::{DecodeAsType, visitor::IgnoreVisitor};
use scale_encode::EncodeAsType;
use scale_info::{PortableRegistry, TypeDef};
use scale_value::Value;
use subxt_metadata::{StorageEntryType, StorageHasher};

/// A collection of storage hashers paired with the type ids of the types they should hash.
/// Can be created for each storage entry in the metadata via [`StorageHashers::new()`].
#[derive(Debug)]
pub struct StorageHashers {
    hashers_and_ty_ids: Vec<(StorageHasher, u32)>,
}

impl StorageHashers {
    /// Creates new [`StorageHashers`] from a storage entry. Looks at the [`StorageEntryType`] and
    /// assigns a hasher to each type id that makes up the key.
    pub fn new(storage_entry: &StorageEntryType, types: &PortableRegistry) -> Result<Self, Error> {
        let mut hashers_and_ty_ids = vec![];
        if let StorageEntryType::Map {
            hashers, key_ty, ..
        } = storage_entry
        {
            let ty = types
                .resolve(*key_ty)
                .ok_or(MetadataError::TypeNotFound(*key_ty))?;

            if hashers.len() == 1 {
                // If there's exactly 1 hasher, then we have a plain StorageMap. We can't
                // break the key down (even if it's a tuple) because the hasher applies to
                // the whole key.
                hashers_and_ty_ids = vec![(hashers[0], *key_ty)];
            } else {
                // If there are multiple hashers, then we have a StorageDoubleMap or StorageNMap.
                // We expect the key type to be tuple, and we will return a MapEntryKey for each
                // key in the tuple.
                let hasher_count = hashers.len();
                let tuple = match &ty.type_def {
                    TypeDef::Tuple(tuple) => tuple,
                    _ => {
                        return Err(StorageAddressError::WrongNumberOfHashers {
                            hashers: hasher_count,
                            fields: 1,
                        }
                        .into());
                    }
                };

                // We should have the same number of hashers and keys.
                let key_count = tuple.fields.len();
                if hasher_count != key_count {
                    return Err(StorageAddressError::WrongNumberOfHashers {
                        hashers: hasher_count,
                        fields: key_count,
                    }
                    .into());
                }

                // Collect them together.
                hashers_and_ty_ids = tuple
                    .fields
                    .iter()
                    .zip(hashers)
                    .map(|(field, hasher)| (*hasher, field.id))
                    .collect();
            }
        }

        Ok(Self { hashers_and_ty_ids })
    }

    /// Creates an iterator over the storage hashers and type ids.
    pub fn iter(&self) -> StorageHashersIter<'_> {
        StorageHashersIter {
            hashers: self,
            idx: 0,
        }
    }
}

/// An iterator over all type ids of the key and the respective hashers.
/// See [`StorageHashers::iter()`].
#[derive(Debug)]
pub struct StorageHashersIter<'a> {
    hashers: &'a StorageHashers,
    idx: usize,
}

impl StorageHashersIter<'_> {
    fn next_or_err(&mut self) -> Result<(StorageHasher, u32), Error> {
        self.next().ok_or_else(|| {
            StorageAddressError::TooManyKeys {
                expected: self.hashers.hashers_and_ty_ids.len(),
            }
            .into()
        })
    }
}

impl Iterator for StorageHashersIter<'_> {
    type Item = (StorageHasher, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.hashers.hashers_and_ty_ids.get(self.idx).copied()?;
        self.idx += 1;
        Some(item)
    }
}

impl ExactSizeIterator for StorageHashersIter<'_> {
    fn len(&self) -> usize {
        self.hashers.hashers_and_ty_ids.len() - self.idx
    }
}

/// This trait should be implemented by anything that can be used as one or multiple storage keys.
pub trait StorageKey {
    /// Encodes the storage key into some bytes
    fn encode_storage_key(
        &self,
        bytes: &mut Vec<u8>,
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<(), Error>;

    /// Attempts to decode the StorageKey given some bytes and a set of hashers and type IDs that they are meant to represent.
    /// The bytes passed to `decode` should start with:
    /// - 1. some fixed size hash (for all hashers except `Identity`)
    /// - 2. the plain key value itself (for `Identity`, `Blake2_128Concat` and `Twox64Concat` hashers)
    fn decode_storage_key(
        bytes: &mut &[u8],
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<Self, Error>
    where
        Self: Sized + 'static;
}

/// Implement `StorageKey` for `()` which can be used for keyless storage entries,
/// or to otherwise just ignore some entry.
impl StorageKey for () {
    fn encode_storage_key(
        &self,
        _bytes: &mut Vec<u8>,
        hashers: &mut StorageHashersIter,
        _types: &PortableRegistry,
    ) -> Result<(), Error> {
        _ = hashers.next_or_err();
        Ok(())
    }

    fn decode_storage_key(
        bytes: &mut &[u8],
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<Self, Error> {
        let (hasher, ty_id) = match hashers.next_or_err() {
            Ok((hasher, ty_id)) => (hasher, ty_id),
            Err(_) if bytes.is_empty() => return Ok(()),
            Err(err) => return Err(err),
        };
        consume_hash_returning_key_bytes(bytes, hasher, ty_id, types)?;
        Ok(())
    }
}

/// A storage key used as part of the static codegen.
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct StaticStorageKey<K> {
    key: K,
}

impl<K> StaticStorageKey<K> {
    /// Creates a new static storage key.
    pub fn new(key: K) -> Self {
        StaticStorageKey { key }
    }
}

impl<K: Clone> StaticStorageKey<K> {
    /// Returns the decoded storage key.
    pub fn into_key(self) -> K {
        self.key
    }
}

impl<K: EncodeAsType + DecodeAsType> StorageKey for StaticStorageKey<K> {
    fn encode_storage_key(
        &self,
        bytes: &mut Vec<u8>,
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<(), Error> {
        let (hasher, ty_id) = hashers.next_or_err()?;
        let encoded_value = self.key.encode_as_type(ty_id, types)?;
        hash_bytes(&encoded_value, hasher, bytes);
        Ok(())
    }

    fn decode_storage_key(
        bytes: &mut &[u8],
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<Self, Error>
    where
        Self: Sized + 'static,
    {
        let (hasher, ty_id) = hashers.next_or_err()?;
        let key_bytes = consume_hash_returning_key_bytes(bytes, hasher, ty_id, types)?;

        // if the hasher had no key appended, we can't decode it into a `StaticStorageKey`.
        let Some(key_bytes) = key_bytes else {
            return Err(StorageAddressError::HasherCannotReconstructKey { ty_id, hasher }.into());
        };

        // Decode and return the key.
        let key = K::decode_as_type(&mut &*key_bytes, ty_id, types)?;
        let key = StaticStorageKey { key };
        Ok(key)
    }
}

impl StorageKey for Vec<scale_value::Value> {
    fn encode_storage_key(
        &self,
        bytes: &mut Vec<u8>,
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<(), Error> {
        for value in self.iter() {
            let (hasher, ty_id) = hashers.next_or_err()?;
            let encoded_value = value.encode_as_type(ty_id, types)?;
            hash_bytes(&encoded_value, hasher, bytes);
        }
        Ok(())
    }

    fn decode_storage_key(
        bytes: &mut &[u8],
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<Self, Error>
    where
        Self: Sized + 'static,
    {
        let mut result: Vec<scale_value::Value> = vec![];
        for (hasher, ty_id) in hashers.by_ref() {
            match consume_hash_returning_key_bytes(bytes, hasher, ty_id, types)? {
                Some(value_bytes) => {
                    let value =
                        scale_value::scale::decode_as_type(&mut &*value_bytes, ty_id, types)?;

                    result.push(value.remove_context());
                }
                None => {
                    result.push(Value::unnamed_composite([]));
                }
            }
        }

        // We've consumed all of the hashers, so we expect to also consume all of the bytes:
        if !bytes.is_empty() {
            return Err(StorageAddressError::TooManyBytes.into());
        }

        Ok(result)
    }
}

// Skip over the hash bytes (including any key at the end), returning bytes
// representing the key if one exists, or None if the hasher has no key appended.
fn consume_hash_returning_key_bytes<'a>(
    bytes: &mut &'a [u8],
    hasher: StorageHasher,
    ty_id: u32,
    types: &PortableRegistry,
) -> Result<Option<&'a [u8]>, Error> {
    // Strip the bytes off for the actual hash, consuming them.
    let bytes_to_strip = hasher.len_excluding_key();
    if bytes.len() < bytes_to_strip {
        return Err(StorageAddressError::NotEnoughBytes.into());
    }
    *bytes = &bytes[bytes_to_strip..];

    // Now, find the bytes representing the key, consuming them.
    let before_key = *bytes;
    if hasher.ends_with_key() {
        scale_decode::visitor::decode_with_visitor(
            bytes,
            ty_id,
            types,
            IgnoreVisitor::<PortableRegistry>::new(),
        )
        .map_err(|err| Error::Decode(err.into()))?;
        // Return the key bytes, having advanced the input cursor past them.
        let key_bytes = &before_key[..before_key.len() - bytes.len()];

        Ok(Some(key_bytes))
    } else {
        // There are no key bytes, so return None.
        Ok(None)
    }
}

/// Generates StorageKey implementations for tuples
macro_rules! impl_tuples {
    ($($ty:ident $n:tt),+) => {{
        impl<$($ty: StorageKey),+> StorageKey for ($( $ty ),+) {
            fn encode_storage_key(
                &self,
                bytes: &mut Vec<u8>,
                hashers: &mut StorageHashersIter,
                types: &PortableRegistry,
            ) -> Result<(), Error> {
                $(    self.$n.encode_storage_key(bytes, hashers, types)?;    )+
                Ok(())
            }

            fn decode_storage_key(
                bytes: &mut &[u8],
                hashers: &mut StorageHashersIter,
                types: &PortableRegistry,
            ) -> Result<Self, Error>
            where
                Self: Sized + 'static,
            {
                Ok( ( $(    $ty::decode_storage_key(bytes, hashers, types)?,    )+ ) )
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

#[cfg(test)]
mod tests {

    use codec::Encode;
    use scale_info::{PortableRegistry, Registry, TypeInfo, meta_type};
    use subxt_metadata::StorageHasher;

    use crate::utils::Era;

    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;

    use super::{StaticStorageKey, StorageKey};

    struct KeyBuilder {
        registry: Registry,
        bytes: Vec<u8>,
        hashers_and_ty_ids: Vec<(StorageHasher, u32)>,
    }

    impl KeyBuilder {
        fn new() -> KeyBuilder {
            KeyBuilder {
                registry: Registry::new(),
                bytes: vec![],
                hashers_and_ty_ids: vec![],
            }
        }

        fn add<T: TypeInfo + Encode + 'static>(mut self, value: T, hasher: StorageHasher) -> Self {
            let id = self.registry.register_type(&meta_type::<T>()).id;

            self.hashers_and_ty_ids.push((hasher, id));
            for _i in 0..hasher.len_excluding_key() {
                self.bytes.push(0);
            }
            value.encode_to(&mut self.bytes);
            self
        }

        fn build(self) -> (PortableRegistry, Vec<u8>, Vec<(StorageHasher, u32)>) {
            (self.registry.into(), self.bytes, self.hashers_and_ty_ids)
        }
    }

    #[test]
    fn storage_key_decoding_fuzz() {
        let hashers = [
            StorageHasher::Blake2_128,
            StorageHasher::Blake2_128Concat,
            StorageHasher::Blake2_256,
            StorageHasher::Identity,
            StorageHasher::Twox128,
            StorageHasher::Twox256,
            StorageHasher::Twox64Concat,
        ];

        let key_preserving_hashers = [
            StorageHasher::Blake2_128Concat,
            StorageHasher::Identity,
            StorageHasher::Twox64Concat,
        ];

        type T4A = (
            (),
            StaticStorageKey<u32>,
            StaticStorageKey<String>,
            StaticStorageKey<Era>,
        );
        type T4B = (
            (),
            (StaticStorageKey<u32>, StaticStorageKey<String>),
            StaticStorageKey<Era>,
        );
        type T4C = (
            ((), StaticStorageKey<u32>),
            (StaticStorageKey<String>, StaticStorageKey<Era>),
        );

        let era = Era::Immortal;
        for h0 in hashers {
            for h1 in key_preserving_hashers {
                for h2 in key_preserving_hashers {
                    for h3 in key_preserving_hashers {
                        let (types, bytes, hashers_and_ty_ids) = KeyBuilder::new()
                            .add((), h0)
                            .add(13u32, h1)
                            .add("Hello", h2)
                            .add(era, h3)
                            .build();

                        let hashers = super::StorageHashers { hashers_and_ty_ids };
                        let keys_a =
                            T4A::decode_storage_key(&mut &bytes[..], &mut hashers.iter(), &types)
                                .unwrap();

                        let keys_b =
                            T4B::decode_storage_key(&mut &bytes[..], &mut hashers.iter(), &types)
                                .unwrap();

                        let keys_c =
                            T4C::decode_storage_key(&mut &bytes[..], &mut hashers.iter(), &types)
                                .unwrap();

                        assert_eq!(keys_a.1.into_key(), 13);
                        assert_eq!(keys_b.1.0.into_key(), 13);
                        assert_eq!(keys_c.0.1.into_key(), 13);

                        assert_eq!(keys_a.2.into_key(), "Hello");
                        assert_eq!(keys_b.1.1.into_key(), "Hello");
                        assert_eq!(keys_c.1.0.into_key(), "Hello");
                        assert_eq!(keys_a.3.into_key(), era);
                        assert_eq!(keys_b.2.into_key(), era);
                        assert_eq!(keys_c.1.1.into_key(), era);
                    }
                }
            }
        }
    }
}
