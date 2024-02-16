use super::{
    storage_address::StaticStorageKey,
    utils::{strip_concat_hash_bytes, strip_storage_addess_root_bytes},
};

use crate::{
    dynamic::DecodedValueThunk,
    error::{Error, StorageAddressError},
    metadata::{DecodeWithMetadata, Metadata},
    utils::{Encoded, Static},
};

use futures::StreamExt;
use scale_encode::EncodeAsType;

use subxt_metadata::StorageHasher;

/// This trait should be implemented by anything that can be used as one or multiple storage keys.
pub trait StorageKey {
    /// Iterator over the storage keys, each key implements EncodeAsType to
    /// give the corresponding bytes to a `StorageHasher`.
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType>;

    /// Attempts to decode the StorageKey from a whole storage address.
    /// The key/keys can only be recovered if all hashers are concat-style hashers.
    /// Example: Imagine The `StorageKey` is a tuple (A,B) and the hashers are: [Blake2_128Concat, Twox64Concat].
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

/// Implement `StorageKey` for `()` which can be used for keyless storage entries
impl StorageKey for () {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageKey`.
        std::iter::empty()
    }

    fn decode_from_address_bytes(
        address_bytes: &[u8],
        _hashers_and_ty_ids: &[(StorageHasher, u32)],
        _metadata: &Metadata,
    ) -> Option<Result<Self, Error>> {
        // We need at least 16 bytes, becauase there are 8 bytes for the pallet hash and
        // another 8 bytes for the entry hash at the beginning of each storage address.
        if address_bytes.len() < 16 {
            return Some(Err(StorageAddressError::UnexpectedAddressBytes.into()));
        }
        Some(Ok(()))
    }
}

// Note: The ?Sized bound is necessary to support e.g. `StorageKey<[u8]>`.
impl<K: EncodeAsType + codec::Decode + ?Sized> StorageKey for StaticStorageKey<K> {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageKey`.
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
) -> Option<Result<StaticStorageKey<K>, Error>> {
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
    let key = StaticStorageKey {
        bytes: Static(Encoded(key_bytes)),
        _marker: std::marker::PhantomData::<K>,
    };
    Some(Ok(key))
}

impl StorageKey for Vec<scale_value::Value> {
    fn keys_iter(&self) -> impl ExactSizeIterator<Item = &dyn EncodeAsType> {
        // Note: this returns the storage root address of the storage entry.
        // It gives the same result as if you were to use `vec![]` as a `StorageKey`.
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
    ($($ty:ident $n:tt),+) => {{
        impl<$($ty: EncodeAsType + ?Sized),+> StorageKey for ($( StaticStorageKey<$ty >),+) {
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
