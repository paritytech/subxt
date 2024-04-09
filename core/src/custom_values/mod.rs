// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Access custom values from metadata.
//!
//! Use [`get`] to retrieve a custom value from some metadata, or [`validate`] to check that a
//! static custom value address lines up with the value seen in the metadata.
//!
//! # Example
//!
//! ```rust
//! use subxt_macro::subxt;
//! use subxt_core::custom_values;
//! use subxt_core::metadata;
//!
//! // If we generate types without `subxt`, we need to point to `::subxt_core`:
//! #[subxt(
//!     crate = "::subxt_core",
//!     runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale",
//! )]
//! pub mod polkadot {}
//!
//! // Some metadata we'd like to access custom values in:
//! let metadata_bytes = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
//! let metadata = metadata::decode_from(&metadata_bytes[..]).unwrap();
//!
//! // At the moment, we don't expect to see any custom values in the metadata
//! // for Polkadot, so this will return an error:
//! let err = custom_values::get(&metadata, "Foo");
//! ```

pub mod address;

use crate::utils::Yes;
use crate::{error::MetadataError, metadata::DecodeWithMetadata, Error, Metadata};
use address::AddressT;
use alloc::vec::Vec;

/// Run the validation logic against some custom value address you'd like to access. Returns `Ok(())`
/// if the address is valid (or if it's not possible to check since the address has no validation hash).
/// Returns an error if the address was not valid (wrong name, type or raw bytes)
pub fn validate<Address: AddressT + ?Sized>(
    metadata: &Metadata,
    address: &Address,
) -> Result<(), Error> {
    if let Some(actual_hash) = address.validation_hash() {
        let custom = metadata.custom();
        let custom_value = custom
            .get(address.name())
            .ok_or_else(|| MetadataError::CustomValueNameNotFound(address.name().into()))?;
        let expected_hash = custom_value.hash();
        if actual_hash != expected_hash {
            return Err(MetadataError::IncompatibleCodegen.into());
        }
    }
    if metadata.custom().get(address.name()).is_none() {
        return Err(MetadataError::IncompatibleCodegen.into());
    }
    Ok(())
}

/// Access a custom value by the address it is registered under. This can be just a [str] to get back a dynamic value,
/// or a static address from the generated static interface to get a value of a static type returned.
pub fn get<Address: AddressT<IsDecodable = Yes> + ?Sized>(
    metadata: &Metadata,
    address: &Address,
) -> Result<Address::Target, Error> {
    // 1. Validate custom value shape if hash given:
    validate(metadata, address)?;

    // 2. Attempt to decode custom value:
    let custom_value = metadata.custom_value_by_name_err(address.name())?;
    let value = <Address::Target as DecodeWithMetadata>::decode_with_metadata(
        &mut custom_value.bytes(),
        custom_value.type_id(),
        metadata,
    )?;
    Ok(value)
}

/// Access the bytes of a custom value by the address it is registered under.
pub fn get_bytes<Address: AddressT + ?Sized>(
    metadata: &Metadata,
    address: &Address,
) -> Result<Vec<u8>, Error> {
    // 1. Validate custom value shape if hash given:
    validate(metadata, address)?;

    // 2. Return the underlying bytes:
    let custom_value = metadata.custom_value_by_name_err(address.name())?;
    Ok(custom_value.bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::collections::BTreeMap;
    use codec::Encode;
    use scale_decode::DecodeAsType;
    use scale_info::form::PortableForm;
    use scale_info::TypeInfo;

    use alloc::borrow::ToOwned;
    use alloc::string::String;
    use alloc::vec;

    use crate::custom_values;

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
                map: BTreeMap::from_iter([("Mr. Robot".to_owned(), person_value_metadata)]),
            },
        };

        let metadata: subxt_metadata::Metadata = frame_metadata.try_into().unwrap();
        Metadata::from(metadata)
    }

    #[test]
    fn test_decoding() {
        let metadata = mock_metadata();

        assert!(custom_values::get(&metadata, "Invalid Address").is_err());
        let person_decoded_value_thunk = custom_values::get(&metadata, "Mr. Robot").unwrap();
        let person: Person = person_decoded_value_thunk.as_type().unwrap();
        assert_eq!(
            person,
            Person {
                age: 42,
                name: "Neo".into()
            }
        )
    }
}
