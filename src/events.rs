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

use crate::{
    metadata::{
        EventArg,
        Metadata,
        MetadataError,
    },
    srml::balances::Balances,
    System,
};
use parity_scale_codec::{
    Codec,
    Compact,
    Decode,
    Encode,
    Error as CodecError,
    Input,
    Output,
};
use log;
use srml_system::Phase;
use std::{
    convert::TryFrom,
    collections::HashMap,
    marker::{
        PhantomData,
        Send,
    },
};

#[derive(Debug)]
pub struct RawEvent {
    pub module: String,
    pub variant: String,
    pub data: Vec<u8>,
}

#[derive(Debug, derive_more::From)]
pub enum EventsError {
    CodecError(CodecError),
    Metadata(MetadataError),
    TypeSizesMissing(String, String),
    TypeSizeUnavailable(String),
}

pub struct EventsDecoder<T> {
    metadata: Metadata, // todo: borrow?
    type_sizes: HashMap<String, usize>,
    marker: PhantomData<fn() -> T>,
}

impl<T: System + Balances + 'static> TryFrom<Metadata> for EventsDecoder<T> {
    type Error = EventsError;

    fn try_from(metadata: Metadata) -> Result<Self, Self::Error> {
        let mut decoder = Self {
            metadata,
            type_sizes: HashMap::new(),
            marker: PhantomData,
        };
        decoder.register_type_size::<T::Hash>("Hash")?;
        decoder.register_type_size::<<T as Balances>::Balance>("Balance")?;
        Ok(decoder)
    }
}

impl<T: System + Balances + 'static> EventsDecoder<T> {
    pub fn register_type_size<U>(&mut self, name: &str) -> Result<usize, EventsError>
    where
        U: Default + Codec + Send + 'static,
    {
        let size = U::default().encode().len();
        if size > 0 {
            self.type_sizes.insert(name.to_string(), size);
            Ok(size)
        } else {
            Err(EventsError::TypeSizeUnavailable(name.to_owned()))
        }
    }

    fn decode_raw_bytes<I: Input, W: Output>(
        &self,
        args: &[EventArg],
        input: &mut I,
        output: &mut W,
    ) -> Result<(), EventsError> {
        for arg in args {
            match arg {
                EventArg::Vec(arg) => {
                    let len = <Compact<u32>>::decode(input)?;
                    len.encode_to(output);
                    for _ in 0..len.0 {
                        self.decode_raw_bytes(&[*arg.clone()], input, output)?
                    }
                }
                EventArg::Tuple(args) => {
                    self.decode_raw_bytes(args, input, output)?
                }
                EventArg::Primitive(name) => {
                    if let Some(size) = self.type_sizes.get(name) {
                        let mut buf = vec![0; *size];
                        input.read(&mut buf)?;
                        buf.encode_to(output);
                    } else {
                        return Err(EventsError::TypeSizeUnavailable(name.to_owned()))
                    }
                }
            }
        }
        Ok(())
    }

    pub fn decode_events(
        &self,
        input: &mut &[u8],
    ) -> Result<Vec<(Phase, RawEvent)>, EventsError> {
        let compact_len = <Compact<u32>>::decode(input)?;
        let len = compact_len.0 as usize;

        use substrate_primitives::hexdisplay::HexDisplay;

        let mut r = Vec::new();
        for _ in 0..len {
            // decode EventRecord
            let phase = Phase::decode(input)?;
            let module_variant = input.read_byte()? as u8;
            let event_variant = input.read_byte()? as u8;

            let module_name = self.metadata.module_name(module_variant)?;
            let module = self.metadata.module(&module_name)?;
            let event_metadata = module.event(event_variant)?;
            log::debug!("decoding event '{}::{}'", module_name, event_metadata.name);

            let mut event_data = Vec::<u8>::new();
            self.decode_raw_bytes(&event_metadata.arguments(), input, &mut event_data)?;
            // topics come after the event data in EventRecord
            let _topics = Vec::<T::Hash>::decode(input)?;

            let raw_event = RawEvent {
                module: module_name.clone(),
                variant: event_metadata.name.clone(),
                data: event_data,
            };
            r.push((phase, raw_event));
        }
        Ok(r)
    }
}
