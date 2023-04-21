// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    client::{OfflineClientT, OnlineClientT},
    config::{Config, Hasher, Header},
    dynamic::DecodedValueThunk,
    error::{BlockError, Error},
    events,
    metadata::DecodeWithMetadata,
    rpc::types::ChainBlockResponse,
    runtime_api::RuntimeApi,
    storage::Storage,
    Metadata,
};
use codec::{Codec, Decode, Encode};
use derivative::Derivative;
use frame_metadata::v15::RuntimeMetadataV15;
use futures::lock::Mutex as AsyncMutex;
use scale_value::Value;
use std::{collections::HashMap, sync::Arc};

/// A representation of a block.
pub struct Block<T: Config, C> {
    header: T::Header,
    client: C,
    // Since we obtain the same events for every extrinsic, let's
    // cache them so that we only ever do that once:
    cached_events: CachedEvents<T>,
}

// A cache for our events so we don't fetch them more than once when
// iterating over events for extrinsics.
type CachedEvents<T> = Arc<AsyncMutex<Option<events::Events<T>>>>;

impl<T, C> Block<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    pub(crate) fn new(header: T::Header, client: C) -> Self {
        Block {
            header,
            client,
            cached_events: Default::default(),
        }
    }

    /// Return the block hash.
    pub fn hash(&self) -> T::Hash {
        self.header.hash()
    }

    /// Return the block number.
    pub fn number(&self) -> <T::Header as crate::config::Header>::Number {
        self.header().number()
    }

    /// Return the entire block header.
    pub fn header(&self) -> &T::Header {
        &self.header
    }
}

impl<T, C> Block<T, C>
where
    T: Config,
    C: OnlineClientT<T>,
{
    /// Return the events associated with the block, fetching them from the node if necessary.
    pub async fn events(&self) -> Result<events::Events<T>, Error> {
        get_events(&self.client, self.header.hash(), &self.cached_events).await
    }

    /// Fetch and return the block body.
    pub async fn body(&self) -> Result<BlockBody<T, C>, Error> {
        let block_hash = self.header.hash();
        let block_details = match self.client.rpc().block(Some(block_hash)).await? {
            Some(block) => block,
            None => return Err(BlockError::not_found(block_hash).into()),
        };

        Ok(BlockBody::new(
            self.client.clone(),
            block_details,
            self.cached_events.clone(),
        ))
    }

    /// Work with storage.
    pub fn storage(&self) -> Storage<T, C> {
        let block_hash = self.hash();
        Storage::new(self.client.clone(), block_hash)
    }

    /// Execute a runtime API call at this block.
    pub async fn runtime_api(&self) -> Result<RuntimeApi<T, C>, Error> {
        Ok(RuntimeApi::new(self.client.clone(), self.hash()))
    }
}

/// Generic type IDs passed to the `UncheckedExtrinsic`.
#[derive(Debug, Copy, Clone)]
struct UncheckedExtrinsicIds {
    address: u32,
    call: u32,
    signature: u32,
    extra: u32,
}

#[derive(Debug)]
enum UncheckedExtrinsicIdsError {
    /// Extrinsic type ID cannot be resolved with the provided metadata.
    MissingType,
    /// Extrinsic is missing a type parameter.
    MissingTypeParam,
}

