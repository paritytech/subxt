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

use codec::{
    Codec,
    Compact,
    Decode,
    Encode,
    Input,
};
use dyn_clone::DynClone;
use std::marker::{
    PhantomData,
    Send,
};

use crate::{
    metadata::{
        EventMetadata,
        MetadataError,
    },
    Error,
    Metadata,
    Phase,
    Runtime,
    RuntimeError,
};
use scale_info::{
    TypeDef,
    TypeDefPrimitive,
};

/// Raw bytes for an Event
pub struct RawEvent {
    /// The name of the pallet from whence the Event originated
    pub pallet: String,
    /// The name of the Event
    pub variant: String,
    /// The raw Event data
    pub data: Vec<u8>,
}

impl std::fmt::Debug for RawEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RawEvent")
            .field("module", &self.pallet)
            .field("variant", &self.variant)
            .field("data", &hex::encode(&self.data))
            .finish()
    }
}

pub trait TypeSegmenter: DynClone + Send + Sync {
    /// Consumes an object from an input stream, and output the serialized bytes.
    fn segment(&self, input: &mut &[u8], output: &mut Vec<u8>) -> Result<(), Error>;
}

// derive object safe Clone impl for `Box<dyn TypeSegmenter>`
dyn_clone::clone_trait_object!(TypeSegmenter);

struct TypeMarker<T>(PhantomData<T>);
impl<T> TypeSegmenter for TypeMarker<T>
where
    T: Codec + Send + Sync,
{
    fn segment(&self, input: &mut &[u8], output: &mut Vec<u8>) -> Result<(), Error> {
        T::decode(input).map_err(Error::from)?.encode_to(output);
        Ok(())
    }
}

impl<T> Clone for TypeMarker<T> {
    fn clone(&self) -> Self {
        Self(Default::default())
    }
}

