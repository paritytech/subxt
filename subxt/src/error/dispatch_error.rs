// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A representation of the dispatch error; an error returned when
//! something fails in trying to submit/execute a transaction.

use crate::metadata::{DecodeWithMetadata, Metadata};
use crate::prelude::*;
use core::fmt::Debug;
use derive_more::Display;
use scale_decode::{visitor::DecodeAsTypeResult, DecodeAsType};
use std::borrow::Cow;

use super::{Error, MetadataError};

/// An error dispatching a transaction.
#[derive(Debug, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum DispatchError {
    /// Some error occurred.
    #[display(fmt = "Some unknown error occurred.")]
    Other,
    /// Failed to lookup some data.
    #[display(fmt = "Failed to lookup some data.")]
    CannotLookup,
    /// A bad origin.
    #[display(fmt = "Bad origin.")]
    BadOrigin,
    /// A custom error in a module.
    #[display(fmt = "Pallet error: {_0}")]
    Module(ModuleError),
    /// At least one consumer is remaining so the account cannot be destroyed.
    #[display(fmt = "At least one consumer is remaining so the account cannot be destroyed.")]
    ConsumerRemaining,
    /// There are no providers so the account cannot be created.
    #[display(fmt = "There are no providers so the account cannot be created.")]
    NoProviders,
    /// There are too many consumers so the account cannot be created.
    #[display(fmt = "There are too many consumers so the account cannot be created.")]
    TooManyConsumers,
    /// An error to do with tokens.
    #[display(fmt = "Token error: {_0}")]
    Token(TokenError),
    /// An arithmetic error.
    #[display(fmt = "Arithmetic error: {_0}")]
    Arithmetic(ArithmeticError),
    /// The number of transactional layers has been reached, or we are not in a transactional layer.
    #[display(fmt = "Transactional error: {_0}")]
    Transactional(TransactionalError),
    /// Resources exhausted, e.g. attempt to read/write data which is too large to manipulate.
    #[display(
        fmt = "Resources exhausted, e.g. attempt to read/write data which is too large to manipulate."
    )]
    Exhausted,
    /// The state is corrupt; this is generally not going to fix itself.
    #[display(fmt = "The state is corrupt; this is generally not going to fix itself.")]
    Corruption,
    /// Some resource (e.g. a preimage) is unavailable right now. This might fix itself later.
    #[display(
        fmt = "Some resource (e.g. a preimage) is unavailable right now. This might fix itself later."
    )]
    Unavailable,
}

#[cfg(feature = "std")]
impl std::error::Error for DispatchError {}

/// An error relating to tokens when dispatching a transaction.
#[derive(scale_decode::DecodeAsType, Display, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TokenError {
    /// Funds are unavailable.
    #[display(fmt = "Funds are unavailable.")]
    FundsUnavailable,
    /// Some part of the balance gives the only provider reference to the account and thus cannot be (re)moved.
    #[display(
        fmt = "Some part of the balance gives the only provider reference to the account and thus cannot be (re)moved."
    )]
    OnlyProvider,
    /// Account cannot exist with the funds that would be given.
    #[display(fmt = "Account cannot exist with the funds that would be given.")]
    BelowMinimum,
    /// Account cannot be created.
    #[display(fmt = "Account cannot be created.")]
    CannotCreate,
    /// The asset in question is unknown.
    #[display(fmt = "The asset in question is unknown.")]
    UnknownAsset,
    /// Funds exist but are frozen.
    #[display(fmt = "Funds exist but are frozen.")]
    Frozen,
    /// Operation is not supported by the asset.
    #[display(fmt = "Operation is not supported by the asset.")]
    Unsupported,
    /// Account cannot be created for a held balance.
    #[display(fmt = "Account cannot be created for a held balance.")]
    CannotCreateHold,
    /// Withdrawal would cause unwanted loss of account.
    #[display(fmt = "Withdrawal would cause unwanted loss of account.")]
    NotExpendable,
}

#[cfg(feature = "std")]
impl std::error::Error for TokenError {}

/// An error relating to arithmetic when dispatching a transaction.
#[derive(scale_decode::DecodeAsType, Debug, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArithmeticError {
    /// Underflow.
    #[display(fmt = "Underflow.")]
    Underflow,
    /// Overflow.
    #[display(fmt = "Overflow.")]
    Overflow,
    /// Division by zero.
    #[display(fmt = "Division by zero.")]
    DivisionByZero,
}