impl UncheckedExtrinsicIds {
    /// Extract the generic type parameters IDs from the extrinsic type.
    fn new(metadata: &RuntimeMetadataV15) -> Result<Self, UncheckedExtrinsicIdsError> {
        const ADDRESS: &str = "Address";
        const CALL: &str = "Call";
        const SIGNATURE: &str = "Signature";
        const EXTRA: &str = "Extra";

        let id = metadata.extrinsic.ty.id;

        let Some(ty ) = metadata.types.resolve(id) else {
            return Err(UncheckedExtrinsicIdsError::MissingType);
        };

        let params: HashMap<_, _> = ty
            .type_params
            .iter()
            .map(|ty_param| {
                let Some(ty) = ty_param.ty else {
                    return Err(UncheckedExtrinsicIdsError::MissingTypeParam);
                };

                Ok((ty_param.name.as_str(), ty.id))
            })
            .collect::<Result<_, _>>()?;

        let Some(address) = params.get(ADDRESS) else {
            return Err(UncheckedExtrinsicIdsError::MissingTypeParam);
        };
        let Some(call) = params.get(CALL) else {
            return Err(UncheckedExtrinsicIdsError::MissingTypeParam);
        };
        let Some(signature) = params.get(SIGNATURE) else {
            return Err(UncheckedExtrinsicIdsError::MissingTypeParam);
        };
        let Some(extra) = params.get(EXTRA) else {
            return Err(UncheckedExtrinsicIdsError::MissingTypeParam);
        };

        Ok(UncheckedExtrinsicIds {
            address: *address,
            call: *call,
            signature: *signature,
            extra: *extra,
        })
    }
}

/// The body of a block.
pub struct BlockBody<T: Config, C> {
    details: ChainBlockResponse<T>,
    client: C,
    cached_events: CachedEvents<T>,
}

impl<T, C> BlockBody<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    pub(crate) fn new(
        client: C,
        details: ChainBlockResponse<T>,
        cached_events: CachedEvents<T>,
    ) -> Self {
        Self {
            details,
            client,
            cached_events,
        }
    }

    /// Returns an iterator over the extrinsics in the block body.
    pub fn extrinsics(&self) -> impl Iterator<Item = Extrinsic<'_, T, C>> {
        let ids = UncheckedExtrinsicIds::new(self.client.metadata().runtime_metadata())
            .expect("Should have IDS; qed");

        // let runtime_call_id = runtime_call_id(self.client.metadata().runtime_metadata());

        self.details
            .block
            .extrinsics
            .iter()
            .enumerate()
            .map(move |(idx, e)| Extrinsic {
                index: idx as u32,
                bytes: &e.0,
                client: self.client.clone(),
                block_hash: self.details.block.header.hash(),
                cached_events: self.cached_events.clone(),
                ids,
                _marker: std::marker::PhantomData,
            })
    }
}

/// A single extrinsic in a block.
pub struct Extrinsic<'a, T: Config, C> {
    index: u32,
    bytes: &'a [u8],
    client: C,
    block_hash: T::Hash,
    cached_events: CachedEvents<T>,
    ids: UncheckedExtrinsicIds,
    _marker: std::marker::PhantomData<T>,
}

// pub trait SignedExtension: Codec + Sync + Send + Clone + Eq + PartialEq {
//     /// Unique identifier of this signed extension.
//     ///
//     /// This will be exposed in the metadata to identify the signed extension used
//     /// in an extrinsic.
//     const IDENTIFIER: &'static str;

//     /// The type which encodes the sender identity.
//     type AccountId;

//     /// The type which encodes the call to be dispatched.
//     type Call;

//     /// Any additional data that will go into the signed payload. This may be created dynamically
//     /// from the transaction using the `additional_signed` function.
//     type AdditionalSigned: Encode;

//     /// The type that encodes information that can be passed from pre_dispatch to post-dispatch.
//     type Pre;
// }

/// A extrinsic right from the external world.
pub struct GenericExtrinsic<Address, Signature, Extra>
where
    Address: Decode,
    Signature: Decode,
    Extra: Decode,
{
    /// The signature, address, number of extrinsics have come before from
    /// the same signer and an era describing the longevity of this transaction,
    /// if this is a signed extrinsic.
    pub signature: Option<(Address, Signature, Extra)>,
    /// The function that should be called.
    pub function: DecodedValueThunk,
}

