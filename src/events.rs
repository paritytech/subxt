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
    SystemEvent,
};
use log;
use parity_scale_codec::{
    Codec,
    Compact,
    Decode,
    Encode,
    Error as CodecError,
    Input,
    Output,
};
use srml_system::Phase;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    convert::TryFrom,
    marker::{
        PhantomData,
        Send,
    },
};

/// Top level Event that can be produced by a substrate runtime
#[derive(Debug)]
pub enum RuntimeEvent {
    System(SystemEvent),
    Raw(RawEvent),
}

/// Raw bytes for an Event
#[derive(Debug)]
pub struct RawEvent {
    /// The name of the module from whence the Event originated
    pub module: String,
    /// The name of the Event
    pub variant: String,
    /// The raw Event data
    pub data: Vec<u8>,
}

#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum EventsError {
    CodecError(CodecError),
    Metadata(MetadataError),
    #[display(fmt = "Type Sizes Missing: {:?}", _0)]
    TypeSizesMissing(Vec<String>),
    TypeSizeUnavailable(String),
}

pub struct EventsDecoder<T> {
    metadata: Metadata, // todo: [AJ] borrow?
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
        // register default event arg type sizes for dynamic decoding of events
        decoder.register_type_size::<bool>("bool")?;
        decoder.register_type_size::<u32>("ReferendumIndex")?;
        decoder.register_type_size::<[u8; 16]>("Kind")?;
        decoder.register_type_size::<[u8; 32]>("AuthorityId")?;
        decoder.register_type_size::<u8>("u8")?;
        decoder.register_type_size::<u32>("u32")?;
        decoder.register_type_size::<u32>("AccountIndex")?;
        decoder.register_type_size::<u32>("SessionIndex")?;
        decoder.register_type_size::<u32>("PropIndex")?;
        decoder.register_type_size::<u32>("ProposalIndex")?;
        decoder.register_type_size::<u32>("AuthorityIndex")?;
        decoder.register_type_size::<u64>("AuthorityWeight")?;
        decoder.register_type_size::<u32>("MemberCount")?;
        decoder.register_type_size::<T::AccountId>("AccountId")?;
        decoder.register_type_size::<T::BlockNumber>("BlockNumber")?;
        decoder.register_type_size::<T::Hash>("Hash")?;
        decoder.register_type_size::<<T as Balances>::Balance>("Balance")?;
        // VoteThreshold enum index
        decoder.register_type_size::<u8>("VoteThreshold")?;

        // Ignore these unregistered types, which are not fixed size primitives
        decoder.check_missing_type_sizes(vec![
            "DispatchError",
            "OpaqueTimeSlot",
            "rstd::marker::PhantomData<(AccountId, Event)>",
        ])?;
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

    fn check_missing_type_sizes<I: IntoIterator<Item = &'static str>>(
        &self,
        ignore: I,
    ) -> Result<(), Vec<String>> {
        let mut missing = HashSet::new();
        let mut ignore_set = HashSet::new();
        ignore_set.extend(ignore);
        for module in self.metadata.modules() {
            for event in module.events() {
                for arg in event.arguments() {
                    for primitive in arg.primitives() {
                        if !self.type_sizes.contains_key(&primitive)
                            && !ignore_set.contains(primitive.as_str())
                        {
                            missing.insert(primitive);
                        }
                    }
                }
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing.into_iter().collect())
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
                EventArg::Tuple(args) => self.decode_raw_bytes(args, input, output)?,
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
    ) -> Result<Vec<(Phase, RuntimeEvent)>, EventsError> {
        let compact_len = <Compact<u32>>::decode(input)?;
        let len = compact_len.0 as usize;

        let mut r = Vec::new();
        for _ in 0..len {
            // decode EventRecord
            let phase = Phase::decode(input)?;
            let module_variant = input.read_byte()? as u8;

            let module_name = self.metadata.module_name(module_variant)?;
            let event = if module_name == "System" {
                let system_event = SystemEvent::decode(input)?;
                RuntimeEvent::System(system_event)
            } else {
                let event_variant = input.read_byte()? as u8;
                let module = self.metadata.module(&module_name)?;
                let event_metadata = module.event(event_variant)?;
                log::debug!("decoding event '{}::{}'", module_name, event_metadata.name);

                let mut event_data = Vec::<u8>::new();
                self.decode_raw_bytes(
                    &event_metadata.arguments(),
                    input,
                    &mut event_data,
                )?;
                RuntimeEvent::Raw(RawEvent {
                    module: module_name.clone(),
                    variant: event_metadata.name.clone(),
                    data: event_data,
                })
            };

            // topics come after the event data in EventRecord
            let _topics = Vec::<T::Hash>::decode(input)?;
            r.push((phase, event));
        }
        Ok(r)
    }
}
