mod block_number_or_ref;
mod blocks;

use super::ClientAtBlock;
use super::OfflineClientAtBlockT;
use crate::backend::{Backend, BlockRef, CombinedBackend};
use crate::config::{Config, HashFor, Hasher, Header};
use crate::error::{BlocksError, OnlineClientAtBlockError};
use blocks::Blocks;
use codec::{Compact, Decode, Encode};
use core::marker::PhantomData;
use frame_decode::helpers::ToTypeRegistry;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use scale_info_legacy::TypeRegistrySet;
use std::sync::Arc;
use subxt_metadata::Metadata;
use subxt_rpcs::RpcClient;

#[cfg(feature = "jsonrpsee")]
#[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee")))]
use crate::error::OnlineClientError;

pub use block_number_or_ref::BlockNumberOrRef;

/// A client which exposes the means to decode historic data on a chain online.
#[derive(Clone, Debug)]
pub struct OnlineClient<T: Config> {
    inner: Arc<OnlineClientInner<T>>,
}

struct OnlineClientInner<T: Config> {
    /// The configuration for this client.
    config: T,
    /// Chain genesis hash. Needed to construct transactions,
    /// so we obtain it up front on constructing this.
    genesis_hash: HashFor<T>,
    /// The RPC methods used to communicate with the node.
    backend: Arc<dyn Backend<T>>,
}

impl<T: Config> std::fmt::Debug for OnlineClientInner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnlineClientInner")
            .field("config", &"<config>")
            .field("backend", &"Arc<backend impl>")
            .finish()
    }
}

impl<T: Config> OnlineClient<T> {
    /// Construct a new [`OnlineClient`] using default settings which
    /// point to a locally running node on `ws://127.0.0.1:9944`.
    ///
    /// **Note:** This will only work if the local node is an archive node.
    #[cfg(all(feature = "jsonrpsee", feature = "runtime"))]
    pub async fn new(config: T) -> Result<OnlineClient<T>, OnlineClientError> {
        let url = "ws://127.0.0.1:9944";
        OnlineClient::from_url(config, url).await
    }

    /// Construct a new [`OnlineClient`], providing a URL to connect to.
    #[cfg(all(feature = "jsonrpsee", feature = "runtime"))]
    pub async fn from_url(
        config: T,
        url: impl AsRef<str>,
    ) -> Result<OnlineClient<T>, OnlineClientError> {
        let url_str = url.as_ref();
        let url = url::Url::parse(url_str).map_err(|_| OnlineClientError::InvalidUrl {
            url: url_str.to_string(),
        })?;
        if !Self::is_url_secure(&url) {
            return Err(OnlineClientError::RpcError(subxt_rpcs::Error::InsecureUrl(
                url_str.to_string(),
            )));
        }
        OnlineClient::from_insecure_url(config, url).await
    }

    /// Construct a new [`OnlineClient`], providing a URL to connect to.
    ///
    /// Allows insecure URLs without SSL encryption, e.g. (http:// and ws:// URLs).
    #[cfg(all(feature = "jsonrpsee", feature = "runtime"))]
    pub async fn from_insecure_url(
        config: T,
        url: impl AsRef<str>,
    ) -> Result<OnlineClient<T>, OnlineClientError> {
        let rpc_client = RpcClient::from_insecure_url(url).await?;
        OnlineClient::from_rpc_client(config, rpc_client).await
    }

    fn is_url_secure(url: &url::Url) -> bool {
        let secure_scheme = url.scheme() == "https" || url.scheme() == "wss";
        let is_localhost = url.host().is_some_and(|e| match e {
            url::Host::Domain(e) => e == "localhost",
            url::Host::Ipv4(e) => e.is_loopback(),
            url::Host::Ipv6(e) => e.is_loopback(),
        });
        secure_scheme || is_localhost
    }

    /// Construct a new [`OnlineClient`] by providing an [`RpcClient`] to drive the connection.
    /// This will use the current default [`Backend`], which may change in future releases.
    #[cfg(all(feature = "jsonrpsee", feature = "runtime"))]
    pub async fn from_rpc_client(
        config: T,
        rpc_client: impl Into<RpcClient>,
    ) -> Result<OnlineClient<T>, OnlineClientError> {
        let rpc_client = rpc_client.into();
        let backend = CombinedBackend::builder()
            .build_with_background_driver(rpc_client)
            .await
            .map_err(OnlineClientError::CannotBuildCombinedBackend)?;
        let backend: Arc<dyn Backend<T>> = Arc::new(backend);
        OnlineClient::from_backend(config, backend).await
    }

