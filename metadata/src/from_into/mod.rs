// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::prelude::*;
use derive_more::Display;
use string::String;

mod v14;
mod v15;

/// An error emitted if something goes wrong converting [`frame_metadata`]
/// types into [`crate::Metadata`].
#[derive(Debug, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum TryFromError {
    /// Type missing from type registry
    #[display(fmt = "Type id {_0} is expected but not found in the type registry")]
    TypeNotFound(u32),
    /// Type was not a variant/enum type
    #[display(fmt = "Type {_0} was not a variant/enum type, but is expected to be one")]
    VariantExpected(u32),
    /// An unsupported metadata version was provided.
    #[display(fmt = "Cannot convert v{_0} metadata into Metadata type")]
    UnsupportedMetadataVersion(u32),
    /// Type name missing from type registry
    #[display(fmt = "Type name {_0} is expected but not found in the type registry")]
    TypeNameNotFound(String),
    /// Invalid type path.
    #[display(fmt = "Type has an invalid path {_0}")]
    InvalidTypePath(String),
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromError {}

impl From<crate::Metadata> for frame_metadata::RuntimeMetadataPrefixed {
    fn from(value: crate::Metadata) -> Self {
        let m: frame_metadata::v15::RuntimeMetadataV15 = value.into();
        m.into()
    }
}

impl TryFrom<frame_metadata::RuntimeMetadataPrefixed> for crate::Metadata {
    type Error = TryFromError;

    fn try_from(value: frame_metadata::RuntimeMetadataPrefixed) -> Result<Self, Self::Error> {
        match value.1 {
            frame_metadata::RuntimeMetadata::V0(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(0))
            }
            frame_metadata::RuntimeMetadata::V1(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(1))
            }
            frame_metadata::RuntimeMetadata::V2(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(2))
            }
            frame_metadata::RuntimeMetadata::V3(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(3))
            }
            frame_metadata::RuntimeMetadata::V4(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(4))
            }
            frame_metadata::RuntimeMetadata::V5(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(5))
            }
            frame_metadata::RuntimeMetadata::V6(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(6))
            }
            frame_metadata::RuntimeMetadata::V7(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(7))
            }
            frame_metadata::RuntimeMetadata::V8(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(8))
            }
            frame_metadata::RuntimeMetadata::V9(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(9))
            }
            frame_metadata::RuntimeMetadata::V10(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(10))
            }
            frame_metadata::RuntimeMetadata::V11(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(11))
            }
            frame_metadata::RuntimeMetadata::V12(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(12))
            }
            frame_metadata::RuntimeMetadata::V13(_) => {
                Err(TryFromError::UnsupportedMetadataVersion(13))
            }
            frame_metadata::RuntimeMetadata::V14(m) => m.try_into(),
            frame_metadata::RuntimeMetadata::V15(m) => m.try_into(),
        }
    }
}
