use super::extrinsic_info::{AnyExtrinsicInfo, with_info};
use crate::error::ExtrinsicCallError;
use crate::utils::Either;
use crate::utils::{AnyResolver, AnyTypeId};
use scale_info_legacy::{LookupName, TypeRegistrySet};

/// This represents the call data in the extrinsic.
pub struct ExtrinsicCall<'extrinsics, 'atblock> {
    all_bytes: &'extrinsics [u8],
    info: &'extrinsics AnyExtrinsicInfo<'atblock>,
}

impl<'extrinsics, 'atblock> ExtrinsicCall<'extrinsics, 'atblock> {
    pub(crate) fn new(
        all_bytes: &'extrinsics [u8],
        info: &'extrinsics AnyExtrinsicInfo<'atblock>,
    ) -> Self {
        Self { all_bytes, info }
    }

    /// The index of the pallet that this call is for
    pub fn pallet_index(&self) -> u8 {
        with_info!(&self.info => info.info.pallet_index())
    }

    /// The name of the pallet that this call is for.
    pub fn pallet_name(&self) -> &str {
        with_info!(&self.info => info.info.pallet_name())
    }

    /// The index of this call.
    pub fn index(&self) -> u8 {
        with_info!(&self.info => info.info.call_index())
    }

    /// The name of this call.
    pub fn name(&self) -> &str {
        with_info!(&self.info => info.info.call_name())
    }

    /// Get the raw bytes for the entire call, which includes the pallet and call index
    /// bytes as well as the encoded arguments for each of the fields.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        with_info!(&self.info => &self.all_bytes[info.info.call_data_range()])
    }

    /// Work with the fields in this call.
    pub fn fields(&self) -> ExtrinsicCallFields<'extrinsics, 'atblock> {
        ExtrinsicCallFields::new(self.all_bytes, self.info)
    }
}

/// This represents the fields of the call.
pub struct ExtrinsicCallFields<'extrinsics, 'atblock> {
    all_bytes: &'extrinsics [u8],
    info: &'extrinsics AnyExtrinsicInfo<'atblock>,
    resolver: AnyResolver<'atblock, 'atblock>,
}

impl<'extrinsics, 'atblock> ExtrinsicCallFields<'extrinsics, 'atblock> {
    pub(crate) fn new(
        all_bytes: &'extrinsics [u8],
        info: &'extrinsics AnyExtrinsicInfo<'atblock>,
    ) -> Self {
        let resolver = match info {
            AnyExtrinsicInfo::Legacy(info) => AnyResolver::B(info.resolver),
            AnyExtrinsicInfo::Current(info) => AnyResolver::A(info.resolver),
        };

        Self {
            all_bytes,
            info,
            resolver,
        }
    }

    /// Return the bytes representing the fields stored in this extrinsic.
    ///
    /// # Note
    ///
    /// This is a subset of [`ExtrinsicCall::bytes`] that does not include the
    /// first two bytes that denote the pallet index and the variant index.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        with_info!(&self.info => &self.all_bytes[info.info.call_data_args_range()])
    }

    /// Iterate over each of the fields of the extrinsic call data.
    pub fn iter(&self) -> impl Iterator<Item = ExtrinsicCallField<'_, 'extrinsics, 'atblock>> {
        match &self.info {
            AnyExtrinsicInfo::Legacy(info) => {
                Either::A(info.info.call_data().map(|named_arg| ExtrinsicCallField {
                    field_bytes: &self.all_bytes[named_arg.range()],
                    resolver: &self.resolver,
                    info: AnyExtrinsicCallFieldInfo::Legacy(ExtrinsicCallFieldInfo {
                        info: named_arg,
                        resolver: info.resolver,
                    }),
                }))
            }
            AnyExtrinsicInfo::Current(info) => {
                Either::B(info.info.call_data().map(|named_arg| ExtrinsicCallField {
                    field_bytes: &self.all_bytes[named_arg.range()],
                    resolver: &self.resolver,
                    info: AnyExtrinsicCallFieldInfo::Current(ExtrinsicCallFieldInfo {
                        info: named_arg,
                        resolver: info.resolver,
                    }),
                }))
            }
        }
    }

    /// Attempt to decode the fields into the given type.
    pub fn decode_as<T: scale_decode::DecodeAsFields>(&self) -> Result<T, ExtrinsicCallError> {
        with_info!(&self.info => {
            let cursor = &mut self.bytes();
            let mut fields = &mut info.info.call_data().map(|named_arg| {
                scale_decode::Field::new(named_arg.ty().clone(), Some(named_arg.name()))
            });

            let decoded = T::decode_as_fields(cursor, &mut fields, info.resolver)
                .map_err(|e| ExtrinsicCallError::FieldsDecodeError { reason: e })?;

            if !cursor.is_empty() {
                return Err(ExtrinsicCallError::FieldsLeftoverBytes {
                    leftover_bytes: cursor.to_vec(),
                })
            }

            Ok(decoded)
        })
    }
}

