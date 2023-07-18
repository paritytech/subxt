// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Errors that can be emitted from codegen.

use proc_macro2::{Span, TokenStream as TokenStream2};

/// Error returned when the Codegen cannot generate the runtime API.
#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    /// The given metadata type could not be found.
    #[error("Could not find type with ID {0} in the type registry; please raise a support issue.")]
    TypeNotFound(u32),
    /// Cannot fetch the metadata bytes.
    #[error("Failed to fetch metadata, make sure that you're pointing at a node which is providing substrate-based metadata: {0}")]
    Fetch(#[from] FetchMetadataError),
    /// Failed IO for the metadata file.
    #[error("Failed IO for {0}, make sure that you are providing the correct file path for metadata: {1}")]
    Io(String, std::io::Error),
    /// Cannot decode the metadata bytes.
    #[error("Could not decode metadata, only V14 and V15 metadata are supported: {0}")]
    Decode(#[from] codec::Error),
    /// Out of line modules are not supported.
    #[error("Out-of-line subxt modules are not supported, make sure you are providing a body to your module: pub mod polkadot {{ ... }}")]
    InvalidModule(Span),
    /// Expected named or unnamed fields.
    #[error("Fields should either be all named or all unnamed, make sure you are providing a valid metadata: {0}")]
    InvalidFields(String),
    /// Substitute types must have a valid path.
    #[error("Type substitution error: {0}")]
    TypeSubstitutionError(#[from] TypeSubstitutionError),
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
    #[error(
        "{0} type should be an variant/enum type. Make sure you are providing a valid substrate-based metadata"
    )]
    InvalidType(String),
    /// Extrinsic call type could not be found.
    #[error(
        "Extrinsic call type could not be found. Make sure you are providing a valid substrate-based metadata"
    )]
    MissingCallType,
}

impl CodegenError {
    /// Fetch the location for this error.
    // Todo: Probably worth storing location outside of the variant,
    // so that there's a common way to set a location for some error.
    fn get_location(&self) -> Span {
        match self {
            Self::InvalidModule(span) => *span,
            Self::TypeSubstitutionError(err) => err.get_location(),
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

/// Error attempting to load metadata.
#[derive(Debug, thiserror::Error)]
pub enum FetchMetadataError {
    #[error("Cannot decode hex value: {0}")]
    DecodeError(#[from] hex::FromHexError),
    #[error("Cannot scale encode/decode value: {0}")]
    CodecError(#[from] codec::Error),
    #[error("Request error: {0}")]
    RequestError(#[from] jsonrpsee::core::Error),
    #[error("'{0}' not supported, supported URI schemes are http, https, ws or wss.")]
    InvalidScheme(String),
    #[error("Other error: {0}")]
    Other(String),
}

/// Error attempting to do type substitution.
#[derive(Debug, thiserror::Error)]
pub enum TypeSubstitutionError {
    /// Substitute "to" type must be an absolute path.
    #[error("`substitute_type(with = <path>)` must be a path prefixed with 'crate::' or '::'")]
    ExpectedAbsolutePath(Span),
    /// Substitute types must have a valid path.
    #[error("Substitute types must have a valid path.")]
    EmptySubstitutePath(Span),
    /// From/To substitution types should use angle bracket generics.
    #[error("Expected the from/to type generics to have the form 'Foo<A,B,C..>'.")]
    ExpectedAngleBracketGenerics(Span),
    /// Source substitute type must be an ident.
    #[error("Expected an ident like 'Foo' or 'A' to mark a type to be substituted.")]
    InvalidFromType(Span),
    /// Target type is invalid.
    #[error("Expected an ident like 'Foo' or an absolute concrete path like '::path::to::Bar' for the substitute type.")]
    InvalidToType(Span),
    /// Target ident doesn't correspond to any source type.
    #[error("Cannot find matching param on 'from' type.")]
    NoMatchingFromType(Span),
}

impl TypeSubstitutionError {
    /// Fetch the location for this error.
    // Todo: Probably worth storing location outside of the variant,
    // so that there's a common way to set a location for some error.
    fn get_location(&self) -> Span {
        match self {
            TypeSubstitutionError::ExpectedAbsolutePath(span) => *span,
            TypeSubstitutionError::EmptySubstitutePath(span) => *span,
            TypeSubstitutionError::ExpectedAngleBracketGenerics(span) => *span,
            TypeSubstitutionError::InvalidFromType(span) => *span,
            TypeSubstitutionError::InvalidToType(span) => *span,
            TypeSubstitutionError::NoMatchingFromType(span) => *span,
        }
    }
}
