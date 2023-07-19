// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    blocks::block_types::{get_events, CachedEvents},
    client::{OfflineClientT, OnlineClientT},
    config::{Config, Hasher},
    error::{BlockError, Error, MetadataError},
    events,
    metadata::types::PalletMetadata,
    rpc::types::ChainBlockExtrinsic,
    Metadata,
};

use crate::utils::strip_compact_prefix;
use codec::Decode;
use derivative::Derivative;
use scale_decode::{DecodeAsFields, DecodeAsType};
use std::sync::Arc;

/// Trait to uniquely identify the extrinsic's identity from the runtime metadata.
///
/// Generated API structures that represent an extrinsic implement this trait.
///
/// The trait is utilized to decode emitted extrinsics from a block, via obtaining the
/// form of the `Extrinsic` from the metadata.
pub trait StaticExtrinsic: DecodeAsFields {
    /// Pallet name.
    const PALLET: &'static str;
    /// Call name.
    const CALL: &'static str;

    /// Returns true if the given pallet and call names match this extrinsic.
    fn is_extrinsic(pallet: &str, call: &str) -> bool {
        Self::PALLET == pallet && Self::CALL == call
    }
}

/// The body of a block.
pub struct Extrinsics<T: Config, C> {
    client: C,
    extrinsics: Vec<ChainBlockExtrinsic>,
    cached_events: CachedEvents<T>,
    ids: ExtrinsicPartTypeIds,
    hash: T::Hash,
}

