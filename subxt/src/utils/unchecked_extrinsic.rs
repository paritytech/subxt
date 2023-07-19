// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The "default" Substrate/Polkadot UncheckedExtrinsic.
//! This is used in codegen for runtime API calls.
//!
//! The inner bytes represent the encoded extrinsic expected by the
//! runtime APIs. Deriving `EncodeAsType` would lead to the inner
//! bytes to be re-encoded (length prefixed).

use std::marker::PhantomData;

use codec::Decode;
use scale_decode::{visitor::DecodeAsTypeResult, IntoVisitor, Visitor};

/// The unchecked extrinsic from substrate.
#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct UncheckedExtrinsic<Address, Call, Signature, Extra>(
    pub Vec<u8>,
    #[codec(skip)] pub PhantomData<(Address, Call, Signature, Extra)>,
);

impl<Address, Call, Signature, Extra> codec::Encode
    for UncheckedExtrinsic<Address, Call, Signature, Extra>
{
    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.0)
    }
}

impl<Address, Call, Signature, Extra> scale_encode::EncodeAsType
    for UncheckedExtrinsic<Address, Call, Signature, Extra>
{
    fn encode_as_type_to(
        &self,
        _type_id: u32,
        _types: &scale_info::PortableRegistry,
        out: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error> {
        out.extend_from_slice(&self.0);
        Ok(())
    }
}

pub struct UncheckedExtrinsicDecodeAsTypeVisitor<Address, Call, Signature, Extra>(
    PhantomData<(Address, Call, Signature, Extra)>,
);

impl<Address, Call, Signature, Extra> Visitor
    for UncheckedExtrinsicDecodeAsTypeVisitor<Address, Call, Signature, Extra>
{
    type Value<'scale, 'info> = UncheckedExtrinsic<Address, Call, Signature, Extra>;
    type Error = scale_decode::Error;

    fn unchecked_decode_as_type<'scale, 'info>(
        self,
        input: &mut &'scale [u8],
        _type_id: scale_decode::visitor::TypeId,
        _types: &'info scale_info::PortableRegistry,
    ) -> DecodeAsTypeResult<Self, Result<Self::Value<'scale, 'info>, Self::Error>> {
        use scale_decode::{visitor::DecodeError, Error};
        let decoded = UncheckedExtrinsic::decode(input)
            .map_err(|e| Error::new(DecodeError::CodecError(e).into()));
        DecodeAsTypeResult::Decoded(decoded)
    }
}

impl<Address, Call, Signature, Extra> IntoVisitor
    for UncheckedExtrinsic<Address, Call, Signature, Extra>
{
    type Visitor = UncheckedExtrinsicDecodeAsTypeVisitor<Address, Call, Signature, Extra>;

    fn into_visitor() -> Self::Visitor {
        UncheckedExtrinsicDecodeAsTypeVisitor(PhantomData)
    }
}
