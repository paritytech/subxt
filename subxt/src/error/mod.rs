// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the errors that can be returned.

mod dispatch_error;

crate::macros::cfg_unstable_light_client! {
    pub use subxt_lightclient::LightClientError;
}

// Re-export dispatch error types:
pub use dispatch_error::{
    ArithmeticError, DispatchError, ModuleError, TokenError, TransactionalError,
};
use subxt_metadata::StorageHasher;

// Re-expose the errors we use from other crates here:
pub use crate::config::ExtrinsicParamsError;
pub use crate::metadata::Metadata;
pub use scale_decode::Error as DecodeError;
pub use scale_encode::Error as EncodeError;
pub use subxt_metadata::TryFromError as MetadataTryFromError;

/// The underlying error enum, generic over the type held by the `Runtime`
/// variant. Prefer to use the [`Error<E>`] and [`Error`] aliases over
/// using this type directly.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Io error.
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    /// Codec error.
    #[error("Scale codec error: {0}")]
    Codec(#[from] codec::Error),
    /// Rpc error.
    #[error("Rpc error: {0}")]
    Rpc(#[from] RpcError),
    /// Serde serialization error
    #[error("Serde json error: {0}")]
    Serialization(#[from] serde_json::error::Error),
    /// Error working with metadata.
    #[error("Metadata error: {0}")]
    Metadata(#[from] MetadataError),
    /// Error decoding metadata.
    #[error("Metadata Decoding error: {0}")]
    MetadataDecoding(#[from] MetadataTryFromError),
    /// Runtime error.
    #[error("Runtime error: {0}")]
    Runtime(#[from] DispatchError),
    /// Error decoding to a [`crate::dynamic::Value`].
    #[error("Error decoding into dynamic value: {0}")]
    Decode(#[from] DecodeError),
    /// Error encoding from a [`crate::dynamic::Value`].
    #[error("Error encoding from dynamic value: {0}")]
    Encode(#[from] EncodeError),
    /// Transaction progress error.
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
    /// Error constructing the appropriate extrinsic params.
    #[error("Extrinsic params error: {0}")]
    ExtrinsicParams(#[from] ExtrinsicParamsError),
    /// Block related error.
    #[error("Block error: {0}")]
    Block(#[from] BlockError),
    /// An error encoding a storage address.
    #[error("Error encoding storage address: {0}")]
    StorageAddress(#[from] StorageAddressError),
    /// The bytes representing an error that we were unable to decode.
    #[error("An error occurred but it could not be decoded: {0:?}")]
    Unknown(Vec<u8>),
    /// Light client error.
    #[cfg(feature = "unstable-light-client")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-light-client")))]
    #[error("An error occurred but it could not be decoded: {0}")]
    LightClient(#[from] LightClientError),
    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}

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

impl From<scale_decode::visitor::DecodeError> for Error {
    fn from(value: scale_decode::visitor::DecodeError) -> Self {
        Error::Decode(value.into())
    }
}

impl Error {
    /// Checks whether the error was caused by a RPC re-connection.
    pub fn is_disconnected_will_reconnect(&self) -> bool {
        matches!(self, Error::Rpc(RpcError::DisconnectedWillReconnect(_)))
    }
}

/// An RPC error. Since we are generic over the RPC client that is used,
/// the error is boxed and could be casted.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RpcError {
    // Dev note: We need the error to be safely sent between threads
    // for `subscribe_to_block_headers_filling_in_gaps` and friends.
    /// Error related to the RPC client.
    #[error("RPC error: {0}")]
    ClientError(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// This error signals that the request was rejected for some reason.
    /// The specific reason is provided.
    #[error("RPC error: request rejected: {0}")]
    RequestRejected(String),
    /// The RPC subscription dropped.
    #[error("RPC error: subscription dropped.")]
    SubscriptionDropped,
    /// The requested URL is insecure.
    #[error("RPC error: insecure URL: {0}")]
    InsecureUrl(String),
    /// The connection was lost and automatically reconnected.
    #[error("RPC error: the connection was lost `{0}`; reconnect automatically initiated")]
    DisconnectedWillReconnect(String),
}

impl RpcError {
    /// Create a `RequestRejected` error from anything that can be turned into a string.
    pub fn request_rejected<S: Into<String>>(s: S) -> RpcError {
        RpcError::RequestRejected(s.into())
    }
}

/// Block error
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum BlockError {
    /// An error containing the hash of the block that was not found.
    #[error("Could not find a block with hash {0} (perhaps it was on a non-finalized fork?)")]
    NotFound(String),
    /// Extrinsic type ID cannot be resolved with the provided metadata.
    #[error("Extrinsic type ID cannot be resolved with the provided metadata. Make sure this is a valid metadata")]
    MissingType,
    /// Unsupported signature.
    #[error("Unsupported extrinsic version, only version 4 is supported currently")]
    /// The extrinsic has an unsupported version.
    UnsupportedVersion(u8),
    /// Decoding error.
    #[error("Cannot decode extrinsic: {0}")]
    DecodingError(codec::Error),
}

impl BlockError {
    /// Produce an error that a block with the given hash cannot be found.
    pub fn not_found(hash: impl AsRef<[u8]>) -> BlockError {
        let hash = format!("0x{}", hex::encode(hash));
        BlockError::NotFound(hash)
    }
}

/// Transaction error.
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum TransactionError {
    /// The block hash that the transaction was added to could not be found.
    /// This is probably because the block was retracted before being finalized.
    #[error("The block containing the transaction can no longer be found (perhaps it was on a non-finalized fork?)")]
    BlockNotFound,
    /// An error happened on the node that the transaction was submitted to.
    #[error("Error handling transaction: {0}")]
    Error(String),
    /// The transaction was deemed invalid.
    #[error("The transaction is not valid: {0}")]
    Invalid(String),
    /// The transaction was dropped.
    #[error("The transaction was dropped: {0}")]
    Dropped(String),
}

/// Something went wrong trying to encode a storage address.
#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StorageAddressError {
    /// Storage lookup does not have the expected number of keys.
    #[error("Storage lookup requires {expected} keys but more keys have been provided.")]
    TooManyKeys {
        /// The number of keys provided in the storage address.
        expected: usize,
    },
    /// This storage entry in the metadata does not have the correct number of hashers to fields.
    #[error("Storage entry in metadata does not have the correct number of hashers to fields")]
    WrongNumberOfHashers {
        /// The number of hashers in the metadata for this storage entry.
        hashers: usize,
        /// The number of fields in the metadata for this storage entry.
        fields: usize,
    },
    /// We weren't given enough bytes to decode the storage address/key.
    #[error("Not enough remaining bytes to decode the storage address/key")]
    NotEnoughBytes,
    /// We have leftover bytes after decoding the storage address.
    #[error("We have leftover bytes after decoding the storage address")]
    TooManyBytes,
    /// The bytes of a storage address are not the expected address for decoding the storage keys of the address.
    #[error("Storage address bytes are not the expected format. Addresses need to be at least 16 bytes (pallet ++ entry) and follow a structure given by the hashers defined in the metadata")]
    UnexpectedAddressBytes,
    /// An invalid hasher was used to reconstruct a value from a chunk of bytes that is part of a storage address. Hashers where the hash does not contain the original value are invalid for this purpose.
    #[error("An invalid hasher was used to reconstruct a value with type ID {ty_id} from a hash formed by a {hasher:?} hasher. This is only possible for concat-style hashers or the identity hasher")]
    HasherCannotReconstructKey {
        /// Type id of the key's type.
        ty_id: u32,
        /// The invalid hasher that caused this error.
        hasher: StorageHasher,
    },
}

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum MetadataError {
    /// The DispatchError type isn't available in the metadata
    #[error("The DispatchError type isn't available")]
    DispatchErrorNotFound,
    /// Type not found in metadata.
    #[error("Type with ID {0} not found")]
    TypeNotFound(u32),
    /// Pallet not found (index).
    #[error("Pallet with index {0} not found")]
    PalletIndexNotFound(u8),
    /// Pallet not found (name).
    #[error("Pallet with name {0} not found")]
    PalletNameNotFound(String),
    /// Variant not found.
    #[error("Variant with index {0} not found")]
    VariantIndexNotFound(u8),
    /// Constant not found.
    #[error("Constant with name {0} not found")]
    ConstantNameNotFound(String),
    /// Call not found.
    #[error("Call with name {0} not found")]
    CallNameNotFound(String),
    /// Runtime trait not found.
    #[error("Runtime trait with name {0} not found")]
    RuntimeTraitNotFound(String),
    /// Runtime method not found.
    #[error("Runtime method with name {0} not found")]
    RuntimeMethodNotFound(String),
    /// Call type not found in metadata.
    #[error("Call type not found in pallet with index {0}")]
    CallTypeNotFoundInPallet(u8),
    /// Event type not found in metadata.
    #[error("Event type not found in pallet with index {0}")]
    EventTypeNotFoundInPallet(u8),
    /// Storage details not found in metadata.
    #[error("Storage details not found in pallet with name {0}")]
    StorageNotFoundInPallet(String),
    /// Storage entry not found.
    #[error("Storage entry {0} not found")]
    StorageEntryNotFound(String),
    /// The generated interface used is not compatible with the node.
    #[error("The generated code is not compatible with the node")]
    IncompatibleCodegen,
    /// Custom value not found.
    #[error("Custom value with name {0} not found")]
    CustomValueNameNotFound(String),
}
