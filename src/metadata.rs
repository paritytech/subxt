use crate::codec::Encoded;
use parity_scale_codec::{
    Decode,
    Encode,
};
use runtime_metadata::{
    DecodeDifferent,
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
    StorageEntryModifier,
    StorageEntryType,
    StorageHasher,
    META_RESERVED,
};
use std::{
    collections::HashMap,
    convert::TryFrom,
    marker::PhantomData,
};
use substrate_primitives::storage::StorageKey;

#[derive(Debug)]
pub enum MetadataError {
    ModuleNotFound(&'static str),
    CallNotFound(&'static str),
    StorageNotFound(&'static str),
    StorageTypeError,
    MapValueTypeError,
}

#[derive(Clone, Debug)]
pub struct Metadata {
    modules: HashMap<String, ModuleMetadata>,
}

impl Metadata {
    pub fn module(&self, name: &'static str) -> Result<&ModuleMetadata, MetadataError> {
        self.modules
            .get(name)
            .ok_or(MetadataError::ModuleNotFound(name))
    }
}

#[derive(Clone, Debug)]
pub struct ModuleMetadata {
    index: Vec<u8>,
    storage: HashMap<String, StorageMetadata>,
    calls: HashMap<String, Vec<u8>>,
    events: HashMap<String, Vec<u8>>,
    // constants
}

impl ModuleMetadata {
    pub fn call<T: Encode>(
        &self,
        function: &'static str,
        params: T,
    ) -> Result<Encoded, MetadataError> {
        let fn_bytes = self
            .calls
            .get(function)
            .ok_or(MetadataError::CallNotFound(function))?;
        let mut bytes = self.index.clone();
        bytes.extend(fn_bytes);
        bytes.extend(params.encode());
        Ok(Encoded(bytes))
    }

    pub fn storage(&self, key: &'static str) -> Result<&StorageMetadata, MetadataError> {
        self.storage
            .get(key)
            .ok_or(MetadataError::StorageNotFound(key))
    }
}

#[derive(Clone, Debug)]
pub struct StorageMetadata {
    prefix: String,
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
                let prefix = self.prefix.as_bytes().to_vec();
                let hasher = hasher.to_owned();
                let default = Decode::decode(&mut &self.default[..])
                    .map_err(|_| MetadataError::MapValueTypeError)?;
                Some(StorageMap {
                    _marker: PhantomData,
                    prefix,
                    hasher,
                    default,
                })
            }
            _ => None,
        }
        .ok_or(MetadataError::StorageTypeError)
    }
}

#[derive(Clone, Debug)]
pub struct StorageMap<K, V> {
    _marker: PhantomData<K>,
    prefix: Vec<u8>,
    hasher: StorageHasher,
    default: V,
}

impl<K: Encode, V: Decode + Clone> StorageMap<K, V> {
    pub fn key(&self, key: K) -> StorageKey {
        let mut bytes = self.prefix.clone();
        bytes.extend(key.encode());
        let hash = match self.hasher {
            StorageHasher::Blake2_128 => {
                substrate_primitives::blake2_128(&bytes).to_vec()
            }
            StorageHasher::Blake2_256 => {
                substrate_primitives::blake2_256(&bytes).to_vec()
            }
            StorageHasher::Twox128 => substrate_primitives::twox_128(&bytes).to_vec(),
            StorageHasher::Twox256 => substrate_primitives::twox_256(&bytes).to_vec(),
            StorageHasher::Twox64Concat => substrate_primitives::twox_64(&bytes).to_vec(),
        };
        StorageKey(hash)
    }

    pub fn default(&self) -> V {
        self.default.clone()
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidPrefix,
    InvalidVersion,
    ExpectedDecoded,
}

impl TryFrom<RuntimeMetadataPrefixed> for Metadata {
    type Error = Error;

    fn try_from(metadata: RuntimeMetadataPrefixed) -> Result<Self, Self::Error> {
        if metadata.0 != META_RESERVED {
            Err(Error::InvalidPrefix)?;
        }
        let meta = match metadata.1 {
            RuntimeMetadata::V7(meta) => meta,
            _ => Err(Error::InvalidVersion)?,
        };
        let mut modules = HashMap::new();
        for (i, module) in convert(meta.modules)?.into_iter().enumerate() {
            modules.insert(convert(module.name.clone())?, convert_module(i, module)?);
        }
        Ok(Metadata { modules })
    }
}

fn convert<B: 'static, O: 'static>(dd: DecodeDifferent<B, O>) -> Result<O, Error> {
    match dd {
        DecodeDifferent::Decoded(value) => Ok(value),
        _ => Err(Error::ExpectedDecoded),
    }
}

fn convert_module(
    index: usize,
    module: runtime_metadata::ModuleMetadata,
) -> Result<ModuleMetadata, Error> {
    let mut storage_map = HashMap::new();
    if let Some(storage) = module.storage {
        let storage = convert(storage)?;
        let prefix = convert(storage.prefix)?;
        for entry in convert(storage.entries)?.into_iter() {
            let entry_name = convert(entry.name.clone())?;
            let entry_prefix = format!("{} {}", prefix, entry_name);
            let entry = convert_entry(entry_prefix, entry)?;
            storage_map.insert(entry_name, entry);
        }
    }
    let mut call_map = HashMap::new();
    if let Some(calls) = module.calls {
        for (index, call) in convert(calls)?.into_iter().enumerate() {
            let name = convert(call.name)?;
            call_map.insert(name, vec![index as u8]);
        }
    }
    let mut event_map = HashMap::new();
    if let Some(events) = module.event {
        for (index, event) in convert(events)?.into_iter().enumerate() {
            let name = convert(event.name)?;
            event_map.insert(name, vec![index as u8]);
        }
    }
    Ok(ModuleMetadata {
        index: vec![index as u8],
        storage: storage_map,
        calls: call_map,
        events: event_map,
    })
}

fn convert_entry(
    prefix: String,
    entry: runtime_metadata::StorageEntryMetadata,
) -> Result<StorageMetadata, Error> {
    let default = convert(entry.default)?;
    Ok(StorageMetadata {
        prefix,
        modifier: entry.modifier,
        ty: entry.ty,
        default,
    })
}
