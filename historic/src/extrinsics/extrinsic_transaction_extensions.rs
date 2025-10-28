use super::extrinsic_info::AnyExtrinsicInfo;
use crate::error::ExtrinsicTransactionExtensionError;
use crate::utils::Either;
use frame_decode::helpers::scale_decode;
use scale_info_legacy::{LookupName, TypeRegistrySet};

// Extrinsic extensions information for modern or legacy extrinsics.
enum AnyExtrinsicExtensionsInfo<'extrinsics, 'atblock> {
    Legacy(ExtrinsicExtensionsInfo<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(ExtrinsicExtensionsInfo<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>),
}

struct ExtrinsicExtensionsInfo<'extrinsics, 'atblock, TypeId, Resolver> {
    info: &'extrinsics frame_decode::extrinsics::ExtrinsicExtensions<'atblock, TypeId>,
    resolver: &'atblock Resolver,
}

/// This represents the transaction extensions of an extrinsic.
pub struct ExtrinsicTransactionParams<'extrinsics, 'atblock> {
    all_bytes: &'extrinsics [u8],
    info: AnyExtrinsicExtensionsInfo<'extrinsics, 'atblock>,
}

macro_rules! with_extensions_info {
    (&$self:ident.$info:ident => $fn:expr) => {
        #[allow(clippy::clone_on_copy)]
        match &$self.$info {
            AnyExtrinsicExtensionsInfo::Legacy($info) => $fn,
            AnyExtrinsicExtensionsInfo::Current($info) => $fn,
        }
    };
}

impl<'extrinsics, 'atblock> ExtrinsicTransactionParams<'extrinsics, 'atblock> {
    pub(crate) fn new(
        all_bytes: &'extrinsics [u8],
        info: &'extrinsics AnyExtrinsicInfo<'atblock>,
    ) -> Option<Self> {
        match info {
            AnyExtrinsicInfo::Current(info) => {
                let extension_info = info.info.transaction_extension_payload()?;
                Some(Self {
                    all_bytes,
                    info: AnyExtrinsicExtensionsInfo::Current(ExtrinsicExtensionsInfo {
                        info: extension_info,
                        resolver: info.resolver,
                    }),
                })
            }
            AnyExtrinsicInfo::Legacy(info) => {
                let extension_info = info.info.transaction_extension_payload()?;
                Some(Self {
                    all_bytes,
                    info: AnyExtrinsicExtensionsInfo::Legacy(ExtrinsicExtensionsInfo {
                        info: extension_info,
                        resolver: info.resolver,
                    }),
                })
            }
        }
    }

    /// Get the raw bytes for all of the transaction extensions.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        with_extensions_info!(&self.info => &self.all_bytes[info.info.range()])
    }

    /// iterate over each of the transaction extensions in this extrinsic.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ExtrinsicTransactionExtension<'extrinsics, 'atblock>> {
        match &self.info {
            AnyExtrinsicExtensionsInfo::Legacy(extension_info) => {
                let iter = extension_info
                    .info
                    .iter()
                    .map(|s| ExtrinsicTransactionExtension {
                        bytes: &self.all_bytes[s.range()],
                        info: ExtrinsicExtensionInfo {
                            name: s.name(),
                            type_id: s.ty(),
                            resolver: extension_info.resolver,
                        }
                        .into(),
                    });
                Either::A(iter)
            }
            AnyExtrinsicExtensionsInfo::Current(extension_info) => {
                let iter = extension_info
                    .info
                    .iter()
                    .map(|s| ExtrinsicTransactionExtension {
                        bytes: &self.all_bytes[s.range()],
                        info: ExtrinsicExtensionInfo {
                            name: s.name(),
                            type_id: s.ty(),
                            resolver: extension_info.resolver,
                        }
                        .into(),
                    });
                Either::B(iter)
            }
        }
    }

    /// Attempt to decode the transaction extensions into a type where each field name is the name of the transaction
    /// extension and the field value is the decoded extension.
    pub fn decode_as<T: scale_decode::DecodeAsFields>(
        &self,
    ) -> Result<T, ExtrinsicTransactionExtensionError> {
        with_extensions_info!(&self.info => {
            let cursor = &mut self.bytes();
            let mut fields = &mut info.info.iter().map(|named_arg| {
                scale_decode::Field::new(named_arg.ty().clone(), Some(named_arg.name()))
            });

            let decoded = T::decode_as_fields(cursor, &mut fields, info.resolver)
                .map_err(|e| ExtrinsicTransactionExtensionError::AllDecodeError { reason: e })?;

            if !cursor.is_empty() {
                return Err(ExtrinsicTransactionExtensionError::AllLeftoverBytes {
                    leftover_bytes: cursor.to_vec(),
                })
            }

            Ok(decoded)
        })
    }
}

