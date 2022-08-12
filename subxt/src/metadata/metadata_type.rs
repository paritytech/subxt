// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::hash_cache::HashCache;
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
    PortableRegistry,
    Type,
};
use std::{
    collections::HashMap,
    convert::TryFrom,
    sync::Arc,
};

/// Metadata error originated from inspecting the internal representation of the runtime metadata.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
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
    // Errors are hashed by pallet index.
    errors: HashMap<(u8, u8), ErrorMetadata>,
    // Type of the DispatchError type, which is what comes back if
    // an extrinsic fails.
    dispatch_error_ty: Option<u32>,
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
    pub fn pallet(&self, name: &str) -> Result<&PalletMetadata, MetadataError> {
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

    /// Return the DispatchError type ID if it exists.
    pub fn dispatch_error_ty(&self) -> Option<u32> {
        self.inner.dispatch_error_ty
    }

    /// Return the type registry embedded within the metadata.
    pub fn types(&self) -> &PortableRegistry {
        &self.inner.metadata.types
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
    pub fn storage_hash(
        &self,
        pallet: &str,
        storage: &str,
    ) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_storage_hashes
            .get_or_insert(pallet, storage, || {
                subxt_metadata::get_storage_hash(&self.inner.metadata, pallet, storage)
                    .map_err(|e| {
                        match e {
                            subxt_metadata::NotFound::Pallet => {
                                MetadataError::PalletNotFound
                            }
                            subxt_metadata::NotFound::Item => {
                                MetadataError::StorageNotFound
                            }
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
    pub fn call_hash(
        &self,
        pallet: &str,
        function: &str,
    ) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_call_hashes
            .get_or_insert(pallet, function, || {
                subxt_metadata::get_call_hash(&self.inner.metadata, pallet, function)
                    .map_err(|e| {
                        match e {
                            subxt_metadata::NotFound::Pallet => {
                                MetadataError::PalletNotFound
                            }
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
    call_indexes: HashMap<String, u8>,
    call_ty_id: Option<u32>,
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

    /// If calls exist for this pallet, this returns the type ID of the variant
    /// representing the different possible calls.
    pub fn call_ty_id(&self) -> Option<u32> {
        self.call_ty_id
    }

    /// Attempt to resolve a call into an index in this pallet, failing
    /// if the call is not found in this pallet.
    pub fn call_index(&self, function: &str) -> Result<u8, MetadataError> {
        let fn_index = *self
            .call_indexes
            .get(function)
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
    // The pallet name is shared across every event, so put it
    // behind an Arc to avoid lots of needless clones of it existing.
    pallet: Arc<str>,
    event: String,
    fields: Vec<(Option<String>, u32)>,
    docs: Vec<String>,
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

    /// The names and types of each field in the event.
    pub fn fields(&self) -> &[(Option<String>, u32)] {
        &self.fields
    }

    /// Documentation for this event.
    pub fn docs(&self) -> &[String] {
        &self.docs
    }
}

/// Details about a specific runtime error.
#[derive(Clone, Debug)]
pub struct ErrorMetadata {
    // The pallet name is shared across every event, so put it
    // behind an Arc to avoid lots of needless clones of it existing.
    pallet: Arc<str>,
    error: String,
    docs: Vec<String>,
}

impl ErrorMetadata {
    /// Get the name of the pallet from which the error originates.
    pub fn pallet(&self) -> &str {
        &self.pallet
    }

    /// The name of the error.
    pub fn error(&self) -> &str {
        &self.error
    }

    /// Documentation for the error.
    pub fn docs(&self) -> &[String] {
        &self.docs
    }
}

/// Error originated from converting a runtime metadata [RuntimeMetadataPrefixed] to
/// the internal [Metadata] representation.
#[derive(Debug, thiserror::Error)]
pub enum InvalidMetadataError {
    /// Invalid prefix
    #[error("Invalid prefix")]
    InvalidPrefix,
    /// Invalid version
    #[error("Invalid version")]
    InvalidVersion,
    /// Type missing from type registry
    #[error("Type {0} missing from type registry")]
    MissingType(u32),
    /// Type was not a variant/enum type
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
                let call_ty_id = pallet.calls.as_ref().map(|c| c.ty.id());

                let call_indexes =
                    pallet.calls.as_ref().map_or(Ok(HashMap::new()), |call| {
                        let type_def_variant = get_type_def_variant(call.ty.id())?;
                        let call_indexes = type_def_variant
                            .variants()
                            .iter()
                            .map(|v| (v.name().clone(), v.index()))
                            .collect();
                        Ok(call_indexes)
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
                    call_indexes,
                    call_ty_id,
                    storage,
                    constants,
                };

                Ok((pallet.name.to_string(), pallet_metadata))
            })
            .collect::<Result<_, _>>()?;

        let mut events = HashMap::<(u8, u8), EventMetadata>::new();
        for pallet in &metadata.pallets {
            if let Some(event) = &pallet.event {
                let pallet_name: Arc<str> = pallet.name.to_string().into();
                let event_type_id = event.ty.id();
                let event_variant = get_type_def_variant(event_type_id)?;
                for variant in event_variant.variants() {
                    events.insert(
                        (pallet.index, variant.index()),
                        EventMetadata {
                            pallet: pallet_name.clone(),
                            event: variant.name().to_owned(),
                            fields: variant
                                .fields()
                                .iter()
                                .map(|f| (f.name().map(|n| n.to_owned()), f.ty().id()))
                                .collect(),
                            docs: variant.docs().to_vec(),
                        },
                    );
                }
            }
        }

        let mut errors = HashMap::<(u8, u8), ErrorMetadata>::new();
        for pallet in &metadata.pallets {
            if let Some(error) = &pallet.error {
                let pallet_name: Arc<str> = pallet.name.to_string().into();
                let error_variant = get_type_def_variant(error.ty.id())?;
                for variant in error_variant.variants() {
                    errors.insert(
                        (pallet.index, variant.index()),
                        ErrorMetadata {
                            pallet: pallet_name.clone(),
                            error: variant.name().clone(),
                            docs: variant.docs().to_vec(),
                        },
                    );
                }
            }
        }

        let dispatch_error_ty = metadata
            .types
            .types()
            .iter()
            .find(|ty| ty.ty().path().segments() == ["sp_runtime", "DispatchError"])
            .map(|ty| ty.id());

        Ok(Metadata {
            inner: Arc::new(MetadataInner {
                metadata,
                pallets,
                events,
                errors,
                dispatch_error_ty,
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

        let hash = metadata.call_hash("System", "fill_block");

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
        let hash = metadata.storage_hash("System", "Account");

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
