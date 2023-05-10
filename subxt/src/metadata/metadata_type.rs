// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::{collections::HashMap, convert::TryFrom, sync::Arc};

use codec::Error as CodecError;
use frame_metadata::{
    v15::{PalletConstantMetadata, RuntimeMetadataV15, StorageEntryMetadata},
    RuntimeMetadata, RuntimeMetadataPrefixed, META_RESERVED,
};
use parking_lot::RwLock;
use scale_info::{form::PortableForm, PortableRegistry, Type};

use super::hash_cache::HashCache;

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
    /// Extrinsic is not in metadata.
    #[error("Pallet {0}, Extrinsic {0} not found")]
    ExtrinsicNotFound(u8, u8),
    /// Event is not in metadata.
    #[error("Pallet {0}, Error {0} not found")]
    ErrorNotFound(u8, u8),
    /// Runtime function is not in metadata.
    #[error("Runtime function not found")]
    RuntimeFnNotFound,
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
    /// Runtime constant metadata is incompatible with the static one.
    #[error("Pallet {0} Constant {0} has incompatible metadata")]
    IncompatibleConstantMetadata(String, String),
    /// Runtime call metadata is incompatible with the static one.
    #[error("Pallet {0} Call {0} has incompatible metadata")]
    IncompatibleCallMetadata(String, String),
    /// Runtime storage metadata is incompatible with the static one.
    #[error("Pallet {0} Storage {0} has incompatible metadata")]
    IncompatibleStorageMetadata(String, String),
    /// Runtime API metadata is incompatible with the static one.
    #[error("Runtime API Trait {0} Method {0} has incompatible metadata")]
    IncompatibleRuntimeApiMetadata(String, String),
    /// Runtime metadata is not fully compatible with the static one.
    #[error("Node metadata is not fully compatible")]
    IncompatibleMetadata,
}

// We hide the innards behind an Arc so that it's easy to clone and share.
#[derive(Debug)]
struct MetadataInner {
    metadata: RuntimeMetadataV15,

    // Events are hashed by pallet an error index (decode oriented)
    events: HashMap<(u8, u8), EventMetadata>,
    // Extrinsics are hashed by pallet an error index (decode oriented)
    extrinsics: HashMap<(u8, u8), ExtrinsicMetadata>,
    // Errors are hashed by pallet and error index (decode oriented)
    errors: HashMap<(u8, u8), ErrorMetadata>,

    // Other pallet details are hashed by pallet name.
    pallets: HashMap<String, PalletMetadata>,

    // Type of the DispatchError type, which is what comes back if
    // an extrinsic fails.
    dispatch_error_ty: Option<u32>,

    // Runtime API metadata
    runtime_apis: HashMap<String, RuntimeFnMetadata>,

    // The hashes uniquely identify parts of the metadata; different
    // hashes mean some type difference exists between static and runtime
    // versions. We cache them here to avoid recalculating:
    cached_metadata_hash: RwLock<Option<[u8; 32]>>,
    cached_call_hashes: HashCache,
    cached_constant_hashes: HashCache,
    cached_storage_hashes: HashCache,
    cached_runtime_hashes: HashCache,
}

/// A representation of the runtime metadata received from a node.
#[derive(Clone, Debug)]
pub struct Metadata {
    inner: Arc<MetadataInner>,
}

impl Metadata {
    /// Returns a reference to [`RuntimeFnMetadata`].
    pub fn runtime_fn(&self, name: &str) -> Result<&RuntimeFnMetadata, MetadataError> {
        self.inner
            .runtime_apis
            .get(name)
            .ok_or(MetadataError::RuntimeFnNotFound)
    }

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

    /// Returns the metadata for the extrinsic at the given pallet and call indices.
    pub fn extrinsic(
        &self,
        pallet_index: u8,
        call_index: u8,
    ) -> Result<&ExtrinsicMetadata, MetadataError> {
        let event = self
            .inner
            .extrinsics
            .get(&(pallet_index, call_index))
            .ok_or(MetadataError::ExtrinsicNotFound(pallet_index, call_index))?;
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
    pub fn runtime_metadata(&self) -> &RuntimeMetadataV15 {
        &self.inner.metadata
    }

    /// Obtain the unique hash for a specific storage entry.
    pub fn storage_hash(&self, pallet: &str, storage: &str) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_storage_hashes
            .get_or_insert(pallet, storage, || {
                subxt_metadata::get_storage_hash(&self.inner.metadata, pallet, storage).map_err(
                    |e| match e {
                        subxt_metadata::NotFound::Root => MetadataError::PalletNotFound,
                        subxt_metadata::NotFound::Item => MetadataError::StorageNotFound,
                    },
                )
            })
    }