impl<T, C> Extrinsics<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    pub(crate) fn new(
        client: C,
        extrinsics: Vec<ChainBlockExtrinsic>,
        cached_events: CachedEvents<T>,
        ids: ExtrinsicPartTypeIds,
        hash: T::Hash,
    ) -> Self {
        Self {
            client,
            extrinsics,
            cached_events,
            ids,
            hash,
        }
    }

    /// The number of extrinsics.
    pub fn len(&self) -> usize {
        self.extrinsics.len()
    }

    /// Are there no extrinsics in this block?
    // Note: mainly here to satisfy clippy.
    pub fn is_empty(&self) -> bool {
        self.extrinsics.is_empty()
    }

    /// Return the block hash that these extrinsics are from.
    pub fn block_hash(&self) -> T::Hash {
        self.hash
    }

    /// Returns an iterator over the extrinsics in the block body.
    // Dev note: The returned iterator is 'static + Send so that we can box it up and make
    // use of it with our `FilterExtrinsic` stuff.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<ExtrinsicDetails<T, C>, Error>> + Send + Sync + 'static {
        let extrinsics = self.extrinsics.clone();
        let num_extrinsics = self.extrinsics.len();
        let client = self.client.clone();
        let hash = self.hash;
        let cached_events = self.cached_events.clone();
        let ids = self.ids;
        let mut index = 0;

        std::iter::from_fn(move || {
            if index == num_extrinsics {
                None
            } else {
                match ExtrinsicDetails::decode_from(
                    index as u32,
                    &extrinsics[index].0,
                    client.clone(),
                    hash,
                    cached_events.clone(),
                    ids,
                ) {
                    Ok(extrinsic_details) => {
                        index += 1;
                        Some(Ok(extrinsic_details))
                    }
                    Err(e) => {
                        index = num_extrinsics;
                        Some(Err(e))
                    }
                }
            }
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return only those which should decode to the provided `E` type.
    /// If an error occurs, all subsequent iterations return `None`.
    pub fn find<E: StaticExtrinsic>(&self) -> impl Iterator<Item = Result<E, Error>> + '_ {
        self.iter().filter_map(|e| {
            e.and_then(|e| e.as_extrinsic::<E>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the first extrinsic found which decodes to the provided `E` type.
    pub fn find_first<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        self.find::<E>().next().transpose()
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the last extrinsic found which decodes to the provided `Ev` type.
    pub fn find_last<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        self.find::<E>().last().transpose()
    }

    /// Find an extrinsics that decodes to the type provided. Returns true if it was found.
    pub fn has<E: StaticExtrinsic>(&self) -> Result<bool, Error> {
        Ok(self.find::<E>().next().transpose()?.is_some())
    }
}

/// A single extrinsic in a block.
pub struct ExtrinsicDetails<T: Config, C> {
    /// The index of the extrinsic in the block.
    index: u32,
    /// Extrinsic bytes.
    bytes: Arc<[u8]>,
    /// True if the extrinsic payload is signed.
    is_signed: bool,
    /// The start index in the `bytes` from which the address is encoded.
    address_start_idx: usize,
    /// The end index of the address in the encoded `bytes`.
    address_end_idx: usize,
    /// The start index in the `bytes` from which the call is encoded.
    call_start_idx: usize,
    /// The pallet index.
    pallet_index: u8,
    /// The variant index.
    variant_index: u8,
    /// The block hash of this extrinsic (needed to fetch events).
    block_hash: T::Hash,
    /// Subxt client.
    client: C,
    /// Cached events.
    cached_events: CachedEvents<T>,
    /// Subxt metadata to fetch the extrinsic metadata.
    metadata: Metadata,
    _marker: std::marker::PhantomData<T>,
}

impl<T, C> ExtrinsicDetails<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    // Attempt to dynamically decode a single extrinsic from the given input.
    pub(crate) fn decode_from(
        index: u32,
        extrinsic_bytes: &[u8],
        client: C,
        block_hash: T::Hash,
        cached_events: CachedEvents<T>,
        ids: ExtrinsicPartTypeIds,
    ) -> Result<ExtrinsicDetails<T, C>, Error> {
        const SIGNATURE_MASK: u8 = 0b1000_0000;
        const VERSION_MASK: u8 = 0b0111_1111;
        const LATEST_EXTRINSIC_VERSION: u8 = 4;

        let metadata = client.metadata();

        // removing the compact encoded prefix:
        let bytes: Arc<[u8]> = strip_compact_prefix(extrinsic_bytes)?.1.into();

        // Extrinsic are encoded in memory in the following way:
        //   - first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
        //   - signature: [unknown TBD with metadata].
        //   - extrinsic data
        let first_byte: u8 = Decode::decode(&mut &bytes[..])?;

        let version = first_byte & VERSION_MASK;
        if version != LATEST_EXTRINSIC_VERSION {
            return Err(BlockError::UnsupportedVersion(version).into());
        }

        let is_signed = first_byte & SIGNATURE_MASK != 0;

        // Skip over the first byte which denotes the version and signing.
        let cursor = &mut &bytes[1..];

        let mut address_start_idx = 0;
        let mut address_end_idx = 0;

        if is_signed {
            address_start_idx = bytes.len() - cursor.len();

            // Skip over the address, signature and extra fields.
            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.address,
                metadata.types(),
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;
            address_end_idx = bytes.len() - cursor.len();

            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.signature,
                metadata.types(),
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;

            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.extra,
                metadata.types(),
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;
        }

        let call_start_idx = bytes.len() - cursor.len();

        // Decode the pallet index, then the call variant.
        let cursor = &mut &bytes[call_start_idx..];

        let pallet_index: u8 = Decode::decode(cursor)?;
        let variant_index: u8 = Decode::decode(cursor)?;

        Ok(ExtrinsicDetails {
            index,
            bytes,
            is_signed,
            address_start_idx,
            address_end_idx,
            call_start_idx,
            pallet_index,
            variant_index,
            block_hash,
            client,
            cached_events,
            metadata,
            _marker: std::marker::PhantomData,
        })
    }

    /// Is the extrinsic signed?
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }

    /// The index of the extrinsic in the block.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Return _all_ of the bytes representing this extrinsic, which include, in order:
    /// - First byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
    /// - SignatureType (if the payload is signed)
    ///   - Address
    ///   - Signature
    ///   - Extra fields
    /// - Extrinsic call bytes
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return only the bytes representing this extrinsic call:
    /// - First byte is the pallet index
    /// - Second byte is the variant (call) index
    /// - Followed by field bytes.
    ///
    /// # Note
    ///
    /// Please use [`Self::bytes`] if you want to get all extrinsic bytes.
    pub fn call_bytes(&self) -> &[u8] {
        &self.bytes[self.call_start_idx..]
    }

    /// Return the bytes representing the fields stored in this extrinsic.
    ///
    /// # Note
    ///
    /// This is a subset of [`Self::call_bytes`] that does not include the
    /// first two bytes that denote the pallet index and the variant index.
    pub fn field_bytes(&self) -> &[u8] {
        // Note: this cannot panic because we checked the extrinsic bytes
        // to contain at least two bytes.
        &self.call_bytes()[2..]
    }

    /// Return only the bytes of the address that signed this extrinsic.
    ///
    /// # Note
    ///
    /// Returns `None` if the extrinsic is not signed.
    pub fn address_bytes(&self) -> Option<&[u8]> {
        self.is_signed
            .then(|| &self.bytes[self.address_start_idx..self.address_end_idx])
    }

    /// The index of the pallet that the extrinsic originated from.
    pub fn pallet_index(&self) -> u8 {
        self.pallet_index
    }

    /// The index of the extrinsic variant that the extrinsic originated from.
    pub fn variant_index(&self) -> u8 {
        self.variant_index
    }

    /// The name of the pallet from whence the extrinsic originated.
    pub fn pallet_name(&self) -> Result<&str, Error> {
        Ok(self.extrinsic_metadata()?.pallet.name())
    }

    /// The name of the call (ie the name of the variant that it corresponds to).
    pub fn variant_name(&self) -> Result<&str, Error> {
        Ok(&self.extrinsic_metadata()?.variant.name)
    }

    /// Fetch the metadata for this extrinsic.
    pub fn extrinsic_metadata(&self) -> Result<ExtrinsicMetadataDetails, Error> {
        let pallet = self.metadata.pallet_by_index_err(self.pallet_index())?;
        let variant = pallet
            .call_variant_by_index(self.variant_index())
            .ok_or_else(|| MetadataError::VariantIndexNotFound(self.variant_index()))?;

        Ok(ExtrinsicMetadataDetails { pallet, variant })
    }

    /// Decode and provide the extrinsic fields back in the form of a [`scale_value::Composite`]
    /// type which represents the named or unnamed fields that were present in the extrinsic.
    pub fn field_values(
        &self,
    ) -> Result<scale_value::Composite<scale_value::scale::TypeId>, Error> {
        let bytes = &mut self.field_bytes();
        let extrinsic_metadata = self.extrinsic_metadata()?;

        let mut fields = extrinsic_metadata
            .variant
            .fields
            .iter()
            .map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));
        let decoded = <scale_value::Composite<scale_value::scale::TypeId>>::decode_as_fields(
            bytes,
            &mut fields,
            self.metadata.types(),
        )?;

        Ok(decoded)
    }

    /// Attempt to decode these [`ExtrinsicDetails`] into a type representing the extrinsic fields.
    /// Such types are exposed in the codegen as `pallet_name::calls::types::CallName` types.
    pub fn as_extrinsic<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        let extrinsic_metadata = self.extrinsic_metadata()?;
        if extrinsic_metadata.pallet.name() == E::PALLET
            && extrinsic_metadata.variant.name == E::CALL
        {
            let mut fields = extrinsic_metadata
                .variant
                .fields
                .iter()
                .map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));
            let decoded =
                E::decode_as_fields(&mut self.field_bytes(), &mut fields, self.metadata.types())?;
            Ok(Some(decoded))
        } else {
            Ok(None)
        }
    }

    /// Attempt to decode these [`ExtrinsicDetails`] into an outer call enum type (which includes
    /// the pallet and extrinsic enum variants as well as the extrinsic fields). A compatible
    /// type for this is exposed via static codegen as a root level `Call` type.
    pub fn as_root_extrinsic<E: DecodeAsType>(&self) -> Result<E, Error> {
        let decoded = E::decode_as_type(
            &mut &self.call_bytes()[..],
            self.metadata.outer_enums().call_enum_ty(),
            self.metadata.types(),
        )?;

        Ok(decoded)
    }
}

