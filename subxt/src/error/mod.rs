// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the errors that can be returned.

mod dispatch_error;

use core::fmt::Debug;

crate::macros::cfg_unstable_light_client! {
    pub use crate::client::LightClientError;
}

// Re-export dispatch error types:
pub use dispatch_error::{
    ArithmeticError, DispatchError, ModuleError, TokenError, TransactionalError,
};

// Re-expose the errors we use from other crates here:
pub use crate::config::ExtrinsicParamsError;
pub use crate::metadata::Metadata;
use crate::prelude::*;
use derive_more::{Display, From};
pub use scale_decode::Error as DecodeError;
pub use scale_encode::Error as EncodeError;
pub use subxt_metadata::TryFromError as MetadataTryFromError;

/// The underlying error enum, generic over the type held by the `Runtime`
/// variant. Prefer to use the [`Error<E>`] and [`Error`] aliases over
/// using this type directly.
#[derive(Debug, Display, From)]
#[non_exhaustive]
pub enum Error {
    /// Io error.
    #[display(fmt = "Io error: {_0}")]
    Io(std::io::Error),
    /// Codec error.
    #[display(fmt = "Scale codec error: {_0}")]
    Codec(codec::Error),
    /// Rpc error.
    #[cfg(feature = "std")]
    #[display(fmt = "Rpc error: {_0}")]
    Rpc(RpcError),
    /// Serde serialization error
    #[display(fmt = "Serde json error: {_0}")]
    Serialization(serde_json::error::Error),
    /// Error working with metadata.
    #[display(fmt = "Metadata error: {_0}")]
    Metadata(MetadataError),
    /// Error decoding metadata.
    #[display(fmt = "Metadata Decoding error: {_0}")]
    MetadataDecoding(MetadataTryFromError),
    /// Runtime error.
    #[display(fmt = "Runtime error: {_0}")]
    Runtime(DispatchError),
    /// Error decoding to a [`crate::dynamic::Value`].
    #[display(fmt = "Error decoding into dynamic value: {_0}")]
    Decode(DecodeError),
    /// Error encoding from a [`crate::dynamic::Value`].
    #[display(fmt = "Error encoding from dynamic value: {_0}")]
    Encode(EncodeError),
    /// Transaction progress error.
    #[display(fmt = "Transaction error: {_0}")]
    Transaction(TransactionError),
    /// Error constructing the appropriate extrinsic params.
    #[display(fmt = "Extrinsic params error: {_0}")]
    ExtrinsicParams(ExtrinsicParamsError),
    /// Block related error.
    #[display(fmt = "Block error: {_0}")]
    Block(BlockError),
    /// An error encoding a storage address.
    #[display(fmt = "Error encoding storage address: {_0}")]
    StorageAddress(StorageAddressError),
    /// The bytes representing an error that we were unable to decode.
    #[display(fmt = "An error occurred but it could not be decoded: {_0:?}")]
    #[from(ignore)]
    Unknown(Vec<u8>),
    /// Light client error.
    #[cfg(feature = "unstable-light-client")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-light-client")))]
    #[display(fmt = "An error occurred but it could not be decoded: {_0}")]
    LightClient(LightClientError),
    /// Other error.
    #[display(fmt = "Other error: {_0}")]
    #[from(ignore)]
    Other(String),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<'a> From<&'a str> for Error {
    fn from(error: &'a str) -> Self {
        Error::Other(error.into())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Other(error)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}

/// An RPC error. Since we are generic over the RPC client that is used,
/// the error is boxed and could be casted.

#[cfg(feature = "std")]
#[derive(Debug, Display)]
#[non_exhaustive]
pub enum RpcError {
    // Dev note: We need the error to be safely sent between threads
    // for `subscribe_to_block_headers_filling_in_gaps` and friends.
    /// Error related to the RPC client.
    #[display(fmt = "RPC error: {_0}")]
    ClientError(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// This error signals that the request was rejected for some reason.
    /// The specific reason is provided.
    #[display(fmt = "RPC error: request rejected: {_0}")]
    RequestRejected(String),
    /// The RPC subscription dropped.
    #[display(fmt = "RPC error: subscription dropped.")]
    SubscriptionDropped,
    /// The requested URL is insecure.
    #[display(fmt = "RPC error: insecure URL: {_0}")]
    InsecureUrl(String),
}

#[cfg(feature = "std")]
impl std::error::Error for RpcError {}

#[cfg(feature = "std")]
impl RpcError {
    /// Create a `RequestRejected` error from anything that can be turned into a string.
    pub fn request_rejected<S: Into<String>>(s: S) -> RpcError {
        RpcError::RequestRejected(s.into())
    }
}

/// Block error
#[derive(Clone, Debug, Eq, Display, PartialEq)]
#[non_exhaustive]
pub enum BlockError {
    /// An error containing the hash of the block that was not found.
    #[display(
        fmt = "Could not find a block with hash {_0} (perhaps it was on a non-finalized fork?)"
    )]
    NotFound(String),
    /// Extrinsic type ID cannot be resolved with the provided metadata.
    #[display(
        fmt = "Extrinsic type ID cannot be resolved with the provided metadata. Make sure this is a valid metadata"
    )]
    MissingType,
    /// Unsupported signature.
    #[display(fmt = "Unsupported extrinsic version, only version 4 is supported currently")]
    /// The extrinsic has an unsupported version.
    UnsupportedVersion(u8),
    /// Decoding error.
    #[display(fmt = "Cannot decode extrinsic: {_0}")]
    DecodingError(codec::Error),
}

