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

use codec::{
    Codec,
    Compact,
    Decode,
    Encode,
    Error as CodecError,
    Input,
};
use std::marker::PhantomData;

use crate::{
    metadata::{
        EventMetadata,
        MetadataError,
    },
    Config,
    Error,
    Event,
    Metadata,
    Phase,
};
use scale_info::{
    TypeDef,
    TypeDefPrimitive,
};
use sp_core::Bytes;

/// Raw bytes for an Event
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Clone))]
pub struct RawEvent {
    /// The name of the pallet from whence the Event originated.
    pub pallet: String,
    /// The index of the pallet from whence the Event originated.
    pub pallet_index: u8,
    /// The name of the pallet Event variant.
    pub variant: String,
    /// The index of the pallet Event variant.
    pub variant_index: u8,
    /// The raw Event data
    pub data: Bytes,
}

impl RawEvent {
    /// Attempt to decode this [`RawEvent`] into a specific event.
    pub fn as_event<E: Event>(&self) -> Result<Option<E>, CodecError> {
        if self.pallet == E::PALLET && self.variant == E::EVENT {
            Ok(Some(E::decode(&mut &self.data[..])?))
        } else {
            Ok(None)
        }
    }
}

/// Events decoder.
#[derive(Debug, Clone)]
pub struct EventsDecoder<T> {
    metadata: Metadata,
    marker: PhantomData<T>,
}

