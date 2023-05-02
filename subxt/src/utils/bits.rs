// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Generic `scale_bits` over `bitvec`-like `BitOrder` and `BitFormat` types.

use codec::{Compact, Input};
use scale_bits::{
    scale::format::{Format, OrderFormat, StoreFormat},
    Bits,
};
use scale_decode::IntoVisitor;
use std::marker::PhantomData;

/// Associates `bitvec::store::BitStore` trait with corresponding, type-erased `scale_bits::StoreFormat` enum.
///
/// Used to decode bit sequences by providing `scale_bits::StoreFormat` using
/// `bitvec`-like type type parameters.
pub trait BitStore {
    /// Corresponding `scale_bits::StoreFormat` value.
    const FORMAT: StoreFormat;
    /// Number of bits that the backing store types holds.
    const BITS: u32;
}
macro_rules! impl_store {
    ($ty:ident, $wrapped:ty) => {
        impl BitStore for $wrapped {
            const FORMAT: StoreFormat = StoreFormat::$ty;
            const BITS: u32 = <$wrapped>::BITS;
        }
    };
}
impl_store!(U8, u8);
impl_store!(U16, u16);
impl_store!(U32, u32);
impl_store!(U64, u64);

/// Associates `bitvec::order::BitOrder` trait with corresponding, type-erased `scale_bits::OrderFormat` enum.
///
/// Used to decode bit sequences in runtime by providing `scale_bits::OrderFormat` using
/// `bitvec`-like type type parameters.
pub trait BitOrder {
    /// Corresponding `scale_bits::OrderFormat` value.
    const FORMAT: OrderFormat;
}
macro_rules! impl_order {
    ($ty:ident) => {
        #[doc = concat!("Type-level value that corresponds to `scale_bits::OrderFormat::", stringify!($ty), "` at run-time")]
        #[doc = concat!(" and `bitvec::order::BitOrder::", stringify!($ty), "` at the type level.")]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum $ty {}
        impl BitOrder for $ty {
            const FORMAT: OrderFormat = OrderFormat::$ty;
        }
    };
}
impl_order!(Lsb0);
impl_order!(Msb0);

/// Constructs a run-time format parameters based on the corresponding type-level parameters.
fn bit_format<Store: BitStore, Order: BitOrder>() -> Format {
    Format {
        order: Order::FORMAT,
        store: Store::FORMAT,
    }
}

/// `scale_bits::Bits` generic over the bit store (`u8`/`u16`/`u32`/`u64`) and bit order (LSB, MSB)
/// used for SCALE encoding/decoding. Uses `scale_bits::Bits`-default `u8` and LSB format underneath.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedBits<Store, Order> {
    bits: Bits,
    _marker: PhantomData<(Store, Order)>,
}

impl<Store, Order> DecodedBits<Store, Order> {
    /// Extracts the underlying `scale_bits::Bits` value.
    pub fn into_bits(self) -> Bits {
        self.bits
    }

    /// References the underlying `scale_bits::Bits` value.
    pub fn as_bits(&self) -> &Bits {
        &self.bits
    }
}

impl<Store, Order> core::iter::FromIterator<bool> for DecodedBits<Store, Order> {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        DecodedBits {
            bits: Bits::from_iter(iter),
            _marker: PhantomData,
        }
    }
}

impl<Store: BitStore, Order: BitOrder> codec::Decode for DecodedBits<Store, Order> {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        /// Equivalent of `BitSlice::MAX_BITS` on 32bit machine.
        const ARCH32BIT_BITSLICE_MAX_BITS: u32 = 0x1fff_ffff;

        let Compact(bits) = <Compact<u32>>::decode(input)?;
        // Otherwise it is impossible to store it on 32bit machine.
        if bits > ARCH32BIT_BITSLICE_MAX_BITS {
            return Err("Attempt to decode a BitVec with too many bits".into());
        }
        // NOTE: Replace with `bits.div_ceil(Store::BITS)` if `int_roundings` is stabilised
        let elements = (bits / Store::BITS) + u32::from(bits % Store::BITS != 0);
        let bytes_in_elem = Store::BITS.saturating_div(u8::BITS);
        let bytes_needed = (elements * bytes_in_elem) as usize;

        // NOTE: We could reduce allocations if it would be possible to directly
        // decode from an `Input` type using a custom format (rather than default <u8, Lsb0>)
        // for the `Bits` type.
        let mut storage = codec::Encode::encode(&Compact(bits));
        let prefix_len = storage.len();
        storage.reserve_exact(bytes_needed);
        storage.extend(vec![0; bytes_needed]);
        input.read(&mut storage[prefix_len..])?;

        let decoder = scale_bits::decode_using_format_from(&storage, bit_format::<Store, Order>())?;
        let bits = decoder.collect::<Result<Vec<_>, _>>()?;
        let bits = Bits::from_iter(bits);

        Ok(DecodedBits {
            bits,
            _marker: PhantomData,
        })
    }
}

impl<Store: BitStore, Order: BitOrder> codec::Encode for DecodedBits<Store, Order> {
    fn size_hint(&self) -> usize {
        self.bits.size_hint()
    }