    /// Obtain the unique hash for a constant.
    pub fn constant_hash(&self, pallet: &str, constant: &str) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_constant_hashes
            .get_or_insert(pallet, constant, || {
                subxt_metadata::get_constant_hash(&self.inner.metadata, pallet, constant).map_err(
                    |e| match e {
                        subxt_metadata::NotFound::Root => MetadataError::PalletNotFound,
                        subxt_metadata::NotFound::Item => MetadataError::ConstantNotFound,
                    },
                )
            })
    }

    /// Obtain the unique hash for a call.
    pub fn call_hash(&self, pallet: &str, function: &str) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_call_hashes
            .get_or_insert(pallet, function, || {
                subxt_metadata::get_call_hash(&self.inner.metadata, pallet, function).map_err(|e| {
                    match e {
                        subxt_metadata::NotFound::Root => MetadataError::PalletNotFound,
                        subxt_metadata::NotFound::Item => MetadataError::CallNotFound,
                    }
                })
            })
    }

    /// Obtain the unique hash for a runtime API function.
    pub fn runtime_api_hash(
        &self,
        trait_name: &str,
        method_name: &str,
    ) -> Result<[u8; 32], MetadataError> {
        self.inner
            .cached_runtime_hashes
            .get_or_insert(trait_name, method_name, || {
                subxt_metadata::get_runtime_api_hash(&self.inner.metadata, trait_name, method_name)
                    .map_err(|_| MetadataError::RuntimeFnNotFound)
            })
    }

    /// Obtain the unique hash for this metadata.
    pub fn metadata_hash<T: AsRef<str>>(&self, pallets: &[T]) -> [u8; 32] {
        if let Some(hash) = *self.inner.cached_metadata_hash.read() {
            return hash;
        }

        let hash = subxt_metadata::get_metadata_per_pallet_hash(self.runtime_metadata(), pallets);
        *self.inner.cached_metadata_hash.write() = Some(hash);

        hash
    }
}

/// Metadata for a specific runtime API function.
#[derive(Clone, Debug)]
pub struct RuntimeFnMetadata {
    /// The trait name of the runtime function.
    trait_name: String,
    /// The method name of the runtime function.
    method_name: String,
    /// The parameter name and type IDs interpreted as `scale_info::Field`
    /// for ease of decoding.
    fields: Vec<scale_info::Field<scale_info::form::PortableForm>>,
    /// The type ID of the return type.
    return_id: u32,
}

impl RuntimeFnMetadata {
    /// Get the parameters as fields.
    pub fn fields(&self) -> &[scale_info::Field<scale_info::form::PortableForm>] {
        &self.fields
    }

    /// Return the trait name of the runtime function.
    pub fn trait_name(&self) -> &str {
        &self.trait_name
    }

    /// Return the method name of the runtime function.
    pub fn method_name(&self) -> &str {
        &self.method_name
    }

    /// Get the type ID of the return type.
    pub fn return_id(&self) -> u32 {
        self.return_id
    }
}

/// Metadata for a specific pallet.
#[derive(Clone, Debug)]
pub struct PalletMetadata {
    index: u8,
    name: String,
    call_metadata: HashMap<String, CallMetadata>,
    call_ty_id: Option<u32>,
    event_ty_id: Option<u32>,
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

    /// If events exist for this pallet, this returns the type ID of the variant
    /// representing the different possible events.
    pub fn event_ty_id(&self) -> Option<u32> {
        self.event_ty_id
    }

    /// Attempt to resolve a call into an index in this pallet, failing
    /// if the call is not found in this pallet.
    pub fn call(&self, function: &str) -> Result<&CallMetadata, MetadataError> {
        let fn_index = self
            .call_metadata
            .get(function)
            .ok_or(MetadataError::CallNotFound)?;
        Ok(fn_index)
    }

