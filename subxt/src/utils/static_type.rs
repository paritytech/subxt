// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{Decode, Encode};
use scale_decode::{visitor::DecodeAsTypeResult, IntoVisitor, Visitor};
use scale_encode::EncodeAsType;

/// If the type inside this implements [`Encode`], this will implement [`scale_encode::EncodeAsType`].
/// If the type inside this implements [`Decode`], this will implement [`scale_decode::DecodeAsType`].
///
/// In either direction, we ignore any type information and just attempt to encode/decode statically
/// via the [`Encode`] and [`Decode`] implementations. This can be useful as an adapter for types which
/// do not implement [`scale_encode::EncodeAsType`] and [`scale_decode::DecodeAsType`] themselves, but
/// it's best to avoid using it where possible as it will not take into account any type information,
/// and is thus more likely to encode or decode incorrectly.
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Static<T>(pub T);

impl<T: Encode> EncodeAsType for Static<T> {
    fn encode_as_type_to(
        &self,
        _type_id: u32,
        _types: &scale_decode::PortableRegistry,
        out: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error> {
        self.0.encode_to(out);
        Ok(())
    }
}

pub struct StaticDecodeAsTypeVisitor<T>(std::marker::PhantomData<T>);

impl<T: Decode> Visitor for StaticDecodeAsTypeVisitor<T> {
    type Value<'scale, 'info> = Static<T>;
    type Error = scale_decode::Error;

    fn unchecked_decode_as_type<'scale, 'info>(
        self,
        input: &mut &'scale [u8],
        _type_id: scale_decode::visitor::TypeId,
        _types: &'info scale_info::PortableRegistry,
    ) -> DecodeAsTypeResult<Self, Result<Self::Value<'scale, 'info>, Self::Error>> {
        use scale_decode::{visitor::DecodeError, Error};
        let decoded = T::decode(input)
            .map(Static)
            .map_err(|e| Error::new(DecodeError::CodecError(e).into()));
        DecodeAsTypeResult::Decoded(decoded)
    }
}

impl<T: Decode> IntoVisitor for Static<T> {
    type Visitor = StaticDecodeAsTypeVisitor<T>;
    fn into_visitor() -> Self::Visitor {
        StaticDecodeAsTypeVisitor(std::marker::PhantomData)
    }
}

// Make it easy to convert types into Static where required.
impl<T> From<T> for Static<T> {
    fn from(value: T) -> Self {
        Static(value)
    }
}

// Static<T> is just a marker type and should be as transparent as possible:
impl<T> std::ops::Deref for Static<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Static<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
