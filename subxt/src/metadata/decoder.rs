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

//! Extrinsic decoder

use crate::{
    metadata::{
        env_types::{self, CustomTypeDecoder, EnvTypesTranscoder},
        MetadataPalletCalls, PathKey,
    },
    u8_map::U8Map,
    BasicError,
};
use codec::{Compact, Decode};
use frame_metadata::SignedExtensionMetadata;
use scale_info::{form::PortableForm, TypeInfo};
use scale_value::Value;
use serde::Serialize;
use sp_runtime::{AccountId32, MultiAddress, MultiSignature};
use std::{collections::HashMap, fmt::Debug};
// use metadata::Metadata;

/// The result of successfully decoding an extrinsic.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Extrinsic {
    /// Decoded call data and associated type information about the call.
    pub call_data: CallData,
    /// The signature and signed extensions (if any) associated with the extrinsic
    pub signature: Option<ExtrinsicSignature>,
}

/// Decoded call data and associated type information.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CallData {
    /// Pallet name
    pub pallet_name: String,
    /// Function
    pub pallet_fn: String,
    /// Arguments
    pub arguments: Vec<Value<scale_value::scale::TypeId>>,
}

/// The signature information embedded in an extrinsic.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExtrinsicSignature {
    /// Address the extrinsic is being sent from
    #[serde(with = "super::util::RemoteAddress")]
    pub address: MultiAddress<AccountId32, u32>,
    /// Signature to prove validity
    pub signature: MultiSignature,
    /// Signed extensions, which can vary by node. Here, we
    /// return the name and value of each.
    pub extensions: Vec<(String, Value<scale_value::scale::TypeId>)>,
}
/// Pallet name
pub struct DecoderBuilder {
    /// Pallet name
    registry: scale_value::scale::PortableRegistry,
    /// Decoders
    decoders: HashMap<u32, Box<dyn CustomTypeDecoder + Send + Sync>>,
    /// Pallet calls by index
    pallet_calls_by_index: U8Map<MetadataPalletCalls>,
    /// Signed extensions
    signed_extensions: Vec<SignedExtensionMetadata<PortableForm>>,
}

impl DecoderBuilder {
    /// New decoder
    pub fn new(
        registry: scale_value::scale::PortableRegistry,
        pallet_calls_by_index: U8Map<MetadataPalletCalls>,
        signed_extensions: Vec<SignedExtensionMetadata<PortableForm>>,
    ) -> Self {
        Self {
            registry,
            pallet_calls_by_index,
            signed_extensions,
            decoders: HashMap::new(),
        }
    }

    /// Register default custom types decoder
    pub fn with_default_custom_type_decodes(self) -> Self {
        self.register_custom_type_decoder::<tidefi_primitives::AccountId, _>(
            env_types::AccountId,
        )
        .register_custom_type_decoder::<sp_core::H256, _>(env_types::Hash)
        .register_custom_type_decoder::<tidefi_primitives::Hash, _>(env_types::Hash)
        .register_custom_type_decoder::<sp_core::H256, _>(env_types::Hash)
        .register_custom_type_decoder::<tidefi_primitives::CurrencyId, _>(
            env_types::CurrencyId,
        )
    }

    /// Register custom type decoder
    pub fn register_custom_type_decoder<T, U>(mut self, encoder: U) -> Self
    where
        T: TypeInfo + 'static,
        U: CustomTypeDecoder + 'static + Send + Sync,
    {
        let path_key = PathKey::from_type::<T>();

        let types_by_path = self.registry.types().into_iter().find_map(|portable_ty| {
            let pathkey_src = PathKey::from(portable_ty.ty().path());
            if pathkey_src == path_key {
                Some((portable_ty.id(), portable_ty.ty()))
            } else {
                None
            }
        });

        match types_by_path {
            Some((type_id, _)) => {
                let existing = self.decoders.insert(type_id, Box::new(encoder));
                // log::debug!("Registered custom decoder for type `{:?}`", type_id);
                if existing.is_some() {
                    panic!(
                        "Attempted to register decoder with existing type id {:?}",
                        type_id
                    );
                }
            }
            None => {
                // if the type is not present in the registry, it just means it has not been used.
                // log::info!("No matching type in registry for path {:?}.", path_key);
            }
        }
        self
    }

    /// Build decoder
    pub fn build(self) -> Result<Decoder, BasicError> {
        let env_types_transcoder = EnvTypesTranscoder::new(self.decoders);
        Ok(Decoder::new(
            self.registry,
            env_types_transcoder,
            self.pallet_calls_by_index,
            self.signed_extensions,
        ))
    }
}

/// Decoder
#[derive(Debug)]
pub struct Decoder {
    /// Registry
    registry: scale_value::scale::PortableRegistry,
    /// Custom decoder
    env_types: EnvTypesTranscoder,
    /// Pallet calls by index
    pallet_calls_by_index: U8Map<MetadataPalletCalls>,
    /// Signed extensions
    signed_extensions: Vec<SignedExtensionMetadata<PortableForm>>,
}

impl Decoder {
    /// New decoder
    pub fn new(
        registry: scale_value::scale::PortableRegistry,
        env_types: EnvTypesTranscoder,
        pallet_calls_by_index: U8Map<MetadataPalletCalls>,
        signed_extensions: Vec<SignedExtensionMetadata<PortableForm>>,
    ) -> Self {
        Self {
            registry,
            env_types,
            pallet_calls_by_index,
            signed_extensions,
        }
    }