impl BlockError {
    /// Produce an error that a block with the given hash cannot be found.
    pub fn not_found(hash: impl AsRef<[u8]>) -> BlockError {
        let hash = format!("0x{}", hex::encode(hash));
        BlockError::NotFound(hash)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for BlockError {}

/// Transaction error.
#[derive(Clone, Debug, Eq, Display, PartialEq)]
#[non_exhaustive]
pub enum TransactionError {
    /// The block hash that the transaction was added to could not be found.
    /// This is probably because the block was retracted before being finalized.
    #[display(
        fmt = "The block containing the transaction can no longer be found (perhaps it was on a non-finalized fork?)"
    )]
    BlockNotFound,
    /// An error happened on the node that the transaction was submitted to.
    #[display(fmt = "Error handling transaction: {_0}")]
    Error(String),
    /// The transaction was deemed invalid.
    #[display(fmt = "The transaction is not valid: {_0}")]
    Invalid(String),
    /// The transaction was dropped.
    #[display(fmt = "The transaction was dropped: {_0}")]
    Dropped(String),
}

#[cfg(feature = "std")]
impl std::error::Error for TransactionError {}

/// Something went wrong trying to encode a storage address.
#[derive(Clone, Debug, Display)]
#[non_exhaustive]
pub enum StorageAddressError {
    /// Storage map type must be a composite type.
    #[display(fmt = "Storage map type must be a composite type")]
    MapTypeMustBeTuple,
    /// Storage lookup does not have the expected number of keys.
    #[display(fmt = "Storage lookup requires {expected} keys but got {actual} keys")]
    WrongNumberOfKeys {
        /// The actual number of keys needed, based on the metadata.
        actual: usize,
        /// The number of keys provided in the storage address.
        expected: usize,
    },
    /// This storage entry in the metadata does not have the correct number of hashers to fields.
    #[display(
        fmt = "Storage entry in metadata does not have the correct number of hashers to fields"
    )]
    WrongNumberOfHashers {
        /// The number of hashers in the metadata for this storage entry.
        hashers: usize,
        /// The number of fields in the metadata for this storage entry.
        fields: usize,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for StorageAddressError {}

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq, Display)]
#[non_exhaustive]
pub enum MetadataError {
    /// The DispatchError type isn't available in the metadata
    #[display(fmt = "The DispatchError type isn't available")]
    DispatchErrorNotFound,
    /// Type not found in metadata.
    #[display(fmt = "Type with ID {_0} not found")]
    TypeNotFound(u32),
    /// Pallet not found (index).
    #[display(fmt = "Pallet with index {_0} not found")]
    PalletIndexNotFound(u8),
    /// Pallet not found (name).
    #[display(fmt = "Pallet with name {_0} not found")]
    PalletNameNotFound(String),
    /// Variant not found.
    #[display(fmt = "Variant with index {_0} not found")]
    VariantIndexNotFound(u8),
    /// Constant not found.
    #[display(fmt = "Constant with name {_0} not found")]
    ConstantNameNotFound(String),
    /// Call not found.
    #[display(fmt = "Call with name {_0} not found")]
    CallNameNotFound(String),
    /// Runtime trait not found.
    #[display(fmt = "Runtime trait with name {_0} not found")]
    RuntimeTraitNotFound(String),
    /// Runtime method not found.
    #[display(fmt = "Runtime method with name {_0} not found")]
    RuntimeMethodNotFound(String),
    /// Call type not found in metadata.
    #[display(fmt = "Call type not found in pallet with index {_0}")]
    CallTypeNotFoundInPallet(u8),
    /// Event type not found in metadata.
    #[display(fmt = "Event type not found in pallet with index {_0}")]
    EventTypeNotFoundInPallet(u8),
    /// Storage details not found in metadata.
    #[display(fmt = "Storage details not found in pallet with name {_0}")]
    StorageNotFoundInPallet(String),
    /// Storage entry not found.
    #[display(fmt = "Storage entry {_0} not found")]
    StorageEntryNotFound(String),
    /// The generated interface used is not compatible with the node.
    #[display(fmt = "The generated code is not compatible with the node")]
    IncompatibleCodegen,
    /// Custom value not found.
    #[display(fmt = "Custom value with name {_0} not found")]
    CustomValueNameNotFound(String),
}

#[cfg(feature = "std")]
impl std::error::Error for MetadataError {}
