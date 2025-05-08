// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{Decode, Encode};
use scale_decode::{IntoVisitor, TypeResolver, Visitor, visitor::DecodeAsTypeResult};
use scale_encode::EncodeAsType;

use alloc::vec::Vec;

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
    fn encode_as_type_to<R: TypeResolver>(
        &self,
        _type_id: R::TypeId,
        _types: &R,
        out: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error> {
        self.0.encode_to(out);
        Ok(())
    }
}

pub struct StaticDecodeAsTypeVisitor<T, R>(core::marker::PhantomData<(T, R)>);

impl<T: Decode, R: TypeResolver> Visitor for StaticDecodeAsTypeVisitor<T, R> {
    type Value<'scale, 'info> = Static<T>;
    type Error = scale_decode::Error;
    type TypeResolver = R;

    fn unchecked_decode_as_type<'scale, 'info>(
        self,
        input: &mut &'scale [u8],
        _type_id: R::TypeId,
        _types: &'info R,
    ) -> DecodeAsTypeResult<Self, Result<Self::Value<'scale, 'info>, Self::Error>> {
        use scale_decode::{Error, visitor::DecodeError};
        let decoded = T::decode(input)
            .map(Static)
            .map_err(|e| Error::new(DecodeError::CodecError(e).into()));
        DecodeAsTypeResult::Decoded(decoded)
    }
}

impl<T: Decode> IntoVisitor for Static<T> {
    type AnyVisitor<R: TypeResolver> = StaticDecodeAsTypeVisitor<T, R>;
    fn into_visitor<R: TypeResolver>() -> StaticDecodeAsTypeVisitor<T, R> {
        StaticDecodeAsTypeVisitor(core::marker::PhantomData)
    }
}

// Make it easy to convert types into Static where required.
impl<T> From<T> for Static<T> {
    fn from(value: T) -> Self {
        Static(value)
    }
}

// Static<T> is just a marker type and should be as transparent as possible:
impl<T> core::ops::Deref for Static<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Static<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