pub struct ExtrinsicCallField<'fields, 'extrinsics, 'atblock> {
    field_bytes: &'extrinsics [u8],
    info: AnyExtrinsicCallFieldInfo<'extrinsics, 'atblock>,
    resolver: &'fields AnyResolver<'atblock, 'atblock>,
}

enum AnyExtrinsicCallFieldInfo<'extrinsics, 'atblock> {
    Legacy(ExtrinsicCallFieldInfo<'extrinsics, 'atblock, LookupName, TypeRegistrySet<'atblock>>),
    Current(ExtrinsicCallFieldInfo<'extrinsics, 'atblock, u32, scale_info::PortableRegistry>),
}

struct ExtrinsicCallFieldInfo<'extrinsics, 'atblock, TypeId, Resolver> {
    info: &'extrinsics frame_decode::extrinsics::NamedArg<'atblock, TypeId>,
    resolver: &'atblock Resolver,
}

macro_rules! with_call_field_info {
    (&$self:ident.$info:ident => $fn:expr) => {
        #[allow(clippy::clone_on_copy)]
        match &$self.$info {
            AnyExtrinsicCallFieldInfo::Legacy($info) => $fn,
            AnyExtrinsicCallFieldInfo::Current($info) => $fn,
        }
    };
}

impl<'fields, 'extrinsics, 'atblock> ExtrinsicCallField<'fields, 'extrinsics, 'atblock> {
    /// Get the raw bytes for this field.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        self.field_bytes
    }

    /// Get the name of this field.
    pub fn name(&self) -> &'extrinsics str {
        with_call_field_info!(&self.info => info.info.name())
    }

    /// Visit the given field with a [`scale_decode::visitor::Visitor`]. This is like a lower level
    /// version of [`ExtrinsicCallField::decode_as`], as the visitor is able to preserve lifetimes
    /// and has access to more type information than is available via [`ExtrinsicCallField::decode_as`].
    pub fn visit<
        V: scale_decode::visitor::Visitor<TypeResolver = AnyResolver<'atblock, 'atblock>>,
    >(
        &self,
        visitor: V,
    ) -> Result<V::Value<'extrinsics, 'fields>, V::Error> {
        let type_id = match &self.info {
            AnyExtrinsicCallFieldInfo::Current(info) => AnyTypeId::A(*info.info.ty()),
            AnyExtrinsicCallFieldInfo::Legacy(info) => AnyTypeId::B(info.info.ty().clone()),
        };
        let cursor = &mut self.bytes();

        scale_decode::visitor::decode_with_visitor(cursor, type_id, self.resolver, visitor)
    }

    /// Attempt to decode the value of this field into the given type.
    pub fn decode_as<T: scale_decode::DecodeAsType>(&self) -> Result<T, ExtrinsicCallError> {
        with_call_field_info!(&self.info => {
            let cursor = &mut &*self.field_bytes;
            let decoded = T::decode_as_type(cursor, info.info.ty().clone(), info.resolver)
                .map_err(|e| ExtrinsicCallError::FieldDecodeError {
                    name: info.info.name().to_string(),
                    reason: e,
                })?;

            if !cursor.is_empty() {
                return Err(ExtrinsicCallError::FieldLeftoverBytes {
                    name: info.info.name().to_string(),
                    leftover_bytes: cursor.to_vec(),
                });
            }

            Ok(decoded)
        })
    }
}