impl<T> EventsDecoder<T>
where
    T: Config,
{
    /// Creates a new `EventsDecoder`.
    pub fn new(metadata: Metadata) -> Self {
        Self {
            metadata,
            marker: Default::default(),
        }
    }

    /// Decode events.
    pub fn decode_events(
        &self,
        input: &mut &[u8],
    ) -> Result<Vec<(Phase, RawEvent)>, Error> {
        let compact_len = <Compact<u32>>::decode(input)?;
        let len = compact_len.0 as usize;
        log::debug!("decoding {} events", len);

        let mut r = Vec::new();
        for _ in 0..len {
            // decode EventRecord
            let phase = Phase::decode(input)?;
            let pallet_index = input.read_byte()?;
            let variant_index = input.read_byte()?;
            log::debug!(
                "phase {:?}, pallet_index {}, event_variant: {}",
                phase,
                pallet_index,
                variant_index
            );
            log::debug!("remaining input: {}", hex::encode(&input));

            let event_metadata = self.metadata.event(pallet_index, variant_index)?;

            let mut event_data = Vec::<u8>::new();
            let result = self.decode_raw_event(event_metadata, input, &mut event_data);
            let raw = match result {
                Ok(()) => {
                    log::debug!("raw bytes: {}", hex::encode(&event_data),);

                    let event = RawEvent {
                        pallet: event_metadata.pallet().to_string(),
                        pallet_index,
                        variant: event_metadata.event().to_string(),
                        variant_index,
                        data: event_data.into(),
                    };

                    // topics come after the event data in EventRecord
                    let topics = Vec::<T::Hash>::decode(input)?;
                    log::debug!("topics: {:?}", topics);

                    event
                }
                Err(err) => return Err(err),
            };
            r.push((phase.clone(), raw));
        }
        Ok(r)
    }

    fn decode_raw_event(
        &self,
        event_metadata: &EventMetadata,
        input: &mut &[u8],
        output: &mut Vec<u8>,
    ) -> Result<(), Error> {
        log::debug!(
            "Decoding Event '{}::{}'",
            event_metadata.pallet(),
            event_metadata.event()
        );
        for arg in event_metadata.variant().fields() {
            let type_id = arg.ty().id();
            self.decode_type(type_id, input, output)?
        }
        Ok(())
    }

    fn decode_type(
        &self,
        type_id: u32,
        input: &mut &[u8],
        output: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let ty = self
            .metadata
            .resolve_type(type_id)
            .ok_or(MetadataError::TypeNotFound(type_id))?;

        fn decode_raw<T: Codec>(
            input: &mut &[u8],
            output: &mut Vec<u8>,
        ) -> Result<(), Error> {
            let decoded = T::decode(input)?;
            decoded.encode_to(output);
            Ok(())
        }

        match ty.type_def() {
            TypeDef::Composite(composite) => {
                for field in composite.fields() {
                    self.decode_type(field.ty().id(), input, output)?
                }
                Ok(())
            }
            TypeDef::Variant(variant) => {
                let variant_index = u8::decode(input)?;
                variant_index.encode_to(output);
                let variant =
                    variant
                        .variants()
                        .get(variant_index as usize)
                        .ok_or_else(|| {
                            Error::Other(format!("Variant {} not found", variant_index))
                        })?;
                for field in variant.fields() {
                    self.decode_type(field.ty().id(), input, output)?;
                }
                Ok(())
            }
            TypeDef::Sequence(seq) => {
                let len = <Compact<u32>>::decode(input)?;
                len.encode_to(output);
                for _ in 0..len.0 {
                    self.decode_type(seq.type_param().id(), input, output)?;
                }
                Ok(())
            }
            TypeDef::Array(arr) => {
                for _ in 0..arr.len() {
                    self.decode_type(arr.type_param().id(), input, output)?;
                }
                Ok(())
            }
            TypeDef::Tuple(tuple) => {
                for field in tuple.fields() {
                    self.decode_type(field.id(), input, output)?;
                }
                Ok(())
            }
            TypeDef::Primitive(primitive) => {
                match primitive {
                    TypeDefPrimitive::Bool => decode_raw::<bool>(input, output),
                    TypeDefPrimitive::Char => {
                        Err(EventsDecodingError::UnsupportedPrimitive(
                            TypeDefPrimitive::Char,
                        )
                        .into())
                    }
                    TypeDefPrimitive::Str => decode_raw::<String>(input, output),
                    TypeDefPrimitive::U8 => decode_raw::<u8>(input, output),
                    TypeDefPrimitive::U16 => decode_raw::<u16>(input, output),
                    TypeDefPrimitive::U32 => decode_raw::<u32>(input, output),
                    TypeDefPrimitive::U64 => decode_raw::<u64>(input, output),
                    TypeDefPrimitive::U128 => decode_raw::<u128>(input, output),
                    TypeDefPrimitive::U256 => {
                        Err(EventsDecodingError::UnsupportedPrimitive(
                            TypeDefPrimitive::U256,
                        )
                        .into())
                    }
                    TypeDefPrimitive::I8 => decode_raw::<i8>(input, output),
                    TypeDefPrimitive::I16 => decode_raw::<i16>(input, output),
                    TypeDefPrimitive::I32 => decode_raw::<i32>(input, output),
                    TypeDefPrimitive::I64 => decode_raw::<i64>(input, output),
                    TypeDefPrimitive::I128 => decode_raw::<i128>(input, output),
                    TypeDefPrimitive::I256 => {
                        Err(EventsDecodingError::UnsupportedPrimitive(
                            TypeDefPrimitive::I256,
                        )
                        .into())
                    }
                }
            }
            TypeDef::Compact(_compact) => {
                let inner = self
                    .metadata
                    .resolve_type(type_id)
                    .ok_or(MetadataError::TypeNotFound(type_id))?;
                let mut decode_compact_primitive = |primitive: &TypeDefPrimitive| {
                    match primitive {
                        TypeDefPrimitive::U8 => decode_raw::<Compact<u8>>(input, output),
                        TypeDefPrimitive::U16 => {
                            decode_raw::<Compact<u16>>(input, output)
                        }
                        TypeDefPrimitive::U32 => {
                            decode_raw::<Compact<u32>>(input, output)
                        }
                        TypeDefPrimitive::U64 => {
                            decode_raw::<Compact<u64>>(input, output)
                        }
                        TypeDefPrimitive::U128 => {
                            decode_raw::<Compact<u128>>(input, output)
                        }
                        prim => {
                            Err(EventsDecodingError::InvalidCompactPrimitive(
                                prim.clone(),
                            )
                            .into())
                        }
                    }
                };
                match inner.type_def() {
                    TypeDef::Primitive(primitive) => decode_compact_primitive(primitive),
                    TypeDef::Composite(composite) => {
                        match composite.fields() {
                            [field] => {
                                let field_ty = self
                                    .metadata
                                    .resolve_type(field.ty().id())
                                    .ok_or_else(|| {
                                        MetadataError::TypeNotFound(field.ty().id())
                                    })?;
                                if let TypeDef::Primitive(primitive) = field_ty.type_def()
                                {
                                    decode_compact_primitive(primitive)
                                } else {
                                    Err(EventsDecodingError::InvalidCompactType(
                                    "Composite type must have a single primitive field"
                                        .into(),
                                )
                                .into())
                                }
                            }
                            _ => {
                                Err(EventsDecodingError::InvalidCompactType(
                                    "Composite type must have a single field".into(),
                                )
                                .into())
                            }
                        }
                    }
                    _ => {
                        Err(EventsDecodingError::InvalidCompactType(
                            "Compact type must be a primitive or a composite type".into(),
                        )
                        .into())
                    }
                }
            }
            TypeDef::BitSequence(_bitseq) => {
                // decode_raw::<bitvec::BitVec>
                unimplemented!("BitVec decoding for events not implemented yet")
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EventsDecodingError {
    /// Unsupported primitive type
    #[error("Unsupported primitive type {0:?}")]
    UnsupportedPrimitive(TypeDefPrimitive),
    /// Invalid compact type, must be an unsigned int.
    #[error("Invalid compact primitive {0:?}")]
    InvalidCompactPrimitive(TypeDefPrimitive),
    #[error("Invalid compact composite type {0}")]
    InvalidCompactType(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use crate::{Config, DefaultConfig, Phase};
    use frame_metadata::{
        RuntimeMetadataPrefixed,
        v14::{
            ExtrinsicMetadata,
            RuntimeMetadataLastVersion,
            PalletMetadata,
            PalletEventMetadata,
        }
    };
    use scale_info::{
        TypeInfo,
        meta_type
    };

    #[derive(Encode)]
    pub struct EventRecord<E: Encode> {
        phase: Phase,
        pallet_index: u8,
        event: E,
        topics: Vec<<DefaultConfig as Config>::Hash>,
    }

    fn event_record<E: Encode>(pallet_index: u8, event: E) -> EventRecord<E> {
        EventRecord {
            phase: Phase::Finalization,
            pallet_index,
            event,
            topics: vec![]
        }
    }

    #[test]
    fn it_works() {
        #[derive(Clone, Encode, TypeInfo)]
        enum Event {
            A(u8),
        }

        let extrinsic = ExtrinsicMetadata {
            ty: meta_type::<()>(),
            version: 0,
            signed_extensions: vec![]
        };
        let event = PalletEventMetadata {
            ty: meta_type::<Event>()
        };
        let pallet_index = 0;
        let pallet = PalletMetadata {
            name: "Test",
            storage: None,
            calls: None,
            event: Some(event),
            constants: vec![],
            error: None,
            index: pallet_index
        };
        let v14 = RuntimeMetadataLastVersion::new(vec![pallet], extrinsic, meta_type::<()>());
        let runtime_metadata: RuntimeMetadataPrefixed = v14.into();
        let metadata = Metadata::try_from(runtime_metadata).unwrap();
        let decoder = EventsDecoder::<DefaultConfig>::new(metadata);

        let event = Event::A(1);
        let encoded_event = event.encode();
        let event_records = vec![
            event_record(pallet_index, event)
        ];

        let mut input = Vec::new();
        event_records.encode_to(&mut input);

        let events = decoder.decode_events(&mut &input[..]).unwrap();

        assert_eq!(events[0].1.variant_index, encoded_event[0]);
        assert_eq!(events[0].1.data.0, encoded_event[1..]);
    }
}
