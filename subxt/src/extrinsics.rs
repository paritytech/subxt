//! This module exposes [`ExtrinsicsClient`], which has methods for working with extrinsics.
//! It's created by calling [`crate::client::ClientAtBlock::extrinsics()`].
//!
//! ```rust,no_run
//! pub use subxt::{OnlineClient, PolkadotConfig};
//!
//! let client = OnlineClient::new().await?;
//! let at_block = client.at_current_block().await?;
//!
//! let extrinsics = at_block.extrinsics();
//! ```

mod decode_as_extrinsic;
mod extrinsic_transaction_extensions;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::{Config, HashFor, Hasher};
use crate::error::{
    EventsError, ExtrinsicDecodeErrorAt, ExtrinsicDecodeErrorAtReason, ExtrinsicError,
};
use crate::events::{self, DecodeAsEvent};
use frame_decode::extrinsics::Extrinsic as ExtrinsicInfo;
use scale_decode::{DecodeAsFields, DecodeAsType};
use std::marker::PhantomData;
use std::sync::Arc;
use subxt_metadata::Metadata;

pub use decode_as_extrinsic::DecodeAsExtrinsic;
pub use extrinsic_transaction_extensions::{
    ExtrinsicTransactionExtension, ExtrinsicTransactionExtensions,
};

/// A client for working with extrinsics. See [the module docs](crate::extrinsics) for more.
pub struct ExtrinsicsClient<'atblock, T, Client> {
    client: &'atblock Client,
    marker: PhantomData<T>,
}

impl<'atblock, T, Client> ExtrinsicsClient<'atblock, T, Client> {
    pub(crate) fn new(client: &'atblock Client) -> Self {
        ExtrinsicsClient {
            client,
            marker: PhantomData,
        }
    }
}

impl<'atblock, T: Config, Client: OfflineClientAtBlockT<T>> ExtrinsicsClient<'atblock, T, Client> {
    /// Work with the block body bytes given.
    ///
    /// No attempt to validate the provided bytes is made here; if invalid bytes are
    /// provided then attempting to iterate and decode them will fail.
    pub async fn from_bytes(&self, extrinsics: Vec<Vec<u8>>) -> Extrinsics<'atblock, T, Client> {
        Extrinsics {
            client: self.client,
            extrinsics: Arc::new(extrinsics),
            marker: PhantomData,
        }
    }
}

impl<'atblock, T: Config, Client: OnlineClientAtBlockT<T>> ExtrinsicsClient<'atblock, T, Client> {
    /// Fetch the extrinsics at this block.
    pub async fn fetch(&self) -> Result<Extrinsics<'atblock, T, Client>, ExtrinsicError> {
        let client = self.client;
        let block_hash = client.block_hash();
        let extrinsics = client
            .backend()
            .block_body(block_hash)
            .await
            .map_err(ExtrinsicError::CannotGetBlockBody)?
            .ok_or_else(|| ExtrinsicError::BlockNotFound(block_hash.into()))?;

        Ok(Extrinsics {
            client,
            extrinsics: Arc::new(extrinsics),
            marker: PhantomData,
        })
    }
}

/// The extrinsics in a block.
#[derive(Debug, Clone)]
pub struct Extrinsics<'atblock, T, C> {
    client: &'atblock C,
    extrinsics: Arc<Vec<Vec<u8>>>,
    marker: PhantomData<T>,
}

impl<'atblock, T: Config, C: OfflineClientAtBlockT<T>> Extrinsics<'atblock, T, C> {
    /// The number of extrinsics.
    pub fn len(&self) -> usize {
        self.extrinsics.len()
    }

    /// Are there no extrinsics in this block?
    // Note: mainly here to satisfy clippy.
    pub fn is_empty(&self) -> bool {
        self.extrinsics.is_empty()
    }

    /// Returns an iterator over the extrinsics in the block body. We decode the extrinsics on
    /// demand as we iterate, and so if any fail to decode an error will be returned.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<Extrinsic<'_, T, C>, ExtrinsicDecodeErrorAt>> {
        let hasher = self.client.hasher();
        let metadata = self.client.metadata_ref();
        let client = self.client;
        let all_extrinsic_bytes = self.extrinsics.clone();

