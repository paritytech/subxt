#[cfg(feature = "scale-info-legacy")]
pub mod substrate;

use subxt_rpcs::RpcConfig;
use scale_type_resolver::{ TypeResolver, ResolvedTypeVisitor};
use scale_info_legacy::TypeRegistrySet;
use std::fmt::Display;

/// This represents the configuration needed for a specific chain. This includes
/// any hardcoded types we need to know about for that chain, as well as a means to
/// obtain historic types for that chain.
pub trait Config: RpcConfig {   
    /// The type of hashing used by the runtime.
    type Hash: Clone + Copy + Display + Into<<Self as RpcConfig>::Hash> + From<<Self as RpcConfig>::Hash>;

    /// Return legacy types (ie types to use with Runtimes that return pre-V14 metadata) for a given spec version.
    fn legacy_types_for_spec_version<'this>(&'this self, spec_version: u32) -> TypeRegistrySet<'this>;

    /// Hash some bytes, for instance a block header or extrinsic, for this chain.
    fn hash(s: &[u8]) -> <Self as Config>::Hash;
}

/// A struct which can be used as [`Config::LegacyTypes`] when no legacy types are available/required for a chain.
pub struct NoLegacyTypes;
impl TypeResolver for NoLegacyTypes {
    type TypeId = NoTypeId;
    type Error = NoLegacyTypesError;

    fn resolve_type<'this, V: ResolvedTypeVisitor<'this, TypeId = Self::TypeId>>(
        &'this self,
        _type_id: Self::TypeId,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(NoLegacyTypesError)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoLegacyTypesError;
impl std::fmt::Display for NoLegacyTypesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No legacy types have been provided for this chain")
    }
}

#[derive(Default, Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct NoTypeId;
impl std::fmt::Display for NoTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<No Type ID>")
    }
}

