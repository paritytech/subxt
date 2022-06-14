// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use super::hash_cache::HashCache;
use crate::Call;
use codec::Error as CodecError;
use frame_metadata::{
    PalletConstantMetadata,
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
    RuntimeMetadataV14,
    StorageEntryMetadata,
    META_RESERVED,
};
use parking_lot::RwLock;
use scale_info::{
    form::PortableForm,
    Type,
    Variant,
};
use std::{
    collections::HashMap,
    convert::TryFrom,
    sync::Arc,
};

/// Metadata error originated from inspecting the internal representation of the runtime metadata.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum MetadataError {
    /// Module is not in metadata.
    #[error("Pallet not found")]
    PalletNotFound,
    /// Pallet is not in metadata.
    #[error("Pallet index {0} not found")]
    PalletIndexNotFound(u8),
    /// Call is not in metadata.
    #[error("Call not found")]
    CallNotFound,
    /// Event is not in metadata.
    #[error("Pallet {0}, Event {0} not found")]
    EventNotFound(u8, u8),
    /// Event is not in metadata.
    #[error("Pallet {0}, Error {0} not found")]
    ErrorNotFound(u8, u8),
    /// Storage is not in metadata.
    #[error("Storage not found")]
    StorageNotFound,
    /// Storage type does not match requested type.
    #[error("Storage type error")]
    StorageTypeError,
    /// Default error.
    #[error("Failed to decode default: {0}")]
    DefaultError(CodecError),
    /// Failure to decode constant value.
    #[error("Failed to decode constant value: {0}")]
    ConstantValueError(CodecError),
    /// Constant is not in metadata.
    #[error("Constant not found")]
    ConstantNotFound,
    /// Type is not in metadata.
    #[error("Type {0} missing from type registry")]
    TypeNotFound(u32),
    /// Runtime pallet metadata is incompatible with the static one.
    #[error("Pallet {0} has incompatible metadata")]
    IncompatiblePalletMetadata(&'static str),
    /// Runtime metadata is not fully compatible with the static one.
    #[error("Node metadata is not fully compatible")]
    IncompatibleMetadata,
}

// We hide the innards behind an Arc so that it's easy to clone and share.
#[derive(Debug)]
struct MetadataInner {
    metadata: RuntimeMetadataV14,
    pallets: HashMap<String, PalletMetadata>,
    events: HashMap<(u8, u8), EventMetadata>,
    errors: HashMap<(u8, u8), ErrorMetadata>,
    // The hashes uniquely identify parts of the metadata; different
    // hashes mean some type difference exists between static and runtime
    // versions. We cache them here to avoid recalculating:
    cached_metadata_hash: RwLock<Option<[u8; 32]>>,
    cached_call_hashes: HashCache,
    cached_constant_hashes: HashCache,
    cached_storage_hashes: HashCache,
}

/// A representation of the runtime metadata received from a node.
#[derive(Clone, Debug)]
pub struct Metadata {
    inner: Arc<MetadataInner>,
}

impl Metadata {
    /// Returns a reference to [`PalletMetadata`].
    pub fn pallet(&self, name: &'static str) -> Result<&PalletMetadata, MetadataError> {
        self.inner
            .pallets
            .get(name)
            .ok_or(MetadataError::PalletNotFound)
    }

    /// Returns the metadata for the event at the given pallet and event indices.
    pub fn event(
        &self,
        pallet_index: u8,
        event_index: u8,
    ) -> Result<&EventMetadata, MetadataError> {
        let event = self
            .inner
            .events
            .get(&(pallet_index, event_index))
            .ok_or(MetadataError::EventNotFound(pallet_index, event_index))?;
        Ok(event)
    }

    /// Returns the metadata for the error at the given pallet and error indices.
    pub fn error(
        &self,
        pallet_index: u8,
        error_index: u8,
    ) -> Result<&ErrorMetadata, MetadataError> {
        let error = self
            .inner
            .errors
            .get(&(pallet_index, error_index))
            .ok_or(MetadataError::ErrorNotFound(pallet_index, error_index))?;
        Ok(error)
    }

    /// Resolve a type definition.
    pub fn resolve_type(&self, id: u32) -> Option<&Type<PortableForm>> {
        self.inner.metadata.types.resolve(id)
    }

    /// Return the runtime metadata.
    pub fn runtime_metadata(&self) -> &RuntimeMetadataV14 {
        &self.inner.metadata
    }