        self.extrinsics
            .iter()
            .enumerate()
            .map(move |(extrinsic_index, extrinsic_bytes)| {
                let cursor = &mut &**extrinsic_bytes;

                // Try to decode the extrinsic.
                let info =
                    frame_decode::extrinsics::decode_extrinsic(cursor, metadata, metadata.types())
                        .map_err(|error| ExtrinsicDecodeErrorAt {
                            extrinsic_index,
                            error: ExtrinsicDecodeErrorAtReason::DecodeError(error),
                        })?
                        .into_owned();

                // We didn't consume all bytes, so decoding probably failed.
                if !cursor.is_empty() {
                    return Err(ExtrinsicDecodeErrorAt {
                        extrinsic_index,
                        error: ExtrinsicDecodeErrorAtReason::LeftoverBytes(cursor.to_vec()),
                    });
                }

                Ok(Extrinsic {
                    client: client,
                    index: extrinsic_index,
                    info: Arc::new(info),
                    extrinsics: Arc::clone(&all_extrinsic_bytes),
                    hasher,
                    metadata,
                })
            })
    }

    /// Iterate through the extrinsics, Decoding and returning any that match the given type.
    ///
    /// This is a convenience function for calling [`Self::iter`] and then [`Extrinsic::decode_call_data_fields_as`]
    /// on each extrinsic that we iterate over, filtering those that don't match.
    pub fn find<E: DecodeAsExtrinsic>(&self) -> impl Iterator<Item = Result<E, ExtrinsicError>> {
        self.iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.decode_call_data_fields_as::<E>())
    }

    /// Find the first extrinsic matching the given type, returning `None` if it doesn't exist,
    /// and the result of decoding it if it does.
    pub fn find_first<E: DecodeAsExtrinsic>(&self) -> Option<Result<E, ExtrinsicError>> {
        self.find::<E>().next()
    }

    /// Find an extrinsic matching the given type, returning true if it exists. This function does _not_
    /// try to actually decode the extrinsic bytes into the given type.
    pub fn has<E: DecodeAsExtrinsic>(&self) -> bool {
        self.iter().filter_map(|e| e.ok()).any(|e| e.is::<E>())
    }
}

/// A single extrinsic in a block.
pub struct Extrinsic<'atblock, T: Config, C> {
    client: &'atblock C,
    /// The index of the extrinsic in the block.
    index: usize,
    /// Information about the extrinsic
    info: Arc<ExtrinsicInfo<'atblock, u32>>,
    /// All extrinsic bytes. use the index to select the correct bytes.
    extrinsics: Arc<Vec<Vec<u8>>>,
    /// Hash the extrinsic if we want.
    hasher: &'atblock T::Hasher,
    /// Subxt metadata to fetch the extrinsic metadata.
    metadata: &'atblock Metadata,
}

