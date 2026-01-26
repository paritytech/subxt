use crate::config::Config;
use crate::config::TransactionExtension;
use crate::config::transaction_extensions::{
    ChargeAssetTxPayment, ChargeTransactionPayment, CheckNonce,
};
use crate::error::ExtrinsicError;
use frame_decode::extrinsics::ExtrinsicExtensions as ExtrinsicExtensionsInfo;
use scale_decode::DecodeAsType;
use subxt_metadata::ArcMetadata;

/// The signed extensions of an extrinsic.
#[derive(Debug, Clone)]
pub struct ExtrinsicTransactionExtensions<'extrinsic, T: Config> {
    bytes: &'extrinsic [u8],
    metadata: ArcMetadata,
    decoded_info: &'extrinsic ExtrinsicExtensionsInfo<'extrinsic, u32>,
    marker: core::marker::PhantomData<T>,
}

impl<'extrinsic, T: Config> ExtrinsicTransactionExtensions<'extrinsic, T> {
    pub(crate) fn new(
        bytes: &'extrinsic [u8],
        metadata: ArcMetadata,
        decoded_info: &'extrinsic ExtrinsicExtensionsInfo<'extrinsic, u32>,
    ) -> Self {
        Self {
            bytes,
            metadata,
            decoded_info,
            marker: core::marker::PhantomData,
        }
    }

    /// Returns an iterator over each of the signed extension details of the extrinsic.
    pub fn iter(&self) -> impl Iterator<Item = ExtrinsicTransactionExtension<'extrinsic, T>> {
        self.decoded_info
            .iter()
            .map(move |s| ExtrinsicTransactionExtension {
                bytes: &self.bytes[s.range()],
                ty_id: *s.ty(),
                identifier: s.name(),
                metadata: self.metadata.clone(),
                _marker: core::marker::PhantomData,
            })
    }

    /// Searches through all signed extensions to find a specific one.
    /// If the Signed Extension is not found `Ok(None)` is returned.
    /// If the Signed Extension is found but decoding failed `Err(_)` is returned.
    pub fn find<S: TransactionExtension<T>>(&self) -> Option<Result<S::Decoded, ExtrinsicError>> {
        for ext in self.iter() {
            if let Some(e) = ext.decode_as::<S>() {
                return Some(e);
            }
        }
        None
    }

    /// The tip of an extrinsic, extracted from the ChargeTransactionPayment or ChargeAssetTxPayment
    /// signed extension, depending on which is present.
    ///
    /// Returns `None` if  `tip` was not found or decoding failed.
    pub fn tip(&self) -> Option<u128> {
        // Note: the overhead of iterating multiple time should be negligible.
        if let Some(tip) = self.find::<ChargeTransactionPayment>() {
            return Some(tip.ok()?.tip());
        }
        if let Some(tip) = self.find::<ChargeAssetTxPayment<T>>() {
            return Some(tip.ok()?.tip());
        }
        None
    }

    /// The nonce of the account that submitted the extrinsic, extracted from the CheckNonce signed extension.
    ///
    /// Returns `None` if `nonce` was not found or decoding failed.
    pub fn nonce(&self) -> Option<u64> {
        self.find::<CheckNonce>()?.ok()
    }
}

/// A single signed extension
#[derive(Debug, Clone)]
pub struct ExtrinsicTransactionExtension<'extrinsic, T: Config> {
    bytes: &'extrinsic [u8],
    ty_id: u32,
    identifier: &'extrinsic str,
    metadata: ArcMetadata,
    _marker: core::marker::PhantomData<T>,
}

impl<'extrinsic, T: Config> ExtrinsicTransactionExtension<'extrinsic, T> {
    /// The bytes representing this signed extension.
    pub fn bytes(&self) -> &'extrinsic [u8] {
        self.bytes
    }

    /// The name of the signed extension.
    pub fn name(&self) -> &'extrinsic str {
        self.identifier
    }

    /// The type id of the signed extension.
    pub fn type_id(&self) -> u32 {
        self.ty_id
    }

    /// Decodes this signed extension based on the provided [`TransactionExtension`] type.
    pub fn decode_as<S: TransactionExtension<T>>(
        &self,
    ) -> Option<Result<S::Decoded, ExtrinsicError>> {
        if !S::matches(self.identifier, self.ty_id, self.metadata.types()) {
            return None;
        }
        Some(self.decode_unchecked_as::<S::Decoded>())
    }

    /// Decode the extension into some type which implements [`DecodeAsType`].
    ///
    /// This ignores the extension name, so you should first check that this is what you expect
    /// via [`Self::name()`].
    ///
    /// Prefer to use [`Self::decode_as`] where possible.
    pub fn decode_unchecked_as<E: DecodeAsType>(&self) -> Result<E, ExtrinsicError> {
        let value = E::decode_as_type(&mut &self.bytes[..], self.ty_id, self.metadata.types())
            .map_err(|e| ExtrinsicError::CouldNotDecodeTransactionExtension {
                name: self.identifier.to_owned(),
                error: e,
            })?;
        Ok(value)
    }
}
