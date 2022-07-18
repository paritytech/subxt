// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    Metadata,
    MetadataError,
    MetadataLocation,
};
use crate::error::Error;
use codec::Encode;
use std::borrow::Cow;

/// This trait represents any type that can be encoded to bytes with the support of [`Metadata`].
pub trait EncodeWithMetadata {
    /// Given some metadata, attempt to SCALE encode `Self` to the provided bytes.
    fn encode_to_with_metadata(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error>;

    /// Given some metadata, attempt to SCALE encode `Self` and return the resulting bytes.
    fn encode_with_metadata(&self, metadata: &Metadata) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        self.encode_to_with_metadata(metadata, &mut out)?;
        Ok(out)
    }
}

/// A wrapper which implements [`EncodeWithMetadata`] if the data provided implements [`Encode`].
pub struct EncodeStaticCall<T> {
    /// The pallet name
    pub pallet: &'static str,
    /// The call/fucntion name within the pallet
    pub call: &'static str,
    /// Data representing the arguments to pass to the call.
    pub data: T,
}

impl<T: Encode> EncodeWithMetadata for EncodeStaticCall<T> {
    fn encode_to_with_metadata(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let pallet = metadata.pallet(self.pallet)?;
        let pallet_index = pallet.index();
        let call_index = pallet.call_index(self.call)?;

        pallet_index.encode_to(out);
        call_index.encode_to(out);
        self.data.encode_to(out);
        Ok(())
    }
}

impl<T> MetadataLocation for EncodeStaticCall<T> {
    fn pallet(&self) -> &str {
        self.pallet
    }
    fn item(&self) -> &str {
        self.call
    }
}

/// A wrapper which allows dynamic Value types to be SCALE encoded via [`EncodeWithMetadata`].
pub struct EncodeDynamicCall<'a> {
    pallet: Cow<'a, str>,
    call: Cow<'a, str>,
    data: Vec<scale_value::Value>,
}

impl<'a> EncodeDynamicCall<'a> {
    /// Construct a new [`EncodeDynamicCall`], which can be SCALE encoded to call data.
    pub fn new(
        pallet: impl Into<Cow<'a, str>>,
        call: impl Into<Cow<'a, str>>,
        data: Vec<scale_value::Value>,
    ) -> Self {
        Self {
            pallet: pallet.into(),
            call: call.into(),
            data,
        }
    }
}

impl<'a> MetadataLocation for EncodeDynamicCall<'a> {
    fn pallet(&self) -> &str {
        self.pallet.as_ref()
    }
    fn item(&self) -> &str {
        self.call.as_ref()
    }
}

impl<'a> EncodeWithMetadata for EncodeDynamicCall<'a> {
    fn encode_to_with_metadata(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let pallet = metadata.pallet(&self.pallet)?;
        let pallet_index = pallet.index();
        let call_ty = pallet.call_ty_id().ok_or(MetadataError::CallNotFound)?;

        // Assemble the variant representing the specific call within the pallet.
        // (we could do this ourselves a little more efficiently but it's easier
        // reusing scale_value logic).
        let composite = scale_value::Composite::Unnamed(self.data.clone());
        let variant = scale_value::Value::variant(self.call.to_owned(), composite);

        // Encode the pallet index and call variant+data:
        pallet_index.encode_to(out);
        scale_value::scale::encode_as_type(variant, call_ty, metadata.types(), out)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