impl<'atblock, T, C> Extrinsic<'atblock, T, C>
where
    T: Config,
    C: OfflineClientAtBlockT<T>,
{
    /// Calculate and return the hash of the extrinsic, based on the configured hasher.
    pub fn hash(&self) -> HashFor<T> {
        self.hasher.hash(&self.extrinsics[self.index])
    }

    /// Is the extrinsic signed?
    pub fn is_signed(&self) -> bool {
        self.info.is_signed()
    }

    /// The index of the extrinsic in the block.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The index of the pallet that the extrinsic originated from.
    pub fn pallet_index(&self) -> u8 {
        self.info.pallet_index()
    }

    /// The index of the extrinsic variant that the extrinsic originated from.
    pub fn call_index(&self) -> u8 {
        self.info.call_index()
    }

    /// The name of the pallet from whence the extrinsic originated.
    pub fn pallet_name(&self) -> &str {
        self.info.pallet_name()
    }

    /// The name of the call (ie the name of the variant that it corresponds to).
    pub fn call_name(&self) -> &str {
        self.info.call_name()
    }

    /// Return the extrinsic bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.extrinsics[self.index]
    }

    /// Return only the bytes representing this extrinsic call:
    /// - First byte is the pallet index
    /// - Second byte is the variant (call) index
    /// - Followed by field bytes.
    ///
    /// # Note
    ///
    /// Please use [`Self::bytes`] if you want to get all extrinsic bytes.
    pub fn call_data_bytes(&self) -> &[u8] {
        &self.bytes()[self.info.call_data_range()]
    }

    /// Return the bytes representing the fields stored in this extrinsic.
    ///
    /// # Note
    ///
    /// This is a subset of [`Self::call_data_bytes`] that does not include the
    /// first two bytes that denote the pallet index and the variant index.
    pub fn call_data_field_bytes(&self) -> &[u8] {
        &self.bytes()[self.info.call_data_args_range()]
    }

    /// Return only the bytes of the address that signed this extrinsic.
    ///
    /// # Note
    ///
    /// Returns `None` if the extrinsic is not signed.
    pub fn address_bytes(&self) -> Option<&[u8]> {
        self.info
            .signature_payload()
            .map(|s| &self.bytes()[s.address_range()])
    }

    /// Returns Some(signature_bytes) if the extrinsic was signed otherwise None is returned.
    pub fn signature_bytes(&self) -> Option<&[u8]> {
        self.info
            .signature_payload()
            .map(|s| &self.bytes()[s.signature_range()])
    }

    /// Returns the signed extension `extra` bytes of the extrinsic.
    /// Each signed extension has an `extra` type (May be zero-sized).
    /// These bytes are the scale encoded `extra` fields of each signed extension in order of the signed extensions.
    /// They do *not* include the `additional` signed bytes that are used as part of the payload that is signed.
    ///
    /// Note: Returns `None` if the extrinsic is not signed.
    pub fn transaction_extensions_bytes(&self) -> Option<&[u8]> {
        self.info
            .transaction_extension_payload()
            .map(|t| &self.bytes()[t.range()])
    }

    /// Returns `None` if the extrinsic is not signed.
    pub fn transaction_extensions(
        &self,
    ) -> Option<ExtrinsicTransactionExtensions<'atblock, '_, T>> {
        let bytes = self.bytes();
        let metadata = self.metadata;

        self.info
            .transaction_extension_payload()
            .map(move |t| ExtrinsicTransactionExtensions::new(bytes, metadata, t))
    }

    /// Return true if this [`Extrinsic`] matches the provided type.
    pub fn is<E: DecodeAsExtrinsic>(&self) -> bool {
        E::is_extrinsic(self.pallet_name(), self.call_name())
    }

    /// Attempt to decode this [`Extrinsic`] into an outer call enum type (which includes
    /// the pallet and extrinsic enum variants as well as the extrinsic fields). One compatible
    /// type for this is exposed via static codegen as a root level `Call` type.
    pub fn decode_call_data_as<E: DecodeAsType>(&self) -> Result<E, ExtrinsicError> {
        let decoded = E::decode_as_type(
            &mut &self.call_data_bytes()[..],
            self.metadata.outer_enums().call_enum_ty(),
            self.metadata.types(),
        )
        .map_err(|e| ExtrinsicError::CannotDecodeIntoRootExtrinsic {
            extrinsic_index: self.index as usize,
            error: e,
        })?;

        Ok(decoded)
    }

    /// Decode the extrinsic call data fields into some type which implements [`DecodeAsExtrinsic`].
    ///
    /// Extrinsic types generated via the [`macro@crate::subxt`] macro implement this.
    pub fn decode_call_data_fields_as<E: DecodeAsExtrinsic>(
        &self,
    ) -> Option<Result<E, ExtrinsicError>> {
        if self.is::<E>() {
            Some(self.decode_call_data_fields_unchecked_as::<E>())
        } else {
            None
        }
    }

    /// Decode the extrinsic call data fields into some type which implements [`DecodeAsFields`].
    ///
    /// This ignores the pallet and call name information, so you should check those via [`Self::pallet_name()`]
    /// and [`Self::call_name()`] to confirm that this extrinsic is the one you are intending to decode.
    ///
    /// Prefer to use [`Self::decode_call_data_fields_as`] where possible.
    pub fn decode_call_data_fields_unchecked_as<E: DecodeAsFields>(
        &self,
    ) -> Result<E, ExtrinsicError> {
        let bytes = &mut self.call_data_field_bytes();
        let mut fields = self.info.call_data().map(|d| {
            let name = if d.name().is_empty() {
                None
            } else {
                Some(d.name())
            };
            scale_decode::Field::new(*d.ty(), name)
        });
        let decoded =
            E::decode_as_fields(bytes, &mut fields, self.metadata.types()).map_err(|e| {
                ExtrinsicError::CannotDecodeFields {
                    extrinsic_index: self.index as usize,
                    error: e,
                }
            })?;

        Ok(decoded)
    }
}

