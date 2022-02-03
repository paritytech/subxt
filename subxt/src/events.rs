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

use crate::{
    error::BasicError,
    metadata::{
        EventMetadata,
        MetadataError,
    },
    Config,
    Event,
    Metadata,
    PhantomDataSendSync,
    Phase,
};
use bitvec::{
    order::Lsb0,
    vec::BitVec,
};
use codec::{
    Codec,
    Compact,
    Decode,
    Error as CodecError,
    Input,
};
use derivative::Derivative;
use scale_info::{
    PortableRegistry,
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
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct EventsDecoder<T: Config> {
    metadata: Metadata,
    marker: PhantomDataSendSync<T>,
}

impl<T: Config> EventsDecoder<T> {
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
    ) -> Result<Vec<(Phase, RawEvent)>, BasicError> {
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
    ) -> Result<(), BasicError> {
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
    ) -> Result<(), BasicError> {
        let all_bytes = *input;
        // consume some bytes, moving the cursor forward:
        decode_and_consume_type(type_id, &self.metadata.runtime_metadata().types, input)?;
        // count how many bytes were consumed based on remaining length:
        let consumed_len = all_bytes.len() - input.len();
        // move those consumed bytes to the output vec unaltered:
        output.extend(&all_bytes[0..consumed_len]);
        Ok(())
    }
}