impl<T, C> ExtrinsicDetails<T, C>
where
    T: Config,
    C: OnlineClientT<T>,
{
    /// The events associated with the extrinsic.
    pub async fn events(&self) -> Result<ExtrinsicEvents<T>, Error> {
        let events = get_events(&self.client, self.block_hash, &self.cached_events).await?;
        let ext_hash = T::Hasher::hash_of(&self.bytes);
        Ok(ExtrinsicEvents::new(ext_hash, self.index, events))
    }
}

/// Details for the given extrinsic plucked from the metadata.
pub struct ExtrinsicMetadataDetails<'a> {
    pub pallet: PalletMetadata<'a>,
    pub variant: &'a scale_info::Variant<scale_info::form::PortableForm>,
}

/// The type IDs extracted from the metadata that represent the
/// generic type parameters passed to the `UncheckedExtrinsic` from
/// the substrate-based chain.
#[derive(Debug, Copy, Clone)]
pub(crate) struct ExtrinsicPartTypeIds {
    /// The address (source) of the extrinsic.
    address: u32,
    /// The extrinsic call type.
    // Note: the call type can be used to skip over the extrinsic bytes to check
    // they are in line with our metadata. This operation is currently postponed.
    _call: u32,
    /// The signature of the extrinsic.
    signature: u32,
    /// The extra parameters of the extrinsic.
    extra: u32,
}

