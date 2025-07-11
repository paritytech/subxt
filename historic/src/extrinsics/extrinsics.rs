use crate::client::OfflineClientAtBlockT;
use crate::config::Config;
use frame_metadata::RuntimeMetadata;
use crate::error::ExtrinsicsError;
use scale_info_legacy::{ TypeRegistrySet, LookupName };
use super::extrinsic_transaction_extensions::ExtrinsicTransactionExtensions;

// Extrinsic information for modern or legacy extrinsics.
enum ExtrinsicInfo<'atblock> {
    Legacy(ExtrinsicInfoInner<'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(ExtrinsicInfoInner<'atblock, u32, scale_info::PortableRegistry>)
}

impl <'atblock> From<ExtrinsicInfoInner<'atblock, LookupName, TypeRegistrySet<'atblock>>> for ExtrinsicInfo<'atblock> {
    fn from(info: ExtrinsicInfoInner<'atblock, LookupName, TypeRegistrySet<'atblock>>) -> Self {
        ExtrinsicInfo::Legacy(info)
    }
}
impl <'atblock> From<ExtrinsicInfoInner<'atblock, u32, scale_info::PortableRegistry>> for ExtrinsicInfo<'atblock> {
    fn from(info: ExtrinsicInfoInner<'atblock, u32, scale_info::PortableRegistry>) -> Self {
        ExtrinsicInfo::Current(info)
    }
}

struct ExtrinsicInfoInner<'atblock, TypeId, Resolver> {
    info: frame_decode::extrinsics::Extrinsic<'atblock, TypeId>,
    resolver: &'atblock Resolver,
}

macro_rules! with_info {
    (&$self:ident.$info:ident => $fn:expr) => {
        match &$self.$info {
            ExtrinsicInfo::Legacy($info) => $fn,
            ExtrinsicInfo::Current($info) => $fn,
        }
    };
}

/// This represents some extrinsics in a block, and carries everything that we need to decode information out of them.
pub struct Extrinsics<'atblock> {
    bytes: Vec<Vec<u8>>,
    // Each index in this vec should line up with one index in the above vec.
    infos: Vec<ExtrinsicInfo<'atblock>>,
}

impl <'atblock> Extrinsics<'atblock> {
    // In here we hide the messy logic needed to decode extrinsics into a consistent output given either current or legacy metadata.
    pub (crate) fn new<'client: 'atblock, T, Client>(bytes: Vec<Vec<u8>>, client: &'atblock Client) -> Result<Self, ExtrinsicsError> 
    where
        T: Config + 'client,
        Client: OfflineClientAtBlockT<'client, T>
    {
        let infos = match client.metadata() {
            RuntimeMetadata::V8(m) => extrinsic_info_inner(&bytes, m, client.legacy_types()),
            RuntimeMetadata::V9(m) => extrinsic_info_inner(&bytes, m, client.legacy_types()),
            RuntimeMetadata::V10(m) => extrinsic_info_inner(&bytes, m, client.legacy_types()),
            RuntimeMetadata::V11(m) => extrinsic_info_inner(&bytes, m, client.legacy_types()),
            RuntimeMetadata::V12(m) => extrinsic_info_inner(&bytes, m, client.legacy_types()),
            RuntimeMetadata::V13(m) => extrinsic_info_inner(&bytes, m, client.legacy_types()),
            RuntimeMetadata::V14(m) => extrinsic_info_inner(&bytes, m, &m.types),
            RuntimeMetadata::V15(m) => extrinsic_info_inner(&bytes, m, &m.types),
            RuntimeMetadata::V16(m) => extrinsic_info_inner(&bytes, m, &m.types),
            unknown => return Err(ExtrinsicsError::UnsupportedMetadataVersion { version: unknown.version() }),
        }?;

        fn extrinsic_info_inner<'atblock, Info, Resolver>(
            bytes: &[Vec<u8>], 
            args_info: &'atblock Info,
            type_resolver: &'atblock Resolver
        ) -> Result<Vec<ExtrinsicInfo<'atblock>>, ExtrinsicsError> 
        where
            Info: frame_decode::extrinsics::ExtrinsicTypeInfo,
            Info::TypeId: Clone + core::fmt::Display + core::fmt::Debug + Send + Sync + 'static,
            Resolver: scale_type_resolver::TypeResolver<TypeId = Info::TypeId>,
            ExtrinsicInfo<'atblock>: From<ExtrinsicInfoInner<'atblock, Info::TypeId, Resolver>>,
        {
            bytes.iter().enumerate().map(|(index, bytes)| {
                let cursor = &mut &**bytes;
                let extrinsic_info = frame_decode::extrinsics::decode_extrinsic(cursor, args_info, type_resolver)
                    .map_err(|reason| ExtrinsicsError::DecodeError { index, reason })?;

                if !cursor.is_empty() {
                    return Err(ExtrinsicsError::LeftoverBytes {
                        index,
                        leftover_bytes: cursor.to_vec(),
                    });
                }

                Ok(ExtrinsicInfoInner { 
                    info: extrinsic_info, 
                    resolver: type_resolver
                }.into())
            }).collect()
        }
        
        Ok(Extrinsics { bytes, infos })
    }