#[cfg(feature = "std")]
impl std::error::Error for ArithmeticError {}

/// An error relating to thr transactional layers when dispatching a transaction.
#[derive(scale_decode::DecodeAsType, Debug, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransactionalError {
    /// Too many transactional layers have been spawned.
    #[display(fmt = "Too many transactional layers have been spawned.")]
    LimitReached,
    /// A transactional layer was expected, but does not exist.
    #[display(fmt = "A transactional layer was expected, but does not exist.")]
    NoLayer,
}

#[cfg(feature = "std")]
impl std::error::Error for TransactionalError {}

/// Details about a module error that has occurred.
#[derive(Clone)]
#[non_exhaustive]
pub struct ModuleError {
    metadata: Metadata,
    /// Bytes representation:
    ///  - `bytes[0]`:   pallet index
    ///  - `bytes[1]`:   error index
    ///  - `bytes[2..]`: 3 bytes specific for the module error
    bytes: [u8; 5],
}

impl PartialEq for ModuleError {
    fn eq(&self, other: &Self) -> bool {
        // A module error is the same if the raw underlying details are the same.
        self.bytes == other.bytes
    }
}

impl Eq for ModuleError {}

/// Custom `Debug` implementation, ignores the very large `metadata` field, using it instead (as
/// intended) to resolve the actual pallet and error names. This is much more useful for debugging.
impl Debug for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let details = self.details_string();
        write!(f, "ModuleError(<{details}>)")
    }
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let details = self.details_string();
        write!(f, "{details}")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ModuleError {}

impl ModuleError {
    /// Return more details about this error.
    pub fn details(&self) -> Result<ModuleErrorDetails, MetadataError> {
        let pallet = self.metadata.pallet_by_index_err(self.pallet_index())?;
        let variant = pallet
            .error_variant_by_index(self.error_index())
            .ok_or_else(|| MetadataError::VariantIndexNotFound(self.error_index()))?;

        Ok(ModuleErrorDetails { pallet, variant })
    }

    /// Return a formatted string of the resolved error details for debugging/display purposes.
    pub fn details_string(&self) -> String {
        match self.details() {
            Ok(details) => format!(
                "{pallet_name}::{variant_name}",
                pallet_name = details.pallet.name(),
                variant_name = details.variant.name,
            ),
            Err(_) => format!(
                "Unknown pallet error '{bytes:?}' (pallet and error details cannot be retrieved)",
                bytes = self.bytes
            ),
        }
    }

    /// Return the underlying module error data that was decoded.
    pub fn bytes(&self) -> [u8; 5] {
        self.bytes
    }

    /// Obtain the pallet index from the underlying byte data.
    pub fn pallet_index(&self) -> u8 {
        self.bytes[0]
    }

    /// Obtain the error index from the underlying byte data.
    pub fn error_index(&self) -> u8 {
        self.bytes[1]
    }

    /// Attempts to decode the ModuleError into the top outer Error enum.
    pub fn as_root_error<E: DecodeAsType>(&self) -> Result<E, Error> {
        let decoded = E::decode_as_type(
            &mut &self.bytes[..],
            self.metadata.outer_enums().error_enum_ty(),
            self.metadata.types(),
        )?;

        Ok(decoded)
    }
}

/// Details about the module error.
pub struct ModuleErrorDetails<'a> {
    /// The pallet that the error is in
    pub pallet: crate::metadata::types::PalletMetadata<'a>,
    /// The variant representing the error
    pub variant: &'a scale_info::Variant<scale_info::form::PortableForm>,
}

