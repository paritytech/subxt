// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Miscellaneous utility helpers.

use codec::{
    Decode,
    DecodeAll,
    Encode,
};
use derivative::Derivative;

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

/// A wrapper for any type `T` which implement encode/decode in a way compatible with `Vec<u8>`.
///
/// [`WrapperKeepOpaque`] stores the type only in its opaque format, aka as a `Vec<u8>`. To
/// access the real type `T` [`Self::try_decode`] needs to be used.
#[derive(Derivative, Encode, Decode)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Default(bound = ""),
    Hash(bound = "")
)]
pub struct WrapperKeepOpaque<T> {
    data: Vec<u8>,
    _phantom: PhantomDataSendSync<T>,
}

impl<T: Decode> WrapperKeepOpaque<T> {
    /// Try to decode the wrapped type from the inner `data`.
    ///
    /// Returns `None` if the decoding failed.
    pub fn try_decode(&self) -> Option<T> {
        T::decode_all(&mut &self.data[..]).ok()
    }

    /// Returns the length of the encoded `T`.
    pub fn encoded_len(&self) -> usize {
        self.data.len()
    }

    /// Returns the encoded data.
    pub fn encoded(&self) -> &[u8] {
        &self.data
    }

    /// Create from the given encoded `data`.
    pub fn from_encoded(data: Vec<u8>) -> Self {
        Self {
            data,
            _phantom: PhantomDataSendSync::new(),
        }
    }
}

/// A version of [`std::marker::PhantomData`] that is also Send and Sync (which is fine
/// because regardless of the generic param, it is always possible to Send + Sync this
/// 0 size type).
#[derive(Derivative, Encode, Decode, scale_info::TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = ""),
    Default(bound = ""),
    Hash(bound = "")
)]
#[scale_info(skip_type_params(T))]
#[doc(hidden)]
pub struct PhantomDataSendSync<T>(core::marker::PhantomData<T>);

impl<T> PhantomDataSendSync<T> {
    pub(crate) fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

unsafe impl<T> Send for PhantomDataSendSync<T> {}
unsafe impl<T> Sync for PhantomDataSendSync<T> {}

/// This represents a key-value collection and is SCALE compatible
/// with collections like BTreeMap. This has the same type params
/// as `BTreeMap` which allows us to easily swap the two during codegen.
pub type KeyedVec<K, V> = Vec<(K, V)>;

/// Generic `scale_bits` over `bitvec`-like `BitOrder` and `BitFormat` types.
pub mod bits {
    use codec::{
        Compact,
        Input,
    };
    use scale_bits::{
        scale::format::{
            Format,
            OrderFormat,
            StoreFormat,
        },
        Bits,
    };
    use std::marker::PhantomData;

    macro_rules! store {
        ($ident: ident; $(($ty: ident, $wrapped: ty)),*) => {
            /// Associates `bitvec::store::BitStore` trait with corresponding, type-erased `scale_bits::StoreFormat` enum.
            ///
            /// Used to decode bit sequences in runtime by providing `scale_bits::StoreFormat` using
            /// `bitvec`-like type type parameters.
            pub trait $ident {
                /// Corresponding `scale_bits::StoreFormat` value.
                const FORMAT: StoreFormat;
                /// Number of bits that the backing store types holds.
                const BITS: u32;
            }

            $(
                impl $ident for $wrapped {
                    const FORMAT: StoreFormat = StoreFormat::$ty;
                    const BITS: u32 = <$wrapped>::BITS;
                }
            )*
        };
    }

    macro_rules! order {
        ($ident: ident; $($ty: ident),*) => {
            /// Associates `bitvec::order::BitOrder` trait with corresponding, type-erased `scale_bits::OrderFormat` enum.
            ///
            /// Used to decode bit sequences in runtime by providing `scale_bits::OrderFormat` using
            /// `bitvec`-like type type parameters.
            pub trait $ident {
                /// Corresponding `scale_bits::OrderFormat` value.
                const FORMAT: OrderFormat;
            }

            $(
                #[doc = concat!("Type-level value that corresponds to `scale_bits::OrderFormat::", stringify!($ty), "` at run-time")]
                #[doc = concat!(" and `bitvec::order::BitOrder::", stringify!($ty), "` at the type level.")]
                #[derive(Clone, Debug, PartialEq, Eq)]
                pub enum $ty {}
                impl $ident for $ty {
                    const FORMAT: OrderFormat = OrderFormat::$ty;
                }
            )*
        };
    }

    store!(BitStore; (U8, u8), (U16, u16), (U32, u32), (U64, u64));
    order!(BitOrder; Lsb0, Msb0);

    /// Constructs a run-time format parameters based on the corresponding type-level parameters.
    pub fn bit_format<Store: BitStore, Order: BitOrder>() -> Format {
        Format {
            order: Order::FORMAT,
            store: Store::FORMAT,
        }
    }

    /// `scale_bits::Bits` generic over the bit store (`u8`/`u16`/`u32`/`u64`) and bit order (LSB, MSB)
    /// used for SCALE encoding/decoding. Uses `scale_bits::Bits`-default `u8` and LSB format underneath.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DecodedBits<Store: BitStore, Order: BitOrder>(
        pub Bits,
        pub PhantomData<Store>,
        pub PhantomData<Order>,
    );

    impl<Store: BitStore, Order: BitOrder> codec::Decode for DecodedBits<Store, Order> {
        fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
            /// Equivalent of `BitSlice::MAX_BITS` on 32bit machine.
            const ARCH32BIT_BITSLICE_MAX_BITS: usize = 0x1fff_ffff;

            let Compact(bits) = <Compact<u32>>::decode(input)?;
            // Otherwise it is impossible to store it on 32bit machine.
            if bits as usize > ARCH32BIT_BITSLICE_MAX_BITS {
                return Err("Attempt to decode a BitVec with too many bits".into())
            }
            // NOTE: Replace with `bits.div_ceil(Store::BITS)` if `int_roundings` is stabilised
            let elements =
                (bits / Store::BITS) as usize + (bits % Store::BITS != 0) as usize;
            let bytes_needed = elements * Store::BITS.saturating_div(u8::BITS) as usize;

            // NOTE: We could reduce allocations if it would be possible to directly
            // decode from an `Input` type using a custom format (rather than default <u8, Lsb0>)
            // for the `Bits` type.
            let mut storage = codec::Encode::encode(&Compact(bits));
            dbg!(&storage[..]);
            let prefix_len = storage.len();
            storage.reserve_exact(bytes_needed);
            storage.extend(vec![0; bytes_needed]);
            input.read(&mut storage[prefix_len..])?;
            dbg!(&storage[..]);

            let decoder = scale_bits::decode_using_format_from(
                &storage,
                bit_format::<Store, Order>(),
            )?;
            let bits = decoder.collect::<Result<Vec<_>, _>>()?;
            let bits = Bits::from_iter(bits);

            Ok(DecodedBits(bits, PhantomData, PhantomData))
        }
    }

    impl<Store: BitStore, Order: BitOrder> codec::Encode for DecodedBits<Store, Order> {
        fn size_hint(&self) -> usize {
            self.0.size_hint()
        }

        fn encoded_size(&self) -> usize {
            self.0.encoded_size()
        }

        fn encode(&self) -> Vec<u8> {
            scale_bits::encode_using_format(self.0.iter(), bit_format::<Store, Order>())
        }
    }
}

pub use bits::{
    BitOrder,
    BitStore,
    DecodedBits,
    Lsb0,
    Msb0,
};
