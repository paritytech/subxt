use crate::error::OnlineClientAtBlockError;
use crate::config::Config;
use crate::client::OfflineClientAtBlockT;
use super::ClientAtBlock;
use codec::{ Compact, Encode, Decode };
use frame_metadata::{ RuntimeMetadata, RuntimeMetadataPrefixed };
use subxt_rpcs::methods::chain_head::ArchiveCallResult;
use subxt_rpcs::ChainHeadRpcMethods;
use scale_info_legacy::TypeRegistrySet;

/// A client which exposes the means to decode historic data on a chain online.
pub struct OnlineClient<T: Config> {
    /// The configuration for this client.
    config: T,
    /// The RPC methods used to communicate with the node.
    rpc_methods: ChainHeadRpcMethods<T>,
}

impl <T: Config> OnlineClient<T> {
    pub async fn at(&'_ self, block_number: u64) -> Result<ClientAtBlock<OnlineClientAtBlock<'_, T>, T>, OnlineClientAtBlockError> {
        let config = &self.config;
        let rpc_methods = &self.rpc_methods;
        let block_hash = rpc_methods
            .archive_v1_hash_by_height(block_number as usize)
            .await
            .map_err(|e| OnlineClientAtBlockError::CannotGetBlockHash { block_number, reason: e })?
            .pop()
            .ok_or_else(|| OnlineClientAtBlockError::BlockNotFound { block_number })?
            .into();
        let spec_version = get_spec_version(&rpc_methods, block_hash).await?;
        let metadata = get_metadata(&rpc_methods, block_hash).await?;

        let historic_types = self.config.legacy_types_for_spec_version(spec_version);

        Ok(ClientAtBlock::new(OnlineClientAtBlock {
            config,
            historic_types,
            metadata,
            rpc_methods,
            block_hash,
        }))
    }
}

/// This represents an online client at a specific block.
#[doc(hidden)]
pub trait OnlineClientAtBlockT<'client, T: Config + 'client>: OfflineClientAtBlockT<'client, T> {
    /// Return the RPC methods we'll use to interact with the node.
    fn rpc_methods(&self) -> &ChainHeadRpcMethods<T>;
    /// Return the block hash for the current block.
    fn block_hash(&self) -> <T as Config>::Hash;
}


// Dev note: this shouldn't need to be exposed unless there is some
// need to explicitly name the ClientAAtBlock type. Rather keep it
// private to allow changes if possible.
#[doc(hidden)]
pub struct OnlineClientAtBlock<'client, T: Config + 'client> {
    /// The configuration for thie chain.
    config: &'client T,
    /// Historic types to use at this block number.
    historic_types: TypeRegistrySet<'client>,
    /// Metadata to use at this block number.
    metadata: RuntimeMetadata,
    /// We also need RPC methods for online interactions.
    rpc_methods: &'client ChainHeadRpcMethods<T>,
    /// The block hash at which this client is operating.
    block_hash: <T as Config>::Hash,
}

impl <'client, T: Config + 'client> OnlineClientAtBlockT<'client, T> for OnlineClientAtBlock<'client, T> {
    fn rpc_methods(&self) -> &ChainHeadRpcMethods<T> {
        self.rpc_methods
    }
    fn block_hash(&self) -> <T as Config>::Hash {
        self.block_hash
    }
}

impl <'client, T: Config + 'client> OfflineClientAtBlockT<'client, T> for OnlineClientAtBlock<'client, T> {
    fn config(&self) -> &'client T {
        self.config
    }
    fn legacy_types(&'_ self) -> &TypeRegistrySet<'client> {
        &self.historic_types
    }
    fn metadata(&self) -> &RuntimeMetadata {
        &self.metadata
    }
}

