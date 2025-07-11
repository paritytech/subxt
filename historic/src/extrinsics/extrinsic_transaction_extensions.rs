use frame_decode::helpers::scale_decode;
use scale_info_legacy::{LookupName, TypeRegistrySet};
use crate::utils::Either;
use crate::error::ExtrinsicTransactionExtensionError;

// Extrinsic extensions information for modern or legacy extrinsics.
enum ExtrinsicExtensionsInfo<'extrinsics, 'atblock> {
    Legacy(ExtrinsicExtensionsInfoInner<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(ExtrinsicExtensionsInfoInner<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>)
}

impl <'extrinsics, 'atblock> From<ExtrinsicExtensionsInfoInner<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>> for ExtrinsicExtensionsInfo<'extrinsics, 'atblock> {
    fn from(info: ExtrinsicExtensionsInfoInner<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>) -> Self {
        ExtrinsicExtensionsInfo::Legacy(info)
    }
}
impl <'extrinsics, 'atblock> From<ExtrinsicExtensionsInfoInner<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>> for ExtrinsicExtensionsInfo<'extrinsics, 'atblock> {
    fn from(info: ExtrinsicExtensionsInfoInner<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>) -> Self {
        ExtrinsicExtensionsInfo::Current(info)
    }
}

struct ExtrinsicExtensionsInfoInner<'extrinsics, 'atblock, TypeId, Resolver> {
    info: &'extrinsics frame_decode::extrinsics::ExtrinsicExtensions<'atblock, TypeId>,
    resolver: &'atblock Resolver,
}

/// This represents the transaction extensions of an extrinsic.
pub struct ExtrinsicTransactionExtensions<'extrinsics, 'atblock> {
    extension_bytes: &'extrinsics [u8],
    extension_info: ExtrinsicExtensionsInfo<'extrinsics, 'atblock>,
}

impl <'extrinsics, 'atblock> ExtrinsicTransactionExtensions<'extrinsics, 'atblock> {
    #[allow(private_bounds)]
    pub (crate) fn new<TypeId, Resolver>(
        extension_bytes: &'extrinsics [u8],
        info: &'extrinsics frame_decode::extrinsics::ExtrinsicExtensions<'atblock, TypeId>,
        resolver: &'atblock Resolver,
    ) -> Self 
    where ExtrinsicExtensionsInfoInner<'extrinsics, 'atblock, TypeId, Resolver>: Into<ExtrinsicExtensionsInfo<'extrinsics, 'atblock>> {
        Self {
            extension_bytes,
            extension_info: ExtrinsicExtensionsInfoInner {
                info,
                resolver,
            }.into(),
        }
    }

    /// Get the raw bytes for all of the transaction extensions.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        self.extension_bytes
    }

    pub fn iter(&self) -> impl Iterator<Item = ExtrinsicTransactionExtension<'extrinsics, 'atblock>> {
        match &self.extension_info {
            ExtrinsicExtensionsInfo::Legacy(extension_info) => {
                let iter = extension_info.info.iter().map(|s| ExtrinsicTransactionExtension {
                    bytes: &self.extension_bytes[s.range()],
                    info: ExtrinsicExtensionInfoInner {
                        name: s.name(),
                        type_id: s.ty(),
                        resolver: extension_info.resolver,
                    }.into(),
                });
                Either::A(iter)
            },
            ExtrinsicExtensionsInfo::Current(extension_info) => {
                let iter = extension_info.info.iter().map(|s| ExtrinsicTransactionExtension {
                    bytes: &self.extension_bytes[s.range()],
                    info: ExtrinsicExtensionInfoInner {
                        name: s.name(),
                        type_id: s.ty(),
                        resolver: extension_info.resolver,
                    }.into(),
                });
                Either::B(iter)
            },
        }
    }
}

// Extrinsic single extension information for modern or legacy extrinsics.
enum ExtrinsicExtensionInfo<'extrinsics, 'atblock> {
    Legacy(ExtrinsicExtensionInfoInner<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(ExtrinsicExtensionInfoInner<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>)
}

impl <'extrinsics, 'atblock> From<ExtrinsicExtensionInfoInner<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>> for ExtrinsicExtensionInfo<'extrinsics, 'atblock> {
    fn from(info: ExtrinsicExtensionInfoInner<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>) -> Self {
        ExtrinsicExtensionInfo::Legacy(info)
    }
}
impl <'extrinsics, 'atblock> From<ExtrinsicExtensionInfoInner<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>> for ExtrinsicExtensionInfo<'extrinsics, 'atblock> {
    fn from(info: ExtrinsicExtensionInfoInner<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>) -> Self {
        ExtrinsicExtensionInfo::Current(info)
    }
}

struct ExtrinsicExtensionInfoInner<'extrinsics, 'atblock, TypeId, Resolver> {
    name: &'extrinsics str,
    type_id: &'extrinsics TypeId,
    resolver: &'atblock Resolver,
}

macro_rules! with_extension_info {
    (&$self:ident.$info:ident => $fn:expr) => {
        match &$self.$info {
            ExtrinsicExtensionInfo::Legacy($info) => $fn,
            ExtrinsicExtensionInfo::Current($info) => $fn,
        }
    };
}

/// This represents a single transaction extension in an extrinsic.
pub struct ExtrinsicTransactionExtension<'extrinsics, 'atblock> {
    bytes: &'extrinsics [u8],
    info: ExtrinsicExtensionInfo<'extrinsics, 'atblock>,
}

impl <'extrinsics, 'atblock> ExtrinsicTransactionExtension<'extrinsics, 'atblock> {
    /// The bytes for this transaction extension.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        self.bytes
    }

    /// The name/identifier for this transaction extension.
    pub fn name(&self) -> &'extrinsics str {
        with_extension_info!(&self.info => info.name)
    }

    /// Decode the bytes for this transaction extension into a type that implements `scale_decode::DecodeAsType`.
    pub fn decode_as_type<T: scale_decode::DecodeAsType>(&self) -> Result<T, ExtrinsicTransactionExtensionError> {
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