impl DispatchError {
    /// Attempt to decode a runtime [`DispatchError`].
    #[doc(hidden)]
    pub fn decode_from<'a>(
        bytes: impl Into<Cow<'a, [u8]>>,
        metadata: Metadata,
    ) -> Result<Self, super::Error> {
        let bytes = bytes.into();
        let dispatch_error_ty_id = metadata
            .dispatch_error_ty()
            .ok_or(MetadataError::DispatchErrorNotFound)?;

        // The aim is to decode our bytes into roughly this shape. This is copied from
        // `sp_runtime::DispatchError`; we need the variant names and any inner variant
        // names/shapes to line up in order for decoding to be successful.
        #[derive(scale_decode::DecodeAsType)]
        enum DecodedDispatchError {
            Other,
            CannotLookup,
            BadOrigin,
            Module(DecodedModuleErrorBytes),
            ConsumerRemaining,
            NoProviders,
            TooManyConsumers,
            Token(TokenError),
            Arithmetic(ArithmeticError),
            Transactional(TransactionalError),
            Exhausted,
            Corruption,
            Unavailable,
        }

        // ModuleError is a bit special; we want to support being decoded from either
        // a legacy format of 2 bytes, or a newer format of 5 bytes. So, just grab the bytes
        // out when decoding to manually work with them.
        struct DecodedModuleErrorBytes(Vec<u8>);
        struct DecodedModuleErrorBytesVisitor;
        impl scale_decode::Visitor for DecodedModuleErrorBytesVisitor {
            type Error = scale_decode::Error;
            type Value<'scale, 'info> = DecodedModuleErrorBytes;
            fn unchecked_decode_as_type<'scale, 'info>(
                self,
                input: &mut &'scale [u8],
                _type_id: scale_decode::visitor::TypeId,
                _types: &'info scale_info::PortableRegistry,
            ) -> DecodeAsTypeResult<Self, Result<Self::Value<'scale, 'info>, Self::Error>>
            {
                DecodeAsTypeResult::Decoded(Ok(DecodedModuleErrorBytes(input.to_vec())))
            }
        }
        impl scale_decode::IntoVisitor for DecodedModuleErrorBytes {
            type Visitor = DecodedModuleErrorBytesVisitor;
            fn into_visitor() -> Self::Visitor {
                DecodedModuleErrorBytesVisitor
            }
        }

        // Decode into our temporary error:
        let decoded_dispatch_err = DecodedDispatchError::decode_with_metadata(
            &mut &*bytes,
            dispatch_error_ty_id,
            &metadata,
        )?;

        // Convert into the outward-facing error, mainly by handling the Module variant.
        let dispatch_error = match decoded_dispatch_err {
            // Mostly we don't change anything from our decoded to our outward-facing error:
            DecodedDispatchError::Other => DispatchError::Other,
            DecodedDispatchError::CannotLookup => DispatchError::CannotLookup,
            DecodedDispatchError::BadOrigin => DispatchError::BadOrigin,
            DecodedDispatchError::ConsumerRemaining => DispatchError::ConsumerRemaining,
            DecodedDispatchError::NoProviders => DispatchError::NoProviders,
            DecodedDispatchError::TooManyConsumers => DispatchError::TooManyConsumers,
            DecodedDispatchError::Token(val) => DispatchError::Token(val),
            DecodedDispatchError::Arithmetic(val) => DispatchError::Arithmetic(val),
            DecodedDispatchError::Transactional(val) => DispatchError::Transactional(val),
            DecodedDispatchError::Exhausted => DispatchError::Exhausted,
            DecodedDispatchError::Corruption => DispatchError::Corruption,
            DecodedDispatchError::Unavailable => DispatchError::Unavailable,
            // But we apply custom logic to transform the module error into the outward facing version:
            DecodedDispatchError::Module(module_bytes) => {
                let module_bytes = module_bytes.0;

                // The old version is 2 bytes; a pallet and error index.
                // The new version is 5 bytes; a pallet and error index and then 3 extra bytes.
                let bytes = if module_bytes.len() == 2 {
                    [module_bytes[0], module_bytes[1], 0, 0, 0]
                } else if module_bytes.len() == 5 {
                    [
                        module_bytes[0],
                        module_bytes[1],
                        module_bytes[2],
                        module_bytes[3],
                        module_bytes[4],
                    ]
                } else {
                    tracing::warn!("Can't decode error sp_runtime::DispatchError: bytes do not match known shapes");
                    // Return _all_ of the bytes; every "unknown" return should be consistent.
                    return Err(super::Error::Unknown(bytes.to_vec()));
                };

                // And return our outward-facing version:
                DispatchError::Module(ModuleError { metadata, bytes })
            }
        };

        Ok(dispatch_error)
    }
}
