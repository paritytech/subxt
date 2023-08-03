// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing custom types

use crate::error::MetadataError;
use crate::Metadata;
use scale_decode::DecodeAsType;

/// gives you access to all `CustomValueMetadata` entries that are available as custom types on the metadata.
pub struct CustomTypes {
    metadata: Metadata,
}

impl CustomTypes {
    /// create a new CustomTypes instance, taking shared ownership of the metadata
    pub fn new(metadata: Metadata) -> Self {
        CustomTypes { metadata }
    }

    /// get a [CustomValueMetadata] by the string key it is registered under
    pub fn get(&self, key: &str) -> Option<CustomValueMetadata> {
        self.metadata
            .custom_metadata()
            .map
            .get(key)
            .map(|inner| CustomValueMetadata {
                inner,
                custom_types: self,
            })
    }
}

/// a wrapper around [subxt_metadata::CustomValueMetadata]. Can be used to decode a custom type.
pub struct CustomValueMetadata<'a> {
    inner: &'a subxt_metadata::CustomValueMetadata,
    /// contains an Arc to the actual metadata, so this is pretty light-weight
    custom_types: &'a CustomTypes,
}

impl<'a> CustomValueMetadata<'a> {
    /// the scale encoded value
    pub fn encoded(&self) -> &[u8] {
        &self.inner.value
    }

    /// the type id in the TypeRegistry
    pub fn type_id(&self) -> u32 {
        self.inner.ty.id
    }

    /// attempts to decode the scale encoded value of this custom type into the type
    pub fn as_type<T: DecodeAsType>(&self) -> Result<T, crate::Error> {
        let cursor = &mut self.encoded();
        let type_id = self.type_id();
        if self
            .custom_types
            .metadata
            .types()
            .resolve(type_id)
            .is_none()
        {
            return Err(MetadataError::TypeNotFound(type_id).into());
        }
        let decoded = T::decode_as_type(cursor, type_id, self.custom_types.metadata.types())?;
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::custom_types::CustomTypes;
    use crate::Metadata;
    use scale_decode::DecodeAsType;
    use scale_info::form::PortableForm;
    use scale_info::TypeInfo;
    use sp_core::Encode;

    #[derive(Debug, Clone, PartialEq, Eq, Encode, TypeInfo, DecodeAsType)]
    pub struct Person {
        age: u16,
        name: String,
    }

    fn mock_metadata() -> Metadata {
        let person_ty = scale_info::MetaType::new::<Person>();
        let unit = scale_info::MetaType::new::<()>();
        let mut types = scale_info::Registry::new();
        let person_ty_id = types.register_type(&person_ty);
        let unit_id = types.register_type(&unit);
        let types: scale_info::PortableRegistry = types.into();

        let person = Person {
            age: 42,
            name: "Neo".into(),
        };

        let person_value_metadata: frame_metadata::v15::CustomValueMetadata<PortableForm> =
            frame_metadata::v15::CustomValueMetadata {
                ty: person_ty_id,
                value: person.encode(),
            };

        let frame_metadata = frame_metadata::v15::RuntimeMetadataV15 {
            types,
            pallets: vec![],
            extrinsic: frame_metadata::v15::ExtrinsicMetadata {
                version: 0,
                address_ty: unit_id,
                call_ty: unit_id,
                signature_ty: unit_id,
                extra_ty: unit_id,
                signed_extensions: vec![],
            },
            ty: unit_id,
            apis: vec![],
            outer_enums: frame_metadata::v15::OuterEnums {
                call_enum_ty: unit_id,
                event_enum_ty: unit_id,
                error_enum_ty: unit_id,
            },
            custom: frame_metadata::v15::CustomMetadata {
                map: {
                    let mut m = BTreeMap::new();
                    m.insert("Person".to_string(), person_value_metadata);
                    m
                },
            },
        };

        let metadata: subxt_metadata::Metadata = frame_metadata.try_into().unwrap();
        Metadata::new(metadata)
    }

    #[test]
    fn test_decoding() {
        let custom_types = CustomTypes::new(mock_metadata());
        assert!(custom_types.get("No one").is_none());
        let person_value = custom_types.get("Person").unwrap();
        let person: Person = person_value.as_type().unwrap();
        assert_eq!(
            person,
            Person {
                age: 42,
                name: "Neo".into()
            }
        )
    }
}
