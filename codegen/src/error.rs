// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Errors that can be emitted from codegen.

use proc_macro2::{Span, TokenStream as TokenStream2};
use scale_typegen::TypegenError;

/// Error returned when the Codegen cannot generate the runtime API.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CodegenError {
    /// Cannot decode the metadata bytes.
    #[error("Could not decode metadata, only V14 and V15 metadata are supported: {0}")]
    Decode(#[from] codec::Error),
    /// Out of line modules are not supported.
    #[error("Out-of-line subxt modules are not supported, make sure you are providing a body to your module: pub mod polkadot {{ ... }}")]
    InvalidModule(Span),
    /// Invalid type path.
    #[error("Invalid type path {0}: {1}")]
    InvalidTypePath(String, syn::Error),
    /// Metadata for constant could not be found.
    #[error("Metadata for constant entry {0}_{1} could not be found. Make sure you are providing a valid substrate-based metadata")]
    MissingConstantMetadata(String, String),
    /// Metadata for storage could not be found.
    #[error("Metadata for storage entry {0}_{1} could not be found. Make sure you are providing a valid substrate-based metadata")]
    MissingStorageMetadata(String, String),
    /// Metadata for call could not be found.
    #[error("Metadata for call entry {0}_{1} could not be found. Make sure you are providing a valid substrate-based metadata")]
    MissingCallMetadata(String, String),
    /// Metadata for call could not be found.
    #[error("Metadata for runtime API entry {0}_{1} could not be found. Make sure you are providing a valid substrate-based metadata")]
    MissingRuntimeApiMetadata(String, String),
    /// Call variant must have all named fields.
    #[error("Call variant for type {0} must have all named fields. Make sure you are providing a valid substrate-based metadata")]
    InvalidCallVariant(u32),
    /// Type should be an variant/enum.
    #[error("{0} type should be an variant/enum type. Make sure you are providing a valid substrate-based metadata")]
    InvalidType(String),
    /// Extrinsic call type could not be found.
    #[error("Extrinsic call type could not be found. Make sure you are providing a valid substrate-based metadata")]
    MissingCallType,
    /// There are too many or too few hashers.
    #[error("Could not generate functions for storage entry {storage_entry_name}. There are {key_count} keys, but only {hasher_count} hashers. The number of hashers must equal the number of keys or be exactly 1.")]
    InvalidStorageHasherCount {
        /// The name of the storage entry
        storage_entry_name: String,
        /// Number of keys
        key_count: usize,
        /// Number of hashers
        hasher_count: usize,
    },
    /// Cannot generate types.
    #[error("Type Generation failed: {0}")]
    TypeGeneration(#[from] TypegenError),
    /// Error when generating metadata from Wasm-runtime
    #[error("Failed to generate metadata from wasm file. reason: {0}")]
    Wasm(String),
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

impl CodegenError {
    /// Fetch the location for this error.
    // Todo: Probably worth storing location outside of the variant,
    // so that there's a common way to set a location for some error.
    fn get_location(&self) -> Span {
        match self {
            Self::InvalidModule(span) => *span,
            Self::TypeGeneration(TypegenError::InvalidSubstitute(err)) => err.span,
            Self::InvalidTypePath(_, err) => err.span(),
            _ => proc_macro2::Span::call_site(),
        }
    }
    /// Render the error as an invocation of syn::compile_error!.
    pub fn into_compile_error(self) -> TokenStream2 {
        let msg = self.to_string();
        let span = self.get_location();
        syn::Error::new(span, msg).into_compile_error()
    }
}