impl ExtrinsicPartTypeIds {
    /// Extract the generic type parameters IDs from the extrinsic type.
    pub(crate) fn new(metadata: &Metadata) -> Result<Self, BlockError> {
        Ok(ExtrinsicPartTypeIds {
            address: metadata.extrinsic().address_ty(),
            _call: metadata.extrinsic().call_ty(),
            signature: metadata.extrinsic().signature_ty(),
            extra: metadata.extrinsic().extra_ty(),
        })
    }
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
    pub fn iter(&self) -> impl Iterator<Item = Result<events::EventDetails<T>, Error>> + '_ {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rpc::types::RuntimeVersion, OfflineClient, PolkadotConfig};
    use assert_matches::assert_matches;
    use codec::{Decode, Encode};
    use frame_metadata::v15::{CustomMetadata, OuterEnums};
    use frame_metadata::{
        v15::{ExtrinsicMetadata, PalletCallMetadata, PalletMetadata, RuntimeMetadataV15},
        RuntimeMetadataPrefixed,
    };
    use primitive_types::H256;
    use scale_info::{meta_type, TypeInfo};
    use scale_value::Value;

    // Extrinsic needs to contain at least the generic type parameter "Call"
    // for the metadata to be valid.
    // The "Call" type from the metadata is used to decode extrinsics.
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct ExtrinsicType<Address, Call, Signature, Extra> {
        pub signature: Option<(Address, Signature, Extra)>,
        pub function: Call,
    }

