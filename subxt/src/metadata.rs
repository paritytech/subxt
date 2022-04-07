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

use codec::Error as CodecError;
use std::{
    collections::HashMap,
    convert::TryFrom,
    sync::{
        Arc,
        Mutex,
    },
};

use frame_metadata::{
    PalletConstantMetadata,
    RuntimeMetadata,
    RuntimeMetadataLastVersion,
    RuntimeMetadataPrefixed,
    StorageEntryMetadata,
    META_RESERVED,
};

use crate::Call;
use scale_info::{
    form::PortableForm,
    Type,
    Variant,
};

/// Metadata error.
#[derive(Debug, thiserror::Error)]
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

/// Runtime metadata.
#[derive(Clone, Debug)]
pub struct Metadata {
    metadata: RuntimeMetadataLastVersion,
    pallets: HashMap<String, PalletMetadata>,
    events: HashMap<(u8, u8), EventMetadata>,
    errors: HashMap<(u8, u8), ErrorMetadata>,
    cache: Arc<Mutex<subxt_metadata::MetadataHashDetails>>,
}

impl Metadata {
    /// Returns a reference to [`PalletMetadata`].
    pub fn pallet(&self, name: &'static str) -> Result<&PalletMetadata, MetadataError> {
        self.pallets
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
            .errors
            .get(&(pallet_index, error_index))
            .ok_or(MetadataError::ErrorNotFound(pallet_index, error_index))?;
        Ok(error)
    }

    /// Resolve a type definition.
    pub fn resolve_type(&self, id: u32) -> Option<&Type<PortableForm>> {
        self.metadata.types.resolve(id)
    }

    /// Return the runtime metadata.
    pub fn runtime_metadata(&self) -> &RuntimeMetadataLastVersion {
        &self.metadata
    }

    /// Obtain the unique hash for a specific storage entry.
    pub fn storage_hash<S: crate::StorageEntry>(&self) -> Result<[u8; 32], MetadataError> {
        subxt_metadata::get_storage_hash(&self.metadata, S::PALLET, S::STORAGE)
            .map_err(|e| {
                match e {
                    subxt_metadata::NotFound::Pallet => MetadataError::PalletNotFound,
                    subxt_metadata::NotFound::Item => MetadataError::StorageNotFound,
                }
            })
    }

    /// Obtain the unique hash for a constant.
    pub fn constant_hash(&self, pallet_name: &str, constant_name: &str) -> Result<[u8; 32], MetadataError> {
        subxt_metadata::get_constant_hash(&self.metadata, pallet_name,  constant_name)
            .map_err(|e| {
                match e {
                    subxt_metadata::NotFound::Pallet => MetadataError::PalletNotFound,
                    subxt_metadata::NotFound::Item => MetadataError::ConstantNotFound,
                }
            })
    }

    /// Obtain the unique hash for a call.
    pub fn call_hash<C: crate::Call>(&self) -> Result<[u8; 32], MetadataError> {
        subxt_metadata::get_call_hash(&self.metadata, C::PALLET, C::FUNCTION)
            .map_err(|e| {
                match e {
                    subxt_metadata::NotFound::Pallet => MetadataError::PalletNotFound,
                    subxt_metadata::NotFound::Item => MetadataError::CallNotFound,
                }
            })
    }

    /// Obtain the unique hash for a pallet.
    pub fn pallet_hash(&self, name: &str) -> Result<[u8; 32], MetadataError> {
        if let Some(cache) = self.cache.lock().unwrap().pallet_hashes.get(name) {
            return Ok(*cache)
        }

        let metadata = self.runtime_metadata();
        let pallet = match metadata.pallets.iter().find(|pallet| pallet.name == name) {
            Some(pallet) => pallet,
            _ => return Err(MetadataError::PalletNotFound),
        };

        let hash = subxt_metadata::get_pallet_hash(&metadata.types, pallet);
        let mut cache = self.cache.lock().unwrap();
        cache.pallet_hashes.insert(name.to_string(), hash);
        Ok(hash)
    }

    /// Obtain the unique hash for this metadata.
    pub fn metadata_hash<T: AsRef<str>>(&self, pallets: &[T]) -> [u8; 32] {
        let hash = subxt_metadata::get_metadata_per_pallet_hash(
            self.runtime_metadata(),
            pallets,
        );
        let mut cache = self.cache.lock().unwrap();
        *cache = hash;
        cache.metadata_hash
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
        self.storage
            .get(key)
            .ok_or(MetadataError::StorageNotFound)
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

/// Metadata for specific errors.
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

        Ok(Self {
            metadata,
            pallets,
            events,
            errors,
            cache: Arc::new(Mutex::new(subxt_metadata::MetadataHashDetails::new())),
        })
    }
}
