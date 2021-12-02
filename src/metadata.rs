// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use std::{
    collections::HashMap,
    convert::TryFrom,
};

use codec::Error as CodecError;

use frame_metadata::{
    PalletConstantMetadata,
    RuntimeMetadata,
    RuntimeMetadataLastVersion,
    RuntimeMetadataPrefixed,
    StorageEntryMetadata,
    META_RESERVED,
};

use crate::{
    Call,
    Encoded,
};
use scale_info::{
    form::PortableForm,
    Type,
    Variant,
};

/// Metadata error.
#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    /// Module is not in metadata.
    #[error("Pallet {0} not found")]
    PalletNotFound(String),
    /// Pallet is not in metadata.
    #[error("Pallet index {0} not found")]
    PalletIndexNotFound(u8),
    /// Call is not in metadata.
    #[error("Call {0} not found")]
    CallNotFound(&'static str),
    /// Event is not in metadata.
    #[error("Pallet {0}, Event {0} not found")]
    EventNotFound(u8, u8),
    /// Event is not in metadata.
    #[error("Pallet {0}, Error {0} not found")]
    ErrorNotFound(u8, u8),
    /// Storage is not in metadata.
    #[error("Storage {0} not found")]
    StorageNotFound(&'static str),
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
    #[error("Constant {0} not found")]
    ConstantNotFound(&'static str),
    /// Type is not in metadata.
    #[error("Type {0} missing from type registry")]
    TypeNotFound(u32),
}

/// Runtime metadata.
#[derive(Clone, Debug)]
pub struct Metadata {
    metadata: RuntimeMetadataLastVersion,
    pallets: HashMap<String, PalletMetadata>,
    events: HashMap<(u8, u8), EventMetadata>,
    errors: HashMap<(u8, u8), ErrorMetadata>,
}

impl Metadata {
    /// Returns a reference to [`PalletMetadata`].
    pub fn pallet(&self, name: &'static str) -> Result<&PalletMetadata, MetadataError> {
        self.pallets
            .get(name)
            .ok_or_else(|| MetadataError::PalletNotFound(name.to_string()))
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

    /// Encode a call based on this pallet metadata.
    pub fn encode_call<C>(&self, call: &C) -> Result<Encoded, MetadataError>
    where
        C: Call,
    {
        let fn_index = self
            .calls
            .get(C::FUNCTION)
            .ok_or(MetadataError::CallNotFound(C::FUNCTION))?;
        let mut bytes = vec![self.index, *fn_index];
        bytes.extend(call.encode());
        Ok(Encoded(bytes))
    }

    /// Return [`StorageEntryMetadata`] given some storage key.
    pub fn storage(
        &self,
        key: &'static str,
    ) -> Result<&StorageEntryMetadata<PortableForm>, MetadataError> {
        self.storage
            .get(key)
            .ok_or(MetadataError::StorageNotFound(key))
    }

    /// Get a constant's metadata by name.
    pub fn constant(
        &self,
        key: &'static str,
    ) -> Result<&PalletConstantMetadata<PortableForm>, MetadataError> {
        self.constants
            .get(key)
            .ok_or(MetadataError::ConstantNotFound(key))
    }
}

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
        })
    }
}