impl<T> Default for TypeMarker<T> {
    fn default() -> Self {
        Self(Default::default())
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
    T: Runtime,
{
    /// Creates a new `EventsDecoder`.
    pub fn new(metadata: Metadata) -> Self {
        Self {
            metadata,
            marker: Default::default(),
        }
    }

    /// Decode events.
    pub fn decode_events(&self, input: &mut &[u8]) -> Result<Vec<(Phase, Raw)>, Error> {
        let compact_len = <Compact<u32>>::decode(input)?;
        let len = compact_len.0 as usize;
        log::debug!("decoding {} events", len);

        let mut r = Vec::new();
        for _ in 0..len {
            // decode EventRecord
            let phase = Phase::decode(input)?;
            let pallet_index = input.read_byte()?;
            let event_variant = input.read_byte()?;
            log::debug!("phase {:?}, pallet_index {}, event_variant: {}", phase, pallet_index, event_variant);
            log::debug!("remaining input: {}", hex::encode(&input));

            let event_metadata = self.metadata.event(pallet_index, event_variant)?;

            let mut event_data = Vec::<u8>::new();
            let mut event_errors = Vec::<RuntimeError>::new();
            let result = self.decode_raw_event(
                &event_metadata,
                input,
                &mut event_data,
                &mut event_errors,
            );
            let raw = match result {
                Ok(()) => {
                    log::debug!("raw bytes: {}", hex::encode(&event_data),);

                    let event = RawEvent {
                        pallet: event_metadata.pallet().to_string(),
                        variant: event_metadata.event().to_string(),
                        data: event_data,
                    };

                    // topics come after the event data in EventRecord
                    let topics = Vec::<T::Hash>::decode(input)?;
                    log::debug!("topics: {:?}", topics);

                    Raw::Event(event)
                }
                Err(err) => return Err(err),
            };

            if event_errors.is_empty() {
                r.push((phase.clone(), raw));
            }

            for err in event_errors {
                r.push((phase.clone(), Raw::Error(err)));
            }
        }
        Ok(r)
    }

    fn decode_raw_event(
        &self,
        event_metadata: &EventMetadata,
        input: &mut &[u8],
        output: &mut Vec<u8>,
        errors: &mut Vec<RuntimeError>,
    ) -> Result<(), Error> {
        log::debug!("Decoding Event '{}::{}'", event_metadata.pallet(), event_metadata.event());
        for arg in event_metadata.variant().fields() {
            let type_id = arg.ty().id();
            if event_metadata.pallet() == "System"
                && event_metadata.event() == "ExtrinsicFailed"
            {
                let ty = self
                    .metadata
                    .resolve_type(type_id)
                    .ok_or(MetadataError::TypeNotFound(type_id))?;

                if ty.path().ident() == Some("DispatchError".to_string()) {
                    let dispatch_error = sp_runtime::DispatchError::decode(input)?;
                    log::info!("Dispatch Error {:?}", dispatch_error);
                    dispatch_error.encode_to(output);
                    let runtime_error =
                        RuntimeError::from_dispatch(&self.metadata, dispatch_error)?;
                    errors.push(runtime_error);
                    continue
                }
            }
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
                // todo: [AJ] handle if variant is DispatchError?
                let variant_index = u8::decode(input)?;
                variant_index.encode_to(output);
                let variant = variant.variants().get(variant_index as usize).unwrap(); // todo: ok_or
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
                        todo!("Err: scale codec not implemented for char")
                    }
                    TypeDefPrimitive::Str => decode_raw::<String>(input, output),
                    TypeDefPrimitive::U8 => decode_raw::<u8>(input, output),
                    TypeDefPrimitive::U16 => decode_raw::<u16>(input, output),
                    TypeDefPrimitive::U32 => decode_raw::<u32>(input, output),
                    TypeDefPrimitive::U64 => decode_raw::<u64>(input, output),
                    TypeDefPrimitive::U128 => decode_raw::<u128>(input, output),
                    TypeDefPrimitive::U256 => todo!("Err: U256 currently not supported"),
                    TypeDefPrimitive::I8 => decode_raw::<i8>(input, output),
                    TypeDefPrimitive::I16 => decode_raw::<i16>(input, output),
                    TypeDefPrimitive::I32 => decode_raw::<i32>(input, output),
                    TypeDefPrimitive::I64 => decode_raw::<i64>(input, output),
                    TypeDefPrimitive::I128 => decode_raw::<i128>(input, output),
                    TypeDefPrimitive::I256 => todo!("Err(I256 currently not supported)"),
                }
            }
            TypeDef::Compact(_compact) => {
                let inner = self
                    .metadata
                    .resolve_type(type_id)
                    .ok_or(MetadataError::TypeNotFound(type_id))?;
                match inner.type_def() {
                    TypeDef::Primitive(primitive) => {
                        match primitive {
                            TypeDefPrimitive::U8 => decode_raw::<Compact<u8>>(input, output),
                            TypeDefPrimitive::U16 => decode_raw::<Compact<u16>>(input, output),
                            TypeDefPrimitive::U32 => decode_raw::<Compact<u32>>(input, output),
                            TypeDefPrimitive::U64 => decode_raw::<Compact<u64>>(input, output),
                            TypeDefPrimitive::U128 => decode_raw::<Compact<u128>>(input, output),
                            _ => todo!("Add custom err: Compact only supported for unsigned int primitives"),
                        }
                    }
                    // todo: [AJ] single field struct with primitive?, extract primitive decoding as above
                    _ => todo!("Add custom err: Compact only supported for unsigned int primitives"),
                }
            }
            TypeDef::BitSequence(_bitseq) => {
                // decode_raw::<bitvec::BitVec>
                todo!("BitVec")
            }
        }
    }
}

/// Raw event or error event
#[derive(Debug)]
pub enum Raw {
    /// Event
    Event(RawEvent),
    /// Error
    Error(RuntimeError),
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::convert::TryFrom;
//
//     type TestRuntime = crate::NodeTemplateRuntime;
//
//     #[test]
//     fn test_decode_option() {
//         let decoder = EventsDecoder::<TestRuntime>::new(
//             Metadata::default(),
//         );
//
//         let value = Some(0u8);
//         let input = value.encode();
//         let mut output = Vec::<u8>::new();
//         let mut errors = Vec::<RuntimeError>::new();
//
//         decoder
//             .decode_raw_bytes(
//                 &[EventArg::Option(Box::new(EventArg::Primitive(
//                     "u8".to_string(),
//                 )))],
//                 &mut &input[..],
//                 &mut output,
//                 &mut errors,
//             )
//             .unwrap();
//
//         assert_eq!(output, vec![1, 0]);
//     }
// }
