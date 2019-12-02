// Copyright 2019 Parity Technologies (UK) Ltd.
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

use parity_scale_codec::{
    Decode,
    Encode,
};

use frame_metadata::{
    DecodeDifferent,
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
    StorageEntryModifier,
    StorageEntryType,
    StorageHasher,
    META_RESERVED,
};
use sp_core::storage::StorageKey;

use crate::codec::Encoded;

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("Module not found")]
    ModuleNotFound(String),
    #[error("Module with events not found")]
    ModuleWithEventsNotFound(u8),
    #[error("Call not found")]
    CallNotFound(&'static str),
    #[error("Event not found")]
    EventNotFound(u8),
    #[error("Storage not found")]
    StorageNotFound(&'static str),
    #[error("Storage type error")]
    StorageTypeError,
    #[error("Map value type error")]
    MapValueTypeError,
}

#[derive(Clone, Debug)]
pub struct Metadata {
    modules: HashMap<String, ModuleMetadata>,
    modules_with_calls: HashMap<String, ModuleWithCalls>,
    modules_with_events: HashMap<String, ModuleWithEvents>,
}

impl Metadata {
    pub fn module<S>(&self, name: S) -> Result<&ModuleMetadata, MetadataError>
    where
        S: ToString,
    {
        let name = name.to_string();
        self.modules
            .get(&name)
            .ok_or(MetadataError::ModuleNotFound(name))
    }

    pub fn module_with_calls<S>(&self, name: S) -> Result<&ModuleWithCalls, MetadataError>
    where
        S: ToString,
    {
        let name = name.to_string();
        self.modules_with_calls
            .get(&name)
            .ok_or(MetadataError::ModuleNotFound(name))
    }

    pub fn modules_with_events(&self) -> impl Iterator<Item = &ModuleWithEvents> {
        self.modules_with_events.values()
    }

    pub fn module_with_events(
        &self,
        module_index: u8,
    ) -> Result<&ModuleWithEvents, MetadataError> {
        self.modules_with_events
            .values()
            .find(|&module| module.index == module_index)
            .ok_or(MetadataError::ModuleWithEventsNotFound(module_index))
    }

    pub fn pretty(&self) -> String {
        let mut string = String::new();
        for (name, module) in &self.modules {
            string.push_str(name.as_str());
            string.push('\n');
            for storage in module.storage.keys() {
                string.push_str(" s  ");
                string.push_str(storage.as_str());
                string.push('\n');
            }
            if let Some(module) = self.modules_with_calls.get(name) {
                for call in module.calls.keys() {
                    string.push_str(" c  ");
                    string.push_str(call.as_str());
                    string.push('\n');
                }
            }
            if let Some(module) = self.modules_with_events.get(name) {
                for event in module.events.values() {
                    string.push_str(" e  ");
                    string.push_str(event.name.as_str());
                    string.push('\n');
                }
            }
        }
        string
    }
}

#[derive(Clone, Debug)]
pub struct ModuleMetadata {
    name: String,
    storage: HashMap<String, StorageMetadata>,
    // constants
}

impl ModuleMetadata {
    pub fn storage(&self, key: &'static str) -> Result<&StorageMetadata, MetadataError> {
        self.storage
            .get(key)
            .ok_or(MetadataError::StorageNotFound(key))
    }
}

#[derive(Clone, Debug)]
pub struct ModuleWithCalls {
    index: u8,
    calls: HashMap<String, u8>,
}

impl ModuleWithCalls {
    pub fn call<T: Encode>(
        &self,
        function: &'static str,
        params: T,
    ) -> Result<Encoded, MetadataError> {
        let fn_index = self
            .calls
            .get(function)
            .ok_or(MetadataError::CallNotFound(function))?;
        let mut bytes = vec![self.index, *fn_index];
        bytes.extend(params.encode());
        Ok(Encoded(bytes))
    }
}

#[derive(Clone, Debug)]
pub struct ModuleWithEvents {
    index: u8,
    name: String,
    events: HashMap<u8, ModuleEventMetadata>,
}

impl ModuleWithEvents {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn events(&self) -> impl Iterator<Item = &ModuleEventMetadata> {
        self.events.values()
    }

    pub fn event(&self, index: u8) -> Result<&ModuleEventMetadata, MetadataError> {
        self.events
            .get(&index)
            .ok_or(MetadataError::EventNotFound(index))
    }
}

#[derive(Clone, Debug)]
pub struct StorageMetadata {
    module_prefix: String,
    storage_prefix: String,
    modifier: StorageEntryModifier,
    ty: StorageEntryType,
    default: Vec<u8>,
}

impl StorageMetadata {
    pub fn get_map<K: Encode, V: Decode + Clone>(
        &self,
    ) -> Result<StorageMap<K, V>, MetadataError> {
        match &self.ty {
            StorageEntryType::Map { hasher, .. } => {
                let module_prefix = self.module_prefix.as_bytes().to_vec();
                let storage_prefix = self.storage_prefix.as_bytes().to_vec();
                let hasher = hasher.to_owned();
                let default = Decode::decode(&mut &self.default[..])
                    .map_err(|_| MetadataError::MapValueTypeError)?;
                Ok(StorageMap {
                    _marker: PhantomData,
                    module_prefix,
                    storage_prefix,
                    hasher,
                    default,
                })
            }
            _ => Err(MetadataError::StorageTypeError),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StorageMap<K, V> {
    _marker: PhantomData<K>,
    module_prefix: Vec<u8>,
    storage_prefix: Vec<u8>,
    hasher: StorageHasher,
    default: V,
}

impl<K: Encode, V: Decode + Clone> StorageMap<K, V> {
    pub fn key(&self, key: K) -> StorageKey {
        let mut bytes = sp_core::twox_128(&self.module_prefix).to_vec();
        bytes.extend(&sp_core::twox_128(&self.storage_prefix)[..]);
        let encoded_key = key.encode();
        let hash = match self.hasher {
            StorageHasher::Blake2_128 => {
                sp_core::blake2_128(&encoded_key).to_vec()
            }
            StorageHasher::Blake2_256 => {
                sp_core::blake2_256(&encoded_key).to_vec()
            }
            StorageHasher::Twox128 => sp_core::twox_128(&encoded_key).to_vec(),
            StorageHasher::Twox256 => sp_core::twox_256(&encoded_key).to_vec(),
            StorageHasher::Twox64Concat => sp_core::twox_64(&encoded_key).to_vec(),
        };
        bytes.extend(hash);
        StorageKey(bytes)
    }

    pub fn default(&self) -> V {
        self.default.clone()
    }
}

#[derive(Clone, Debug)]
pub struct ModuleEventMetadata {
    pub name: String,
    arguments: Vec<EventArg>,
}

impl ModuleEventMetadata {
    pub fn arguments(&self) -> Vec<EventArg> {
        self.arguments.to_vec()
    }
}

/// Naive representation of event argument types, supports current set of substrate EventArg types.
/// If and when Substrate uses `type-metadata`, this can be replaced.
///
/// Used to calculate the size of a instance of an event variant without having the concrete type,
/// so the raw bytes can be extracted from the encoded `Vec<EventRecord<E>>` (without `E` defined).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EventArg {
    Primitive(String),
    Vec(Box<EventArg>),
    Tuple(Vec<EventArg>),
}

impl FromStr for EventArg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Vec<") {
            if s.ends_with('>') {
                Ok(EventArg::Vec(Box::new(s[4..s.len() - 1].parse()?)))
            } else {
                Err(Error::InvalidEventArg(
                    s.to_string(),
                    "Expected closing `>` for `Vec`",
                ))
            }
        } else if s.starts_with('(') {
            if s.ends_with(')') {
                let mut args = Vec::new();
                for arg in s[1..s.len() - 1].split(',') {
                    let arg = arg.trim().parse()?;
                    args.push(arg)
                }
                Ok(EventArg::Tuple(args))
            } else {
                Err(Error::InvalidEventArg(
                    s.to_string(),
                    "Expecting closing `)` for tuple",
                ))
            }
        } else {
            Ok(EventArg::Primitive(s.to_string()))
        }
    }
}

