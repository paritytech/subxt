mod block_number_or_ref;

use super::ClientAtBlock;
use super::OfflineClientAtBlockT;
use crate::config::Header;
use crate::config::{ Config, HashFor, RpcConfigFor };
use crate::error::OnlineClientAtBlockError;
use crate::backend::{ Backend, CombinedBackend, BlockRef };
use codec::{Compact, Decode, Encode};
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use std::sync::Arc;
use subxt_rpcs::methods::chain_head::ArchiveCallResult;
use subxt_rpcs::{ChainHeadRpcMethods, RpcClient};
use subxt_metadata::Metadata;

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
    /// The RPC methods used to communicate with the node.
    backend: Arc<dyn Backend<T>>,
}

impl <T: Config> std::fmt::Debug for OnlineClientInner<T> {
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
            return Err(OnlineClientError::RpcError(
                subxt_rpcs::Error::InsecureUrl(url_str.to_string()),
            ));
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
        Ok(OnlineClient::from_backend(config, backend))
    }

    /// Construct a new [`OnlineClient`] by providing an underlying [`Backend`]
    /// implementation to power it.
    pub fn from_backend(
        config: T,
        backend: impl Into<Arc<dyn Backend<T>>>,
    ) -> OnlineClient<T> {
        OnlineClient {
            inner: Arc::new(OnlineClientInner {
                config,
                backend: backend.into()
            })
        }
    }

    /// Pick the block height at which to operate. This references data from the
    /// [`OnlineClient`] it's called on, and so cannot outlive it.
    pub async fn at_block(
        &self,
        number_or_hash: impl Into<BlockNumberOrRef<T>>,
    ) -> Result<ClientAtBlock<OnlineClientAtBlock<T>, T>, OnlineClientAtBlockError> {
        let number_or_hash = number_or_hash.into();

        // We are given either a block hash or number. We need both.
        let (block_ref, block_num) = match number_or_hash {
            BlockNumberOrRef::BlockRef(block_ref) => {
                let block_hash = block_ref.hash();
                let block_header = self
                    .inner
                    .backend
                    .block_header(block_hash)
                    .await
                    .map_err(|e| OnlineClientAtBlockError::CannotGetBlockHeader { 
                        block_hash: block_hash.into(), 
                        reason: e
                    })?
                    .ok_or(OnlineClientAtBlockError::BlockHeaderNotFound { 
                        block_hash: block_hash.into()
                    })?;
                (block_ref, block_header.number())
            },
            BlockNumberOrRef::Number(block_num) => {
                let block_ref = self
                    .inner
                    .backend
                    .block_number_to_hash(block_num)
                    .await
                    .map_err(|e| OnlineClientAtBlockError::CannotGetBlockHash { 
                        block_number: block_num, 
                        reason: e 
                    })?
                    .ok_or(OnlineClientAtBlockError::BlockNotFound { 
                        block_number: block_num
                    })?;
                (block_ref, block_num)
            }
        };

        // Obtain the spec version so that we know which metadata to use at this block.
        let spec_version = match self.inner.config.spec_version_for_block_number(block_num) {
            Some(version) => version,
            None => {
                let block_hash = block_ref.hash();
                let spec_version_bytes = self
                    .inner
                    .backend
                    .call("Core_version", None, block_hash)
                    .await
                    .map_err(|e| OnlineClientAtBlockError::CannotGetSpecVersion { 
                        block_hash: block_hash.into(), 
                        reason: e 
                    })?;

                #[derive(codec::Decode)]
                struct SpecVersionHeader {
                    _spec_name: String,
                    _impl_name: String,
                    _authoring_version: u32,
                    spec_version: u32,
                }
                SpecVersionHeader::decode(&mut &spec_version_bytes[..])
                    .map_err(|e| OnlineClientAtBlockError::CannotDecodeSpecVersion {
                        block_hash: block_hash.into(),
                        reason: e,
                    })?
                    .spec_version
            }
        };

        // Obtain the metadata for the block, allowing our config to cache it.
        let metadata = match self.inner.config.metadata_for_spec_version(spec_version) {
            Some(metadata) => metadata,
            None => {
                //self.inner.backend.
                todo!()
            }
        };

        todo!()
    }
}

/// This represents an online client at a specific block.
#[doc(hidden)]
pub trait OnlineClientAtBlockT<T: Config>: OfflineClientAtBlockT
{
    /// Return the RPC methods we'll use to interact with the node.
    fn backend(&self) -> &dyn Backend<T>;
    /// Return the block hash for the current block.
    fn block_hash(&self) -> HashFor<T>;
}

// Dev note: this shouldn't need to be exposed unless there is some
// need to explicitly name the ClientAAtBlock type. Rather keep it
// private to allow changes if possible.
#[doc(hidden)]
pub struct OnlineClientAtBlock<T: Config> {
    metadata: Arc<Metadata>,
    backend: Arc<dyn Backend<T>>,
    hasher: T::Hasher,
    block_ref: BlockRef<HashFor<T>>,
}

impl<T: Config> OnlineClientAtBlockT<T> for OnlineClientAtBlock<T> {
    fn backend(&self) -> &dyn Backend<T> {
        &*self.backend
    }
    fn block_hash(&self) -> HashFor<T> {
        self.block_ref.hash()
    }
}

impl<T: Config> OfflineClientAtBlockT for OnlineClientAtBlock<T> {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
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
            .call("Metadata_metadata_at_version", Some(&version_bytes), block_hash)
            .await
            .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
                block_hash: block_hash.to_string(),
                reason: format!("Error calling Metadata_metadata_at_version: {e}"),
            })
            .and_then(|res| match res {
                ArchiveCallResult::Success(bytes) => Ok(bytes.0),
                ArchiveCallResult::Error(e) => Err(OnlineClientAtBlockError::CannotGetMetadata {
                    block_hash: block_hash.to_string(),
                    reason: format!("Calling Metadata_metadata_at_version returned an error: {e}"),
                }),
            })?;

        // Option because we may have asked for a version that doesn't exist. Compact because we get back a Vec<u8>
        // of the metadata bytes, and the Vec is preceded by it's compact encoded length. The actual bytes are then
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

        return Ok(metadata.1);
    }

    // We didn't get a version from Metadata_metadata_versions, so fall back to the "old" API.
    let metadata_bytes = rpc_methods
        .archive_v1_call(block_hash.into(), "Metadata_metadata", &[])
        .await
        .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
            block_hash: block_hash.to_string(),
            reason: format!("Error calling Metadata_metadata: {e}"),
        })
        .and_then(|res| match res {
            ArchiveCallResult::Success(bytes) => Ok(bytes.0),
            ArchiveCallResult::Error(e) => Err(OnlineClientAtBlockError::CannotGetMetadata {
                block_hash: block_hash.to_string(),
                reason: format!("Calling Metadata_metadata returned an error: {e}"),
            }),
        })?;

    let (_, metadata) = <(Compact<u32>, RuntimeMetadataPrefixed)>::decode(&mut &metadata_bytes[..])
        .map_err(|e| OnlineClientAtBlockError::CannotGetMetadata {
            block_hash: block_hash.to_string(),
            reason: format!("Error decoding response for Metadata_metadata: {e}"),
        })?;

    Ok(metadata.1)
}
