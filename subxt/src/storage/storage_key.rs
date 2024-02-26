use super::utils::strip_storage_hash_bytes;
use crate::{
    dynamic::DecodedValueThunk,
    error::{Error, StorageAddressError},
    metadata::{DecodeWithMetadata, Metadata},
    utils::{Encoded, Static},
};
use scale_encode::EncodeAsType;
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

    /// Attempts to decode the StorageKey from a storage address that has been stripped of its root bytes.
    ///
    /// Example: Imagine The `StorageKey` is a tuple (A,B) and the hashers are: [Blake2_128Concat, Twox64Concat].
    /// Then the memory layout of the storage address is:
    /// ```txt
    /// | 8 bytes pallet hash | 8 bytes entry hash | 16 bytes hash of A | ... bytes of A | 8 bytes hash of B | ... bytes of B |
    /// ```
    /// `cursor` should point into a region after those first 16 bytes, at the start of a new hash.
    /// `hashers_and_ty_ids` should consume all the hashers that have been used for decoding, such that there are less hashers coming to the next key.
    fn decode_from_bytes(
        cursor: &mut &[u8],
        hashers_and_ty_ids: &mut &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Result<Self, Error>
    where
        Self: Sized + 'static;
}

/// Implement `StorageKey` for `()` which can be used for keyless storage entries.
impl StorageKey for () {
    fn keys_iter(&self) -> impl Iterator<Item = &dyn EncodeAsType> {
        std::iter::empty()
    }

    fn keys_len(&self) -> usize {
        0
    }

    fn decode_from_bytes(
        _cursor: &mut &[u8],
        hashers_and_ty_ids: &mut &[(StorageHasher, u32)],
        _metadata: &Metadata,
    ) -> Result<Self, Error> {
        if hashers_and_ty_ids.is_empty() {
            return Err(StorageAddressError::WrongNumberOfHashers {
                hashers: 0,
                fields: 1,
            }
            .into());
        }
        *hashers_and_ty_ids = &hashers_and_ty_ids[1..]; // Advance cursor by 1
        Ok(())
    }
}

/// A storage key for static encoded values.
/// The original value is only present at construction, but can be decoded from the contained bytes.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct StaticStorageKey<K: ?Sized> {
    pub(super) bytes: Static<Encoded>,
    pub(super) _marker: std::marker::PhantomData<K>,
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
        cursor: &mut &[u8],
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
        *hashers_and_ty_ids = &hashers_and_ty_ids[1..]; // Advance cursor by 1
        decode_storage_key_from_hash(cursor, hasher, *ty_id, metadata)
    }
}

pub fn decode_storage_key_from_hash<K: ?Sized>(
    cursor: &mut &[u8],
    hasher: &StorageHasher,
    ty_id: u32,
    metadata: &Metadata,
) -> Result<StaticStorageKey<K>, Error> {
    strip_storage_hash_bytes(cursor, hasher)?;

    let bytes = *cursor;
    if let Err(err) = scale_decode::visitor::decode_with_visitor(
        cursor,
        ty_id,
        metadata.types(),
        scale_decode::visitor::IgnoreVisitor,
    ) {
        return Err(scale_decode::Error::from(err).into());
    }
    let bytes_decoded = bytes.len() - cursor.len();

    // Note: This validation check makes sure, only zero-sized types can be decoded from
    // hashers that do not support reconstruction of a value
    if !hasher.hash_contains_unhashed_key() && bytes_decoded > 0 {
        let ty_name = metadata
            .types()
            .resolve(ty_id)
            .expect("ty_id is in metadata, because decode_with_visitor did not fail above; qed")
            .path
            .to_string();
        return Err(StorageAddressError::HasherCannotReconstructKey {
            ty_id,
            ty_name,
            hasher: *hasher,
        }
        .into());
    };

    let key_bytes = bytes[..bytes_decoded].to_vec();
    let key = StaticStorageKey {
        bytes: Static(Encoded(key_bytes)),
        _marker: std::marker::PhantomData::<K>,
    };
    Ok(key)
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
        cursor: &mut &[u8],
        hashers_and_ty_ids: &mut &[(StorageHasher, u32)],
        metadata: &Metadata,
    ) -> Result<Self, Error>
    where
        Self: Sized + 'static,
    {
        let mut hashers_and_ty_ids_iter = hashers_and_ty_ids.iter();
        let mut result: Vec<scale_value::Value> = vec![];
        let mut n = 0;
        while !cursor.is_empty() {
            let Some((hasher, ty_id)) = hashers_and_ty_ids_iter.next() else {
                // Still bytes left, but no hashers and type ids anymore to pull from: this is an unexpected error.
                return Err(StorageAddressError::UnexpectedAddressBytes.into());
            };
            strip_storage_hash_bytes(cursor, hasher)?;
            let decoded = DecodedValueThunk::decode_with_metadata(cursor, *ty_id, metadata)?;
            let value = decoded.to_value()?;
            result.push(value.remove_context());
            n += 1;
        }
        *hashers_and_ty_ids = &hashers_and_ty_ids[n..]; // Advance cursor by n
        Ok(result)
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
    impl_tuples!(A iter_a 0, B iter_ab 1);
    impl_tuples!(A iter_a 0, B iter_ab 1, C iter_c 2);
    impl_tuples!(A iter_a 0, B iter_ab 1, C iter_c 2, D iter_d 3);
    impl_tuples!(A iter_a 0, B iter_ab 1, C iter_c 2, D iter_d 3, E iter_e 4);
    impl_tuples!(A iter_a 0, B iter_ab 1, C iter_c 2, D iter_d 3, E iter_e 4, F iter_f 5);
    impl_tuples!(A iter_a 0, B iter_ab 1, C iter_c 2, D iter_d 3, E iter_e 4, F iter_f 5, G iter_g 6);
    impl_tuples!(A iter_a 0, B iter_ab 1, C iter_c 2, D iter_d 3, E iter_e 4, F iter_f 5, G iter_g 6, H iter_h 7);
};
