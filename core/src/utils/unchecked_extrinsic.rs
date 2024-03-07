// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The "default" Substrate/Polkadot UncheckedExtrinsic.
//! This is used in codegen for runtime API calls.
//!
//! The inner bytes represent the encoded extrinsic expected by the
//! runtime APIs. Deriving `EncodeAsType` would lead to the inner
//! bytes to be re-encoded (length prefixed).

use core::marker::PhantomData;

use codec::{Decode, Encode};
use scale_decode::{visitor::DecodeAsTypeResult, DecodeAsType, IntoVisitor, TypeResolver, Visitor};

use super::{Encoded, Static};
use alloc::vec::Vec;

/// The unchecked extrinsic from substrate.
#[derive(Clone, Debug, Eq, PartialEq, Encode)]
pub struct UncheckedExtrinsic<Address, Call, Signature, Extra>(
    Static<Encoded>,
    #[codec(skip)] PhantomData<(Address, Call, Signature, Extra)>,
);

impl<Address, Call, Signature, Extra> UncheckedExtrinsic<Address, Call, Signature, Extra> {
    /// Construct a new [`UncheckedExtrinsic`].
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(Static(Encoded(bytes)), PhantomData)
    }

    /// Get the bytes of the encoded extrinsic.
    pub fn bytes(&self) -> &[u8] {
        self.0 .0 .0.as_slice()
    }
}

impl<Address, Call, Signature, Extra> Decode
    for UncheckedExtrinsic<Address, Call, Signature, Extra>
{
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        // The bytes for an UncheckedExtrinsic are first a compact
        // encoded length, and then the bytes following. This is the
        // same encoding as a Vec, so easiest ATM is just to decode
        // into that, and then encode the vec bytes to get our extrinsic
        // bytes, which we save into an `Encoded` to preserve as-is.
        let xt_vec: Vec<u8> = Decode::decode(input)?;
        Ok(UncheckedExtrinsic::new(xt_vec))
    }
}

impl<Address, Call, Signature, Extra> scale_encode::EncodeAsType
    for UncheckedExtrinsic<Address, Call, Signature, Extra>
{
    fn encode_as_type_to<R: TypeResolver>(
        &self,
        type_id: &R::TypeId,
        types: &R,
        out: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error> {
        self.0.encode_as_type_to(type_id, types, out)
    }
}

impl<Address, Call, Signature, Extra> From<Vec<u8>>
    for UncheckedExtrinsic<Address, Call, Signature, Extra>
{
    fn from(bytes: Vec<u8>) -> Self {
        UncheckedExtrinsic::new(bytes)
    }
}

impl<Address, Call, Signature, Extra> From<UncheckedExtrinsic<Address, Call, Signature, Extra>>
    for Vec<u8>
{
    fn from(bytes: UncheckedExtrinsic<Address, Call, Signature, Extra>) -> Self {
        bytes.0 .0 .0
    }
}

pub struct UncheckedExtrinsicDecodeAsTypeVisitor<Address, Call, Signature, Extra, R: TypeResolver>(
    PhantomData<(Address, Call, Signature, Extra, R)>,
);

impl<Address, Call, Signature, Extra, R: TypeResolver> Visitor
    for UncheckedExtrinsicDecodeAsTypeVisitor<Address, Call, Signature, Extra, R>
{
    type Value<'scale, 'info> = UncheckedExtrinsic<Address, Call, Signature, Extra>;
    type Error = scale_decode::Error;
    type TypeResolver = R;

    fn unchecked_decode_as_type<'scale, 'info>(
        self,
        input: &mut &'scale [u8],
        type_id: &R::TypeId,
        types: &'info R,
    ) -> DecodeAsTypeResult<Self, Result<Self::Value<'scale, 'info>, Self::Error>> {
        DecodeAsTypeResult::Decoded(Self::Value::decode_as_type(input, type_id, types))
    }
}

impl<Address, Call, Signature, Extra> IntoVisitor
    for UncheckedExtrinsic<Address, Call, Signature, Extra>
{
    type AnyVisitor<R: TypeResolver> =
        UncheckedExtrinsicDecodeAsTypeVisitor<Address, Call, Signature, Extra, R>;

    fn into_visitor<R: TypeResolver>(
    ) -> UncheckedExtrinsicDecodeAsTypeVisitor<Address, Call, Signature, Extra, R> {
        UncheckedExtrinsicDecodeAsTypeVisitor(PhantomData)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn unchecked_extrinsic_encoding() {
        // A tx is basically some bytes with a compact length prefix; ie an encoded vec:
        let tx_bytes = vec![1u8, 2, 3].encode();

        let unchecked_extrinsic = UncheckedExtrinsic::<(), (), (), ()>::new(tx_bytes.clone());
        let encoded_tx_bytes = unchecked_extrinsic.encode();

        // The encoded representation must not alter the provided bytes.
        assert_eq!(tx_bytes, encoded_tx_bytes);

        // However, for decoding we expect to be able to read the extrinsic from the wire
        // which would be length prefixed.
        let decoded_tx = UncheckedExtrinsic::<(), (), (), ()>::decode(&mut &tx_bytes[..]).unwrap();
        let decoded_tx_bytes = decoded_tx.bytes();
        let encoded_tx_bytes = decoded_tx.encode();

        assert_eq!(decoded_tx_bytes, encoded_tx_bytes);
        // Ensure we can decode the tx and fetch only the tx bytes.
        assert_eq!(vec![1, 2, 3], encoded_tx_bytes);
    }
}
