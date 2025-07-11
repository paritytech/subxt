use scale_info_legacy::{LookupName, TypeRegistrySet};

// Extrinsic extension information for modern or legacy extrinsics.
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
    info: &'extrinsics frame_decode::extrinsics::ExtrinsicExtensions<'atblock, TypeId>,
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

/// This represents the transaction extensions of an extrinsic.
pub struct ExtrinsicTransactionExtensions<'extrinsics, 'atblock> {
    extension_bytes: &'extrinsics [u8],
    extension_info: ExtrinsicExtensionInfo<'extrinsics, 'atblock>,
}

impl <'extrinsics, 'atblock> ExtrinsicTransactionExtensions<'extrinsics, 'atblock> {
    pub (crate) fn new<TypeId, Resolver>(
        extension_bytes: &'extrinsics [u8],
        info: &'extrinsics frame_decode::extrinsics::ExtrinsicExtensions<'atblock, TypeId>,
        resolver: &'atblock Resolver,
    ) -> Self 
    where ExtrinsicExtensionInfoInner<'extrinsics, 'atblock, TypeId, Resolver>: Into<ExtrinsicExtensionInfo<'extrinsics, 'atblock>> {
        Self {
            extension_bytes,
            extension_info: ExtrinsicExtensionInfoInner {
                info,
                resolver,
            }.into(),
        }
    }

    /// Get the raw bytes for all of the transaction extensions.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        self.extension_bytes
    }
}