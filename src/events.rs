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
    Output,
};
use dyn_clone::DynClone;
use sp_runtime::{
    DispatchError,
    DispatchResult,
};
use std::{
    collections::{
        hash_map::{
            Entry,
            HashMap,
        },
        HashSet,
    },
    fmt,
    marker::{
        PhantomData,
        Send,
    },
};

use crate::{
    Error,
    Metadata,
    Phase,
    Runtime,
    RuntimeError,
};

/// Raw bytes for an Event
pub struct RawEvent {
    /// The name of the module from whence the Event originated
    pub module: String,
    /// The name of the Event
    pub variant: String,
    /// The raw Event data
    pub data: Vec<u8>,
}

impl std::fmt::Debug for RawEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RawEvent")
            .field("module", &self.module)
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
        todo!()
        // let compact_len = <Compact<u32>>::decode(input)?;
        // let len = compact_len.0 as usize;
        //
        // let mut r = Vec::new();
        // for _ in 0..len {
        //     // decode EventRecord
        //     let phase = Phase::decode(input)?;
        //     let module_variant = input.read_byte()?;
        //
        //     let module = self.metadata.module_with_events(module_variant)?;
        //     let event_variant = input.read_byte()?;
        //     let event_metadata = module.event(event_variant)?;
        //
        //     log::debug!(
        //         "received event '{}::{}' ({:?})",
        //         module.name(),
        //         event_metadata.name,
        //         event_metadata.arguments()
        //     );
        //
        //     let mut event_data = Vec::<u8>::new();
        //     let mut event_errors = Vec::<RuntimeError>::new();
        //     let result = self.decode_raw_bytes(
        //         &event_metadata.arguments(),
        //         input,
        //         &mut event_data,
        //         &mut event_errors,
        //     );
        //     let raw = match result {
        //         Ok(()) => {
        //             log::debug!("raw bytes: {}", hex::encode(&event_data),);
        //
        //             let event = RawEvent {
        //                 module: module.name().to_string(),
        //                 variant: event_metadata.name.clone(),
        //                 data: event_data,
        //             };
        //
        //             // topics come after the event data in EventRecord
        //             let _topics = Vec::<T::Hash>::decode(input)?;
        //             Raw::Event(event)
        //         }
        //         Err(err) => return Err(err),
        //     };
        //
        //     if event_errors.is_empty() {
        //         r.push((phase.clone(), raw));
        //     }
        //
        //     for err in event_errors {
        //         r.push((phase.clone(), Raw::Error(err)));
        //     }
        // }
        // Ok(r)
    }

    fn decode_raw_bytes<W: Output>(
        &self,
        _args: &scale_info::Variant,
        _input: &mut &[u8],
        _output: &mut W,
        _errors: &mut Vec<RuntimeError>,
    ) -> Result<(), Error> {
        todo!()
        // for arg in args {
        //     match arg {
        //         EventArg::Vec(arg) => {
        //             let len = <Compact<u32>>::decode(input)?;
        //             len.encode_to(output);
        //             for _ in 0..len.0 {
        //                 self.decode_raw_bytes(&[*arg.clone()], input, output, errors)?
        //             }
        //         }
        //         EventArg::Option(arg) => {
        //             match input.read_byte()? {
        //                 0 => output.push_byte(0),
        //                 1 => {
        //                     output.push_byte(1);
        //                     self.decode_raw_bytes(&[*arg.clone()], input, output, errors)?
        //                 }
        //                 _ => {
        //                     return Err(Error::Other(
        //                         "unexpected first byte decoding Option".into(),
        //                     ))
        //                 }
        //             }
        //         }
        //         EventArg::Tuple(args) => {
        //             self.decode_raw_bytes(args, input, output, errors)?
        //         }
        //         EventArg::Primitive(name) => {
        //             let result = match name.as_str() {
        //                 "DispatchResult" => DispatchResult::decode(input)?,
        //                 "DispatchError" => Err(DispatchError::decode(input)?),
        //                 _ => {
        //                     if let Some(seg) = self.event_type_registry.resolve(name) {
        //                         let mut buf = Vec::<u8>::new();
        //                         seg.segment(input, &mut buf)?;
        //                         output.write(&buf);
        //                         Ok(())
        //                     } else {
        //                         return Err(Error::TypeSizeUnavailable(name.to_owned()))
        //                     }
        //                 }
        //             };
        //             if let Err(error) = result {
        //                 // since the input may contain any number of args we propagate
        //                 // runtime errors to the caller for handling
        //                 errors.push(RuntimeError::from_dispatch(&self.metadata, error)?);
        //             }
        //         }
        //     }
        // }
        // Ok(())
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
