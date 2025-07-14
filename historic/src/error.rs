/// Any error emitted by this crate can convert into this.
// Dev Note: All errors here are transparent, because in many places
// the inner errors are returned and so need to provide enough context
// as-is, so there shouldn't be anything to add here.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    OnlineClientError(#[from] OnlineClientError),
    #[error(transparent)]
    OfflineClientAtBlockError(#[from] OfflineClientAtBlockError),
    #[error(transparent)]
    OnlineClientAtBlockError(#[from] OnlineClientAtBlockError),
    #[error(transparent)]
    ExtrinsicsError(#[from] ExtrinsicsError),
    #[error(transparent)]
    ExtrinsicTransactionExtensionError(#[from] ExtrinsicTransactionExtensionError),
}

/// Errors consctructing an online client.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OnlineClientError {
    #[error("Cannot construct OnlineClient: The URL provided is invalid: {url}")]
    InvalidUrl {
        /// The URL that was invalid.
        url: String
    },
    #[error("Cannot construct OnlineClient owing to an RPC client error: {0}")]
    RpcClientError(#[from] subxt_rpcs::Error),
}

/// Errors constructing an offline client at a specific block number.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OfflineClientAtBlockError {
    #[error("Cannot construct OfflineClientAtBlock: spec version not found for block number {block_number}")]
    SpecVersionNotFound {
        /// The block number for which the spec version was not found.
        block_number: u64,
    },
    #[error("Cannot construct OfflineClientAtBlock: metadata not found for spec version {spec_version}")]
    MetadataNotFound {
        /// The spec version for which the metadata was not found.
        spec_version: u32,
    },
}

/// Errors constructing an online client at a specific block number.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OnlineClientAtBlockError {
    #[error("Cannot construct OnlineClientAtBlock: failed to get block hash from node for block {block_number}: {reason}")]
    CannotGetBlockHash { 
        /// Block number we failed to get the hash for.
        block_number: u64,
        /// The error we encountered.
        reason: subxt_rpcs::Error 
    },
    #[error("Cannot construct OnlineClientAtBlock: block number {block_number} not found")]
    BlockNotFound {
        /// The block number for which a block was not found.
        block_number: u64,
    },
    #[error("Cannot construct OnlineClientAtBlock: failed to get spec version for block hash {block_hash}: {reason}")]
    CannotGetSpecVersion {
        /// The block hash for which we failed to get the spec version.
        block_hash: String,
        /// The error we encountered.
        reason: String,
    },
    #[error("Cannot construct OnlineClientAtBlock: failed to get metadata for block hash {block_hash}: {reason}")]
    CannotGetMetadata {
        /// The block hash for which we failed to get the metadata.
        block_hash: String,
        /// The error we encountered.
        reason: String,
    },
}

/// Errors working with extrinsics.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ExtrinsicsError {
    #[error("Could not fetch extrinsics: {reason}")]
    FetchError {
        /// The error that occurred while fetching the extrinsics.
        reason: subxt_rpcs::Error,
    },
    #[error("Could not decode extrinsic at index {index}: {reason}")]
    DecodeError {
        /// The extrinsic index that failed to decode.
        index: usize,
        /// The error that occurred during decoding.
        reason: frame_decode::extrinsics::ExtrinsicDecodeError
    },
    #[error("Could not decode extrinsic at index {index}: there were undecoded bytes at the end, which implies that we did not decode it properly")]
    LeftoverBytes {
        /// The extrinsic index that had leftover bytes
        index: usize,
        /// The bytes that were left over after decoding the extrinsic.
        leftover_bytes: Vec<u8>,
    },
    #[error("Could not decode extrinsics: Unsupported metadata version ({version})")]
    UnsupportedMetadataVersion {
        /// The metadata version that is not supported.
        version: u32,
    },
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ExtrinsicTransactionExtensionError {
    #[error("Could not decode extrinsic transaction extension {name}: {reason}")]
    DecodeError {
        /// The name of the transaction extension that failed to decode.
        name: String,
        /// The error that occurred during decoding.
        reason: scale_decode::Error,
    },
    #[error("Could not decode extrinsic transaction extension {name}: there were undecoded bytes at the end, which implies that we did not decode it properly")]
    LeftoverBytes {
        /// The name of the transaction extension that had leftover bytes.
        name: String,
        /// The bytes that were left over after decoding the transaction extension.
        leftover_bytes: Vec<u8>,
    },
}