    /// Construct a new [`OnlineClient`] by providing an underlying [`Backend`]
    /// implementation to power it.
    pub async fn from_backend(
        config: T,
        backend: impl Into<Arc<dyn Backend<T>>>,
    ) -> Result<OnlineClient<T>, OnlineClientError> {
        let backend = backend.into();
        let genesis_hash = match config.genesis_hash() {
            Some(hash) => hash,
            None => backend
                .genesis_hash()
                .await
                .map_err(OnlineClientError::CannotGetGenesisHash)?,
        };

        Ok(OnlineClient {
            inner: Arc::new(OnlineClientInner {
                config,
                genesis_hash,
                backend: backend.into(),
            }),
        })
    }

    /// Obtain a stream of all blocks imported by the node.
    ///
    /// **Note:** You probably want to use [`Self::stream_blocks()`] most of
    /// the time. Blocks returned here may be pruned at any time and become inaccessible,
    /// leading to errors when trying to work with them.
    pub async fn stream_all_blocks(&self) -> Result<Blocks<T>, BlocksError> {
        // We need a hasher to know how to hash things. Thus, we need metadata to instantiate
        // the hasher, so let's use the current block.
        let current_block = self
            .at_current_block()
            .await
            .map_err(BlocksError::CannotGetCurrentBlock)?;
        let hasher = current_block.client.hasher.clone();

        let stream = self
            .inner
            .backend
            .stream_all_block_headers(hasher)
            .await
            .map_err(BlocksError::CannotGetBlockHeaderStream)?;

        Ok(Blocks::from_headers_stream(self.clone(), stream))
    }

    /// Obtain a stream of blocks imported by the node onto the current best fork.
    ///
    /// **Note:** You probably want to use [`Self::stream_blocks()`] most of
    /// the time. Blocks returned here may be pruned at any time and become inaccessible,
    /// leading to errors when trying to work with them.
    pub async fn stream_best_blocks(&self) -> Result<Blocks<T>, BlocksError> {
        // We need a hasher to know how to hash things. Thus, we need metadata to instantiate
        // the hasher, so let's use the current block.
        let current_block = self
            .at_current_block()
            .await
            .map_err(BlocksError::CannotGetCurrentBlock)?;
        let hasher = current_block.client.hasher.clone();

        let stream = self
            .inner
            .backend
            .stream_best_block_headers(hasher)
            .await
            .map_err(BlocksError::CannotGetBlockHeaderStream)?;

        Ok(Blocks::from_headers_stream(self.clone(), stream))
    }

    /// Obtain a stream of finalized blocks.
    pub async fn stream_blocks(&self) -> Result<Blocks<T>, BlocksError> {
        // We need a hasher to know how to hash things. Thus, we need metadata to instantiate
        // the hasher, so let's use the current block.
        let current_block = self
            .at_current_block()
            .await
            .map_err(BlocksError::CannotGetCurrentBlock)?;
        let hasher = current_block.client.hasher.clone();

        let stream = self
            .inner
            .backend
            .stream_finalized_block_headers(hasher)
            .await
            .map_err(BlocksError::CannotGetBlockHeaderStream)?;

        Ok(Blocks::from_headers_stream(self.clone(), stream))
    }

    /// Instantiate a client to work at the current finalized block _at the time of instantiation_.
    /// This does not track new blocks.
    pub async fn at_current_block(
        &self,
    ) -> Result<ClientAtBlock<OnlineClientAtBlock<T>, T>, OnlineClientAtBlockError> {
        let latest_block = self
            .inner
            .backend
            .latest_finalized_block_ref()
            .await
            .map_err(|e| OnlineClientAtBlockError::CannotGetCurrentBlock { reason: e })?;

        self.at_block(latest_block).await
    }