    fn encoded_size(&self) -> usize {
        self.bits.encoded_size()
    }

    fn encode(&self) -> Vec<u8> {
        scale_bits::encode_using_format(self.bits.iter(), bit_format::<Store, Order>())
    }
}

#[doc(hidden)]
pub struct DecodedBitsVisitor<S, O>(std::marker::PhantomData<(S, O)>);
impl<Store, Order> scale_decode::Visitor for DecodedBitsVisitor<Store, Order> {
    type Value<'scale, 'info> = DecodedBits<Store, Order>;
    type Error = scale_decode::Error;

    fn unchecked_decode_as_type<'scale, 'info>(
        self,
        input: &mut &'scale [u8],
        type_id: scale_decode::visitor::TypeId,
        types: &'info scale_info::PortableRegistry,
    ) -> scale_decode::visitor::DecodeAsTypeResult<
        Self,
        Result<Self::Value<'scale, 'info>, Self::Error>,
    > {
        let res = scale_decode::visitor::decode_with_visitor(
            input,
            type_id.0,
            types,
            Bits::into_visitor(),
        )
        .map(|bits| DecodedBits {
            bits,
            _marker: PhantomData,
        });
        scale_decode::visitor::DecodeAsTypeResult::Decoded(res)
    }
}
impl<Store, Order> scale_decode::IntoVisitor for DecodedBits<Store, Order> {
    type Visitor = DecodedBitsVisitor<Store, Order>;
    fn into_visitor() -> Self::Visitor {
        DecodedBitsVisitor(PhantomData)
    }
}

impl<Store, Order> scale_encode::EncodeAsType for DecodedBits<Store, Order> {
    fn encode_as_type_to(
        &self,
        type_id: u32,
        types: &scale_info::PortableRegistry,
        out: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error> {
        self.bits.encode_as_type_to(type_id, types, out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::fmt::Debug;

    use bitvec::vec::BitVec;
    use codec::Decode as _;

    // NOTE: We don't use `bitvec::order` types in our implementation, since we
    // don't want to depend on `bitvec`. Rather than reimplementing the unsafe
    // trait on our types here for testing purposes, we simply convert and
    // delegate to `bitvec`'s own types.
    trait ToBitVec {
        type Order: bitvec::order::BitOrder;
    }
    impl ToBitVec for Lsb0 {
        type Order = bitvec::order::Lsb0;
    }
    impl ToBitVec for Msb0 {
        type Order = bitvec::order::Msb0;
    }

    fn scales_like_bitvec_and_roundtrips<
        'a,
        Store: BitStore + bitvec::store::BitStore + PartialEq,
        Order: BitOrder + ToBitVec + Debug + PartialEq,
    >(
        input: impl IntoIterator<Item = &'a bool>,
    ) where
        BitVec<Store, <Order as ToBitVec>::Order>: codec::Encode + codec::Decode,
    {
        let input: Vec<_> = input.into_iter().copied().collect();

        let decoded_bits = DecodedBits::<Store, Order>::from_iter(input.clone());
        let bitvec = BitVec::<Store, <Order as ToBitVec>::Order>::from_iter(input);

        let decoded_bits_encoded = codec::Encode::encode(&decoded_bits);
        let bitvec_encoded = codec::Encode::encode(&bitvec);
        assert_eq!(decoded_bits_encoded, bitvec_encoded);

        let decoded_bits_decoded =
            DecodedBits::<Store, Order>::decode(&mut &decoded_bits_encoded[..])
                .expect("SCALE-encoding DecodedBits to roundtrip");
        let bitvec_decoded =
            BitVec::<Store, <Order as ToBitVec>::Order>::decode(&mut &bitvec_encoded[..])
                .expect("SCALE-encoding BitVec to roundtrip");
        assert_eq!(decoded_bits, decoded_bits_decoded);
        assert_eq!(bitvec, bitvec_decoded);
    }

    #[test]
    fn decoded_bitvec_scales_and_roundtrips() {
        let test_cases = [
            vec![],
            vec![true],
            vec![false],
            vec![true, false, true],
            vec![true, false, true, false, false, false, false, false, true],
            [vec![true; 5], vec![false; 5], vec![true; 1], vec![false; 3]].concat(),
            [vec![true; 9], vec![false; 9], vec![true; 9], vec![false; 9]].concat(),
        ];

        for test_case in &test_cases {
            scales_like_bitvec_and_roundtrips::<u8, Lsb0>(test_case);
            scales_like_bitvec_and_roundtrips::<u16, Lsb0>(test_case);
            scales_like_bitvec_and_roundtrips::<u32, Lsb0>(test_case);
            scales_like_bitvec_and_roundtrips::<u64, Lsb0>(test_case);
            scales_like_bitvec_and_roundtrips::<u8, Msb0>(test_case);
            scales_like_bitvec_and_roundtrips::<u16, Msb0>(test_case);
            scales_like_bitvec_and_roundtrips::<u32, Msb0>(test_case);
            scales_like_bitvec_and_roundtrips::<u64, Msb0>(test_case);
        }
    }
}
