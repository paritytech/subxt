use frame_decode::extrinsics::ExtrinsicDecodeError;
use frame_decode::storage::{ StorageKeyDecodeError, StorageValueDecodeError };

/// Any error emitted by this crate can convert into this.
// Dev Note: All errors here are transparent, because in many places
// the inner errors are returned and so need to provide enough context
// as-is, so there shouldn't be anything to add here.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    OfflineClientAtBlockError(#[from] OfflineClientAtBlockError),
    #[error(transparent)]
    OnlineClientAtBlockError(#[from] OnlineClientAtBlockError),
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
    #[error("Failed to get block hash from node for block {}: {}", .block_number, .reason)]
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
    #[error("Failed to get spec version for block hash {block_hash}: {reason}")]
    CannotGetSpecVersion {
        /// The block hash for which we failed to get the spec version.
        block_hash: String,
        /// The error we encountered.
        reason: String,
    },
    #[error("Failed to get metadata for block hash {block_hash}: {reason}")]
    CannotGetMetadata {
        /// The block hash for which we failed to get the metadata.
        block_hash: String,
        /// The error we encountered.
        reason: String,
    },
}

// #[derive(Debug, thiserror::Error)]
// #[non_exhaustive]
// pub enum Error {
//     /// Error from the RPC client.
//     #[error("RPC client error: {0}")]
//     Rpc(#[from] subxt_rpcs::Error),
//     /// Error SCALE decoding something.
//     #[error("Error SCALE decoding something: {0}")]
//     Codec(#[from] codec::Error),
//     /// Error decoding to some type.
//     #[error("Error decoding into dynamic value: {0}")]
//     ExtrinsicDecodeError(#[from] ExtrinsicDecodeError),
//     /// Error encoding to some type.
//     #[error("Error encoding from dynamic value: {0}")]
//     StorageKeyDecodeError(#[from] StorageKeyDecodeError<String>),
//     /// Error decoding a storage value.
//     #[error("Error decoding storage value: {0}")]
//     StorageValueDecodeError(#[from] StorageValueDecodeError<String>),
//     /// Spec version not found for the given block number.
//     #[error("Spec version not found for block number {block_number}")]
//     SpecVersionNotFound {
//         /// The block number for which the spec version was not found.
//         block_number: u64,
//     },
//     /// Metadata not found for the given spec version.
//     #[error("Metadata not found for spec version {spec_version}")]
//     MetadataNotFound {
//         /// The spec version for which the metadata was not found.
//         spec_version: u64,
//     },
//     /// Some other error.
//     #[error("Other error: {}", .0)]
//     Other(String)
// }

// NOTES:
// - Can we have errors with context, because eg Error::Codec could come from anywhere etc.
//   Could be more like a core error and then a string context
// - ClientAtBlock needs to know block hash for OnlineClient, so it can fetch things for the block.
//   for OfflineClient it does not need this as it just decodes things. How can we unify this? 
//     - Maybe we need an OfflineClientAtBlock and OnlineClientAtBlock trait, one of which has RPCs 
//       and block hash, and the other just the config?
//     - Maybe don't need OnlineClient and OfflineClient traits then, just hardcoded structs that
//       have a way to hand back a *ClientAtBlock impl.
//     - We should have only exactly what we need as a trait and everything else plain structs.