pub struct ExtrinsicValueThunk {
    /// The signature, address, number of extrinsics have come before from
    /// the same signer and an era describing the longevity of this transaction,
    /// if this is a signed extrinsic.
    pub signature: Option<(DecodedValueThunk, DecodedValueThunk, DecodedValueThunk)>,
    /// The function that should be called.
    pub function: DecodedValueThunk,
}

impl<'a, T, C> Extrinsic<'a, T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    /// The index of the extrinsic in the block.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// The bytes of the extrinsic.
    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Decode the extrinsic to the provided return type.
    pub fn decode(&self) -> Result<GenericExtrinsic<T::Address, T::Signature, ()>, ExtrinsicError> {
        const SIGNATURE_MASK: u8 = 0b1000_0000;
        const VERSION_MASK: u8 = 0b0111_1111;
        const LATEST_EXTRINSIC_VERSION: u8 = 4;

        // Extrinsic are encoded in memory in the following way:
        //   - first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
        //   - signature: [unknown TBD with metadata].
        //   - extrinsic data
        if self.bytes.is_empty() {
            return Err(ExtrinsicError::InsufficientData);
        }

        let version = self.bytes[0] & VERSION_MASK;
        if version != LATEST_EXTRINSIC_VERSION {
            return Err(ExtrinsicError::UnsupportedVersion(version));
        }

        let is_signed = self.bytes[0] & SIGNATURE_MASK != 0;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.bytes[1..]);

        let cursor = &mut &bytes[..];
        let signature = is_signed
            .then(|| Decode::decode(cursor))
            .transpose()
            .map_err(|err| ExtrinsicError::DecodingError(err))?;

        let metadata = self.client.metadata();

        let function = <DecodedValueThunk as DecodeWithMetadata>::decode_with_metadata(
            cursor,
            self.ids.call,
            &metadata,
        )
        .map_err(|_| ExtrinsicError::InsufficientData)?;

        // let function: Ext =
        // Decode::decode(cursor).map_err(|err| ExtrinsicError::DecodingError(err))?;

        Ok(GenericExtrinsic {
            signature,
            function,
        })
    }

    /// Decode the extrinsic to the provided return type.
    pub fn decode_generic(&self) -> Result<ExtrinsicValueThunk, ExtrinsicError> {
        const SIGNATURE_MASK: u8 = 0b1000_0000;
        const VERSION_MASK: u8 = 0b0111_1111;
        const LATEST_EXTRINSIC_VERSION: u8 = 4;

        // Extrinsic are encoded in memory in the following way:
        //   - first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
        //   - signature: [unknown TBD with metadata].
        //   - extrinsic data
        if self.bytes.is_empty() {
            return Err(ExtrinsicError::InsufficientData);
        }

        println!("byteslen {:?}", self.bytes.len());
        println!("bytes {:?}", self.bytes);

        let version = self.bytes[0] & VERSION_MASK;
        if version != LATEST_EXTRINSIC_VERSION {
            return Err(ExtrinsicError::UnsupportedVersion(version));
        }

        let is_signed = self.bytes[0] & SIGNATURE_MASK != 0;
        println!("is signed {:?}", is_signed);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.bytes[1..]);

        println!("byteslen {:?}", bytes.len());


        let cursor = &mut &bytes[..];

        let metadata = self.client.metadata();

        let signature = if is_signed {

            println!("bytes {:?}", cursor);

            // When the payload is signed, the signature is encoded as
            // Option<(Address, Signature, Extra)>. Get rid of the first
            // bit which signals the option.
            let is_option: u8 = Decode::decode(cursor).map_err(|_| ExtrinsicError::InsufficientData)?;
            if is_option != 1 {
                println!("Is this option or now: {:?}", is_option);
                // return Err(ExtrinsicError::InvalidSignature);
            }

            // skip over the bytes
            let len = cursor.len();
            let address = <DecodedValueThunk as DecodeWithMetadata>::decode_with_metadata(
                cursor,
                self.ids.address,
                &metadata,
            )
            .map_err(|_| ExtrinsicError::InsufficientData)?;
            let after = cursor.len();
            println!("Curson length: {:?} {:?}", len, after);

            let signature = <DecodedValueThunk as DecodeWithMetadata>::decode_with_metadata(
                cursor,
                self.ids.signature,
                &metadata,
            )
            .map_err(|_| ExtrinsicError::InsufficientData)?;

            let extra: DecodedValueThunk =
                <DecodedValueThunk as DecodeWithMetadata>::decode_with_metadata(
                    cursor,
                    self.ids.extra,
                    &metadata,
                )
                .map_err(|_| ExtrinsicError::InsufficientData)?;

            Some((address, signature, extra))
        } else {
            None
        };

        // let signature = is_signed
        //     .then(|| Decode::decode(cursor))
        //     .transpose()
        //     .map_err(|err| ExtrinsicError::DecodingError(err))?;

        // let metadata = self.client.metadata();

        let function = <DecodedValueThunk as DecodeWithMetadata>::decode_with_metadata(
            cursor,
            self.ids.call,
            &metadata,
        )
        .map_err(|_| ExtrinsicError::InsufficientData)?;

        // let function: Ext =
        // Decode::decode(cursor).map_err(|err| ExtrinsicError::DecodingError(err))?;

        Ok(ExtrinsicValueThunk {
            signature,
            function,
        })
    }
}