    pub (crate) fn empty() -> Self {
        Self {
            bytes: vec![],
            infos: vec![],
        }
    }

    /// How many extrinsics are in this block?
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Are there any extrinsics in this block?
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Iterate over the extrinsics.
    pub fn iter(&self) -> impl Iterator<Item = Extrinsic<'_, 'atblock>> {
        self.bytes.iter().zip(self.infos.iter()).enumerate().map(|(idx, (bytes, info))| Extrinsic {
            idx,
            bytes,
            info,
        })
    }
}

/// This represents an extrinsic, and carries everything that we need to decode information out of it.
pub struct Extrinsic<'extrinsics, 'atblock> {
    idx: usize,
    bytes: &'extrinsics [u8],
    info: &'extrinsics ExtrinsicInfo<'atblock>,
}

impl <'extrinsics, 'atblock> Extrinsic<'extrinsics, 'atblock> {
    /// Get the index of this extrinsic in the block.
    pub fn index(&self) -> usize {
        self.idx
    }

    /// Get the raw bytes of this extrinsic.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        self.bytes
    }

    /// Is this extrinsic signed?
    pub fn is_signed(&self) -> bool {
        with_info!(&self.info => info.info.is_signed())
    }

    /// Return only the bytes representing this extrinsic call:
    /// - First byte is the pallet index
    /// - Second byte is the variant (call) index
    /// - Followed by field bytes.
    ///
    /// # Note
    ///
    /// Please use [`Self::bytes`] if you want to get all extrinsic bytes.
    pub fn call_bytes(&self) -> &'extrinsics [u8] {
        with_info!(&self.info => &self.bytes[info.info.call_data_range()])
    }

    /// Return the bytes representing the fields stored in this extrinsic.
    ///
    /// # Note
    ///
    /// This is a subset of [`Self::call_bytes`] that does not include the
    /// first two bytes that denote the pallet index and the variant index.
    pub fn field_bytes(&self) -> &'extrinsics [u8] {
        with_info!(&self.info => &self.bytes[info.info.call_data_args_range()])
    }

    /// Return only the bytes of the address that signed this extrinsic.
    ///
    /// # Note
    ///
    /// Returns `None` if the extrinsic is not signed.
    pub fn address_bytes(&self) -> Option<&'extrinsics [u8]> {
        with_info!(&self.info => {
            info.info
                .signature_payload()
                .map(|s| &self.bytes[s.address_range()])
        })
    }

    /// Returns Some(signature_bytes) if the extrinsic was signed otherwise None is returned.
    pub fn signature_bytes(&self) -> Option<&'extrinsics [u8]> {
        with_info!(&self.info => {
            info.info
                .signature_payload()
                .map(|s| &self.bytes[s.signature_range()])
        })
    }

    /// Get information about the transaction extensions of this extrinsic.
    pub fn transaction_extensions(&self) -> Option<ExtrinsicTransactionExtensions<'extrinsics, 'atblock>> {
        with_info!(&self.info => {
            info.info
                .transaction_extension_payload()
                .map(|t| ExtrinsicTransactionExtensions::new(&self.bytes[t.range()], t, info.resolver))
        })
    }

    /// The index of the pallet that the extrinsic originated from.
    pub fn pallet_index(&self) -> u8 {
        with_info!(&self.info => info.info.pallet_index())
    }

    /// The index of the extrinsic call in the pallet.
    pub fn call_index(&self) -> u8 {
        with_info!(&self.info => info.info.call_index())
    }

    /// The name of the pallet that the extrinsic originated from.
    pub fn pallet_name(&self) -> &str {
        with_info!(&self.info => info.info.pallet_name())
    }

    /// The name of the extrinsic call in the pallet.
    pub fn call_name(&self) -> &str {
        with_info!(&self.info => info.info.call_name())
    }
}