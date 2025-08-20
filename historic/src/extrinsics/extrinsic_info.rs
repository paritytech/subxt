use crate::error::ExtrinsicsError;
use frame_metadata::RuntimeMetadata;
use scale_info_legacy::{LookupName, TypeRegistrySet};

// Extrinsic information for modern or legacy extrinsics.
#[allow(clippy::large_enum_variant)]
pub enum AnyExtrinsicInfo<'atblock> {
    Legacy(ExtrinsicInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(ExtrinsicInfo<'atblock, u32, scale_info::PortableRegistry>),
}

impl<'atblock> AnyExtrinsicInfo<'atblock> {
    /// For a slice of extrinsics, return a vec of information about each one.
    pub fn new(
        bytes: &[Vec<u8>],
        metadata: &'atblock RuntimeMetadata,
        legacy_types: &'atblock TypeRegistrySet<'atblock>,
    ) -> Result<Vec<Self>, ExtrinsicsError> {
        let infos = match metadata {
            RuntimeMetadata::V8(m) => extrinsic_info_inner(bytes, m, legacy_types),
            RuntimeMetadata::V9(m) => extrinsic_info_inner(bytes, m, legacy_types),
            RuntimeMetadata::V10(m) => extrinsic_info_inner(bytes, m, legacy_types),
            RuntimeMetadata::V11(m) => extrinsic_info_inner(bytes, m, legacy_types),
            RuntimeMetadata::V12(m) => extrinsic_info_inner(bytes, m, legacy_types),
            RuntimeMetadata::V13(m) => extrinsic_info_inner(bytes, m, legacy_types),
            RuntimeMetadata::V14(m) => extrinsic_info_inner(bytes, m, &m.types),
            RuntimeMetadata::V15(m) => extrinsic_info_inner(bytes, m, &m.types),
            RuntimeMetadata::V16(m) => extrinsic_info_inner(bytes, m, &m.types),
            unknown => {
                return Err(ExtrinsicsError::UnsupportedMetadataVersion {
                    version: unknown.version(),
                });
            }
        }?;

        fn extrinsic_info_inner<'atblock, Info, Resolver>(
            bytes: &[Vec<u8>],
            args_info: &'atblock Info,
            type_resolver: &'atblock Resolver,
        ) -> Result<Vec<AnyExtrinsicInfo<'atblock>>, ExtrinsicsError>
        where
            Info: frame_decode::extrinsics::ExtrinsicTypeInfo,
            Info::TypeId: Clone + core::fmt::Display + core::fmt::Debug + Send + Sync + 'static,
            Resolver: scale_type_resolver::TypeResolver<TypeId = Info::TypeId>,
            AnyExtrinsicInfo<'atblock>: From<ExtrinsicInfo<'atblock, Info::TypeId, Resolver>>,
        {
            bytes
                .iter()
                .enumerate()
                .map(|(index, bytes)| {
                    let cursor = &mut &**bytes;
                    let extrinsic_info = frame_decode::extrinsics::decode_extrinsic(
                        cursor,
                        args_info,
                        type_resolver,
                    )
                    .map_err(|reason| ExtrinsicsError::DecodeError { index, reason })?;

                    if !cursor.is_empty() {
                        return Err(ExtrinsicsError::LeftoverBytes {
                            index,
                            leftover_bytes: cursor.to_vec(),
                        });
                    }

                    Ok(ExtrinsicInfo {
                        info: extrinsic_info,
                        resolver: type_resolver,
                    }
                    .into())
                })
                .collect()
        }

        Ok(infos)
    }
}

impl<'atblock> From<ExtrinsicInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>>
    for AnyExtrinsicInfo<'atblock>
{
    fn from(info: ExtrinsicInfo<'atblock, LookupName, TypeRegistrySet<'atblock>>) -> Self {
        AnyExtrinsicInfo::Legacy(info)
    }
}
impl<'atblock> From<ExtrinsicInfo<'atblock, u32, scale_info::PortableRegistry>>
    for AnyExtrinsicInfo<'atblock>
{
    fn from(info: ExtrinsicInfo<'atblock, u32, scale_info::PortableRegistry>) -> Self {
        AnyExtrinsicInfo::Current(info)
    }
}

// Extrinsic information for a specific type ID and resolver type.
pub struct ExtrinsicInfo<'atblock, TypeId, Resolver> {
    pub info: frame_decode::extrinsics::Extrinsic<'atblock, TypeId>,
    pub resolver: &'atblock Resolver,
}

macro_rules! with_info {
    (&$self:ident.$info:ident => $fn:expr) => {
        #[allow(clippy::clone_on_copy)]
        match &$self.$info {
            AnyExtrinsicInfo::Legacy($info) => $fn,
            AnyExtrinsicInfo::Current($info) => $fn,
        }
    };
}
pub(crate) use with_info;
