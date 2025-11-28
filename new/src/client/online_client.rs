use super::ClientAtBlock;
use super::OfflineClientAtBlockT;
use crate::config::{ Config, HashFor, RpcConfigFor };
use crate::error::OnlineClientAtBlockError;
use crate::backend::Backend;
use codec::{Compact, Decode, Encode};
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use scale_info_legacy::TypeRegistrySet;
use std::sync::Arc;
use subxt_rpcs::methods::chain_head::ArchiveCallResult;
use subxt_rpcs::{ChainHeadRpcMethods, RpcClient};
use subxt_metadata::Metadata;

#[cfg(feature = "jsonrpsee")]
#[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee")))]
use crate::error::OnlineClientError;

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
    #[cfg(feature = "jsonrpsee")]
    pub async fn new(config: T) -> Result<OnlineClient<T>, OnlineClientError> {
        let url = "ws://127.0.0.1:9944";
        OnlineClient::from_url(config, url).await
    }

    /// Construct a new [`OnlineClient`], providing a URL to connect to.
    #[cfg(feature = "jsonrpsee")]
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
    #[cfg(feature = "jsonrpsee")]
    pub async fn from_insecure_url(
        config: T,
        url: impl AsRef<str>,
    ) -> Result<OnlineClient<T>, OnlineClientError> {
        let rpc_client = RpcClient::from_insecure_url(url).await?;
        Ok(OnlineClient::from_rpc_client(config, rpc_client))
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
    #[cfg(feature = "jsonrpsee")]
    pub fn from_rpc_client(
        config: T,
        rpc_client: impl Into<RpcClient>,
    ) -> OnlineClient<T> {
        let rpc_client = rpc_client.into();
        let backend = Arc::new(LegacyBackend::builder().build(rpc_client));
        OnlineClient::from_backend(config, backend)
    }

    /// Construct a new [`OnlineClient`] by providing an underlying [`Backend`]
    /// implementation to power it.
    pub fn from_backend<B: Backend<T>>(
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
        block_number: u32,
    ) -> Result<ClientAtBlock<OnlineClientAtBlock<T>, T>, OnlineClientAtBlockError> {
        let config = &self.inner.config;
        let rpc_methods = &self.inner.rpc_methods;

        let block_hash = rpc_methods
            .archive_v1_hash_by_height(block_number as usize)
            .await
            .map_err(|e| OnlineClientAtBlockError::CannotGetBlockHash {
                block_number,
                reason: e,
            })?
            .pop()
            .ok_or_else(|| OnlineClientAtBlockError::BlockNotFound { block_number })?
            .into();

        // Get our configuration, or fetch from the node if not available.
        let spec_version =
            if let Some(spec_version) = config.spec_version_for_block_number(block_number) {
                spec_version
            } else {
                // Fetch spec version. Caching this doesn't really make sense, so either
                // details are provided offline or we fetch them every time.
                get_spec_version(rpc_methods, block_hash).await?
            };
        let metadata = if let Some(metadata) = config.metadata_for_spec_version(spec_version) {
            metadata
        } else {
            // Fetch and then give our config the opportunity to cache this metadata.
            let metadata = get_metadata(rpc_methods, block_hash).await?;
            let metadata = Arc::new(metadata);
            config.set_metadata_for_spec_version(spec_version, metadata.clone());
            metadata
        };

        let mut historic_types = config.legacy_types_for_spec_version(spec_version);
        // The metadata can be used to construct call and event types instead of us having to hardcode them all for every spec version:
        let types_from_metadata = frame_decode::helpers::type_registry_from_metadata_any(&metadata)
            .map_err(
                |parse_error| OnlineClientAtBlockError::CannotInjectMetadataTypes { parse_error },
            )?;
        historic_types.prepend(types_from_metadata);

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
    block_hash: HashFor<T>,
}

impl<T: Config> OnlineClientAtBlockT<T> for OnlineClientAtBlock<T> {
    fn backend(&self) -> &dyn Backend<T> {
        &*self.backend
    }
    fn block_hash(&self) -> HashFor<T> {
        self.block_hash
    }
}

impl<T: Config> OfflineClientAtBlockT for OnlineClientAtBlock<T> {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

async fn get_spec_version<T: Config>(
    rpc_methods: &ChainHeadRpcMethods<RpcConfigFor<T>>,
    block_hash: HashFor<T>,
) -> Result<u32, OnlineClientAtBlockError> {
    use codec::Decode;
    use subxt_rpcs::methods::chain_head::ArchiveCallResult;

    // make a runtime call to get the version information. This is also a constant
    // in the metadata and so we could fetch it from there to avoid the call, but it would be a
    // bit more effort.
    let spec_version_bytes = {
        let call_res = rpc_methods
            .archive_v1_call(block_hash.into(), "Core_version", &[])
            .await
            .map_err(|e| OnlineClientAtBlockError::CannotGetSpecVersion {
                block_hash: block_hash.to_string(),
                reason: format!("Error calling Core_version: {e}"),
            })?;
        match call_res {
            ArchiveCallResult::Success(bytes) => bytes.0,
            ArchiveCallResult::Error(e) => {
                return Err(OnlineClientAtBlockError::CannotGetSpecVersion {
                    block_hash: block_hash.to_string(),
                    reason: format!("Core_version returned an error: {e}"),
                });
            }
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
            spec_version: u32,
        }
        SpecVersionHeader::decode(&mut &spec_version_bytes[..])
            .map_err(|e| OnlineClientAtBlockError::CannotGetSpecVersion {
                block_hash: block_hash.to_string(),
                reason: format!("Error decoding Core_version response: {e}"),
            })?
            .spec_version
    };

    Ok(spec_version)
}

async fn get_metadata<T: Config>(
    rpc_methods: &ChainHeadRpcMethods<RpcConfigFor<T>>,
    block_hash: HashFor<T>,
) -> Result<RuntimeMetadata, OnlineClientAtBlockError> {
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
            .archive_v1_call(
                block_hash.into(),
                "Metadata_metadata_at_version",
                &version_bytes,
            )
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