    /// Decode extrinsic
    pub fn decode_extrinsic(&self, data: &mut &[u8]) -> Result<Extrinsic, BasicError> {
        if data.is_empty() {
            return Err(BasicError::Other(
                "unwrapped extrinsic byte length should be > 0".into(),
            ));
        }

        // Ignore the expected extrinsic length here at the moment, since we know it's 1
        let _len = <Compact<u32>>::decode(data)?;

        // V4 extrinsics (the format we can decode here) are laid out roughly as follows:
        //
        // first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
        //
        // signature, which is made up of (in order):
        // - sp_runtime::MultiAddress enum (sender)
        // - sp_runtime::MultiSignature enum
        // - For polkadot, these extensions (but can vary by chain, so we decode generically):
        //   - sp_runtime::generic::Era enum
        //   - compact encoded u32 (nonce; prior transaction count)
        //   - compact encoded u128 (tip paid to block producer/treasury)
        //
        // call, which is made up roughly of:
        // - u8 enum pallet index (for pallets variant)
        // - u8 call index (for inner variant)
        // - call args (types can be pulled from metadata for each arg we expect)
        //
        // So, we start by getting the version/signed from the first byte and go from there.
        let is_signed = data[0] & 0b1000_0000 != 0;
        let version = data[0] & 0b0111_1111;
        *data = &data[1..];

        // We only know how to decode V4 extrinsics at the moment
        if version != 4 {
            return Err(BasicError::Other(format!("Invalid version {}", version)));
        }

        // If the extrinsic is signed, decode the signature next.
        let signature = match is_signed {
            true => Some(self.decode_signature(data)?),
            false => None,
        };

        // Finally, decode the call data.
        let call_data = self.decode_call_data(data)?;

        Ok(Extrinsic {
            call_data,
            signature,
        })
    }

    fn call_variant_by_enum_index(
        &self,
        pallet: u8,
        call: u8,
    ) -> Option<(&str, &scale_info::Variant<PortableForm>)> {
        self.pallet_calls_by_index.get(pallet).and_then(|p| {
            p.calls.as_ref().and_then(|calls| {
                let type_def_variant = self.get_variant(calls.calls_type_id)?;
                let index = *calls.call_variant_indexes.get(call)?;
                let variant = type_def_variant.variants().get(index)?;
                Some((&*p.name, variant))
            })
        })
    }

    /// A helper function to get hold of a Variant given a type ID, or None if it's not found.
    fn get_variant(
        &self,
        ty: scale_info::interner::UntrackedSymbol<std::any::TypeId>,
    ) -> Option<&scale_info::TypeDefVariant<PortableForm>> {
        self.registry
            .resolve(ty.id())
            .and_then(|ty| match ty.type_def() {
                scale_info::TypeDef::Variant(variant) => Some(variant),
                _ => None,
            })
    }

    fn decode_call_data(&self, data: &mut &[u8]) -> Result<CallData, BasicError> {
        // Pluck out the u8's representing the pallet and call enum next.
        if data.len() < 2 {
            return Err(BasicError::Other(
                "expected at least 2 more bytes for the pallet/call index".into(),
            )
            .into());
        }
        let pallet_index = u8::decode(data)?;
        let call_index = u8::decode(data)?;

        // Work out which call the extrinsic data represents and get type info for it:
        let (pallet_name, variant) =
            match self.call_variant_by_enum_index(pallet_index, call_index) {
                Some(call) => call,
                None => {
                    return Err(BasicError::Other(format!(
                        "Unable to find call for pallet: {} call: {}",
                        pallet_index, call_index
                    )))
                }
            };

        // Decode each of the argument values in the extrinsic:
        let arguments = variant
            .fields()
            .iter()
            .map(|field| self.decode(field.ty().id(), data))
            .collect::<Result<Vec<_>, BasicError>>()?;

        Ok(CallData {
            pallet_name: pallet_name.to_owned(),
            pallet_fn: variant.name().to_owned(),
            arguments,
        })
    }

    fn decode_signature(
        &self,
        data: &mut &[u8],
    ) -> Result<ExtrinsicSignature, BasicError> {
        let address = <MultiAddress<AccountId32, u32>>::decode(data)?;
        let signature = MultiSignature::decode(data)?;
        let extensions = self.decode_signed_extensions(data)?;

        Ok(ExtrinsicSignature {
            address,
            signature,
            extensions,
        })
    }

    fn decode_signed_extensions(
        &self,
        data: &mut &[u8],
    ) -> Result<Vec<(String, Value<scale_value::scale::TypeId>)>, BasicError> {
        self.signed_extensions
            .iter()
            .map(|ext| {
                let val = self.decode(ext.ty.id(), data)?;
                let name = ext.identifier.to_owned();
                Ok((name, val))
            })
            .collect()
    }

    /// Decode type id to scale value
    pub fn decode(
        &self,
        type_id: u32,
        input: &mut &[u8],
    ) -> Result<Value<scale_value::scale::TypeId>, BasicError> {
        match self.env_types.try_decode(type_id, input) {
            // Value was decoded with custom decoder for type.
            Ok(Some(value)) => Ok(value),
            // No custom decoder registered so attempt default decoding.
            Ok(None) => {
                scale_value::scale::decode_as_type(input, type_id, &self.registry)
                    .map_err(Into::into)
            }
            Err(e) => Err(e),
        }
    }
}