// Extrinsic single extension information for modern or legacy extrinsics.
enum AnyExtrinsicExtensionInfo<'extrinsics, 'atblock> {
    Legacy(ExtrinsicExtensionInfo<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(ExtrinsicExtensionInfo<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>),
}

impl<'extrinsics, 'atblock>
    From<ExtrinsicExtensionInfo<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>>
    for AnyExtrinsicExtensionInfo<'extrinsics, 'atblock>
{
    fn from(
        info: ExtrinsicExtensionInfo<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>,
    ) -> Self {
        AnyExtrinsicExtensionInfo::Legacy(info)
    }
}
impl<'extrinsics, 'atblock>
    From<ExtrinsicExtensionInfo<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>>
    for AnyExtrinsicExtensionInfo<'extrinsics, 'atblock>
{
    fn from(
        info: ExtrinsicExtensionInfo<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>,
    ) -> Self {
        AnyExtrinsicExtensionInfo::Current(info)
    }
}

struct ExtrinsicExtensionInfo<'extrinsics, 'atblock, TypeId, Resolver> {
    name: &'extrinsics str,
    type_id: &'extrinsics TypeId,
    resolver: &'atblock Resolver,
}

macro_rules! with_extension_info {
    (&$self:ident.$info:ident => $fn:expr) => {
        #[allow(clippy::clone_on_copy)]
        match &$self.$info {
            AnyExtrinsicExtensionInfo::Legacy($info) => $fn,
            AnyExtrinsicExtensionInfo::Current($info) => $fn,
        }
    };
}

/// This represents a single transaction extension in an extrinsic.
pub struct ExtrinsicTransactionExtension<'extrinsics, 'atblock> {
    bytes: &'extrinsics [u8],
    info: AnyExtrinsicExtensionInfo<'extrinsics, 'atblock>,
}

impl<'extrinsics, 'atblock> ExtrinsicTransactionExtension<'extrinsics, 'atblock> {
    /// The bytes for this transaction extension.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        self.bytes
    }

    /// The name/identifier for this transaction extension.
    pub fn name(&self) -> &'extrinsics str {
        with_extension_info!(&self.info => info.name)
    }

    /// Decode the bytes for this transaction extension into a type that implements `scale_decode::DecodeAsType`.
    pub fn decode_as<T: scale_decode::DecodeAsType>(
        &self,
    ) -> Result<T, ExtrinsicTransactionExtensionError> {
        with_extension_info!(&self.info => {
            let cursor = &mut &*self.bytes;
            let decoded = T::decode_as_type(cursor, info.type_id.clone(), info.resolver)
                .map_err(|reason| ExtrinsicTransactionExtensionError::DecodeError {
                    name: info.name.to_string(),
                    reason
                })?;

            if !cursor.is_empty() {
                return Err(ExtrinsicTransactionExtensionError::LeftoverBytes {
                    name: info.name.to_string(),
                    leftover_bytes: cursor.to_vec(),
                });
            }

            Ok(decoded)
        })
    }
}