    /// Instantiate a client for working at a specific block.
    pub async fn at_block(
        &self,
        number_or_hash: impl Into<BlockNumberOrRef<T>>,
    ) -> Result<ClientAtBlock<OnlineClientAtBlock<T>, T>, OnlineClientAtBlockError> {
        let number_or_hash = number_or_hash.into();

        // We are given either a block hash or number. We need both.
        let (block_ref, block_number) = match number_or_hash {
            BlockNumberOrRef::BlockRef(block_ref) => {
                let block_hash = block_ref.hash();
                let block_header = self
                    .inner
                    .backend
                    .block_header(block_hash)
                    .await
                    .map_err(|e| OnlineClientAtBlockError::CannotGetBlockHeader {
                        block_hash: block_hash.into(),
                        reason: e,
                    })?
                    .ok_or(OnlineClientAtBlockError::BlockHeaderNotFound {
                        block_hash: block_hash.into(),
                    })?;
                (block_ref, block_header.number())
            }
            BlockNumberOrRef::Number(block_number) => {
                let block_ref = self
                    .inner
                    .backend
                    .block_number_to_hash(block_number)
                    .await
                    .map_err(|e| OnlineClientAtBlockError::CannotGetBlockHash {
                        block_number,
                        reason: e,
                    })?
                    .ok_or(OnlineClientAtBlockError::BlockNotFound { block_number })?;
                (block_ref, block_number)
            }
        };

        self.at_block_hash_and_number(block_ref, block_number).await
    }

