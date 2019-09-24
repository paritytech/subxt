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

use parity_scale_codec::{Codec, Decode, Compact, Error as CodecError, Input, Output, Encode};
use crate::{System, Error, metadata::{Metadata, EventArg}};



use std::collections::HashMap;



use srml_system::{Phase};

use std::marker::{Send, PhantomData};
use std::fs::metadata;
use crate::metadata::{MetadataError};
use crate::srml::balances::Balances;
use std::convert::TryFrom;

pub struct RawEvent {
    module: String,
    variant: String,
    data: Vec<u8>,
}

pub struct EventsDecoder<T> {
    metadata: Metadata, // todo: borrow?
    type_sizes: HashMap<String, usize>,
    marker: PhantomData<fn() -> T>,
}

#[derive(Debug, derive_more::From)]
pub enum EventError {
    CodecError(CodecError),
    Metadata(MetadataError),
    TypeSizesMissing(String, String),
    TypeSizeUnavailable(String),
}

impl<T: System + Balances + 'static> TryFrom<Metadata> for EventsDecoder<T> {
    type Error = EventError;

    fn try_from(value: Metadata) -> Result<Self, Self::Error> {
        let mut listener = Self { metadata, type_sizes: HashMap::new(), marker: PhantomData };
        listener.register_type_size::<T::Hash>("Hash")?;
        listener.register_type_size::<<T as Balances>::Balance>("Balance")?;
        listener.check_all_event_types()?;
        listener
    }
}

impl<T: System + Balances + 'static> EventsDecoder<T> {
    pub fn register_type_size<U>(&mut self, name: &str) -> Result<usize, EventError> where U: Default + Codec + Send + 'static {
        let size = U::default().size_hint();
        if size > 0 {
            self.type_sizes.insert(name.to_string(), size);
            Ok(size)
        } else {
            Err(EventError::TypeSizeUnavailable(name.to_owned()))
        }
    }

    fn decode_raw_bytes<I: Input, W: Output>(&self, args: &[EventArg], input: &mut I, output: &mut W) -> Result<(), EventError> {
        for arg in args {
            match arg {
                EventArg::Vec(arg) => {
                    let len = <Compact<u32>>::decode(input)?;
                    len.encode_to(output);
                    for _ in 0..len {
                        self.decode_raw_bytes(arg, input, output)?
                    }
                },
                EventArg::Tuple(args) => {
                    for arg in args {
                        self.decode_raw_bytes(arg, input, output)?
                    }
                },
                EventArg::Primitive(name) => {
                    if let Some(size) = self.type_sizes.get(name) {
                        let mut buf = [0u8; size];
                        input.read(&mut buf[..])?;
                        buf.encode_to(output)
                    } else {
                        Err(EventError::TypeSizeUnavailable(name.to_owned()))
                    }
                }
            }
        }
        Ok(())
    }

    pub fn decode_events(&self, input: &mut &[u8]) -> Result<Vec<(Phase, RawEvent)>, Error> {
        let compact_len = <Compact<u32>>::decode(input)?;
        let len = len.0 as usize;

        let mut r = Vec::new();
        for _ in 0..len {
            // decode EventRecord
            let phase = Phase::decode(input)?;
            let module_variant = u8::decode(input)?;
            let event_variant = u8::decode(input)?;
            let _topics = Vec::<T::Hash>::decode(input)?;

            let module_name = self.metadata.module_name(module_variant)?;
            let module = self.metadata.module(&module_name)?;
            let event_metadata = module.event(event_variant)?;

            let mut event_data = Vec::<u8>::new();
            self.decode_raw_bytes(&event_metadata.arguments, input, event_data)?;
            let raw_event = RawEvent {
                module: module_name,
                variant: event_metadata.name,
                data: event_data,
            };

            println!("phase {:?}, module {:?}, event {:?}", phase, module_name, event_metadata.name);
            r.push((phase, raw_event));
        }
        Ok(r)
    }
}
