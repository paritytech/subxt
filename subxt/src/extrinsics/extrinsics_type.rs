// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    client::{OfflineClientT, OnlineClientT},
    config::{Config, Hasher, Header},
    dynamic::DecodedValueThunk,
    error::{BlockError, Error, ExtrinsicError},
    events,
    metadata::DecodeWithMetadata,
    rpc::types::{ChainBlock, ChainBlockExtrinsic, ChainBlockResponse},
    runtime_api::RuntimeApi,
    storage::Storage,
    Metadata,
};
use codec::{Codec, Decode, Encode};
use derivative::Derivative;
use frame_metadata::v15::RuntimeMetadataV15;
use futures::lock::Mutex as AsyncMutex;
use scale_value::{scale::decode_as_type, Value};
use std::{collections::HashMap, marker::PhantomData, sync::Arc};

/// A collection of extrinsics obtained from a block, bundled with the necessary
/// information needed to decode and iterate over them.
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct Extrinsics<T: Config> {
    metadata: Metadata,
    /// The block hash.
    hash: T::Hash,
    /// The accompanying extrinsics.
    extrinsics: Vec<ChainBlockExtrinsic>,
    /// Generic extrinsic parameter ids from the metadata.
    ids: ExtrinsicIds,
}

impl<T: Config> Extrinsics<T> {
    pub(crate) fn new(metadata: Metadata, block: ChainBlock<T>) -> Result<Self, Error> {
        let ids = ExtrinsicIds::new(metadata.runtime_metadata())?;

        Ok(Self {
            metadata,
            hash: block.header.hash(),
            extrinsics: block.extrinsics,
            ids,
        })
    }

    // /// Obtain the extrinsics from a block hash given custom metadata and a client.
    // ///
    // /// This method gives users the ability to inspect the extrinsics of older blocks,
    // /// where the metadata changed. For those cases, the user is responsible for
    // /// providing a valid metadata.
    // pub async fn new_from_client<Client>(
    //     metadata: Metadata,
    //     block_hash: T::Hash,
    //     client: Client,
    // ) -> Result<Self, Error>
    // where
    //     Client: OnlineClientT<T>,
    // {
    //     // let event_bytes = get_event_bytes(&client, Some(block_hash)).await?;
    //     // Ok(Events::new(metadata, block_hash, event_bytes))
    // }

    /// The number of extrinsics.
    pub fn len(&self) -> usize {
        self.extrinsics.len()
    }

    /// Are there no extrinsics in this block?
    pub fn is_empty(&self) -> bool {
        self.extrinsics.len() == 0
    }

    /// Return the block hash that these extrinsics are from.
    pub fn block_hash(&self) -> T::Hash {
        self.hash
    }

    /// Iterate over all of the events, using metadata to dynamically
    /// decode them as we go, and returning the raw bytes and other associated
    /// details. If an error occurs, all subsequent iterations return `None`.
    // Dev note: The returned iterator is 'static + Send so that we can box it up and make
    // use of it with our `FilterEvents` stuff.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<ExtrinsicDetails, Error>> + Send + Sync + 'static {
        let metadata = self.metadata.clone();
        // TODO: Dummy clone should use Arc<[u8]>.
        let extrinsics = self.extrinsics.clone();
        let ids = self.ids.clone();
        let num_extr = self.extrinsics.len();
        let mut index = 0;
        std::iter::from_fn(move || {
            if index == num_extr {
                None
            } else {
                match ExtrinsicDetails::decode_from::<T>(
                    metadata.clone(),
                    extrinsics[index].clone(),
                    ids,
                    index,
                ) {
                    Ok(event_details) => {
                        // Increment the index:
                        index += 1;
                        // Return the event details:
                        Some(Ok(event_details))
                    }
                    Err(e) => {
                        // By setting the position to the "end" of the event bytes,
                        // the cursor len will become 0 and the iterator will return `None`
                        // from now on:
                        index = num_extr;
                        Some(Err(e))
                    }
                }
            }
        })
    }

    // /// Iterate through the events using metadata to dynamically decode and skip
    // /// them, and return only those which should decode to the provided `Ev` type.
    // /// If an error occurs, all subsequent iterations return `None`.
    // pub fn find<Ev: StaticEvent>(&self) -> impl Iterator<Item = Result<Ev, Error>> + '_ {
    //     self.iter().filter_map(|ev| {
    //         ev.and_then(|ev| ev.as_event::<Ev>().map_err(Into::into))
    //             .transpose()
    //     })
    // }

    // /// Iterate through the events using metadata to dynamically decode and skip
    // /// them, and return the first event found which decodes to the provided `Ev` type.
    // pub fn find_first<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
    //     self.find::<Ev>().next().transpose()
    // }

    // /// Iterate through the events using metadata to dynamically decode and skip
    // /// them, and return the last event found which decodes to the provided `Ev` type.
    // pub fn find_last<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
    //     self.find::<Ev>().last().transpose()
    // }

    // /// Find an event that decodes to the type provided. Returns true if it was found.
    // pub fn has<Ev: StaticEvent>(&self) -> Result<bool, Error> {
    //     Ok(self.find::<Ev>().next().transpose()?.is_some())
    // }
}