    /// Return [`StorageEntryMetadata`] given some storage key.
    pub fn storage(&self, key: &str) -> Result<&StorageEntryMetadata<PortableForm>, MetadataError> {
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

#[derive(Clone, Debug)]
pub struct CallMetadata {
    call_index: u8,
    fields: Vec<scale_info::Field<scale_info::form::PortableForm>>,
}

impl CallMetadata {
    /// Index of this call.
    pub fn index(&self) -> u8 {
        self.call_index
    }

    /// The names, type names & types of each field in the call data.
    pub fn fields(&self) -> &[scale_info::Field<scale_info::form::PortableForm>] {
        &self.fields
    }
}

/// Metadata for specific events.
#[derive(Clone, Debug)]
pub struct EventMetadata {
    // The pallet name is shared across every event, so put it
    // behind an Arc to avoid lots of needless clones of it existing.
    pallet: Arc<str>,
    event: String,
    fields: Vec<scale_info::Field<scale_info::form::PortableForm>>,
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

    /// The names, type names & types of each field in the event.
    pub fn fields(&self) -> &[scale_info::Field<scale_info::form::PortableForm>] {
        &self.fields
    }

    /// Documentation for this event.
    pub fn docs(&self) -> &[String] {
        &self.docs
    }
}

/// Metadata for specific extrinsics.
#[derive(Clone, Debug)]
pub struct ExtrinsicMetadata {
    // The pallet name is shared across every extrinsic, so put it
    // behind an Arc to avoid lots of needless clones of it existing.
    pallet: Arc<str>,
    call: String,
    fields: Vec<scale_info::Field<scale_info::form::PortableForm>>,
    docs: Vec<String>,
}

impl ExtrinsicMetadata {
    /// Get the name of the pallet from which the extrinsic was emitted.
    pub fn pallet(&self) -> &str {
        &self.pallet
    }

    /// Get the name of the extrinsic call.
    pub fn call(&self) -> &str {
        &self.call
    }

    /// The names, type names & types of each field in the extrinsic.
    pub fn fields(&self) -> &[scale_info::Field<scale_info::form::PortableForm>] {
        &self.fields
    }

    /// Documentation for this extrinsic.
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
    /// Type missing extrinsic "Call" type
    #[error("Missing extrinsic Call type")]
    MissingCallType,
    /// The extrinsic variant expected to contain a single field.
    #[error("Extrinsic variant at index {0} expected to contain a single field")]
    InvalidExtrinsicVariant(u8),
    /// Type was not a variant/enum type
    #[error("Type {0} was not a variant/enum type")]
    TypeDefNotVariant(u32),
}

impl TryFrom<RuntimeMetadataPrefixed> for Metadata {
    type Error = InvalidMetadataError;

    fn try_from(metadata: RuntimeMetadataPrefixed) -> Result<Self, Self::Error> {
        if metadata.0 != META_RESERVED {
            return Err(InvalidMetadataError::InvalidPrefix);
        }
        let metadata = match metadata.1 {
            RuntimeMetadata::V14(v14) => subxt_metadata::metadata_v14_to_latest(v14),
            RuntimeMetadata::V15(v15) => v15,
            _ => return Err(InvalidMetadataError::InvalidVersion),
        };

        let runtime_apis: HashMap<String, RuntimeFnMetadata> = metadata
            .apis
            .iter()
            .flat_map(|trait_metadata| {
                let trait_name = &trait_metadata.name;

                trait_metadata
                    .methods
                    .iter()
                    .map(|method_metadata| {
                        // Function named used by substrate to identify the runtime call.
                        let fn_name = format!("{}_{}", trait_name, method_metadata.name);

                        // Parameters mapped as `scale_info::Field` to allow dynamic decoding.
                        let fields: Vec<_> = method_metadata
                            .inputs
                            .iter()
                            .map(|input| {
                                let name = input.name.clone();
                                let ty = input.ty.id;
                                scale_info::Field {
                                    name: Some(name),
                                    ty: ty.into(),
                                    type_name: None,
                                    docs: Default::default(),
                                }
                            })
                            .collect();

                        let return_id = method_metadata.output.id;
                        let metadata = RuntimeFnMetadata {
                            fields,
                            return_id,
                            trait_name: trait_name.clone(),
                            method_name: method_metadata.name.clone(),
                        };

                        (fn_name, metadata)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let get_type_def_variant = |type_id: u32| {
            let ty = metadata
                .types
                .resolve(type_id)
                .ok_or(InvalidMetadataError::MissingType(type_id))?;
            if let scale_info::TypeDef::Variant(var) = &ty.type_def {
                Ok(var)
            } else {
                Err(InvalidMetadataError::TypeDefNotVariant(type_id))
            }
        };
        let pallets = metadata
            .pallets
            .iter()
            .map(|pallet| {
                let call_ty_id = pallet.calls.as_ref().map(|c| c.ty.id);
                let event_ty_id = pallet.event.as_ref().map(|e| e.ty.id);

                let call_metadata = pallet.calls.as_ref().map_or(Ok(HashMap::new()), |call| {
                    let type_def_variant = get_type_def_variant(call.ty.id)?;
                    let call_indexes = type_def_variant
                        .variants
                        .iter()
                        .map(|v| {
                            (
                                v.name.clone(),
                                CallMetadata {
                                    call_index: v.index,
                                    fields: v.fields.clone(),
                                },
                            )
                        })
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
                    call_metadata,
                    call_ty_id,
                    event_ty_id,
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
                let event_type_id = event.ty.id;
                let event_variant = get_type_def_variant(event_type_id)?;
                for variant in &event_variant.variants {
                    events.insert(
                        (pallet.index, variant.index),
                        EventMetadata {
                            pallet: pallet_name.clone(),
                            event: variant.name.clone(),
                            fields: variant.fields.clone(),
                            docs: variant.docs.clone(),
                        },
                    );
                }
            }
        }

        let mut errors = HashMap::<(u8, u8), ErrorMetadata>::new();
        for pallet in &metadata.pallets {
            if let Some(error) = &pallet.error {
                let pallet_name: Arc<str> = pallet.name.to_string().into();
                let error_variant = get_type_def_variant(error.ty.id)?;
                for variant in &error_variant.variants {
                    errors.insert(
                        (pallet.index, variant.index),
                        ErrorMetadata {
                            pallet: pallet_name.clone(),
                            error: variant.name.clone(),
                            docs: variant.docs.clone(),
                        },
                    );
                }
            }
        }

        let dispatch_error_ty = metadata
            .types
            .types
            .iter()
            .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
            .map(|ty| ty.id);

        let extrinsic_ty = metadata
            .types
            .resolve(metadata.extrinsic.ty.id)
            .ok_or(InvalidMetadataError::MissingType(metadata.extrinsic.ty.id))?;

        let Some(call_id) = extrinsic_ty.type_params
            .iter()
            .find(|ty| ty.name == "Call")
            .and_then(|ty| ty.ty)
            .map(|ty| ty.id) else {
            return Err(InvalidMetadataError::MissingCallType);
        };

        let call_type_variants = get_type_def_variant(call_id)?;

        let mut extrinsics = HashMap::<(u8, u8), ExtrinsicMetadata>::new();
        for variant in &call_type_variants.variants {
            let pallet_name: Arc<str> = variant.name.to_string().into();
            let pallet_index = variant.index;

            // Pallet variants must contain one single call variant.
            // In the following form:
            //
            // enum RuntimeCall {
            //   Pallet(pallet_call)
            // }
            if variant.fields.len() != 1 {
                return Err(InvalidMetadataError::InvalidExtrinsicVariant(pallet_index));
            }
            let Some(ty) = variant.fields.first() else {
                return Err(InvalidMetadataError::InvalidExtrinsicVariant(pallet_index));
            };

            // Get the call variant.
            let call_type_variant = get_type_def_variant(ty.ty.id)?;
            for variant in &call_type_variant.variants {
                extrinsics.insert(
                    (pallet_index, variant.index),
                    ExtrinsicMetadata {
                        pallet: pallet_name.clone(),
                        call: variant.name.to_string(),
                        fields: variant.fields.clone(),
                        docs: variant.docs.clone(),
                    },
                );
            }
        }

        Ok(Metadata {
            inner: Arc::new(MetadataInner {
                metadata,
                pallets,
                events,
                extrinsics,
                errors,
                dispatch_error_ty,
                runtime_apis,
                cached_metadata_hash: Default::default(),
                cached_call_hashes: Default::default(),
                cached_constant_hashes: Default::default(),
                cached_storage_hashes: Default::default(),
                cached_runtime_hashes: Default::default(),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use frame_metadata::v15::{
        ExtrinsicMetadata, PalletCallMetadata, PalletMetadata, PalletStorageMetadata,
        StorageEntryModifier, StorageEntryType,
    };
    use scale_info::{meta_type, TypeInfo};

    use super::*;

    fn load_metadata() -> Metadata {
        // Extrinsic needs to contain at least the generic type parameter "Call"
        // for the metadata to be valid.
        // The "Call" type from the metadata is used to decode extrinsics.
        // In reality, the extrinsic type has "Call", "Address", "Extra", "Signature" generic types.
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct ExtrinsicType<Call> {
            call: Call,
        }
        // Because this type is used to decode extrinsics, we expect this to be a TypeDefVariant.
        // Each pallet must contain one single variant.
        #[allow(unused)]
        #[derive(TypeInfo)]
        enum RuntimeCall {
            PalletName(Pallet),
        }
        // The calls of the pallet.
        #[allow(unused)]
        #[derive(TypeInfo)]
        enum Pallet {
            #[allow(unused)]
            SomeCall,
        }

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
        let pallet = PalletMetadata {
            index: 0,
            name: "System",
            calls: Some(PalletCallMetadata {
                ty: meta_type::<Call>(),
            }),
            storage: Some(storage),
            constants: vec![constant],
            event: None,
            error: None,
            docs: vec![],
        };

        let metadata = RuntimeMetadataV15::new(
            vec![pallet],
            ExtrinsicMetadata {
                ty: meta_type::<ExtrinsicType<RuntimeCall>>(),
                version: 0,
                signed_extensions: vec![],
            },
            meta_type::<()>(),
            vec![],
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
