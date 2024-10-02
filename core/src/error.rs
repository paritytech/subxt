// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The errors that can be emitted in this crate.

use core::fmt::Display;

use alloc::boxed::Box;
use alloc::string::String;
use subxt_metadata::StorageHasher;

/// The error emitted when something goes wrong.
#[derive(Debug)]
pub enum Error {
    /// Codec error.
    Codec(codec::Error),
    /// Metadata error.
    Metadata(MetadataError),
    /// Storage address error.
    StorageAddress(StorageAddressError),
    /// Error decoding to a [`crate::dynamic::Value`].
    Decode(scale_decode::Error),
    /// Error encoding from a [`crate::dynamic::Value`].
    Encode(scale_encode::Error),
    /// Error constructing the appropriate extrinsic params.
    ExtrinsicParams(ExtrinsicParamsError),
    /// Block body error.
    Block(BlockError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Codec(e) => write!(f, "Scale codec error: {e}"),
            Error::Metadata(e) => write!(f, "Metadata Error: {e}"),
            Error::StorageAddress(e) => write!(f, "Storage Error: {e}"),
            Error::Decode(e) => write!(f, "Error decoding into dynamic value: {e}"),
            Error::Encode(e) => write!(f, "Error encoding from dynamic value: {e}"),
            Error::ExtrinsicParams(e) => write!(f, "Extrinsic params error: {e}"),
            Error::Block(e) => write!(f, "Error working with block_body: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl_from!(ExtrinsicParamsError => Error::ExtrinsicParams);
impl_from!(BlockError => Error::Block);
impl_from!(MetadataError => Error::Metadata);
impl_from!(scale_decode::Error => Error::Decode);
impl_from!(scale_decode::visitor::DecodeError => Error::Decode);
impl_from!(scale_encode::Error => Error::Encode);
impl_from!(StorageAddressError => Error::StorageAddress);
impl_from!(codec::Error => Error::Codec);

/// Block error
#[derive(Debug)]
pub enum BlockError {
    /// Leftover bytes found after decoding the extrinsic.
    LeftoverBytes(usize),
    /// Something went wrong decoding the extrinsic.
    ExtrinsicDecodeError(ExtrinsicDecodeError),
}
impl Display for BlockError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BlockError::LeftoverBytes(n) => {
                write!(
                    f,
                    "After decoding, {n} bytes were left, suggesting that decoding may have failed"
                )
            }
            BlockError::ExtrinsicDecodeError(e) => {
                write!(f, "{e}")
            }
        }
    }
}

/// An alias for [`frame_decode::extrinsics::ExtrinsicDecodeError`].
///
pub type ExtrinsicDecodeError = frame_decode::extrinsics::ExtrinsicDecodeError;

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum MetadataError {
    /// The DispatchError type isn't available in the metadata
    DispatchErrorNotFound,
    /// Type not found in metadata.
    TypeNotFound(u32),
    /// Pallet not found (index).
    PalletIndexNotFound(u8),
    /// Pallet not found (name).
    PalletNameNotFound(String),
    /// Variant not found.
    VariantIndexNotFound(u8),
    /// Constant not found.
    ConstantNameNotFound(String),
    /// Call not found.
    CallNameNotFound(String),
    /// Runtime trait not found.
    RuntimeTraitNotFound(String),
    /// Runtime method not found.
    RuntimeMethodNotFound(String),
    /// Call type not found in metadata.
    CallTypeNotFoundInPallet(u8),
    /// Event type not found in metadata.
    EventTypeNotFoundInPallet(u8),
    /// Storage details not found in metadata.
    StorageNotFoundInPallet(String),
    /// Storage entry not found.
    StorageEntryNotFound(String),
    /// The generated interface used is not compatible with the node.
    IncompatibleCodegen,
    /// Custom value not found.
    CustomValueNameNotFound(String),
}
impl Display for MetadataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MetadataError::DispatchErrorNotFound => {
                write!(f, "The DispatchError type isn't available")
            }
            MetadataError::TypeNotFound(e) => write!(f, "Type with ID {e} not found"),
            MetadataError::PalletIndexNotFound(e) => write!(f, "Pallet with index {e} not found"),
            MetadataError::PalletNameNotFound(e) => write!(f, "Pallet with name {e} not found"),
            MetadataError::VariantIndexNotFound(e) => write!(f, "Variant with index {e} not found"),
            MetadataError::ConstantNameNotFound(e) => write!(f, "Constant with name {e} not found"),
            MetadataError::CallNameNotFound(e) => write!(f, "Call with name {e} not found"),
            MetadataError::RuntimeTraitNotFound(e) => {
                write!(f, "Runtime trait with name {e} not found")
            }
            MetadataError::RuntimeMethodNotFound(e) => {
                write!(f, "Runtime method with name {e} not found")
            }
            MetadataError::CallTypeNotFoundInPallet(e) => {
                write!(f, "Call type not found in pallet with index {e}")
            }
            MetadataError::EventTypeNotFoundInPallet(e) => {
                write!(f, "Event type not found in pallet with index {e}")
            }
            MetadataError::StorageNotFoundInPallet(e) => {
                write!(f, "Storage details not found in pallet with name {e}")
            }
            MetadataError::StorageEntryNotFound(e) => write!(f, "Storage entry {e} not found"),
            MetadataError::IncompatibleCodegen => {
                write!(f, "The generated code is not compatible with the node")
            }
            MetadataError::CustomValueNameNotFound(e) => {
                write!(f, "Custom value with name {e} not found")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MetadataError {}

/// Something went wrong trying to encode or decode a storage address.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum StorageAddressError {
    /// Storage lookup does not have the expected number of keys.
    TooManyKeys {
        /// The number of keys provided in the storage address.
        expected: usize,
    },
    /// This storage entry in the metadata does not have the correct number of hashers to fields.
    WrongNumberOfHashers {
        /// The number of hashers in the metadata for this storage entry.
        hashers: usize,
        /// The number of fields in the metadata for this storage entry.
        fields: usize,
    },
    /// We weren't given enough bytes to decode the storage address/key.
    NotEnoughBytes,
    /// We have leftover bytes after decoding the storage address.
    TooManyBytes,
    /// The bytes of a storage address are not the expected address for decoding the storage keys of the address.
    UnexpectedAddressBytes,
    /// An invalid hasher was used to reconstruct a value from a chunk of bytes that is part of a storage address. Hashers where the hash does not contain the original value are invalid for this purpose.
    HasherCannotReconstructKey {
        /// Type id of the key's type.
        ty_id: u32,
        /// The invalid hasher that caused this error.
        hasher: StorageHasher,
    },
}

