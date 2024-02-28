use crate::{
    error::{Error, StorageAddressError},
    metadata::Metadata,
    utils::{Encoded, Static},
};
use scale_decode::{visitor::IgnoreVisitor, DecodeAsType};
use scale_encode::EncodeAsType;
use scale_info::PortableRegistry;
use scale_value::Value;
use subxt_metadata::StorageHasher;

use derivative::Derivative;

/// This trait should be implemented by anything that can be used as one or multiple storage keys.
pub trait StorageKey {
    /// Iterator over the storage keys, each key implements EncodeAsType to
    /// give the corresponding bytes to a `StorageHasher`.
    fn keys_iter(&self) -> impl Iterator<Item = &dyn EncodeAsType>;

    /// How many keys are there in total? Each key is an element that needs to be individually hashed.
    // Note: Ideally we would use `impl ExactSizeIterator<Item = &dyn EncodeAsType>` for `keys_iter` above,
    // But that plays poorly with the `Flatten` and `Chain` structs.
    fn keys_len(&self) -> usize;

    /// Attempts to decode the StorageKey given some bytes and a set of hashers and type IDs that they are meant to represent.
    ///
    /// Example: Imagine The `StorageKey` is a tuple `(A,B)` and the hashers are `[Blake2_128Concat, Twox64Concat]`.
    /// Then the memory layout of the storage address is:
    ///
    /// ```txt
    /// | 16 byte hash of A | n bytes for SCALE encoded A | 8 byte hash of B | n bytes for SCALE encoded B |
    /// ```
    ///
    /// Implementations of this must advance the `bytes` and `hashers_and_ty_ids` cursors to consume any that they are using, or
    /// return an error if they cannot appropriately do so. When a tuple of such implementations is given, each implementation
    /// in the tuple receives the remaining un-consumed bytes and hashers from the previous ones.
    fn decode_from_bytes(
        bytes: &mut &[u8],
        hashers_and_ty_ids: &mut &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Result<Self, Error>
    where
        Self: Sized + 'static;
}

/// Implement `StorageKey` for `()` which can be used for keyless storage entries,
/// or to otherwise just ignore some entry.
impl StorageKey for () {
    fn keys_iter(&self) -> impl Iterator<Item = &dyn EncodeAsType> {
        std::iter::empty()
    }

    fn keys_len(&self) -> usize {
        0
    }

