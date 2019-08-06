use parity_codec::Encode;
use runtime_metadata::{
    DecodeDifferent,
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
    META_RESERVED,
};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Metadata {
    modules: Vec<ModuleMetadata>,
}

impl Metadata {
    pub fn module_index(&self, name: &str) -> Option<usize> {
        self.modules
            .iter()
            .enumerate()
            .find(|(_, m)| &m.name == name)
            .map(|(i, _)| i)
    }

    pub fn call<T: Encode>(&self, module: &str, call: T) -> Option<Vec<u8>> {
        self.module_index(module).map(|i| {
            let mut bytes = vec![i as u8];
            bytes.extend(call.encode());
            bytes
        })
    }
}

#[derive(Debug)]
pub struct ModuleMetadata {
    pub name: String,
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
        Ok(Metadata {
            modules: convert(meta.modules)?
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn convert<B: 'static, O: 'static>(dd: DecodeDifferent<B, O>) -> Result<O, Error> {
    match dd {
        DecodeDifferent::Decoded(value) => Ok(value),
        _ => Err(Error::ExpectedDecoded),
    }
}

impl TryFrom<runtime_metadata::ModuleMetadata> for ModuleMetadata {
    type Error = Error;

    fn try_from(module: runtime_metadata::ModuleMetadata) -> Result<Self, Self::Error> {
        Ok(ModuleMetadata {
            name: convert(module.name)?,
        })
    }
}
