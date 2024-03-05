use crate::{
    error::{Error, MetadataError, StorageAddressError},
    utils::{Encoded, Static},
};
use scale_decode::{visitor::IgnoreVisitor, DecodeAsType};
use scale_encode::EncodeAsType;
use scale_info::{PortableRegistry, TypeDef};
use scale_value::Value;
use subxt_metadata::{StorageEntryType, StorageHasher};

use derivative::Derivative;

use super::utils::hash_bytes;

#[derive(Debug, Clone)]
// An iterator over all type ids of the key and the respective hashers.
pub struct StorageHashersIter {
    hashers_and_ty_ids: Vec<(StorageHasher, u32)>,
    idx: usize,
}

impl StorageHashersIter {
    /// Creates a new [`StorageHashersIter`]. Looks at the [`StorageEntryType`] and
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

            if let TypeDef::Tuple(tuple) = &ty.type_def {
                if tuple.fields.len() != hashers.len() {
                    return Err(StorageAddressError::WrongNumberOfHashers {
                        hashers: hashers.len(),
                        fields: tuple.fields.len(),
                    }
                    .into());
                }
                for (i, f) in tuple.fields.iter().enumerate() {
                    hashers_and_ty_ids.push((hashers[i], f.id));
                }
            } else {
                if hashers.len() != 1 {
                    return Err(StorageAddressError::WrongNumberOfHashers {
                        hashers: hashers.len(),
                        fields: 1,
                    }
                    .into());
                }
                hashers_and_ty_ids.push((hashers[0], *key_ty));
            };
        }

        Ok(Self {
            hashers_and_ty_ids,
            idx: 0,
        })
    }

    pub fn next_or_err(&mut self) -> Result<(StorageHasher, u32), Error> {
        self.next().ok_or_else(|| {
            StorageAddressError::TooManyKeys {
                expected: self.hashers_and_ty_ids.len(),
            }
            .into()
        })
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }
}

impl Iterator for StorageHashersIter {
    type Item = (StorageHasher, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.hashers_and_ty_ids.get(self.idx).copied()?;
        self.idx += 1;
        Some(item)
    }
}

impl ExactSizeIterator for StorageHashersIter {
    fn len(&self) -> usize {
        self.hashers_and_ty_ids.len() - self.idx
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
        bytes: &mut Vec<u8>,
        hashers: &mut StorageHashersIter,
        _types: &PortableRegistry,
    ) -> Result<(), Error> {
        let (hasher, _ty) = hashers.next_or_err()?;
        hash_bytes(&[], hasher, bytes);
        Ok(())
    }

    fn decode_storage_key(
        bytes: &mut &[u8],
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<Self, Error> {
        let (hasher, ty_id) = hashers.next_or_err()?;
        consume_hash_returning_key_bytes(bytes, hasher, ty_id, types)?;
        Ok(())
    }
}

/// A storage key for static encoded values.
/// The original value is only present at construction, but can be decoded from the contained bytes.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct StaticStorageKey<K: ?Sized> {
    bytes: Static<Encoded>,
    _marker: std::marker::PhantomData<K>,
}

impl<K: codec::Encode + ?Sized> StaticStorageKey<K> {
    /// Creates a new static storage key
    pub fn new(key: &K) -> Self {
        StaticStorageKey {
            bytes: Static(Encoded(key.encode())),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<K: codec::Decode + ?Sized> StaticStorageKey<K> {
    /// Decodes the encoded inner bytes into the type `K`.
    pub fn decoded(&self) -> Result<K, Error> {
        let decoded = K::decode(&mut self.bytes())?;
        Ok(decoded)
    }
}

impl<K: ?Sized> StaticStorageKey<K> {
    /// Returns the scale-encoded bytes that make up this key
    pub fn bytes(&self) -> &[u8] {
        &self.bytes.0 .0
    }
}

// Note: The ?Sized bound is necessary to support e.g. `StorageKey<[u8]>`.
impl<K: ?Sized> StorageKey for StaticStorageKey<K> {
    fn encode_storage_key(
        &self,
        bytes: &mut Vec<u8>,
        hashers: &mut StorageHashersIter,
        types: &PortableRegistry,
    ) -> Result<(), Error> {
        let (hasher, ty_id) = hashers.next_or_err()?;
        let encoded_value = self.bytes.encode_as_type(ty_id, types)?;
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

        // Return the key bytes.
        let key = StaticStorageKey {
            bytes: Static(Encoded(key_bytes.to_vec())),
            _marker: std::marker::PhantomData::<K>,
        };
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
                    let value = Value::decode_as_type(&mut &*value_bytes, ty_id, types)?;
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
        scale_decode::visitor::decode_with_visitor(bytes, ty_id, types, IgnoreVisitor)
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
    use scale_info::{meta_type, PortableRegistry, Registry, TypeInfo};
    use subxt_metadata::StorageHasher;

    use crate::utils::Era;

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

                        let mut iter = super::StorageHashersIter {
                            hashers_and_ty_ids,
                            idx: 0,
                        };
                        let keys_a =
                            T4A::decode_storage_key(&mut &bytes[..], &mut iter, &types).unwrap();

                        iter.reset();
                        let keys_b =
                            T4B::decode_storage_key(&mut &bytes[..], &mut iter, &types).unwrap();

                        iter.reset();
                        let keys_c =
                            T4C::decode_storage_key(&mut &bytes[..], &mut iter, &types).unwrap();

                        assert_eq!(keys_a.1.decoded().unwrap(), 13);
                        assert_eq!(keys_b.1 .0.decoded().unwrap(), 13);
                        assert_eq!(keys_c.0 .1.decoded().unwrap(), 13);

                        assert_eq!(keys_a.2.decoded().unwrap(), "Hello");
                        assert_eq!(keys_b.1 .1.decoded().unwrap(), "Hello");
                        assert_eq!(keys_c.1 .0.decoded().unwrap(), "Hello");
                        assert_eq!(keys_a.3.decoded().unwrap(), era);
                        assert_eq!(keys_b.2.decoded().unwrap(), era);
                        assert_eq!(keys_c.1 .1.decoded().unwrap(), era);
                    }
                }
            }
        }
    }
}
