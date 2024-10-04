// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::config::signed_extensions::{
    ChargeAssetTxPayment, ChargeTransactionPayment, CheckNonce,
};
use crate::config::SignedExtension;
use crate::dynamic::Value;
use crate::{config::Config, error::Error, Metadata};
use frame_decode::extrinsics::ExtrinsicExtensions;
use scale_decode::DecodeAsType;

/// The signed extensions of an extrinsic.
#[derive(Debug, Clone)]
pub struct ExtrinsicSignedExtensions<'a, T: Config> {
    bytes: &'a [u8],
    metadata: &'a Metadata,
    decoded_info: &'a ExtrinsicExtensions<'static, u32>,
    _marker: core::marker::PhantomData<T>,
}

impl<'a, T: Config> ExtrinsicSignedExtensions<'a, T> {
    pub(crate) fn new(
        bytes: &'a [u8],
        metadata: &'a Metadata,
        decoded_info: &'a ExtrinsicExtensions<'static, u32>,
    ) -> Self {
        Self {
            bytes,
            metadata,
            decoded_info,
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns an iterator over each of the signed extension details of the extrinsic.
    pub fn iter(&self) -> impl Iterator<Item = ExtrinsicSignedExtension<T>> {
        self.decoded_info.iter().map(|s| ExtrinsicSignedExtension {
            bytes: &self.bytes[s.range()],
            ty_id: *s.ty(),
            identifier: s.name(),
            metadata: self.metadata,
            _marker: core::marker::PhantomData,
        })
    }

    /// Searches through all signed extensions to find a specific one.
    /// If the Signed Extension is not found `Ok(None)` is returned.
    /// If the Signed Extension is found but decoding failed `Err(_)` is returned.
    pub fn find<S: SignedExtension<T>>(&self) -> Result<Option<S::Decoded>, Error> {
        for ext in self.iter() {
            match ext.as_signed_extension::<S>() {
                // We found a match; return it:
                Ok(Some(e)) => return Ok(Some(e)),
                // No error, but no match either; next!
                Ok(None) => continue,
                // Error? return it
                Err(e) => return Err(e),
            }
        }
        Ok(None)
    }

    /// The tip of an extrinsic, extracted from the ChargeTransactionPayment or ChargeAssetTxPayment
    /// signed extension, depending on which is present.
    ///
    /// Returns `None` if  `tip` was not found or decoding failed.
    pub fn tip(&self) -> Option<u128> {
        // Note: the overhead of iterating multiple time should be negligible.
        self.find::<ChargeTransactionPayment>()
            .ok()
            .flatten()
            .map(|e| e.tip())
            .or_else(|| {
                self.find::<ChargeAssetTxPayment<T>>()
                    .ok()
                    .flatten()
                    .map(|e| e.tip())
            })
    }

    /// The nonce of the account that submitted the extrinsic, extracted from the CheckNonce signed extension.
    ///
    /// Returns `None` if `nonce` was not found or decoding failed.
    pub fn nonce(&self) -> Option<u64> {
        self.find::<CheckNonce>().ok()?
    }
}

/// A single signed extension
#[derive(Debug, Clone)]
pub struct ExtrinsicSignedExtension<'a, T: Config> {
    bytes: &'a [u8],
    ty_id: u32,
    identifier: &'a str,
    metadata: &'a Metadata,
    _marker: core::marker::PhantomData<T>,
}

impl<'a, T: Config> ExtrinsicSignedExtension<'a, T> {
    /// The bytes representing this signed extension.
    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// The name of the signed extension.
    pub fn name(&self) -> &'a str {
        self.identifier
    }

    /// The type id of the signed extension.
    pub fn type_id(&self) -> u32 {
        self.ty_id
    }

    /// Signed Extension as a [`scale_value::Value`]
    pub fn value(&self) -> Result<Value<u32>, Error> {
        let value = scale_value::scale::decode_as_type(
            &mut &self.bytes[..],
            self.ty_id,
            self.metadata.types(),
        )?;
        Ok(value)
    }

    /// Decodes the bytes of this Signed Extension into its associated `Decoded` type.
    /// Returns `Ok(None)` if the data we have doesn't match the Signed Extension we're asking to
    /// decode with.
    pub fn as_signed_extension<S: SignedExtension<T>>(&self) -> Result<Option<S::Decoded>, Error> {
        if !S::matches(self.identifier, self.ty_id, self.metadata.types()) {
            return Ok(None);
        }
        self.as_type::<S::Decoded>().map(Some)
    }

    fn as_type<E: DecodeAsType>(&self) -> Result<E, Error> {
        let value = E::decode_as_type(&mut &self.bytes[..], self.ty_id, self.metadata.types())?;
        Ok(value)
    }
}