    fn decode_from_bytes(
        bytes: &mut &[u8],
        hashers_and_ty_ids: &mut &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Result<Self, Error> {
        // If no hashers, we just do nothing.
        let Some((hasher, ty_id)) = hashers_and_ty_ids.first() else {
            return Ok(());
        };

        // Consume the hash bytes (we don't care about the key output here).
        consume_hash_returning_key_bytes(bytes, *hasher, *ty_id, metadata.types())?;
        // Advance our hasher cursor as well now that we've used it.
        *hashers_and_ty_ids = &hashers_and_ty_ids[1..];

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
    fn keys_iter(&self) -> impl Iterator<Item = &dyn EncodeAsType> {
        std::iter::once(&self.bytes as &dyn EncodeAsType)
    }

    fn keys_len(&self) -> usize {
        1
    }

    fn decode_from_bytes(
        bytes: &mut &[u8],
        hashers_and_ty_ids: &mut &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Result<Self, Error>
    where
        Self: Sized + 'static,
    {
        let Some((hasher, ty_id)) = hashers_and_ty_ids.first() else {
            return Err(StorageAddressError::WrongNumberOfHashers {
                hashers: 0,
                fields: 1,
            }
            .into());
        };

        // Advance the bytes cursor, returning any key bytes.
        let key_bytes = consume_hash_returning_key_bytes(bytes, *hasher, *ty_id, metadata.types())?;
        // Advance the hasher cursor now we've used it.
        *hashers_and_ty_ids = &hashers_and_ty_ids[1..];

        // if the hasher had no key appended, we can't decode it into a `StaticStorageKey`.
        let Some(key_bytes) = key_bytes else {
            return Err(StorageAddressError::HasherCannotReconstructKey {
                ty_id: *ty_id,
                hasher: *hasher,
            }
            .into());
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
    fn keys_iter(&self) -> impl Iterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageKey`.
        self.iter().map(|e| e as &dyn EncodeAsType)
    }

    fn keys_len(&self) -> usize {
        self.len()
    }

    fn decode_from_bytes(
        bytes: &mut &[u8],
        hashers_and_ty_ids: &mut &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Result<Self, Error>
    where
        Self: Sized + 'static,
    {
        let mut result: Vec<scale_value::Value> = vec![];
        for (hasher, ty_id) in hashers_and_ty_ids.iter() {
            match consume_hash_returning_key_bytes(bytes, *hasher, *ty_id, metadata.types())? {
                Some(value_bytes) => {
                    let value =
                        Value::decode_as_type(&mut &*value_bytes, *ty_id, metadata.types())?;
                    result.push(value.remove_context());
                }
                None => {
                    result.push(Value::unnamed_composite([]));
                }
            }
            *hashers_and_ty_ids = &hashers_and_ty_ids[1..]; // Advance by 1 each time.
        }

        // We've consumed all of the hashers, so we expect to also consume all of the bytes:
        if !bytes.is_empty() {
            return Err(StorageAddressError::UnexpectedAddressBytes.into());
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
        return Err(StorageAddressError::UnexpectedAddressBytes.into());
    }
    *bytes = &bytes[bytes_to_strip..];

    // Now, find the bytes representing the key, consuming them.
    let before_key = *bytes;
    if hasher.ends_with_key() {
        scale_decode::visitor::decode_with_visitor(bytes, ty_id, types, IgnoreVisitor)
            .map_err(|err| Error::Decode(err.into()))?;

        // Return the key bytes, having advanced the input cursor past them.
        let key_bytes = &before_key[before_key.len() - bytes.len()..];
        Ok(Some(key_bytes))
    } else {
        // There are no key bytes, so return None.
        Ok(None)
    }
}

/// Generates StorageKey implementations for tuples, e.g.
/// ```rs,norun
/// impl<A: EncodeAsType, B: EncodeAsType> StorageKey for (StorageKey<A>, StorageKey<B>) {
///     fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
///         let arr = [&self.0 as &dyn EncodeAsType, &self.1 as &dyn EncodeAsType];
///         arr.into_iter()
///     }
/// }
/// ```
macro_rules! impl_tuples {
    ($($ty:ident $iter:ident $n:tt),+) => {{
        impl<$($ty: StorageKey),+> StorageKey for ($( $ty ),+) {
            fn keys_iter(&self) -> impl Iterator<Item = &dyn EncodeAsType> {

                $(
                    let mut $iter = self.$n.keys_iter();
                )+

                // Note: this functions just flattens the iterators (that might all have different types).
                std::iter::from_fn(move || {
                    let mut i = 0;
                    loop {
                        match i {
                            $(
                                $n => {
                                    let el = $iter.next();
                                    if el.is_some(){
                                        return el;
                                    }
                                },
                            )+
                                _ => return None,
                        };
                        i+=1;
                    }
                })
            }

            fn keys_len(&self) -> usize {
                $((self.$n.keys_len())+)+0
            }

            fn decode_from_bytes(
                cursor: &mut &[u8],
                hashers_and_ty_ids: &mut &[(StorageHasher, u32)],
                metadata: &Metadata,
            ) -> Result<Self, Error>
            where
                Self: Sized + 'static,
            {
                // Construct the tuple as a series of expressions.
                let tuple : Self = ( $(
                    {
                        let key =
                        $ty::decode_from_bytes(cursor, hashers_and_ty_ids, metadata)?;
                        key
                    },
                )+);
                return Ok(tuple)
            }
        }
    }};
}

#[rustfmt::skip]
const _: () = {
    impl_tuples!(A iter_a 0, B iter_b 1);
    impl_tuples!(A iter_a 0, B iter_b 1, C iter_c 2);
    impl_tuples!(A iter_a 0, B iter_b 1, C iter_c 2, D iter_d 3);
    impl_tuples!(A iter_a 0, B iter_b 1, C iter_c 2, D iter_d 3, E iter_e 4);
    impl_tuples!(A iter_a 0, B iter_b 1, C iter_c 2, D iter_d 3, E iter_e 4, F iter_f 5);
    impl_tuples!(A iter_a 0, B iter_b 1, C iter_c 2, D iter_d 3, E iter_e 4, F iter_f 5, G iter_g 6);
    impl_tuples!(A iter_a 0, B iter_b 1, C iter_c 2, D iter_d 3, E iter_e 4, F iter_f 5, G iter_g 6, H iter_h 7);
};