async fn get_spec_version<T: Config>(rpc_methods: &ChainHeadRpcMethods<T>, block_hash: <T as Config>::Hash) -> Result<u32, OnlineClientAtBlockError> {
    use codec::Decode;
    use subxt_rpcs::methods::chain_head::ArchiveCallResult;

    // make a runtime call to get the version information. This is also a constant
    // in the metadata and so we could fetch it from there to avoid the call, but it would be a 
    // bit more effort.
    let spec_version_bytes = {
        let call_res = rpc_methods.archive_v1_call(block_hash.into(), "Core_version", &[])
            .await
            .map_err(|e| OnlineClientAtBlockError::CannotGetSpecVersion {
                block_hash: block_hash.to_string(), 
                reason: format!("Error calling Core_version: {e}")
            })?;
        match call_res {
            ArchiveCallResult::Success(bytes) => bytes.0,
            ArchiveCallResult::Error(e) => return Err(OnlineClientAtBlockError::CannotGetSpecVersion {
                block_hash: block_hash.to_string(), 
                reason: format!("Core_version returned an error: {e}")
            }),
        }
    };

    // We only care about the spec version, so just decode enough of this version information
    // to be able to pluck out what we want, and ignore the rest.
    let spec_version = {
        #[derive(codec::Decode)]
        struct SpecVersionHeader {
            _spec_name: String,
            _impl_name: String,
            _authoring_version: u32,
            spec_version: u32
        }
        SpecVersionHeader::decode(&mut &spec_version_bytes[..])
            .map_err(|e| OnlineClientAtBlockError::CannotGetSpecVersion { 
                block_hash: block_hash.to_string(),
                reason: format!("Error decoding Core_version response: {e}")
            })?.spec_version
    };

    Ok(spec_version)
}

async fn get_metadata<T: Config>(rpc_methods: &ChainHeadRpcMethods<T>, block_hash: <T as Config>::Hash) -> Result<RuntimeMetadata, OnlineClientAtBlockError> {
    // First, try to use the "modern" metadata APIs to get the most recent version we can.
    let version_to_get = rpc_methods
        .archive_v1_call(block_hash.into(), "Metadata_metadata_versions", &[])
        .await
        .ok()
        .and_then(|res| res.as_success())
        .and_then(|res| <Vec<u32>>::decode(&mut &res[..]).ok())
        .and_then(|versions| {
            // We want to filter out the "unstable" version, which is represented by u32::MAX.
            versions.into_iter().filter(|v| *v != u32::MAX).max()
        });

    // We had success calling the above API, so we expect the "modern" metadata API to work.
    if let Some(version_to_get) = version_to_get {
        let version_bytes = version_to_get.encode();
        let rpc_response = rpc_methods
            .archive_v1_call(block_hash.into(), "Metadata_metadata_at_version", &version_bytes)
            .await
            .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
                block_hash: block_hash.to_string(),
                reason: format!("Error calling Metadata_metadata_at_version: {e}"),
            })
            .and_then(|res| {
                match res {
                    ArchiveCallResult::Success(bytes) => Ok(bytes.0),
                    ArchiveCallResult::Error(e) => Err(OnlineClientAtBlockError::CannotGetMetadata {
                        block_hash: block_hash.to_string(),
                        reason: format!("Calling Metadata_metadata_at_version returned an error: {e}"),
                    }),
                }
            })?;

        // Option because we may have asked for a version that doesn't exist. Compact because we get back a Vec<u8>
        // of the metadata bytes, and the Vec is preceeded by it's compact encoded length. The actual bytes are then
        // decoded as a `RuntimeMetadataPrefixed`, after this.
        let (_, metadata) = <Option<(Compact<u32>, RuntimeMetadataPrefixed)>>::decode(&mut &rpc_response[..])
            .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
                block_hash: block_hash.to_string(),
                reason: format!("Error decoding response for Metadata_metadata_at_version: {e}"),
            })?
            .ok_or_else(|| OnlineClientAtBlockError::CannotGetMetadata {
                block_hash: block_hash.to_string(),
                reason: format!("No metadata returned for the latest version from Metadata_metadata_versions ({version_to_get})"),
            })?;

        return Ok(metadata.1)
    }

    // We didn't get a version from Metadata_metadata_versions, so fall back to the "old" API.
    let metadata_bytes = rpc_methods
        .archive_v1_call(block_hash.into(), "Metadata_metadata", &[])
        .await
        .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
            block_hash: block_hash.to_string(),
            reason: format!("Error calling Metadata_metadata: {e}"),
        })
        .and_then(|res| {
            match res {
                ArchiveCallResult::Success(bytes) => Ok(bytes.0),
                ArchiveCallResult::Error(e) => Err(OnlineClientAtBlockError::CannotGetMetadata {
                    block_hash: block_hash.to_string(),
                    reason: format!("Calling Metadata_metadata returned an error: {e}"),
                }),
            }
        })?;

    let (_, metadata) = <(Compact<u32>, RuntimeMetadataPrefixed)>::decode(&mut &metadata_bytes[..])
        .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
            block_hash: block_hash.to_string(),
            reason: format!("Error decoding response for Metadata_metadata: {e}"),
        })?;

    Ok(metadata.1)
}
