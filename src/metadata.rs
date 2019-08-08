use parity_scale_codec::Encode;
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
};
use substrate_primitives::storage::StorageKey;

pub struct Encoded(pub Vec<u8>);

impl Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

#[derive(Clone, Debug)]
pub struct Metadata {
    modules: HashMap<String, ModuleMetadata>,
}

impl Metadata {
    pub fn module(&self, name: &str) -> Option<&ModuleMetadata> {
        self.modules.get(name)
    }
}

#[derive(Clone, Debug)]
pub struct ModuleMetadata {
    index: u8,
    storage: HashMap<String, StorageMetadata>,
    // calls, event, constants
}

impl ModuleMetadata {
    pub fn call<T: Encode>(&self, call: T) -> Encoded {
        let mut bytes = vec![self.index];
        bytes.extend(call.encode());
        Encoded(bytes)
    }

    pub fn storage(&self, key: &str) -> Option<&StorageMetadata> {
        self.storage.get(key)
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
    pub fn map(&self) -> Option<StorageMap> {
        match &self.ty {
            StorageEntryType::Map { hasher, .. } => {
                let prefix = self.prefix.as_bytes().to_vec();
                let hasher = hasher.to_owned();
                Some(StorageMap { prefix, hasher })
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StorageMap {
    prefix: Vec<u8>,
    hasher: StorageHasher,
}

impl StorageMap {
    pub fn key<K: Encode>(&self, key: K) -> StorageKey {
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
            modules.insert(
                convert(module.name.clone())?,
                convert_module(i as u8, module)?,
            );
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
    index: u8,
    module: runtime_metadata::ModuleMetadata,
) -> Result<ModuleMetadata, Error> {
    let mut entries = HashMap::new();
    if let Some(storage) = module.storage {
        let storage = convert(storage)?;
        let prefix = convert(storage.prefix)?;
        for entry in convert(storage.entries)?.into_iter() {
            let entry_name = convert(entry.name.clone())?;
            let entry_prefix = format!("{} {}", prefix, entry_name);
            let entry = convert_entry(entry_prefix, entry)?;
            entries.insert(entry_name, entry);
        }
    }

    Ok(ModuleMetadata {
        index,
        storage: entries,
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