impl EventArg {
    /// Returns all primitive types for this EventArg
    pub fn primitives(&self) -> Vec<String> {
        match self {
            EventArg::Primitive(p) => vec![p.clone()],
            EventArg::Vec(arg) => arg.primitives(),
            EventArg::Tuple(args) => {
                let mut primitives = Vec::new();
                for arg in args {
                    primitives.extend(arg.primitives())
                }
                primitives
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidPrefix,
    InvalidVersion,
    ExpectedDecoded,
    InvalidEventArg(String, &'static str),
}

impl TryFrom<RuntimeMetadataPrefixed> for Metadata {
    type Error = Error;

    fn try_from(metadata: RuntimeMetadataPrefixed) -> Result<Self, Self::Error> {
        if metadata.0 != META_RESERVED {
            return Err(Error::InvalidPrefix)
        }
        let meta = match metadata.1 {
            RuntimeMetadata::V9(meta) => meta,
            _ => return Err(Error::InvalidVersion),
        };
        let mut modules = HashMap::new();
        let mut modules_with_calls = HashMap::new();
        let mut modules_with_events = HashMap::new();
        for module in convert(meta.modules)?.into_iter() {
            let module_name = convert(module.name.clone())?;

            let mut storage_map = HashMap::new();
            if let Some(storage) = module.storage {
                let storage = convert(storage)?;
                let module_prefix = convert(storage.prefix)?;
                for entry in convert(storage.entries)?.into_iter() {
                    let storage_prefix = convert(entry.name.clone())?;
                    let entry = convert_entry(module_prefix.clone(), storage_prefix.clone(), entry)?;
                    storage_map.insert(storage_prefix, entry);
                }
            }
            modules.insert(
                module_name.clone(),
                ModuleMetadata {
                    name: module_name.clone(),
                    storage: storage_map,
                },
            );

            if let Some(calls) = module.calls {
                let mut call_map = HashMap::new();
                for (index, call) in convert(calls)?.into_iter().enumerate() {
                    let name = convert(call.name)?;
                    call_map.insert(name, index as u8);
                }
                modules_with_calls.insert(
                    module_name.clone(),
                    ModuleWithCalls {
                        index: modules_with_calls.len() as u8,
                        calls: call_map,
                    },
                );
            }
            if let Some(events) = module.event {
                let mut event_map = HashMap::new();
                for (index, event) in convert(events)?.into_iter().enumerate() {
                    event_map.insert(index as u8, convert_event(event)?);
                }
                modules_with_events.insert(
                    module_name.clone(),
                    ModuleWithEvents {
                        index: modules_with_events.len() as u8,
                        name: module_name.clone(),
                        events: event_map,
                    },
                );
            }
        }
        Ok(Metadata {
            modules,
            modules_with_calls,
            modules_with_events,
        })
    }
}

fn convert<B: 'static, O: 'static>(dd: DecodeDifferent<B, O>) -> Result<O, Error> {
    match dd {
        DecodeDifferent::Decoded(value) => Ok(value),
        _ => Err(Error::ExpectedDecoded),
    }
}

fn convert_event(
    event: frame_metadata::EventMetadata,
) -> Result<ModuleEventMetadata, Error> {
    let name = convert(event.name)?;
    let mut arguments = Vec::new();
    for arg in convert(event.arguments)? {
        let arg = arg.parse::<EventArg>()?;
        arguments.push(arg);
    }
    Ok(ModuleEventMetadata { name, arguments })
}

fn convert_entry(
    module_prefix: String,
    storage_prefix: String,
    entry: frame_metadata::StorageEntryMetadata,
) -> Result<StorageMetadata, Error> {
    let default = convert(entry.default)?;
    Ok(StorageMetadata {
        module_prefix,
        storage_prefix,
        modifier: entry.modifier,
        ty: entry.ty,
        default,
    })
}
