// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::BlockError;
use crate::blocks::extrinsic_signed_extensions::ExtrinsicSignedExtensions;
use crate::{
    config::{Config, Hasher},
    error::{Error, MetadataError},
    Metadata,
};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ops::Deref;
use scale_decode::DecodeAsType;
use subxt_metadata::PalletMetadata;
use frame_decode::extrinsics::Extrinsic;

pub use crate::blocks::StaticExtrinsic;

/// The body of a block.
pub struct Extrinsics<T: Config> {
    extrinsics: Vec<Arc<(Extrinsic<'static, u32>, Vec<u8>)>>,
    metadata: Metadata,
    _marker: core::marker::PhantomData<T>,
}

impl<T: Config> Extrinsics<T> {
    /// Instantiate a new [`Extrinsics`] object, given a vector containing
    /// each extrinsic hash (in the form of bytes) and some metadata that
    /// we'll use to decode them.
    pub fn decode_from(extrinsics: Vec<Vec<u8>>, metadata: Metadata) -> Result<Self, Error> {
        let extrinsics = extrinsics.into_iter().map(|bytes| {
            let cursor = &mut &*bytes;

            // Try to decode the extrinsic.
            let decoded_info =
                frame_decode::extrinsics::decode_extrinsic(cursor, metadata.deref(), metadata.types())
                    .map_err(BlockError::ExtrinsicDecodeError)?
                    .into_owned();
    
            // We didn't consume all bytes, so decoding probably failed.
            if !cursor.is_empty() {
                return Err(BlockError::LeftoverBytes(cursor.len()).into());
            }

            Ok(Arc::new((decoded_info, bytes)))
        }).collect::<Result<_,Error>>()?;

        Ok(Self {
            extrinsics,
            metadata,
            _marker: core::marker::PhantomData,
        })
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

    /// Returns an iterator over the extrinsics in the block body.
    // Dev note: The returned iterator is 'static + Send so that we can box it up and make
    // use of it with our `FilterExtrinsic` stuff.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ExtrinsicDetails<T>> + Send + Sync + 'static {
        let extrinsics = self.extrinsics.clone();
        let num_extrinsics = self.extrinsics.len();
        let metadata = self.metadata.clone();

        (0..num_extrinsics).map(move |index| {
            ExtrinsicDetails::new(
                index as u32,
                extrinsics[index].clone(),
                metadata.clone(),
            )
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return only those which should decode to the provided `E` type.
    /// If an error occurs, all subsequent iterations return `None`.
    pub fn find<E: StaticExtrinsic>(
        &self,
    ) -> impl Iterator<Item = Result<FoundExtrinsic<T, E>, Error>> + '_ {
        self.iter().filter_map(|details| {
            match details.as_extrinsic::<E>() {
                // Failed to decode extrinsic:
                Err(err) => Some(Err(err)),
                // Extrinsic for a different pallet / different call (skip):
                Ok(None) => None,
                Ok(Some(value)) => Some(Ok(FoundExtrinsic { details, value })),
            }
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the first extrinsic found which decodes to the provided `E` type.
    pub fn find_first<E: StaticExtrinsic>(&self) -> Result<Option<FoundExtrinsic<T, E>>, Error> {
        self.find::<E>().next().transpose()
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the last extrinsic found which decodes to the provided `Ev` type.
    pub fn find_last<E: StaticExtrinsic>(&self) -> Result<Option<FoundExtrinsic<T, E>>, Error> {
        self.find::<E>().last().transpose()
    }

    /// Find an extrinsics that decodes to the type provided. Returns true if it was found.
    pub fn has<E: StaticExtrinsic>(&self) -> Result<bool, Error> {
        Ok(self.find::<E>().next().transpose()?.is_some())
    }
}

/// A single extrinsic in a block.
pub struct ExtrinsicDetails<T: Config> {
    /// The index of the extrinsic in the block.
    index: u32,
    /// Extrinsic bytes and decode info.
    ext: Arc<(Extrinsic<'static, u32>, Vec<u8>)>,
    /// Subxt metadata to fetch the extrinsic metadata.
    metadata: Metadata,
    _marker: core::marker::PhantomData<T>,
}

impl<T> ExtrinsicDetails<T>
where
    T: Config,
{
    // Attempt to dynamically decode a single extrinsic from the given input.
    #[doc(hidden)]
    pub fn new(
        index: u32,
        ext: Arc<(Extrinsic<'static, u32>, Vec<u8>)>,
        metadata: Metadata,
    ) -> ExtrinsicDetails<T> {
        ExtrinsicDetails {
            index,
            ext,
            metadata,
            _marker: core::marker::PhantomData,
        }
    }

    /// Calculate and return the hash of the extrinsic, based on the configured hasher.
    pub fn hash(&self) -> T::Hash {
        // Use hash(), not hash_of(), because we don't want to double encode the bytes.
        T::Hasher::hash(&self.bytes())
    }

    /// Is the extrinsic signed?
    pub fn is_signed(&self) -> bool {
        self.decoded_info().is_signed()
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
        &self.ext.1
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
        &self.bytes()[self.decoded_info().call_data_range()]
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
        self.decoded_info()
            .signature_payload()
            .map(|s| &self.bytes()[s.address_range()])
    }

    /// Returns Some(signature_bytes) if the extrinsic was signed otherwise None is returned.
    pub fn signature_bytes(&self) -> Option<&[u8]> {
        self.decoded_info()
            .signature_payload()
            .map(|s| &self.bytes()[s.signature_range()])
    }

    /// Returns the signed extension `extra` bytes of the extrinsic.
    /// Each signed extension has an `extra` type (May be zero-sized).
    /// These bytes are the scale encoded `extra` fields of each signed extension in order of the signed extensions.
    /// They do *not* include the `additional` signed bytes that are used as part of the payload that is signed.
    ///
    /// Note: Returns `None` if the extrinsic is not signed.
    pub fn signed_extensions_bytes(&self) -> Option<&[u8]> {
        self.decoded_info()
            .transaction_extension_payload()
            .map(|t| &self.bytes()[t.range()])
    }

    /// Returns `None` if the extrinsic is not signed.
    pub fn signed_extensions(&self) -> Option<ExtrinsicSignedExtensions<'_, T>> {
        self.decoded_info()
            .transaction_extension_payload()
            .map(|t| ExtrinsicSignedExtensions::new(self.bytes(), &self.metadata, t))
    }

    /// The index of the pallet that the extrinsic originated from.
    pub fn pallet_index(&self) -> u8 {
        self.decoded_info().pallet_index()
    }

    /// The index of the extrinsic variant that the extrinsic originated from.
    pub fn variant_index(&self) -> u8 {
        self.decoded_info().call_index()
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
    pub fn field_values(&self) -> Result<scale_value::Composite<u32>, Error> {
        let bytes = &mut self.field_bytes();
        let extrinsic_metadata = self.extrinsic_metadata()?;

        let mut fields = extrinsic_metadata
            .variant
            .fields
            .iter()
            .map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));
        let decoded =
            scale_value::scale::decode_as_fields(bytes, &mut fields, self.metadata.types())?;

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

    fn decoded_info(&self) -> &Extrinsic<'static, u32> {
        &self.ext.0
    }
}

/// A Static Extrinsic found in a block coupled with it's details.
pub struct FoundExtrinsic<T: Config, E> {
    /// Details for the extrinsic.
    pub details: ExtrinsicDetails<T>,
    /// The decoded extrinsic value.
    pub value: E,
}

/// Details for the given extrinsic plucked from the metadata.
pub struct ExtrinsicMetadataDetails<'a> {
    /// Metadata for the pallet that the extrinsic belongs to.
    pub pallet: PalletMetadata<'a>,
    /// Metadata for the variant which describes the pallet extrinsics.
    pub variant: &'a scale_info::Variant<scale_info::form::PortableForm>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SubstrateConfig;
    use assert_matches::assert_matches;
    use codec::{Decode, Encode};
    use frame_metadata::v15::{CustomMetadata, OuterEnums};
    use frame_metadata::{
        v15::{ExtrinsicMetadata, PalletCallMetadata, PalletMetadata, RuntimeMetadataV15},
        RuntimeMetadataPrefixed,
    };
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
        let metadata: subxt_metadata::Metadata = runtime_metadata.try_into().unwrap();

        Metadata::from(metadata)
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

        // Decode with empty bytes.
        let result = Extrinsics::<SubstrateConfig>::decode_from(vec![vec![]], metadata);
        assert_matches!(result.err(), Some(crate::Error::Codec(_)));
    }

    #[test]
    fn unsupported_version_extrinsic() {
        use frame_decode::extrinsics::ExtrinsicDecodeError;

        let metadata = metadata();

        // Decode with invalid version.
        let result =
        Extrinsics::<SubstrateConfig>::decode_from(vec![vec![3u8].encode()], metadata);

        assert_matches!(
            result.err(),
            Some(crate::Error::Block(
                crate::error::BlockError::ExtrinsicDecodeError(
                    ExtrinsicDecodeError::VersionNotSupported(3)
                )
            ))
        );
    }

    #[test]
    fn tx_hashes_line_up() {
        let metadata = metadata();

        let tx = crate::dynamic::tx(
            "Test",
            "TestCall",
            vec![
                Value::u128(10),
                Value::bool(true),
                Value::string("SomeValue"),
            ],
        );

        // Encoded TX ready to submit.
        let tx_encoded = crate::tx::create_unsigned::<SubstrateConfig, _>(&tx, &metadata)
            .expect("Valid dynamic parameters are provided");

        // Extrinsic details ready to decode.
        let extrinsics =
            Extrinsics::<SubstrateConfig>::decode_from(vec![tx_encoded.encoded().to_owned()], metadata)
                .expect("Valid extrinsic");

        let extrinsic = extrinsics.iter().next().unwrap();

        // Both of these types should produce the same bytes.
        assert_eq!(tx_encoded.encoded(), extrinsic.bytes(), "bytes should eq");
        // Both of these types should produce the same hash.
        assert_eq!(tx_encoded.hash(), extrinsic.hash(), "hashes should eq");
    }

    #[test]
    fn statically_decode_extrinsic() {
        let metadata = metadata();

        let tx = crate::dynamic::tx(
            "Test",
            "TestCall",
            vec![
                Value::u128(10),
                Value::bool(true),
                Value::string("SomeValue"),
            ],
        );
        let tx_encoded = crate::tx::create_unsigned::<SubstrateConfig, _>(&tx, &metadata)
            .expect("Valid dynamic parameters are provided");

        // Note: `create_unsigned` produces the extrinsic bytes by prefixing the extrinsic length.
        // The length is handled deserializing `ChainBlockExtrinsic`, therefore the first byte is not needed.
        let extrinsics =
            Extrinsics::<SubstrateConfig>::decode_from(vec![tx_encoded.encoded().to_owned()], metadata)
                .expect("Valid extrinsic");

        let extrinsic = extrinsics.iter().next().unwrap();

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