    /// Obtain the unique hash for a specific storage entry.
    pub fn storage_hash<S: crate::StorageEntry>(
        &self,
    ) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_storage_hashes
            .get_or_insert(S::PALLET, S::STORAGE, || {
                subxt_metadata::get_storage_hash(
                    &self.inner.metadata,
                    S::PALLET,
                    S::STORAGE,
                )
                .map_err(|e| {
                    match e {
                        subxt_metadata::NotFound::Pallet => MetadataError::PalletNotFound,
                        subxt_metadata::NotFound::Item => MetadataError::StorageNotFound,
                    }
                })
            })
    }

    /// Obtain the unique hash for a constant.
    pub fn constant_hash(
        &self,
        pallet: &str,
        constant: &str,
    ) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_constant_hashes
            .get_or_insert(pallet, constant, || {
                subxt_metadata::get_constant_hash(&self.inner.metadata, pallet, constant)
                    .map_err(|e| {
                        match e {
                            subxt_metadata::NotFound::Pallet => {
                                MetadataError::PalletNotFound
                            }
                            subxt_metadata::NotFound::Item => {
                                MetadataError::ConstantNotFound
                            }
                        }
                    })
            })
    }

    /// Obtain the unique hash for a call.
    pub fn call_hash<C: crate::Call>(&self) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_call_hashes
            .get_or_insert(C::PALLET, C::FUNCTION, || {
                subxt_metadata::get_call_hash(
                    &self.inner.metadata,
                    C::PALLET,
                    C::FUNCTION,
                )
                .map_err(|e| {
                    match e {
                        subxt_metadata::NotFound::Pallet => MetadataError::PalletNotFound,
                        subxt_metadata::NotFound::Item => MetadataError::CallNotFound,
                    }
                })
            })
    }

    /// Obtain the unique hash for this metadata.
    pub fn metadata_hash<T: AsRef<str>>(&self, pallets: &[T]) -> [u8; 32] {
        if let Some(hash) = *self.inner.cached_metadata_hash.read() {
            return hash
        }

        let hash = subxt_metadata::get_metadata_per_pallet_hash(
            self.runtime_metadata(),
            pallets,
        );
        *self.inner.cached_metadata_hash.write() = Some(hash);

        hash
    }
}

/// Metadata for a specific pallet.
#[derive(Clone, Debug)]
pub struct PalletMetadata {
    index: u8,
    name: String,
    calls: HashMap<String, u8>,
    storage: HashMap<String, StorageEntryMetadata<PortableForm>>,
    constants: HashMap<String, PalletConstantMetadata<PortableForm>>,
}

impl PalletMetadata {
    /// Get the name of the pallet.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the index of this pallet.
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Attempt to resolve a call into an index in this pallet, failing
    /// if the call is not found in this pallet.
    pub fn call_index<C>(&self) -> Result<u8, MetadataError>
    where
        C: Call,
    {
        let fn_index = *self
            .calls
            .get(C::FUNCTION)
            .ok_or(MetadataError::CallNotFound)?;
        Ok(fn_index)
    }

    /// Return [`StorageEntryMetadata`] given some storage key.
    pub fn storage(
        &self,
        key: &str,
    ) -> Result<&StorageEntryMetadata<PortableForm>, MetadataError> {
        self.storage.get(key).ok_or(MetadataError::StorageNotFound)
    }

    /// Get a constant's metadata by name.
    pub fn constant(
        &self,
        key: &str,
    ) -> Result<&PalletConstantMetadata<PortableForm>, MetadataError> {
        self.constants
            .get(key)
            .ok_or(MetadataError::ConstantNotFound)
    }
}

/// Metadata for specific events.
#[derive(Clone, Debug)]
pub struct EventMetadata {
    pallet: String,
    event: String,
    variant: Variant<PortableForm>,
}

impl EventMetadata {
    /// Get the name of the pallet from which the event was emitted.
    pub fn pallet(&self) -> &str {
        &self.pallet
    }

    /// Get the name of the pallet event which was emitted.
    pub fn event(&self) -> &str {
        &self.event
    }

    /// Get the type def variant for the pallet event.
    pub fn variant(&self) -> &Variant<PortableForm> {
        &self.variant
    }
}

/// Metadata for specific errors obtained from the pallet's `PalletErrorMetadata`.
///
/// This holds in memory information regarding the Pallet's name, Error's name, and the underlying
/// metadata representation.
#[derive(Clone, Debug)]
pub struct ErrorMetadata {
    pallet: String,
    error: String,
    variant: Variant<PortableForm>,
}

impl ErrorMetadata {
    /// Get the name of the pallet from which the error originates.
    pub fn pallet(&self) -> &str {
        &self.pallet
    }