/// The type IDs extracted from the metadata that represent the
/// generic type parameters passed to the `UncheckedExtrinsic` from
/// the substrate-based chain.
#[derive(Debug, Copy, Clone)]
struct ExtrinsicIds {
    /// The address (source) of the extrinsic.
    address: u32,
    /// The extrinsic call type.
    call: u32,
    /// The signature of the extrinsic.
    signature: u32,
    /// The extra parameters of the extrinsic.
    extra: u32,
}

impl ExtrinsicIds {
    /// Extract the generic type parameters IDs from the extrinsic type.
    fn new(metadata: &RuntimeMetadataV15) -> Result<Self, ExtrinsicError> {
        const ADDRESS: &str = "Address";
        const CALL: &str = "Call";
        const SIGNATURE: &str = "Signature";
        const EXTRA: &str = "Extra";

        let id = metadata.extrinsic.ty.id;

        let Some(ty) = metadata.types.resolve(id) else {
            return Err(ExtrinsicError::MissingType);
        };

        let params: HashMap<_, _> = ty
            .type_params
            .iter()
            .map(|ty_param| {
                let Some(ty) = ty_param.ty else {
                    return Err(ExtrinsicError::MissingType);
                };

                Ok((ty_param.name.as_str(), ty.id))
            })
            .collect::<Result<_, _>>()?;

        let Some(address) = params.get(ADDRESS) else {
            return Err(ExtrinsicError::MissingType);
        };
        let Some(call) = params.get(CALL) else {
            return Err(ExtrinsicError::MissingType);
        };
        let Some(signature) = params.get(SIGNATURE) else {
            return Err(ExtrinsicError::MissingType);
        };
        let Some(extra) = params.get(EXTRA) else {
            return Err(ExtrinsicError::MissingType);
        };

        Ok(ExtrinsicIds {
            address: *address,
            call: *call,
            signature: *signature,
            extra: *extra,
        })
    }
}

/// The extrinsic details.
#[derive(Debug, Clone)]
pub struct ExtrinsicDetails {
    ids: ExtrinsicIds,
    index: usize,
    metadata: Metadata,
    signature: Option<Value<u32>>,
}

impl ExtrinsicDetails {
    // Attempt to dynamically decode a single event from our events input.
    fn decode_from<T: Config>(
        metadata: Metadata,
        extrinsic: ChainBlockExtrinsic,
        ids: ExtrinsicIds,
        index: usize,
    ) -> Result<ExtrinsicDetails, Error> {
        const SIGNATURE_MASK: u8 = 0b1000_0000;
        const VERSION_MASK: u8 = 0b0111_1111;
        const LATEST_EXTRINSIC_VERSION: u8 = 4;

        let bytes = extrinsic.0;

        // Extrinsic are encoded in memory in the following way:
        //   - first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
        //   - signature: [unknown TBD with metadata].
        //   - extrinsic data
        if bytes.is_empty() {
            return Err(ExtrinsicError::InsufficientData.into());
        }

        let version = bytes[0] & VERSION_MASK;
        if version != LATEST_EXTRINSIC_VERSION {
            return Err(ExtrinsicError::UnsupportedVersion(version).into());
        }

        let is_signed = bytes[0] & SIGNATURE_MASK != 0;

        // Skip over the first byte which denotes the version and signing.
        let cursor = &mut &bytes[1..];

        let signature = if is_signed {
            let address = decode_as_type(cursor, ids.address, &metadata.runtime_metadata().types)
                .map_err(scale_decode::Error::from)?;

            let _signature =
                decode_as_type(cursor, ids.signature, &metadata.runtime_metadata().types)
                    .map_err(scale_decode::Error::from)?;

            let _extra = decode_as_type(cursor, ids.extra, &metadata.runtime_metadata().types)
                .map_err(scale_decode::Error::from)?;

            Some(address)
        } else {
            None
        };

        // Decode the extrinsic function call.
        let extrinsic = <DecodedValueThunk as DecodeWithMetadata>::decode_with_metadata(
            cursor, ids.call, &metadata,
        )?;

        Ok(ExtrinsicDetails {
            ids,
            index,
            metadata,
            signature,
        })
    }

    /// The index of the extrinsic in the given block.
    ///  What index is this event in the stored events for this block.
    pub fn index(&self) -> usize {
        self.index
    }

    // /// Return _all_ of the bytes representing this event, which include, in order:
    // /// - The phase.
    // /// - Pallet and event index.
    // /// - Event fields.
    // /// - Event Topics.
    // pub fn bytes(&self) -> &[u8] {
    //     &self.all_bytes[self.start_idx..self.end_idx]
    // }

    // /// Attempt to statically decode these [`EventDetails`] into a type representing the event
    // /// fields. This leans directly on [`codec::Decode`]. You can also attempt to decode the entirety
    // /// of the event using [`EventDetails::as_root_event()`], which is more lenient because it's able
    // /// to lean on [`scale_decode::DecodeAsType`].
    // pub fn as_event<E: StaticEvent>(&self) -> Result<Option<E>, Error> {
    //     let ev_metadata = self.event_metadata();
    //     if ev_metadata.pallet() == E::PALLET && ev_metadata.event() == E::EVENT {
    //         let decoded = E::decode_as_fields(
    //             &mut self.field_bytes(),
    //             ev_metadata.fields(),
    //             self.metadata.types(),
    //         )?;
    //         Ok(Some(decoded))
    //     } else {
    //         Ok(None)
    //     }
    // }
}
