#[cfg(feature = "scale-info-legacy")]
pub mod substrate;

use subxt_rpcs::RpcConfig;
use scale_type_resolver::{ TypeResolver, ResolvedTypeVisitor};
use std::future::Future;

/// This represents the configuration needed for a specific chain. This includes
/// any hardcoded types we need to know about for that chain, as well as a means to
/// obtain historic types for that chain.
pub trait Config {
    /// The block hash type.
    type Hash: Hash;
    /// The shape of our historic type definitions.
    type LegacyTypes<'a>: TypeResolver where Self: 'a;

    /// Return legacy types (ie types to use with Runtimes that return pre-V14 metadata) for a given spec version.
    fn legacy_types_for_spec_version(&'_ self, spec_version: u64) -> Self::LegacyTypes<'_>;
}

/// A trait which is applied to any type that is a valid block hash.
pub trait Hash: serde::de::DeserializeOwned + serde::Serialize {}
impl<T> Hash for T where T: serde::de::DeserializeOwned + serde::Serialize {}

/// A struct which can be used as [`Config::LegacyTypes`] when no legacy types are available/required for a chain.
pub struct NoLegacyTypes;
impl TypeResolver for NoLegacyTypes {
    type TypeId = ();
    type Error = NoLegacyTypesError;

    fn resolve_type<'this, V: ResolvedTypeVisitor<'this, TypeId = Self::TypeId>>(
        &'this self,
        _type_id: Self::TypeId,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(NoLegacyTypesError)
    }
}

pub struct NoLegacyTypesError;
impl std::fmt::Display for NoLegacyTypesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No legacy types have been provided for this chain")
    }
}
impl std::fmt::Debug for NoLegacyTypesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoLegacyTypesError").finish()
    }
}