impl Display for StorageAddressError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StorageAddressError::TooManyKeys { expected } => write!(
                f,
                "Storage lookup requires {expected} keys but more keys have been provided."
            ),
            StorageAddressError::WrongNumberOfHashers { .. } => write!(
                f,
                "Storage entry in metadata does not have the correct number of hashers to fields"
            ),
            StorageAddressError::NotEnoughBytes => write!(
                f,
                "Not enough remaining bytes to decode the storage address/key"
            ),
            StorageAddressError::TooManyBytes => write!(
                f,
                "We have leftover bytes after decoding the storage address"
            ),
            StorageAddressError::UnexpectedAddressBytes => write!(
                f,
                "Storage address bytes are not the expected format. Addresses need to be at least 16 bytes (pallet ++ entry) and follow a structure given by the hashers defined in the metadata"
            ),
            StorageAddressError::HasherCannotReconstructKey { ty_id, hasher } => write!(
                f,
                "An invalid hasher was used to reconstruct a value with type ID {ty_id} from a hash formed by a {hasher:?} hasher. This is only possible for concat-style hashers or the identity hasher"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for StorageAddressError {}

/// An error that can be emitted when trying to construct an instance of [`crate::config::ExtrinsicParams`],
/// encode data from the instance, or match on signed extensions.
#[derive(Debug)]
#[non_exhaustive]
pub enum ExtrinsicParamsError {
    /// Cannot find a type id in the metadata. The context provides some additional
    /// information about the source of the error (eg the signed extension name).
    MissingTypeId {
        /// Type ID.
        type_id: u32,
        /// Some arbitrary context to help narrow the source of the error.
        context: &'static str,
    },
    /// A signed extension in use on some chain was not provided.
    UnknownSignedExtension(String),
    /// Some custom error.
    Custom(Box<dyn CustomError>),
}

impl Display for ExtrinsicParamsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExtrinsicParamsError::MissingTypeId { type_id, context } => write!(
                f,
                "Cannot find type id '{type_id} in the metadata (context: {context})"
            ),
            ExtrinsicParamsError::UnknownSignedExtension(e) => write!(
                f,
                "The chain expects a signed extension with the name {e}, but we did not provide one"
            ),
            ExtrinsicParamsError::Custom(e) => {
                write!(f, "Error constructing extrinsic parameters: {e}")
            }
        }
    }
}

/// Anything implementing this trait can be used in [`ExtrinsicParamsError::Custom`].
#[cfg(feature = "std")]
pub trait CustomError: std::error::Error + Send + Sync + 'static {}
#[cfg(feature = "std")]
impl<T: std::error::Error + Send + Sync + 'static> CustomError for T {}

/// Anything implementing this trait can be used in [`ExtrinsicParamsError::Custom`].
#[cfg(not(feature = "std"))]
pub trait CustomError: core::fmt::Debug + core::fmt::Display + Send + Sync + 'static {}
#[cfg(not(feature = "std"))]
impl<T: core::fmt::Debug + core::fmt::Display + Send + Sync + 'static> CustomError for T {}

#[cfg(feature = "std")]
impl std::error::Error for ExtrinsicParamsError {}

impl From<core::convert::Infallible> for ExtrinsicParamsError {
    fn from(value: core::convert::Infallible) -> Self {
        match value {}
    }
}

impl From<Box<dyn CustomError>> for ExtrinsicParamsError {
    fn from(value: Box<dyn CustomError>) -> Self {
        ExtrinsicParamsError::Custom(value)
    }
}
