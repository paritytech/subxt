// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::Metadata;
use codec::{
    Error as DecodeError,
    Encode,
    Decode,
};

pub trait DecodeWithMetadata: Sized {
    type Target;
    /// Given some metadata and a type ID, attempt to SCALE decode the provided bytes into `Self`.
    fn decode_with_metadata(bytes: &[u8], type_id: u32, metadata: &Metadata) -> Result<Self::Target, DecodeError>;
}

pub trait EncodeWithMetadata {
    /// Given some metadata and a type ID, attempt to SCALE encode `Self` to the provided bytes.
    fn encode_to_with_metadata(&self, type_id: u32, metadata: &Metadata, out: &mut Vec<u8>);

    /// Given some metadata and a type ID, attempt to SCALE encode `Self` and return the resulting bytes.
    fn encode_with_metadata(&self, type_id: u32, metadata: &Metadata) {
        let mut out = Vec::new();
        self.encode_to_with_metadata(type_id, metadata, &mut out)
    }
}

/// A wrapper which implements [`DecodeWithMetadata`] if the inner type implements [`Decode`],
/// and [`EncodeWithMetadata`] if the inner type implements [`Encode`].
pub struct EncodeDecodeWrapper<T>(T);

impl <T: Encode> EncodeWithMetadata for EncodeDecodeWrapper<T> {
    fn encode_to_with_metadata(&self, _type_id: u32, _metadata: &Metadata, out: &mut Vec<u8>) {
        self.0.encode_to(out)
    }
}

impl <T: Decode> DecodeWithMetadata for EncodeDecodeWrapper<T> {
    type Target = T;
    fn decode_with_metadata(bytes: &[u8], type_id: u32, metadata: &Metadata) -> Result<Self::Target, DecodeError> {
        T::decode(&mut bytes)
    }
}