    // Because this type is used to decode extrinsics, we expect this to be a TypeDefVariant.
    // Each pallet must contain one single variant.
    #[allow(unused)]
    #[derive(
        Encode,
        Decode,
        TypeInfo,
        Clone,
        Debug,
        PartialEq,
        Eq,
        scale_encode::EncodeAsType,
        scale_decode::DecodeAsType,
    )]
    enum RuntimeCall {
        Test(Pallet),
    }

    // The calls of the pallet.
    #[allow(unused)]
    #[derive(
        Encode,
        Decode,
        TypeInfo,
        Clone,
        Debug,
        PartialEq,
        Eq,
        scale_encode::EncodeAsType,
        scale_decode::DecodeAsType,
    )]
    enum Pallet {
        #[allow(unused)]
        #[codec(index = 2)]
        TestCall {
            value: u128,
            signed: bool,
            name: String,
        },
    }

    #[allow(unused)]
    #[derive(
        Encode,
        Decode,
        TypeInfo,
        Clone,
        Debug,
        PartialEq,
        Eq,
        scale_encode::EncodeAsType,
        scale_decode::DecodeAsType,
    )]
    struct TestCallExtrinsic {
        value: u128,
        signed: bool,
        name: String,
    }

    impl StaticExtrinsic for TestCallExtrinsic {
        const PALLET: &'static str = "Test";
        const CALL: &'static str = "TestCall";
    }

    /// Build fake metadata consisting the types needed to represent an extrinsic.
    fn metadata() -> Metadata {
        let pallets = vec![PalletMetadata {
            name: "Test",
            storage: None,
            calls: Some(PalletCallMetadata {
                ty: meta_type::<Pallet>(),
            }),
            event: None,
            constants: vec![],
            error: None,
            index: 0,
            docs: vec![],
        }];

        let extrinsic = ExtrinsicMetadata {
            version: 4,
            signed_extensions: vec![],
            address_ty: meta_type::<()>(),
            call_ty: meta_type::<RuntimeCall>(),
            signature_ty: meta_type::<()>(),
            extra_ty: meta_type::<()>(),
        };

        let meta = RuntimeMetadataV15::new(
            pallets,
            extrinsic,
            meta_type::<()>(),
            vec![],
            OuterEnums {
                call_enum_ty: meta_type::<RuntimeCall>(),
                event_enum_ty: meta_type::<()>(),
                error_enum_ty: meta_type::<()>(),
            },
            CustomMetadata {
                map: Default::default(),
            },
        );
        let runtime_metadata: RuntimeMetadataPrefixed = meta.into();

        Metadata::new(runtime_metadata.try_into().unwrap())
    }

    /// Build an offline client to work with the test metadata.
    fn client(metadata: Metadata) -> OfflineClient<PolkadotConfig> {
        // Create the encoded extrinsic bytes.
        let rt_version = RuntimeVersion {
            spec_version: 1,
            transaction_version: 4,
            other: Default::default(),
        };
        let block_hash = H256::random();
        OfflineClient::new(block_hash, rt_version, metadata)
    }

    #[test]
    fn extrinsic_metadata_consistency() {
        let metadata = metadata();

        // Except our metadata to contain the registered types.
        let pallet = metadata.pallet_by_index(0).expect("pallet exists");
        let extrinsic = pallet
            .call_variant_by_index(2)
            .expect("metadata contains the RuntimeCall enum with this pallet");

        assert_eq!(pallet.name(), "Test");
        assert_eq!(&extrinsic.name, "TestCall");
    }

    #[test]
    fn insufficient_extrinsic_bytes() {
        let metadata = metadata();
        let client = client(metadata.clone());
        let ids = ExtrinsicPartTypeIds::new(&metadata).unwrap();

        // Decode with empty bytes.
        let result =
            ExtrinsicDetails::decode_from(1, &[], client, H256::random(), Default::default(), ids);
        assert_matches!(result.err(), Some(crate::Error::Codec(_)));
    }

    #[test]
    fn unsupported_version_extrinsic() {
        let metadata = metadata();
        let client = client(metadata.clone());
        let ids = ExtrinsicPartTypeIds::new(&metadata).unwrap();

        // Decode with invalid version.
        let result = ExtrinsicDetails::decode_from(
            1,
            &vec![3u8].encode(),
            client,
            H256::random(),
            Default::default(),
            ids,
        );

        assert_matches!(
            result.err(),
            Some(crate::Error::Block(
                crate::error::BlockError::UnsupportedVersion(3)
            ))
        );
    }

    #[test]
    fn statically_decode_extrinsic() {
        let metadata = metadata();
        let client = client(metadata.clone());
        let ids = ExtrinsicPartTypeIds::new(&metadata).unwrap();

        let tx = crate::tx::dynamic(
            "Test",
            "TestCall",
            vec![
                Value::u128(10),
                Value::bool(true),
                Value::string("SomeValue"),
            ],
        );
        let tx_encoded = client
            .tx()
            .create_unsigned(&tx)
            .expect("Valid dynamic parameters are provided");

        // Note: `create_unsigned` produces the extrinsic bytes by prefixing the extrinsic length.
        // The length is handled deserializing `ChainBlockExtrinsic`, therefore the first byte is not needed.
        let extrinsic = ExtrinsicDetails::decode_from(
            1,
            tx_encoded.encoded(),
            client,
            H256::random(),
            Default::default(),
            ids,
        )
        .expect("Valid extrinsic");

        assert!(!extrinsic.is_signed());

        assert_eq!(extrinsic.index(), 1);

        assert_eq!(extrinsic.pallet_index(), 0);
        assert_eq!(
            extrinsic
                .pallet_name()
                .expect("Valid metadata contains pallet name"),
            "Test"
        );

        assert_eq!(extrinsic.variant_index(), 2);
        assert_eq!(
            extrinsic
                .variant_name()
                .expect("Valid metadata contains variant name"),
            "TestCall"
        );

        // Decode the extrinsic to the root enum.
        let decoded_extrinsic = extrinsic
            .as_root_extrinsic::<RuntimeCall>()
            .expect("can decode extrinsic to root enum");

        assert_eq!(
            decoded_extrinsic,
            RuntimeCall::Test(Pallet::TestCall {
                value: 10,
                signed: true,
                name: "SomeValue".into(),
            })
        );

        // Decode the extrinsic to the extrinsic variant.
        let decoded_extrinsic = extrinsic
            .as_extrinsic::<TestCallExtrinsic>()
            .expect("can decode extrinsic to extrinsic variant")
            .expect("value cannot be None");

        assert_eq!(
            decoded_extrinsic,
            TestCallExtrinsic {
                value: 10,
                signed: true,
                name: "SomeValue".into(),
            }
        );
    }
}
