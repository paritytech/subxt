// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    dynamic::Value,
    error::Error,
    metadata::Metadata,
};
use codec::Encode;

/// This trait is implemented for types which can be encoded with the help of metadata.
pub trait EncodeWithMetadata {
    /// SCALE encode this type to bytes, possibly with the help of metadata.
    fn encode_with_metadata(
        &self,
        type_id: u32,
        metadata: &Metadata,
        bytes: &mut Vec<u8>,
    ) -> Result<(), Error>;
}

impl EncodeWithMetadata for Value<()> {
    fn encode_with_metadata(
        &self,
        type_id: u32,
        metadata: &Metadata,
        bytes: &mut Vec<u8>,
    ) -> Result<(), Error> {
        scale_value::scale::encode_as_type(self, type_id, metadata.types(), bytes)
            .map_err(|e| e.into())
    }
}

/// Any type implementing [`Encode`] can also be encoded with the help of metadata.
pub struct EncodeStaticType<T>(pub T);

impl<T: Encode> EncodeWithMetadata for EncodeStaticType<T> {
    fn encode_with_metadata(
        &self,
        _type_id: u32,
        _metadata: &Metadata,
        bytes: &mut Vec<u8>,
    ) -> Result<(), Error> {
        self.0.encode_to(bytes);
        Ok(())
    }
}

// We can transparently Encode anything wrapped in EncodeStaticType, too.
impl<E: Encode> Encode for EncodeStaticType<E> {
    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        self.0.encode_to(dest)
    }
    fn encode(&self) -> Vec<u8> {
        self.0.encode()
    }
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        self.0.using_encoded(f)
    }
    fn encoded_size(&self) -> usize {
        self.0.encoded_size()
    }
}