impl<'atblock, T, C> Extrinsic<'atblock, T, C>
where
    T: Config,
    C: OnlineClientAtBlockT<T>,
{
    /// The events associated with the extrinsic.
    pub async fn events(&self) -> Result<ExtrinsicEvents<T>, EventsError> {
        ExtrinsicEvents::fetch(self.client, self.hash(), self.index()).await
    }
}

/// The events associated with a given extrinsic.
#[derive(Debug)]
pub struct ExtrinsicEvents<T: Config> {
    // The hash of the extrinsic (handy to expose here because
    // this type is returned from TxProgress things in the most
    // basic flows, so it's the only place people can access it
    // without complicating things for themselves).
    extrinsic_hash: HashFor<T>,
    // The index of the extrinsic:
    extrinsic_index: usize,
    // All of the events in the block:
    events: crate::events::Events<T>,
}

impl<T: Config> ExtrinsicEvents<T> {
    pub(crate) async fn fetch(
        client: &impl OnlineClientAtBlockT<T>,
        extrinsic_hash: HashFor<T>,
        extrinsic_index: usize,
    ) -> Result<Self, EventsError> {
        let events = crate::events::EventsClient::new(client).fetch().await?;
        Ok(ExtrinsicEvents {
            extrinsic_hash,
            extrinsic_index,
            events,
        })
    }

    /// The index of the extrinsic that these events are produced from.
    pub fn extrinsic_index(&self) -> usize {
        self.extrinsic_index
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> HashFor<T> {
        self.extrinsic_hash
    }

    /// Return all of the events in the block that the extrinsic is in.
    pub fn all_events_in_block(&self) -> &events::Events<T> {
        &self.events
    }

    /// Iterate over all of the raw events associated with this extrinsic.
    ///
    /// This works in the same way that [`events::Events::iter()`] does, with the
    /// exception that it filters out events not related to the current extrinsic.
    pub fn iter(&'_ self) -> impl Iterator<Item = Result<events::Event<'_, T>, EventsError>> {
        self.events.iter().filter(|ev| {
            ev.as_ref()
                .map(|ev| ev.phase() == events::Phase::ApplyExtrinsic(self.extrinsic_index as u32))
                .unwrap_or(true) // Keep any errors.
        })
    }

    /// Iterate through the extrinsic's events, Decoding and returning any that match the given type.
    ///
    /// This is a convenience function for calling [`Self::iter`] and then [`events::Event::decode_fields_as`]
    /// on each event that we iterate over, filtering those that don't match.
    pub fn find<E: DecodeAsEvent>(&self) -> impl Iterator<Item = Result<E, EventsError>> {
        self.iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.decode_fields_as::<E>())
    }

    /// Find the first event matching the given type, returning `None` if it doesn't exist,
    /// and the result of decoding it if it does.
    pub fn find_first<E: DecodeAsEvent>(&self) -> Option<Result<E, EventsError>> {
        self.find::<E>().next()
    }

    /// Find an event matching the given type, returning true if it exists. This function does _not_
    /// try to actually decode the event bytes into the given type.
    pub fn has<E: DecodeAsEvent>(&self) -> bool {
        self.iter().filter_map(|e| e.ok()).any(|e| e.is::<E>())
    }
}
