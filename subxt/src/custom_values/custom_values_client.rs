use crate::client::OfflineClientT;
use crate::{Config, Error};
use derivative::Derivative;

use subxt_core::custom_values::{
    get_custom_value, get_custom_value_bytes, validate_custom_value, CustomValueAddress,
};
use subxt_core::Yes;

/// A client for accessing custom values stored in the metadata.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct CustomValuesClient<T, Client> {
    client: Client,
    _marker: std::marker::PhantomData<T>,
}

impl<T, Client> CustomValuesClient<T, Client> {
    /// Create a new [`CustomValuesClient`].
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Config, Client: OfflineClientT<T>> CustomValuesClient<T, Client> {
    /// Access a custom value by the address it is registered under. This can be just a [str] to get back a dynamic value,
    /// or a static address from the generated static interface to get a value of a static type returned.
    pub fn at<Address: CustomValueAddress<IsDecodable = Yes> + ?Sized>(
        &self,
        address: &Address,
    ) -> Result<Address::Target, Error> {
        get_custom_value(&self.client.metadata(), address).map_err(Into::into)
    }

    /// Access the bytes of a custom value by the address it is registered under.
    pub fn bytes_at<Address: CustomValueAddress + ?Sized>(
        &self,
        address: &Address,
    ) -> Result<Vec<u8>, Error> {
        get_custom_value_bytes(&self.client.metadata(), address).map_err(Into::into)
    }

    /// Run the validation logic against some custom value address you'd like to access. Returns `Ok(())`
    /// if the address is valid (or if it's not possible to check since the address has no validation hash).
    /// Returns an error if the address was not valid (wrong name, type or raw bytes)
    pub fn validate<Address: CustomValueAddress + ?Sized>(
        &self,
        address: &Address,
    ) -> Result<(), Error> {
        validate_custom_value(&self.client.metadata(), address).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::custom_values::CustomValuesClient;
    use crate::{Metadata, OfflineClient, SubstrateConfig};
    use codec::Encode;
    use scale_decode::DecodeAsType;
    use scale_info::form::PortableForm;
    use scale_info::TypeInfo;
    use std::collections::BTreeMap;
    use subxt_core::RuntimeVersion;

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
                map: BTreeMap::from_iter([("Person".to_string(), person_value_metadata)]),
            },
        };

        let metadata: subxt_metadata::Metadata = frame_metadata.try_into().unwrap();
        Metadata::new(metadata)
    }

    #[test]
    fn test_decoding() {
        let client = OfflineClient::<SubstrateConfig>::new(
            Default::default(),
            RuntimeVersion::new(0, 0),
            mock_metadata(),
        );
        let custom_value_client = CustomValuesClient::new(client);
        assert!(custom_value_client.at("No one").is_err());
        let person_decoded_value_thunk = custom_value_client.at("Person").unwrap();
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