impl<'a, T, C> Extrinsic<'a, T, C>
where
    T: Config,
    C: OnlineClientT<T>,
{
    // /// Decode the extrinsic to the provided return type.
    // pub fn decode<Ext: Decode>(
    //     &self,
    // ) -> Result<GenericExtrinsic<T::Address, Ext, T::Signature, ()>, ExtrinsicError> {
    //     const SIGNATURE_MASK: u8 = 0b1000_0000;
    //     const VERSION_MASK: u8 = 0b0111_1111;
    //     const LATEST_EXTRINSIC_VERSION: u8 = 4;

    //     // Extrinsic are encoded in memory in the following way:
    //     //   - Compact<u32>: Length of the extrinsic
    //     //   - first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
    //     //   - signature: [unknown TBD with metadata].
    //     //   - extrinsic data
    //     if self.bytes.is_empty() {
    //         return Err(ExtrinsicError::InsufficientData);
    //     }

    //     let version = self.bytes[0] & VERSION_MASK;
    //     if version != LATEST_EXTRINSIC_VERSION {
    //         return Err(ExtrinsicError::UnsupportedVersion(version));
    //     }

    //     let is_signed = self.bytes[0] & SIGNATURE_MASK != 0;

    //     let mut bytes = Vec::new();
    //     bytes.extend_from_slice(&self.bytes[1..]);

    //     let cursor = &mut &bytes[..];
    //     let signature = is_signed
    //         .then(|| Decode::decode(cursor))
    //         .transpose()
    //         .map_err(|err| ExtrinsicError::DecodingError(err))?;

    //     let metadata = self.client.metadata();
    //     let value = <DecodedValueThunk as DecodeWithMetadata>::decode_with_metadata(
    //         cursor,
    //         ,
    //         &metadata,
    //     )?;

    //     let function: Ext =
    //         Decode::decode(cursor).map_err(|err| ExtrinsicError::DecodingError(err))?;

    //     Ok(GenericExtrinsic {
    //         signature,
    //         function,
    //     })
    // }

    /// The events associated with the extrinsic.
    pub async fn events(&self) -> Result<ExtrinsicEvents<T>, Error> {
        let events = get_events(&self.client, self.block_hash, &self.cached_events).await?;
        let ext_hash = T::Hasher::hash_of(&self.bytes);
        Ok(ExtrinsicEvents::new(ext_hash, self.index, events))
    }
}