    /// Instantiate a client for working at a specific block. This takes a block hash/ref _and_ the
    /// corresponding block number. When both are available, this saves an RPC call to obtain one from
    /// the other.
    ///
    /// **Warning:** If the block hash and number do not align, then things will go wrong. Prefer to
    /// use [`Self::at_block`] if in any doubt.
    pub async fn at_block_hash_and_number(
        &self,
        block_ref: impl Into<BlockRef<HashFor<T>>>,
        block_number: u64,
    ) -> Result<ClientAtBlock<OnlineClientAtBlock<T>, T>, OnlineClientAtBlockError> {
        let block_ref = block_ref.into();
        let block_hash = block_ref.hash();

        // Obtain the spec version so that we know which metadata to use at this block.
        // Obtain the transaction version because it's required for constructing extrinsics.
        let (spec_version, transaction_version) = match self
            .inner
            .config
            .spec_and_transaction_version_for_block_number(block_number)
        {
            Some(version) => version,
            None => {
                let spec_version_bytes = self
                    .inner
                    .backend
                    .call("Core_version", None, block_hash)
                    .await
                    .map_err(|e| OnlineClientAtBlockError::CannotGetSpecVersion {
                        block_hash: block_hash.into(),
                        reason: e,
                    })?;

                #[derive(codec::Decode)]
                struct SpecVersionHeader {
                    _spec_name: String,
                    _impl_name: String,
                    _authoring_version: u32,
                    spec_version: u32,
                    _impl_version: u32,
                    _apis: Vec<([u8; 8], u32)>,
                    transaction_version: u32,
                }
                let version =
                    SpecVersionHeader::decode(&mut &spec_version_bytes[..]).map_err(|e| {
                        OnlineClientAtBlockError::CannotDecodeSpecVersion {
                            block_hash: block_hash.into(),
                            reason: e,
                        }
                    })?;
                (version.spec_version, version.transaction_version)
            }
        };

        // Obtain the metadata for the block. Allow our config to cache it.
        let metadata = match self.inner.config.metadata_for_spec_version(spec_version) {
            Some(metadata) => metadata,
            None => {
                let metadata: Metadata =
                    match get_metadata(&*self.inner.backend, block_hash).await? {
                        m @ RuntimeMetadata::V0(_)
                        | m @ RuntimeMetadata::V1(_)
                        | m @ RuntimeMetadata::V2(_)
                        | m @ RuntimeMetadata::V3(_)
                        | m @ RuntimeMetadata::V4(_)
                        | m @ RuntimeMetadata::V5(_)
                        | m @ RuntimeMetadata::V6(_)
                        | m @ RuntimeMetadata::V7(_) => {
                            return Err(OnlineClientAtBlockError::UnsupportedMetadataVersion {
                                block_hash: block_hash.into(),
                                version: m.version(),
                            });
                        }
                        RuntimeMetadata::V8(m) => {
                            let types = get_legacy_types(self, &m, spec_version)?;
                            Metadata::from_v8(&m, &types).map_err(|e| {
                                OnlineClientAtBlockError::CannotConvertLegacyMetadata {
                                    block_hash: block_hash.into(),
                                    metadata_version: 8,
                                    reason: e,
                                }
                            })?
                        }
                        RuntimeMetadata::V9(m) => {
                            let types = get_legacy_types(self, &m, spec_version)?;
                            Metadata::from_v9(&m, &types).map_err(|e| {
                                OnlineClientAtBlockError::CannotConvertLegacyMetadata {
                                    block_hash: block_hash.into(),
                                    metadata_version: 9,
                                    reason: e,
                                }
                            })?
                        }
                        RuntimeMetadata::V10(m) => {
                            let types = get_legacy_types(self, &m, spec_version)?;
                            Metadata::from_v10(&m, &types).map_err(|e| {
                                OnlineClientAtBlockError::CannotConvertLegacyMetadata {
                                    block_hash: block_hash.into(),
                                    metadata_version: 10,
                                    reason: e,
                                }
                            })?
                        }
                        RuntimeMetadata::V11(m) => {
                            let types = get_legacy_types(self, &m, spec_version)?;
                            Metadata::from_v11(&m, &types).map_err(|e| {
                                OnlineClientAtBlockError::CannotConvertLegacyMetadata {
                                    block_hash: block_hash.into(),
                                    metadata_version: 11,
                                    reason: e,
                                }
                            })?
                        }
                        RuntimeMetadata::V12(m) => {
                            let types = get_legacy_types(self, &m, spec_version)?;
                            Metadata::from_v12(&m, &types).map_err(|e| {
                                OnlineClientAtBlockError::CannotConvertLegacyMetadata {
                                    block_hash: block_hash.into(),
                                    metadata_version: 12,
                                    reason: e,
                                }
                            })?
                        }
                        RuntimeMetadata::V13(m) => {
                            let types = get_legacy_types(self, &m, spec_version)?;
                            Metadata::from_v13(&m, &types).map_err(|e| {
                                OnlineClientAtBlockError::CannotConvertLegacyMetadata {
                                    block_hash: block_hash.into(),
                                    metadata_version: 13,
                                    reason: e,
                                }
                            })?
                        }
                        RuntimeMetadata::V14(m) => Metadata::from_v14(m).map_err(|e| {
                            OnlineClientAtBlockError::CannotConvertModernMetadata {
                                block_hash: block_hash.into(),
                                metadata_version: 14,
                                reason: e,
                            }
                        })?,
                        RuntimeMetadata::V15(m) => Metadata::from_v15(m).map_err(|e| {
                            OnlineClientAtBlockError::CannotConvertModernMetadata {
                                block_hash: block_hash.into(),
                                metadata_version: 15,
                                reason: e,
                            }
                        })?,
                        RuntimeMetadata::V16(m) => Metadata::from_v16(m).map_err(|e| {
                            OnlineClientAtBlockError::CannotConvertModernMetadata {
                                block_hash: block_hash.into(),
                                metadata_version: 16,
                                reason: e,
                            }
                        })?,
                    };
                let metadata = Arc::new(metadata);
                self.inner
                    .config
                    .set_metadata_for_spec_version(spec_version, metadata.clone());
                metadata
            }
        };

        let online_client_at_block = OnlineClientAtBlock {
            hasher: <T::Hasher as Hasher>::new(&metadata),
            metadata,
            backend: self.inner.backend.clone(),
            block_ref,
            block_number,
            spec_version,
            genesis_hash: self.inner.genesis_hash,
            transaction_version,
        };

        Ok(ClientAtBlock {
            client: online_client_at_block,
            marker: PhantomData,
        })
    }
}

/// This represents an online client at a specific block.
#[doc(hidden)]
pub trait OnlineClientAtBlockT<T: Config>: OfflineClientAtBlockT<T> {
    /// Return the RPC methods we'll use to interact with the node.
    fn backend(&self) -> &dyn Backend<T>;
    /// Return the block hash for the current block.
    fn block_hash(&self) -> HashFor<T>;
    /// Return a hasher that works at the current block.
    fn hasher(&self) -> &T::Hasher;
}

/// The inner type providing the necessary data to work online at a specific block.
#[derive(Clone)]
pub struct OnlineClientAtBlock<T: Config> {
    metadata: Arc<Metadata>,
    backend: Arc<dyn Backend<T>>,
    hasher: T::Hasher,
    block_ref: BlockRef<HashFor<T>>,
    block_number: u64,
    spec_version: u32,
    genesis_hash: HashFor<T>,
    transaction_version: u32,
}

