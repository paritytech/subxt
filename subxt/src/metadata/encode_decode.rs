// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    Metadata,
    MetadataError,
};
use crate::{
    error::BasicError,
};
use codec::{
    Encode,
};

pub trait EncodeWithMetadata {
    /// Given some metadata, attempt to SCALE encode `Self` to the provided bytes.
    fn encode_to_with_metadata(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), BasicError>;

    /// Given some metadata, attempt to SCALE encode `Self` and return the resulting bytes.
    fn encode_with_metadata(&self, metadata: &Metadata) -> Result<Vec<u8>, BasicError> {
        let mut out = Vec::new();
        self.encode_to_with_metadata(metadata, &mut out)?;
        Ok(out)
    }
}

/// A wrapper which implements [`EncodeWithMetadata`] if the data provided implements [`Encode`].
pub struct EncodeStaticCall<T> {
    pub pallet: &'static str,
    pub call: &'static str,
    pub data: T,
}

impl <T: Encode> EncodeWithMetadata for EncodeStaticCall<T> {
    fn encode_to_with_metadata(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), BasicError> {
        let pallet = metadata.pallet(self.pallet)?;
        let pallet_index = pallet.index();
        let call_index = pallet.call_index(self.call)?;

        pallet_index.encode_to(out);
        call_index.encode_to(out);
        self.data.encode_to(out);
        Ok(())
    }
}

/// A wrapper which allows dynamic Value types to be SCALE encoded via [`EncodeWithMetadata`].
pub struct EncodeDynamicCall {
    pub pallet: &'static str,
    pub call: &'static str,
    pub data: Vec<scale_value::Value>,
}

impl EncodeWithMetadata for EncodeDynamicCall {
    fn encode_to_with_metadata(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), BasicError> {
        let pallet = metadata.pallet(self.pallet)?;
        let pallet_index = pallet.index();
        let call_ty = pallet
            .call_ty_id()
            .ok_or_else(|| MetadataError::CallNotFound)?;

        // Assemble the variant representing the specific call within the pallet.
        // (we could do this ourselves a little more efficiently but it's easier
        // reusing scale_value logic).
        let composite = scale_value::Composite::Unnamed(self.data.clone());
        let variant = scale_value::Value::variant(self.call, composite);

        // Encode the pallet index and call variant+data:
        pallet_index.encode_to(out);
        scale_value::scale::encode_as_type(variant, call_ty, metadata.types(), out)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// pub trait DecodeWithMetadata: Sized {
//     type Target;
//     /// Given some metadata and a type ID, attempt to SCALE decode the provided bytes into `Self`.
//     fn decode_with_metadata(bytes: &[u8], type_id: u32, metadata: &Metadata) -> Result<Self::Target, BasicError>;
// }
//
// impl <T: Decode> DecodeWithMetadata for EncodeDecodeStaticCall<T> {
//     type Target = T;
//     fn decode_with_metadata(mut bytes: &[u8], _type_id: u32, _metadata: &Metadata) -> Result<Self::Target, BasicError> {
//         T::decode(&mut bytes)
//     }
// }