/// Error resulted from decoding an extrinsic.
#[derive(Debug)]
pub enum ExtrinsicError {
    /// Expected more extrinsic bytes.
    InsufficientData,
    /// Unsupported signature.
    SignatureUnsupported,
    /// Invalid signature.
    InvalidSignature, 
    /// The extrinsic has an unsupported version.
    UnsupportedVersion(u8),
    /// Decoding error.
    DecodingError(codec::Error),
}

/// The events associated with a given extrinsic.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct ExtrinsicEvents<T: Config> {
    // The hash of the extrinsic (handy to expose here because
    // this type is returned from TxProgress things in the most
    // basic flows, so it's the only place people can access it
    // without complicating things for themselves).
    ext_hash: T::Hash,
    // The index of the extrinsic:
    idx: u32,
    // All of the events in the block:
    events: events::Events<T>,
}

impl<T: Config> ExtrinsicEvents<T> {
    pub(crate) fn new(ext_hash: T::Hash, idx: u32, events: events::Events<T>) -> Self {
        Self {
            ext_hash,
            idx,
            events,
        }
    }

    /// Return the hash of the block that the extrinsic is in.
    pub fn block_hash(&self) -> T::Hash {
        self.events.block_hash()
    }

    /// The index of the extrinsic that these events are produced from.
    pub fn extrinsic_index(&self) -> u32 {
        self.idx
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> T::Hash {
        self.ext_hash
    }

    /// Return all of the events in the block that the extrinsic is in.
    pub fn all_events_in_block(&self) -> &events::Events<T> {
        &self.events
    }

    /// Iterate over all of the raw events associated with this transaction.
    ///
    /// This works in the same way that [`events::Events::iter()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn iter(&self) -> impl Iterator<Item = Result<events::EventDetails, Error>> + '_ {
        self.events.iter().filter(|ev| {
            ev.as_ref()
                .map(|ev| ev.phase() == events::Phase::ApplyExtrinsic(self.idx))
                .unwrap_or(true) // Keep any errors.
        })
    }

    /// Find all of the transaction events matching the event type provided as a generic parameter.
    ///
    /// This works in the same way that [`events::Events::find()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn find<Ev: events::StaticEvent>(&self) -> impl Iterator<Item = Result<Ev, Error>> + '_ {
        self.iter().filter_map(|ev| {
            ev.and_then(|ev| ev.as_event::<Ev>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the transaction events using metadata to dynamically decode and skip
    /// them, and return the first event found which decodes to the provided `Ev` type.
    ///
    /// This works in the same way that [`events::Events::find_first()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn find_first<Ev: events::StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.find::<Ev>().next().transpose()
    }

    /// Iterate through the transaction events using metadata to dynamically decode and skip
    /// them, and return the last event found which decodes to the provided `Ev` type.
    ///
    /// This works in the same way that [`events::Events::find_last()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn find_last<Ev: events::StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.find::<Ev>().last().transpose()
    }

    /// Find an event in those associated with this transaction. Returns true if it was found.
    ///
    /// This works in the same way that [`events::Events::has()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn has<Ev: events::StaticEvent>(&self) -> Result<bool, Error> {
        Ok(self.find::<Ev>().next().transpose()?.is_some())
    }
}

// Return Events from the cache, or fetch from the node if needed.
async fn get_events<C, T>(
    client: &C,
    block_hash: T::Hash,
    cached_events: &AsyncMutex<Option<events::Events<T>>>,
) -> Result<events::Events<T>, Error>
where
    T: Config,
    C: OnlineClientT<T>,
{
    // Acquire lock on the events cache. We either get back our events or we fetch and set them
    // before unlocking, so only one fetch call should ever be made. We do this because the
    // same events can be shared across all extrinsics in the block.
    let lock = cached_events.lock().await;
    let events = match &*lock {
        Some(events) => events.clone(),
        None => {
            events::EventsClient::new(client.clone())
                .at(block_hash)
                .await?
        }
    };

    Ok(events)
}