impl<T: Config> OnlineClientAtBlockT<T> for OnlineClientAtBlock<T> {
    fn backend(&self) -> &dyn Backend<T> {
        &*self.backend
    }
    fn block_hash(&self) -> HashFor<T> {
        self.block_ref.hash()
    }
    fn hasher(&self) -> &T::Hasher {
        &self.hasher
    }
}

impl<T: Config> OfflineClientAtBlockT<T> for OnlineClientAtBlock<T> {
    fn metadata_ref(&self) -> &Metadata {
        &self.metadata
    }
    fn metadata(&self) -> Arc<Metadata> {
        self.metadata.clone()
    }
    fn block_number(&self) -> u64 {
        self.block_number
    }
    fn genesis_hash(&self) -> Option<HashFor<T>> {
        Some(self.genesis_hash)
    }
    fn spec_version(&self) -> u32 {
        self.spec_version
    }
    fn transaction_version(&self) -> u32 {
        self.transaction_version
    }
}

fn get_legacy_types<'a, T: Config, Md: ToTypeRegistry>(
    client: &'a OnlineClient<T>,
    metadata: &Md,
    spec_version: u32,
) -> Result<TypeRegistrySet<'a>, OnlineClientAtBlockError> {
    let mut types = client
        .inner
        .config
        .legacy_types_for_spec_version(spec_version)
        .ok_or(OnlineClientAtBlockError::MissingLegacyTypes)?;

    // Extend the types with information from the metadata (ie event/error/call enums):
    let additional_types = frame_decode::helpers::type_registry_from_metadata(metadata)
        .map_err(|e| OnlineClientAtBlockError::CannotInjectMetadataTypes { parse_error: e })?;
    types.prepend(additional_types);

    Ok(types)
}

async fn get_metadata<T: Config>(
    backend: &dyn Backend<T>,
    block_hash: HashFor<T>,
) -> Result<RuntimeMetadata, OnlineClientAtBlockError> {
    // First, try to use the "modern" metadata APIs to get the most recent version we can.
    let version_to_get = backend
        .call("Metadata_metadata_versions", None, block_hash)
        .await
        .ok()
        .and_then(|res| <Vec<u32>>::decode(&mut &res[..]).ok())
        .and_then(|versions| {
            // We want to filter out the "unstable" version, which is represented by u32::MAX.
            versions.into_iter().filter(|v| *v != u32::MAX).max()
        });

    // We had success calling the above API, so we expect the "modern" metadata API to work.
    if let Some(version_to_get) = version_to_get {
        let version_bytes = version_to_get.encode();
        let rpc_response = backend
            .call(
                "Metadata_metadata_at_version",
                Some(&version_bytes),
                block_hash,
            )
            .await
            .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
                block_hash: block_hash.into(),
                reason: format!("Error calling Metadata_metadata_at_version: {e}"),
            })?;

        // Option because we may have asked for a version that doesn't exist. Compact because we get back a Vec<u8>
        // of the metadata bytes, and the Vec is preceded by it's compact encoded length. The actual bytes are then
        // decoded as a `RuntimeMetadataPrefixed`, after this.
        let (_, metadata) = <Option<(Compact<u32>, RuntimeMetadataPrefixed)>>::decode(&mut &rpc_response[..])
            .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
                block_hash: block_hash.into(),
                reason: format!("Error decoding response for Metadata_metadata_at_version: {e}"),
            })?
            .ok_or_else(|| OnlineClientAtBlockError::CannotGetMetadata {
                block_hash: block_hash.into(),
                reason: format!("No metadata returned for the latest version from Metadata_metadata_versions ({version_to_get})"),
            })?;

        return Ok(metadata.1);
    }

    // We didn't get a version from Metadata_metadata_versions, so fall back to the "old" API.
    let metadata_bytes = backend
        .call("Metadata_metadata", None, block_hash)
        .await
        .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
            block_hash: block_hash.into(),
            reason: format!("Error calling Metadata_metadata: {e}"),
        })?;

    let (_, metadata) = <(Compact<u32>, RuntimeMetadataPrefixed)>::decode(&mut &metadata_bytes[..])
        .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
            block_hash: block_hash.into(),
            reason: format!("Error decoding response for Metadata_metadata: {e}"),
        })?;

    Ok(metadata.1)
}
