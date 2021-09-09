// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use std::{
    collections::HashMap,
    convert::TryFrom,
    marker::PhantomData,
    str::FromStr,
};

use codec::{
    Decode,
    Encode,
    Error as CodecError,
};

use frame_metadata::{
    PalletConstantMetadata,
    PalletEventMetadata,
    RuntimeMetadata,
    RuntimeMetadataLastVersion,
    RuntimeMetadataPrefixed,
    StorageEntryModifier,
    StorageEntryType,
    StorageHasher,
    META_RESERVED,
};
use sp_core::storage::StorageKey;

use crate::{
    Call,
    Encoded,
};
use scale_info::{
    form::{
        Form,
        PortableForm,
    },
    Variant,
};

/// Metadata error.
#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    /// Module is not in metadata.
    #[error("Module {0} not found")]
    PalletNotFound(String),
    /// Module is not in metadata.
    #[error("Module index {0} not found")]
    ModuleIndexNotFound(u8),
    /// Call is not in metadata.
    #[error("Call {0} not found")]
    CallNotFound(&'static str),
    /// Event is not in metadata.
    #[error("Pallet {0}, Event {0} not found")]
    EventNotFound(u8, u8),
    /// Event is not in metadata.
    #[error("Error {0} not found")]
    ErrorNotFound(u8),
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
}

/// Runtime metadata.
#[derive(Clone, Debug)]
pub struct Metadata {
    metadata: RuntimeMetadataLastVersion,
    pallets: HashMap<String, PalletMetadata>,
    events: HashMap<(u8, u8), EventMetadata>,
}

impl Metadata {
    /// Returns `PalletMetadata`.
    pub fn pallet(&self, name: &'static str) -> Result<&PalletMetadata, MetadataError> {
        self.pallets
            .get(name)
            .ok_or(MetadataError::PalletNotFound(name.to_string()))
    }

    /// Returns the variant for the event at the given pallet and event indices.
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
}

#[derive(Clone, Debug)]
pub struct PalletMetadata {
    index: u8,
    name: String,
    calls: HashMap<String, u8>,
    storage: HashMap<String, StorageMetadata>,
    constants: HashMap<String, PalletConstantMetadata<PortableForm>>,
}

impl PalletMetadata {
    pub fn encode_call<C>(&self, call: C) -> Result<Encoded, MetadataError>
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

    pub fn storage(&self, key: &'static str) -> Result<&StorageMetadata, MetadataError> {
        self.storage
            .get(key)
            .ok_or(MetadataError::StorageNotFound(key))
    }

    /// Get a constant's metadata by name
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
}

#[derive(Clone, Debug)]
pub struct StorageMetadata {
    module_prefix: String,
    storage_prefix: String,
    modifier: StorageEntryModifier,
    ty: StorageEntryType<PortableForm>,
    default: Vec<u8>,
}

impl StorageMetadata {
    pub fn default<V: Decode>(&self) -> Result<V, MetadataError> {
        Decode::decode(&mut &self.default[..]).map_err(MetadataError::DefaultError)
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
            return Err(InvalidMetadataError::InvalidPrefix.into())
        }
        let metadata = match metadata.1 {
            RuntimeMetadata::V14(meta) => meta,
            _ => return Err(InvalidMetadataError::InvalidVersion.into()),
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

                let pallet_metadata = PalletMetadata {
                    index: pallet.index,
                    name: pallet.name.to_string(),
                    calls,
                    storage: Default::default(),
                    constants: Default::default(),
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
        Ok(Self {
            metadata,
            pallets,
            events,
        })
    }
}