    /// Get the name of the specific pallet error.
    pub fn error(&self) -> &str {
        &self.error
    }

    /// Get the description of the specific pallet error.
    pub fn description(&self) -> &[String] {
        self.variant.docs()
    }
}

/// Error originated from converting a runtime metadata [RuntimeMetadataPrefixed] to
/// the internal [Metadata] representation.
///
/// The runtime metadata is converted when building the [crate::client::Client].
#[derive(Debug, thiserror::Error)]
pub enum InvalidMetadataError {
    #[error("Invalid prefix")]
    InvalidPrefix,
    #[error("Invalid version")]
    InvalidVersion,
    #[error("Type {0} missing from type registry")]
    MissingType(u32),
    #[error("Type {0} was not a variant/enum type")]
    TypeDefNotVariant(u32),
}

impl TryFrom<RuntimeMetadataPrefixed> for Metadata {
    type Error = InvalidMetadataError;

    fn try_from(metadata: RuntimeMetadataPrefixed) -> Result<Self, Self::Error> {
        if metadata.0 != META_RESERVED {
            return Err(InvalidMetadataError::InvalidPrefix)
        }
        let metadata = match metadata.1 {
            RuntimeMetadata::V14(meta) => meta,
            _ => return Err(InvalidMetadataError::InvalidVersion),
        };

        let get_type_def_variant = |type_id: u32| {
            let ty = metadata
                .types
                .resolve(type_id)
                .ok_or(InvalidMetadataError::MissingType(type_id))?;
            if let scale_info::TypeDef::Variant(var) = ty.type_def() {
                Ok(var)
            } else {
                Err(InvalidMetadataError::TypeDefNotVariant(type_id))
            }
        };
        let pallets = metadata
            .pallets
            .iter()
            .map(|pallet| {
                let calls = pallet.calls.as_ref().map_or(Ok(HashMap::new()), |call| {
                    let type_def_variant = get_type_def_variant(call.ty.id())?;
                    let calls = type_def_variant
                        .variants()
                        .iter()
                        .map(|v| (v.name().clone(), v.index()))
                        .collect();
                    Ok(calls)
                })?;

                let storage = pallet.storage.as_ref().map_or(HashMap::new(), |storage| {
                    storage
                        .entries
                        .iter()
                        .map(|entry| (entry.name.clone(), entry.clone()))
                        .collect()
                });

                let constants = pallet
                    .constants
                    .iter()
                    .map(|constant| (constant.name.clone(), constant.clone()))
                    .collect();

                let pallet_metadata = PalletMetadata {
                    index: pallet.index,
                    name: pallet.name.to_string(),
                    calls,
                    storage,
                    constants,
                };

                Ok((pallet.name.to_string(), pallet_metadata))
            })
            .collect::<Result<_, _>>()?;

        let pallet_events = metadata
            .pallets
            .iter()
            .filter_map(|pallet| {
                pallet.event.as_ref().map(|event| {
                    let type_def_variant = get_type_def_variant(event.ty.id())?;
                    Ok((pallet, type_def_variant))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let events = pallet_events
            .iter()
            .flat_map(|(pallet, type_def_variant)| {
                type_def_variant.variants().iter().map(move |var| {
                    let key = (pallet.index, var.index());
                    let value = EventMetadata {
                        pallet: pallet.name.clone(),
                        event: var.name().clone(),
                        variant: var.clone(),
                    };
                    (key, value)
                })
            })
            .collect();

        let pallet_errors = metadata
            .pallets
            .iter()
            .filter_map(|pallet| {
                pallet.error.as_ref().map(|error| {
                    let type_def_variant = get_type_def_variant(error.ty.id())?;
                    Ok((pallet, type_def_variant))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let errors = pallet_errors
            .iter()
            .flat_map(|(pallet, type_def_variant)| {
                type_def_variant.variants().iter().map(move |var| {
                    let key = (pallet.index, var.index());
                    let value = ErrorMetadata {
                        pallet: pallet.name.clone(),
                        error: var.name().clone(),
                        variant: var.clone(),
                    };
                    (key, value)
                })
            })
            .collect();

        Ok(Metadata {
            inner: Arc::new(MetadataInner {
                metadata,
                pallets,
                events,
                errors,
                cached_metadata_hash: Default::default(),
                cached_call_hashes: Default::default(),
                cached_constant_hashes: Default::default(),
                cached_storage_hashes: Default::default(),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StorageEntryKey;
    use frame_metadata::{
        ExtrinsicMetadata,
        PalletStorageMetadata,
        StorageEntryModifier,
        StorageEntryType,
    };
    use scale_info::{
        meta_type,
        TypeInfo,
    };

    fn load_metadata() -> Metadata {
        #[allow(dead_code)]
        #[allow(non_camel_case_types)]
        #[derive(TypeInfo)]
        enum Call {
            fill_block { param: u128 },
        }
        let storage = PalletStorageMetadata {
            prefix: "System",
            entries: vec![StorageEntryMetadata {
                name: "Account",
                modifier: StorageEntryModifier::Optional,
                ty: StorageEntryType::Plain(meta_type::<u32>()),
                default: vec![0],
                docs: vec![],
            }],
        };
        let constant = PalletConstantMetadata {
            name: "BlockWeights",
            ty: meta_type::<u32>(),
            value: vec![1, 2, 3],
            docs: vec![],
        };
        let pallet = frame_metadata::PalletMetadata {
            index: 0,
            name: "System",
            calls: Some(frame_metadata::PalletCallMetadata {
                ty: meta_type::<Call>(),
            }),
            storage: Some(storage),
            constants: vec![constant],
            event: None,
            error: None,
        };

        let metadata = RuntimeMetadataV14::new(
            vec![pallet],
            ExtrinsicMetadata {
                ty: meta_type::<()>(),
                version: 0,
                signed_extensions: vec![],
            },
            meta_type::<()>(),
        );
        let prefixed = RuntimeMetadataPrefixed::from(metadata);

        Metadata::try_from(prefixed)
            .expect("Cannot translate runtime metadata to internal Metadata")
    }

    #[test]
    fn metadata_inner_cache() {
        // Note: Dependency on test_runtime can be removed if complex metadata
        // is manually constructed.
        let metadata = load_metadata();

        let hash = metadata.metadata_hash(&["System"]);
        // Check inner caching.
        assert_eq!(metadata.inner.cached_metadata_hash.read().unwrap(), hash);

        // The cache `metadata.inner.cached_metadata_hash` is already populated from
        // the previous call. Therefore, changing the pallets argument must not
        // change the methods behavior.
        let hash_old = metadata.metadata_hash(&["no-pallet"]);
        assert_eq!(hash_old, hash);
    }

    #[test]
    fn metadata_call_inner_cache() {
        let metadata = load_metadata();

        #[derive(codec::Encode)]
        struct ValidCall;
        impl crate::Call for ValidCall {
            const PALLET: &'static str = "System";
            const FUNCTION: &'static str = "fill_block";
        }

        let hash = metadata.call_hash::<ValidCall>();

        let mut call_number = 0;
        let hash_cached = metadata.inner.cached_call_hashes.get_or_insert(
            "System",
            "fill_block",
            || -> Result<[u8; 32], MetadataError> {
                call_number += 1;
                Ok([0; 32])
            },
        );

        // Check function is never called (e.i, value fetched from cache).
        assert_eq!(call_number, 0);
        assert_eq!(hash.unwrap(), hash_cached.unwrap());
    }

    #[test]
    fn metadata_constant_inner_cache() {
        let metadata = load_metadata();

        let hash = metadata.constant_hash("System", "BlockWeights");

        let mut call_number = 0;
        let hash_cached = metadata.inner.cached_constant_hashes.get_or_insert(
            "System",
            "BlockWeights",
            || -> Result<[u8; 32], MetadataError> {
                call_number += 1;
                Ok([0; 32])
            },
        );

        // Check function is never called (e.i, value fetched from cache).
        assert_eq!(call_number, 0);
        assert_eq!(hash.unwrap(), hash_cached.unwrap());
    }

    #[test]
    fn metadata_storage_inner_cache() {
        let metadata = load_metadata();

        #[derive(codec::Encode)]
        struct ValidStorage;
        impl crate::StorageEntry for ValidStorage {
            const PALLET: &'static str = "System";
            const STORAGE: &'static str = "Account";
            type Value = ();

            fn key(&self) -> StorageEntryKey {
                unreachable!("Should not be called");
            }
        }

        let hash = metadata.storage_hash::<ValidStorage>();

        let mut call_number = 0;
        let hash_cached = metadata.inner.cached_storage_hashes.get_or_insert(
            "System",
            "Account",
            || -> Result<[u8; 32], MetadataError> {
                call_number += 1;
                Ok([0; 32])
            },
        );

        // Check function is never called (e.i, value fetched from cache).
        assert_eq!(call_number, 0);
        assert_eq!(hash.unwrap(), hash_cached.unwrap());
    }
}
