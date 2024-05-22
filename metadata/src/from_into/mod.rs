// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use alloc::string::String;
use snafu::Snafu;
mod v14;
mod v15;

/// An error emitted if something goes wrong converting [`frame_metadata`]
/// types into [`crate::Metadata`].
#[derive(Debug, PartialEq, Eq, Snafu)]
#[non_exhaustive]
pub enum TryFromError {
    /// Type missing from type registry
    #[snafu(display("Type id {type_id} is expected but not found in the type registry"))]
    TypeNotFound {
        /// Id of the type
        type_id: u32,
    },
    /// Type was not a variant/enum type
    #[snafu(display("Type {type_id} was not a variant/enum type, but is expected to be one"))]
    VariantExpected {
        /// Id of the type
        type_id: u32,
    },
    /// An unsupported metadata version was provided.
    #[snafu(display("Cannot convert v{version} metadata into Metadata type"))]
    UnsupportedMetadataVersion {
        /// Metadata version
        version: u32,
    },
    /// Type name missing from type registry
    #[snafu(display("Type name {name} is expected but not found in the type registry"))]
    TypeNameNotFound {
        /// Name of the type
        name: String,
    },
    /// Invalid type path.
    #[snafu(display("Type has an invalid path {path}"))]
    InvalidTypePath {
        /// Path of the type
        path: String,
    },
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
                Err(TryFromError::UnsupportedMetadataVersion { version: 0 })
            }
            frame_metadata::RuntimeMetadata::V1(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 1 })
            }
            frame_metadata::RuntimeMetadata::V2(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 2 })
            }
            frame_metadata::RuntimeMetadata::V3(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 3 })
            }
            frame_metadata::RuntimeMetadata::V4(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 4 })
            }
            frame_metadata::RuntimeMetadata::V5(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 5 })
            }
            frame_metadata::RuntimeMetadata::V6(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 6 })
            }
            frame_metadata::RuntimeMetadata::V7(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 7 })
            }
            frame_metadata::RuntimeMetadata::V8(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 8 })
            }
            frame_metadata::RuntimeMetadata::V9(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 9 })
            }
            frame_metadata::RuntimeMetadata::V10(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 10 })
            }
            frame_metadata::RuntimeMetadata::V11(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 11 })
            }
            frame_metadata::RuntimeMetadata::V12(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 12 })
            }
            frame_metadata::RuntimeMetadata::V13(_) => {
                Err(TryFromError::UnsupportedMetadataVersion { version: 13 })
            }
            frame_metadata::RuntimeMetadata::V14(m) => m.try_into(),
            frame_metadata::RuntimeMetadata::V15(m) => m.try_into(),
        }
    }
}
