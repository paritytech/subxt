// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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
use frame_support::dispatch::DispatchInfo;
use sp_runtime::{
    DispatchError,
    DispatchResult,
};
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    marker::{
        PhantomData,
        Send,
    },
};

use crate::{
    error::{
        Error,
        RuntimeError,
    },
    metadata::{
        EventArg,
        Metadata,
    },
    Phase,
    System,
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

/// Events decoder.
#[derive(Debug)]
pub struct EventsDecoder<T> {
    metadata: Metadata,
    type_sizes: HashMap<String, usize>,
    marker: PhantomData<fn() -> T>,
}

impl<T: System> EventsDecoder<T> {
    /// Creates a new `EventsDecoder`.
    pub fn new(metadata: Metadata) -> Self {
        let mut decoder = Self {
            metadata,
            type_sizes: HashMap::new(),
            marker: PhantomData,
        };
        // register default event arg type sizes for dynamic decoding of events
        decoder.register_type_size::<()>("PhantomData");
        decoder.register_type_size::<DispatchInfo>("DispatchInfo");
        decoder.register_type_size::<bool>("bool");
        decoder.register_type_size::<u32>("ReferendumIndex");
        decoder.register_type_size::<[u8; 16]>("Kind");
        decoder.register_type_size::<[u8; 32]>("AuthorityId");
        decoder.register_type_size::<u8>("u8");
        decoder.register_type_size::<u32>("u32");
        decoder.register_type_size::<u32>("AccountIndex");
        decoder.register_type_size::<u32>("SessionIndex");
        decoder.register_type_size::<u32>("PropIndex");
        decoder.register_type_size::<u32>("ProposalIndex");
        decoder.register_type_size::<u32>("AuthorityIndex");
        decoder.register_type_size::<u64>("AuthorityWeight");
        decoder.register_type_size::<u32>("MemberCount");
        decoder.register_type_size::<T::AccountId>("AccountId");
        decoder.register_type_size::<T::BlockNumber>("BlockNumber");
        decoder.register_type_size::<T::Hash>("Hash");
        decoder.register_type_size::<u8>("VoteThreshold");
        decoder
    }

    /// Register a type.
    pub fn register_type_size<U>(&mut self, name: &str) -> usize
    where
        U: Default + Codec + Send + 'static,
    {
        let size = U::default().encode().len();
        self.type_sizes.insert(name.to_string(), size);
        size
    }

    /// Check missing type sizes.
    pub fn check_missing_type_sizes(&self) {
        let mut missing = HashSet::new();
        for module in self.metadata.modules_with_events() {
            for event in module.events() {
                for arg in event.arguments() {
                    for primitive in arg.primitives() {
                        if !self.type_sizes.contains_key(&primitive) {
                            missing.insert(format!(
                                "{}::{}::{}",
                                module.name(),
                                event.name,
                                primitive
                            ));
                        }
                    }
                }
            }
        }
        if !missing.is_empty() {
            log::warn!(
                "The following primitive types do not have registered sizes: {:?} \
                If any of these events are received, an error will occur since we cannot decode them",
                missing
            );
        }
    }

    fn decode_raw_bytes<I: Input, W: Output>(
        &self,
        args: &[EventArg],
        input: &mut I,
        output: &mut W,
    ) -> Result<(), Error> {
        for arg in args {
            match arg {
                EventArg::Vec(arg) => {
                    let len = <Compact<u32>>::decode(input)?;
                    len.encode_to(output);
                    for _ in 0..len.0 {
                        self.decode_raw_bytes(&[*arg.clone()], input, output)?
                    }
                }
                EventArg::Option(arg) => {
                    match input.read_byte()? {
                        0 => output.push_byte(0),
                        1 => {
                            output.push_byte(1);
                            self.decode_raw_bytes(&[*arg.clone()], input, output)?
                        }
                        _ => {
                            return Err(Error::Other(
                                "unexpected first byte decoding Option".into(),
                            ))
                        }
                    }
                }
                EventArg::Tuple(args) => self.decode_raw_bytes(args, input, output)?,
                EventArg::Primitive(name) => {
                    let result = match name.as_str() {
                        "DispatchResult" => DispatchResult::decode(input)?,
                        "DispatchError" => Err(DispatchError::decode(input)?),
                        _ => {
                            if let Some(size) = self.type_sizes.get(name) {
                                let mut buf = vec![0; *size];
                                input.read(&mut buf)?;
                                output.write(&buf);
                                Ok(())
                            } else {
                                return Err(Error::TypeSizeUnavailable(name.to_owned()))
                            }
                        }
                    };
                    if let Err(error) = result {
                        return Err(
                            RuntimeError::from_dispatch(&self.metadata, error)?.into()
                        )
                    }
                }
            }
        }
        Ok(())
    }

    /// Decode events.
    pub fn decode_events(&self, input: &mut &[u8]) -> Result<Vec<(Phase, Raw)>, Error> {
        let compact_len = <Compact<u32>>::decode(input)?;
        let len = compact_len.0 as usize;

        let mut r = Vec::new();
        for _ in 0..len {
            // decode EventRecord
            let phase = Phase::decode(input)?;
            let module_variant = input.read_byte()?;

            let module = self.metadata.module_with_events(module_variant)?;
            let event_variant = input.read_byte()?;
            let event_metadata = module.event(event_variant)?;

            log::debug!(
                "received event '{}::{}'",
                module.name(),
                event_metadata.name
            );

            let mut event_data = Vec::<u8>::new();
            let result = self.decode_raw_bytes(
                &event_metadata.arguments(),
                input,
                &mut event_data,
            );
            let raw = match result {
                Ok(()) => {
                    log::debug!("raw bytes: {}", hex::encode(&event_data),);

                    let event = RawEvent {
                        module: module.name().to_string(),
                        variant: event_metadata.name.clone(),
                        data: event_data,
                    };

                    // topics come after the event data in EventRecord
                    let _topics = Vec::<T::Hash>::decode(input)?;
                    Raw::Event(event)
                }
                Err(Error::Runtime(err)) => Raw::Error(err),
                Err(err) => return Err(err),
            };

            r.push((phase, raw));
        }
        Ok(r)
    }
}

pub enum Raw {
    Event(RawEvent),
    Error(RuntimeError),
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestRuntime = crate::NodeTemplateRuntime;

    #[test]
    fn test_decode_option() {
        let decoder = EventsDecoder::<TestRuntime>::new(Metadata::default());

        let value = Some(0u8);
        let input = value.encode();
        let mut output = Vec::<u8>::new();

        decoder
            .decode_raw_bytes(
                &[EventArg::Option(Box::new(EventArg::Primitive(
                    "u8".to_string(),
                )))],
                &mut &input[..],
                &mut output,
            )
            .unwrap();

        assert_eq!(output, vec![1, 0]);
    }
}
