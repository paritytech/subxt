pub mod polkadot;
pub mod substrate;

use scale_info_legacy::TypeRegistrySet;
use std::fmt::Display;
use std::sync::Arc;
use subxt_rpcs::RpcConfig;

pub use polkadot::PolkadotConfig;
pub use substrate::SubstrateConfig;

/// This represents the configuration needed for a specific chain. This includes
/// any hardcoded types we need to know about for that chain, as well as a means to
/// obtain historic types for that chain.
pub trait Config: RpcConfig {
    /// The type of hashing used by the runtime.
    type Hash: Clone
        + Copy
        + Display
        + Into<<Self as RpcConfig>::Hash>
        + From<<Self as RpcConfig>::Hash>;

    /// Return the spec version for a given block number, if available.
    ///
    /// The [`crate::client::OnlineClient`] will look this up on chain if it's not available here,
    /// but the [`crate::client::OfflineClient`] will error if this is not available for the required block number.
    fn spec_version_for_block_number(&self, block_number: u64) -> Option<u32>;

    /// Return the metadata for a given spec version, if available.
    ///
    /// The [`crate::client::OnlineClient`] will look this up on chain if it's not available here, and then
    /// call [`Config::set_metadata_for_spec_version`] to give the configuration the opportunity to cache it.
    /// The [`crate::client::OfflineClient`] will error if this is not available for the required spec version.
    fn metadata_for_spec_version(
        &self,
        spec_version: u32,
    ) -> Option<Arc<frame_metadata::RuntimeMetadata>>;

    /// Set some metadata for a given spec version. the [`crate::client::OnlineClient`] will call this if it has
    /// to retrieve metadata from the chain, to give this the opportunity to cache it. The configuration can
    /// do nothing if it prefers.
    fn set_metadata_for_spec_version(
        &self,
        spec_version: u32,
        metadata: Arc<frame_metadata::RuntimeMetadata>,
    );

    /// Return legacy types (ie types to use with Runtimes that return pre-V14 metadata) for a given spec version.
    fn legacy_types_for_spec_version<'this>(
        &'this self,
        spec_version: u32,
    ) -> TypeRegistrySet<'this>;

    /// Hash some bytes, for instance a block header or extrinsic, for this chain.
    fn hash(s: &[u8]) -> <Self as Config>::Hash;
}