// Given a type Id and a type registry, attempt to consume the bytes
// corresponding to that type from our input.
fn decode_and_consume_type(
    type_id: u32,
    types: &PortableRegistry,
    input: &mut &[u8],
) -> Result<(), BasicError> {
    let ty = types
        .resolve(type_id)
        .ok_or(MetadataError::TypeNotFound(type_id))?;

    fn consume_type<T: Codec>(input: &mut &[u8]) -> Result<(), BasicError> {
        T::decode(input)?;
        Ok(())
    }

    match ty.type_def() {
        TypeDef::Composite(composite) => {
            for field in composite.fields() {
                decode_and_consume_type(field.ty().id(), types, input)?
            }
            Ok(())
        }
        TypeDef::Variant(variant) => {
            let variant_index = u8::decode(input)?;
            let variant = variant
                .variants()
                .iter()
                .find(|v| v.index() == variant_index)
                .ok_or_else(|| {
                    BasicError::Other(format!("Variant {} not found", variant_index))
                })?;
            for field in variant.fields() {
                decode_and_consume_type(field.ty().id(), types, input)?;
            }
            Ok(())
        }
        TypeDef::Sequence(seq) => {
            let len = <Compact<u32>>::decode(input)?;
            for _ in 0..len.0 {
                decode_and_consume_type(seq.type_param().id(), types, input)?;
            }
            Ok(())
        }
        TypeDef::Array(arr) => {
            for _ in 0..arr.len() {
                decode_and_consume_type(arr.type_param().id(), types, input)?;
            }
            Ok(())
        }
        TypeDef::Tuple(tuple) => {
            for field in tuple.fields() {
                decode_and_consume_type(field.id(), types, input)?;
            }
            Ok(())
        }
        TypeDef::Primitive(primitive) => {
            match primitive {
                TypeDefPrimitive::Bool => consume_type::<bool>(input),
                TypeDefPrimitive::Char => {
                    Err(
                        EventsDecodingError::UnsupportedPrimitive(TypeDefPrimitive::Char)
                            .into(),
                    )
                }
                TypeDefPrimitive::Str => consume_type::<String>(input),
                TypeDefPrimitive::U8 => consume_type::<u8>(input),
                TypeDefPrimitive::U16 => consume_type::<u16>(input),
                TypeDefPrimitive::U32 => consume_type::<u32>(input),
                TypeDefPrimitive::U64 => consume_type::<u64>(input),
                TypeDefPrimitive::U128 => consume_type::<u128>(input),
                TypeDefPrimitive::U256 => {
                    Err(
                        EventsDecodingError::UnsupportedPrimitive(TypeDefPrimitive::U256)
                            .into(),
                    )
                }
                TypeDefPrimitive::I8 => consume_type::<i8>(input),
                TypeDefPrimitive::I16 => consume_type::<i16>(input),
                TypeDefPrimitive::I32 => consume_type::<i32>(input),
                TypeDefPrimitive::I64 => consume_type::<i64>(input),
                TypeDefPrimitive::I128 => consume_type::<i128>(input),
                TypeDefPrimitive::I256 => {
                    Err(
                        EventsDecodingError::UnsupportedPrimitive(TypeDefPrimitive::I256)
                            .into(),
                    )
                }
            }
        }
        TypeDef::Compact(compact) => {
            let inner = types
                .resolve(compact.type_param().id())
                .ok_or(MetadataError::TypeNotFound(type_id))?;
            let mut decode_compact_primitive = |primitive: &TypeDefPrimitive| {
                match primitive {
                    TypeDefPrimitive::U8 => consume_type::<Compact<u8>>(input),
                    TypeDefPrimitive::U16 => consume_type::<Compact<u16>>(input),
                    TypeDefPrimitive::U32 => consume_type::<Compact<u32>>(input),
                    TypeDefPrimitive::U64 => consume_type::<Compact<u64>>(input),
                    TypeDefPrimitive::U128 => consume_type::<Compact<u128>>(input),
                    prim => {
                        Err(EventsDecodingError::InvalidCompactPrimitive(prim.clone())
                            .into())
                    }
                }
            };
            match inner.type_def() {
                TypeDef::Primitive(primitive) => decode_compact_primitive(primitive),
                TypeDef::Composite(composite) => {
                    match composite.fields() {
                        [field] => {
                            let field_ty =
                                types.resolve(field.ty().id()).ok_or_else(|| {
                                    MetadataError::TypeNotFound(field.ty().id())
                                })?;
                            if let TypeDef::Primitive(primitive) = field_ty.type_def() {
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
        TypeDef::BitSequence(bitseq) => {
            let bit_store_def = types
                .resolve(bitseq.bit_store_type().id())
                .ok_or(MetadataError::TypeNotFound(type_id))?
                .type_def();

            // We just need to consume the correct number of bytes. Roughly, we encode this
            // as a Compact<u32> length, and then a slice of T of that length, where T is the
            // bit store type. So, we ignore the bit order and only care that the bit store type
            // used lines up in terms of the number of bytes it will take to encode/decode it.
            match bit_store_def {
                TypeDef::Primitive(TypeDefPrimitive::U8) => {
                    consume_type::<BitVec<Lsb0, u8>>(input)
                }
                TypeDef::Primitive(TypeDefPrimitive::U16) => {
                    consume_type::<BitVec<Lsb0, u16>>(input)
                }
                TypeDef::Primitive(TypeDefPrimitive::U32) => {
                    consume_type::<BitVec<Lsb0, u32>>(input)
                }
                TypeDef::Primitive(TypeDefPrimitive::U64) => {
                    consume_type::<BitVec<Lsb0, u64>>(input)
                }
                store => {
                    return Err(EventsDecodingError::InvalidBitSequenceType(format!(
                        "{:?}",
                        store
                    ))
                    .into())
                }
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
    /// Invalid compact type; error details in string.
    #[error("Invalid compact composite type {0}")]
    InvalidCompactType(String),
    /// Invalid bit sequence type; bit store type or bit order type used aren't supported.
    #[error("Invalid bit sequence type; bit store type {0} is not supported")]
    InvalidBitSequenceType(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::GenericError::{
            Codec,
            EventsDecoding,
            Other,
        },
        events::EventsDecodingError::UnsupportedPrimitive,
        Config,
        DefaultConfig,
        Phase,
    };
    use assert_matches::assert_matches;
    use codec::Encode;
    use frame_metadata::{
        v14::{
            ExtrinsicMetadata,
            PalletEventMetadata,
            PalletMetadata,
            RuntimeMetadataLastVersion,
        },
        RuntimeMetadataPrefixed,
    };
    use scale_info::{
        meta_type,
        TypeInfo,
    };
    use std::convert::TryFrom;

    type TypeId = scale_info::interner::UntrackedSymbol<std::any::TypeId>;

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
            topics: vec![],
        }
    }

    fn singleton_type_registry<T: scale_info::TypeInfo + 'static>(
    ) -> (TypeId, PortableRegistry) {
        let m = scale_info::MetaType::new::<T>();
        let mut types = scale_info::Registry::new();
        let id = types.register_type(&m);
        let portable_registry: PortableRegistry = types.into();

        (id, portable_registry)
    }

    fn pallet_metadata<E: TypeInfo + 'static>(pallet_index: u8) -> PalletMetadata {
        let event = PalletEventMetadata {
            ty: meta_type::<E>(),
        };
        PalletMetadata {
            name: "Test",
            storage: None,
            calls: None,
            event: Some(event),
            constants: vec![],
            error: None,
            index: pallet_index,
        }
    }

    fn init_decoder(pallets: Vec<PalletMetadata>) -> EventsDecoder<DefaultConfig> {
        let extrinsic = ExtrinsicMetadata {
            ty: meta_type::<()>(),
            version: 0,
            signed_extensions: vec![],
        };
        let v14 = RuntimeMetadataLastVersion::new(pallets, extrinsic, meta_type::<()>());
        let runtime_metadata: RuntimeMetadataPrefixed = v14.into();
        let metadata = Metadata::try_from(runtime_metadata).unwrap();
        EventsDecoder::<DefaultConfig>::new(metadata)
    }

    fn decode_and_consume_type_consumes_all_bytes<
        T: codec::Encode + scale_info::TypeInfo + 'static,
    >(
        val: T,
    ) {
        let (type_id, registry) = singleton_type_registry::<T>();
        let bytes = val.encode();
        let cursor = &mut &*bytes;

        decode_and_consume_type(type_id.id(), &registry, cursor).unwrap();
        assert_eq!(cursor.len(), 0);
    }

    #[test]
    fn decode_single_event() {
        #[derive(Clone, Encode, TypeInfo)]
        enum Event {
            A(u8),
        }

        let pallet_index = 0;
        let pallet = pallet_metadata::<Event>(pallet_index);
        let decoder = init_decoder(vec![pallet]);

        let event = Event::A(1);
        let encoded_event = event.encode();
        let event_records = vec![event_record(pallet_index, event)];

        let mut input = Vec::new();
        event_records.encode_to(&mut input);

        let events = decoder.decode_events(&mut &input[..]).unwrap();

        assert_eq!(events[0].1.variant_index, encoded_event[0]);
        assert_eq!(events[0].1.data.0, encoded_event[1..]);
    }

    #[test]
    fn decode_multiple_events() {
        #[derive(Clone, Encode, TypeInfo)]
        enum Event {
            A(u8),
            B,
            C { a: u32 },
        }

        let pallet_index = 0;
        let pallet = pallet_metadata::<Event>(pallet_index);
        let decoder = init_decoder(vec![pallet]);

        let event1 = Event::A(1);
        let event2 = Event::B;
        let event3 = Event::C { a: 3 };

        let encoded_event1 = event1.encode();
        let encoded_event2 = event2.encode();
        let encoded_event3 = event3.encode();

        let event_records = vec![
            event_record(pallet_index, event1),
            event_record(pallet_index, event2),
            event_record(pallet_index, event3),
        ];

        let mut input = Vec::new();
        event_records.encode_to(&mut input);

        let events = decoder.decode_events(&mut &input[..]).unwrap();

        assert_eq!(events[0].1.variant_index, encoded_event1[0]);
        assert_eq!(events[0].1.data.0, encoded_event1[1..]);

        assert_eq!(events[1].1.variant_index, encoded_event2[0]);
        assert_eq!(events[1].1.data.0, encoded_event2[1..]);

        assert_eq!(events[2].1.variant_index, encoded_event3[0]);
        assert_eq!(events[2].1.data.0, encoded_event3[1..]);
    }

    #[test]
    fn compact_event_field() {
        #[derive(Clone, Encode, TypeInfo)]
        enum Event {
            A(#[codec(compact)] u32),
        }

        let pallet_index = 0;
        let pallet = pallet_metadata::<Event>(pallet_index);
        let decoder = init_decoder(vec![pallet]);

        let event = Event::A(u32::MAX);
        let encoded_event = event.encode();
        let event_records = vec![event_record(pallet_index, event)];

        let mut input = Vec::new();
        event_records.encode_to(&mut input);

        let events = decoder.decode_events(&mut &input[..]).unwrap();

        assert_eq!(events[0].1.variant_index, encoded_event[0]);
        assert_eq!(events[0].1.data.0, encoded_event[1..]);
    }

    #[test]
    fn compact_wrapper_struct_field() {
        #[derive(Clone, Encode, TypeInfo)]
        enum Event {
            A(#[codec(compact)] CompactWrapper),
        }

        #[derive(Clone, codec::CompactAs, Encode, TypeInfo)]
        struct CompactWrapper(u64);

        let pallet_index = 0;
        let pallet = pallet_metadata::<Event>(pallet_index);
        let decoder = init_decoder(vec![pallet]);

        let event = Event::A(CompactWrapper(0));
        let encoded_event = event.encode();
        let event_records = vec![event_record(pallet_index, event)];

        let mut input = Vec::new();
        event_records.encode_to(&mut input);

        let events = decoder.decode_events(&mut &input[..]).unwrap();

        assert_eq!(events[0].1.variant_index, encoded_event[0]);
        assert_eq!(events[0].1.data.0, encoded_event[1..]);
    }

    #[test]
    fn event_containing_explicit_index() {
        #[derive(Clone, Encode, TypeInfo)]
        #[repr(u8)]
        #[allow(trivial_numeric_casts, clippy::unnecessary_cast)] // required because the Encode derive produces a warning otherwise
        pub enum MyType {
            B = 10u8,
        }

        #[derive(Clone, Encode, TypeInfo)]
        enum Event {
            A(MyType),
        }

        let pallet_index = 0;
        let pallet = pallet_metadata::<Event>(pallet_index);
        let decoder = init_decoder(vec![pallet]);

        let event = Event::A(MyType::B);
        let encoded_event = event.encode();
        let event_records = vec![event_record(pallet_index, event)];

        let mut input = Vec::new();
        event_records.encode_to(&mut input);

        // this would panic if the explicit enum item index were not correctly used
        let events = decoder.decode_events(&mut &input[..]).unwrap();

        assert_eq!(events[0].1.variant_index, encoded_event[0]);
        assert_eq!(events[0].1.data.0, encoded_event[1..]);
    }

    #[test]
    fn decode_bitvec() {
        use bitvec::order::Msb0;

        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Lsb0, u8; 0, 1, 1, 0, 1],
        );
        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Msb0, u8; 0, 1, 1, 0, 1, 0, 1, 0, 0],
        );

        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Lsb0, u16; 0, 1, 1, 0, 1],
        );
        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Msb0, u16; 0, 1, 1, 0, 1, 0, 1, 0, 0],
        );

        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Lsb0, u32; 0, 1, 1, 0, 1],
        );
        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Msb0, u32; 0, 1, 1, 0, 1, 0, 1, 0, 0],
        );

        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Lsb0, u64; 0, 1, 1, 0, 1],
        );
        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Msb0, u64; 0, 1, 1, 0, 1, 0, 1, 0, 0],
        );
    }

    #[test]
    fn decode_primitive() {
        decode_and_consume_type_consumes_all_bytes(false);
        decode_and_consume_type_consumes_all_bytes(true);

        let dummy_data = vec![0u8];
        let dummy_cursor = &mut &*dummy_data;
        let (id, reg) = singleton_type_registry::<char>();
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(
            res,
            Err(EventsDecoding(UnsupportedPrimitive(TypeDefPrimitive::Char)))
        );

        decode_and_consume_type_consumes_all_bytes("str".to_string());

        decode_and_consume_type_consumes_all_bytes(1u8);
        decode_and_consume_type_consumes_all_bytes(1i8);

        decode_and_consume_type_consumes_all_bytes(1u16);
        decode_and_consume_type_consumes_all_bytes(1i16);

        decode_and_consume_type_consumes_all_bytes(1u32);
        decode_and_consume_type_consumes_all_bytes(1i32);

        decode_and_consume_type_consumes_all_bytes(1u64);
        decode_and_consume_type_consumes_all_bytes(1i64);

        decode_and_consume_type_consumes_all_bytes(1u128);
        decode_and_consume_type_consumes_all_bytes(1i128);
    }

    #[test]
    fn decode_tuple() {
        decode_and_consume_type_consumes_all_bytes(());

        decode_and_consume_type_consumes_all_bytes((true,));

        decode_and_consume_type_consumes_all_bytes((true, "str"));

        // Incomplete bytes for decoding
        let dummy_data = false.encode();
        let dummy_cursor = &mut &*dummy_data;
        let (id, reg) = singleton_type_registry::<(bool, &'static str)>();
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(res, Err(Codec(_)));

        // Incomplete bytes for decoding, with invalid char type
        let dummy_data = (false, "str", 0u8).encode();
        let dummy_cursor = &mut &*dummy_data;
        let (id, reg) = singleton_type_registry::<(bool, &'static str, char)>();
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(
            res,
            Err(EventsDecoding(UnsupportedPrimitive(TypeDefPrimitive::Char)))
        );
        // The last byte (0x0 u8) should not be consumed
        assert_eq!(dummy_cursor.len(), 1);
    }

    #[test]
    fn decode_array_and_seq() {
        decode_and_consume_type_consumes_all_bytes([0]);
        decode_and_consume_type_consumes_all_bytes([1, 2, 3, 4, 5]);
        decode_and_consume_type_consumes_all_bytes([0; 500]);
        decode_and_consume_type_consumes_all_bytes(["str", "abc", "cde"]);

        decode_and_consume_type_consumes_all_bytes(vec![0]);
        decode_and_consume_type_consumes_all_bytes(vec![1, 2, 3, 4, 5]);
        decode_and_consume_type_consumes_all_bytes(vec!["str", "abc", "cde"]);
    }

    #[test]
    fn decode_variant() {
        #[derive(Clone, Encode, TypeInfo)]
        enum EnumVar {
            A,
            B((&'static str, u8)),
            C { named: i16 },
        }
        const INVALID_TYPE_ID: u32 = 1024;

        decode_and_consume_type_consumes_all_bytes(EnumVar::A);
        decode_and_consume_type_consumes_all_bytes(EnumVar::B(("str", 1)));
        decode_and_consume_type_consumes_all_bytes(EnumVar::C { named: 1 });

        // Invalid variant index
        let dummy_data = 3u8.encode();
        let dummy_cursor = &mut &*dummy_data;
        let (id, reg) = singleton_type_registry::<EnumVar>();
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(res, Err(Other(_)));

        // Valid index, incomplete data
        let dummy_data = 2u8.encode();
        let dummy_cursor = &mut &*dummy_data;
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(res, Err(Codec(_)));

        let res = decode_and_consume_type(INVALID_TYPE_ID, &reg, dummy_cursor);
        assert_matches!(res, Err(crate::error::GenericError::Metadata(_)));
    }

    #[test]
    fn decode_composite() {
        #[derive(Clone, Encode, TypeInfo)]
        struct Composite {}
        decode_and_consume_type_consumes_all_bytes(Composite {});

        #[derive(Clone, Encode, TypeInfo)]
        struct CompositeV2 {
            id: u32,
            name: String,
        }
        decode_and_consume_type_consumes_all_bytes(CompositeV2 {
            id: 10,
            name: "str".to_string(),
        });

        #[derive(Clone, Encode, TypeInfo)]
        struct CompositeV3<T> {
            id: u32,
            extra: T,
        }
        decode_and_consume_type_consumes_all_bytes(CompositeV3 {
            id: 10,
            extra: vec![0, 1, 2],
        });
        decode_and_consume_type_consumes_all_bytes(CompositeV3 {
            id: 10,
            extra: bitvec::bitvec![Lsb0, u8; 0, 1, 1, 0, 1],
        });
        decode_and_consume_type_consumes_all_bytes(CompositeV3 {
            id: 10,
            extra: ("str", 1),
        });
        decode_and_consume_type_consumes_all_bytes(CompositeV3 {
            id: 10,
            extra: CompositeV2 {
                id: 2,
                name: "str".to_string(),
            },
        });

        #[derive(Clone, Encode, TypeInfo)]
        struct CompositeV4(u32, bool);
        decode_and_consume_type_consumes_all_bytes(CompositeV4(1, true));

        #[derive(Clone, Encode, TypeInfo)]
        struct CompositeV5(u32);
        decode_and_consume_type_consumes_all_bytes(CompositeV5(1));
    }

    #[test]
    fn decode_compact() {
        #[derive(Clone, Encode, TypeInfo)]
        enum Compact {
            A(#[codec(compact)] u32),
        }
        decode_and_consume_type_consumes_all_bytes(Compact::A(1));

        #[derive(Clone, Encode, TypeInfo)]
        struct CompactV2(#[codec(compact)] u32);
        decode_and_consume_type_consumes_all_bytes(CompactV2(1));

        #[derive(Clone, Encode, TypeInfo)]
        struct CompactV3 {
            #[codec(compact)]
            val: u32,
        }
        decode_and_consume_type_consumes_all_bytes(CompactV3 { val: 1 });

        #[derive(Clone, Encode, TypeInfo)]
        struct CompactV4<T> {
            #[codec(compact)]
            val: T,
        }
        decode_and_consume_type_consumes_all_bytes(CompactV4 { val: 0u8 });
        decode_and_consume_type_consumes_all_bytes(CompactV4 { val: 1u16 });
    }
}
