// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    config::{Config, Header},
    error::{Error, ExtrinsicError},
    rpc::types::ChainBlock,
    Metadata,
};
use codec::Decode;
use derivative::Derivative;
use frame_metadata::v15::RuntimeMetadataV15;
use std::{collections::HashMap, sync::Arc};

/// A collection of extrinsics obtained from a block, bundled with the necessary
/// information needed to decode and iterate over them.
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct Extrinsics<T: Config> {
    metadata: Metadata,
    /// The block hash.
    hash: T::Hash,
    /// The accompanying extrinsics.
    extrinsics: Vec<Arc<[u8]>>,
    /// Generic extrinsic parameter ids from the metadata.
    ids: ExtrinsicIds,
}

impl<T: Config> Extrinsics<T> {
    pub(crate) fn new(metadata: Metadata, block: ChainBlock<T>) -> Result<Self, Error> {
        let ids = ExtrinsicIds::new(metadata.runtime_metadata())?;

        let extrinsics: Vec<_> = block
            .extrinsics
            .into_iter()
            .map(|ext| ext.0.into())
            .collect();

        Ok(Self {
            metadata,
            hash: block.header.hash(),
            extrinsics,
            ids,
        })
    }

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
        let extrinsics = self.extrinsics.clone();
        let ids = self.ids;
        let num_extr = self.extrinsics.len();
        let mut index = 0;
        std::iter::from_fn(move || {
            if index == num_extr {
                None
            } else {
                match ExtrinsicDetails::decode_from(
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
    /// The index of the extrinsic in the block.
    index: usize,
    /// True if the extrinsic payload is signed.
    is_signed: bool,
    /// The start index in the `bytes` from which the address is encoded.
    address_start_idx: usize,
    /// The end index of the address in the encoded `bytes`.
    address_end_idx: usize,
    /// The start index in the `bytes` from which the call is encoded.
    call_start_idx: usize,
    /// Extrinsic bytes.
    bytes: Arc<[u8]>,
}

impl ExtrinsicDetails {
    // Attempt to dynamically decode a single event from our events input.
    fn decode_from(
        metadata: Metadata,
        extrinsic_bytes: Arc<[u8]>,
        ids: ExtrinsicIds,
        index: usize,
    ) -> Result<ExtrinsicDetails, Error> {
        const SIGNATURE_MASK: u8 = 0b1000_0000;
        const VERSION_MASK: u8 = 0b0111_1111;
        const LATEST_EXTRINSIC_VERSION: u8 = 4;

        // Extrinsic are encoded in memory in the following way:
        //   - first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
        //   - signature: [unknown TBD with metadata].
        //   - extrinsic data
        if extrinsic_bytes.is_empty() {
            return Err(ExtrinsicError::InsufficientData.into());
        }

        let version = extrinsic_bytes[0] & VERSION_MASK;
        if version != LATEST_EXTRINSIC_VERSION {
            return Err(ExtrinsicError::UnsupportedVersion(version).into());
        }

        let is_signed = extrinsic_bytes[0] & SIGNATURE_MASK != 0;

        // Skip over the first byte which denotes the version and signing.
        let cursor = &mut &extrinsic_bytes[1..];

        let mut address_start_idx = 0;
        let mut address_end_idx = 0;

        if is_signed {
            address_start_idx = extrinsic_bytes.len() - cursor.len();

            // Skip over the address, signature and extra fields.
            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.address,
                &metadata.runtime_metadata().types,
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;
            address_end_idx = extrinsic_bytes.len() - cursor.len();

            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.signature,
                &metadata.runtime_metadata().types,
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;

            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.extra,
                &metadata.runtime_metadata().types,
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;
        }

        let call_start_idx = extrinsic_bytes.len() - cursor.len();

        // Postpone decoding the extrinsic function call.
        scale_decode::visitor::decode_with_visitor(
            cursor,
            ids.call,
            &metadata.runtime_metadata().types,
            scale_decode::visitor::IgnoreVisitor,
        )
        .map_err(scale_decode::Error::from)?;

        Ok(ExtrinsicDetails {
            index,
            is_signed,
            address_start_idx,
            address_end_idx,
            call_start_idx,
            bytes: extrinsic_bytes,
        })
    }

    /// The index of the extrinsic in the given block.
    ///  What index is this event in the stored events for this block.
    pub fn index(&self) -> usize {
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

    /// Return only the bytes representing this extrinsic call.
    ///
    /// # Note
    ///
    /// Please use `[Self::bytes]` if you want to get all extrinsic bytes.
    pub fn call_bytes(&self) -> &[u8] {
        &self.bytes[self.call_start_idx..]
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

    /// Attempt to statically decode the address bytes into the provided type.
    ///
    /// # Note
    ///
    /// Returns `None` if the extrinsic is not signed.
    pub fn as_address<T: Decode>(&self) -> Option<Result<T, Error>> {
        self.address_bytes()
            .map(|bytes| T::decode(&mut &bytes[..]).map_err(Error::Codec))
    }

    /// Attempt to statically decode the extrinsic call bytes into the provided type.
    pub fn as_call<T: Decode>(&self) -> Result<T, Error> {
        let bytes = &mut &self.call_bytes()[..];
        T::decode(bytes).map_err(Error::Codec)
    }

    /// Is the extrinsic signed?
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }
}
