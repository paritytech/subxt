mod address;

use crate::client::OfflineClientAtBlockT;
use crate::config::Config;
use crate::error::CustomValueError;
use crate::utils::Maybe;
use derive_where::derive_where;
use frame_decode::custom_values::CustomValueTypeInfo;
use scale_decode::IntoVisitor;

pub use address::{Address, DynamicAddress, StaticAddress, dynamic};

/// A client for accessing custom values stored in the metadata.
#[derive_where(Clone; Client)]
pub struct CustomValuesClient<'atblock, T, Client> {
    client: &'atblock Client,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, T, Client> CustomValuesClient<'atblock, T, Client> {
    /// Create a new [`CustomValuesClient`].
    pub(crate) fn new(client: &'atblock Client) -> Self {
        Self {
            client,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'atblock, T: Config, Client: OfflineClientAtBlockT<T>>
    CustomValuesClient<'atblock, T, Client>
{
    /// Run the validation logic against some custom value address you'd like to access. Returns `Ok(())`
    /// if the address is valid (or if it's not possible to check since the address has no validation hash).
    /// Returns an error if the address was not valid (wrong name, type or raw bytes)
    pub fn validate<Addr: Address>(&self, address: Addr) -> Result<(), CustomValueError> {
        let metadata = self.client.metadata_ref();
        if let Some(actual_hash) = address.validation_hash() {
            let custom = metadata.custom();
            let custom_value = custom
                .get(address.name())
                .ok_or_else(|| CustomValueError::NotFound(address.name().into()))?;
            let expected_hash = custom_value.hash();
            if actual_hash != expected_hash {
                return Err(CustomValueError::IncompatibleCodegen);
            }
        }
        Ok(())
    }

    /// Access a custom value by the address it is registered under. This can be just a [str] to get back a dynamic value,
    /// or a static address from the generated static interface to get a value of a static type returned.
    pub fn entry<Addr: Address<IsDecodable = Maybe>>(
        &self,
        address: Addr,
    ) -> Result<Addr::Target, CustomValueError> {
        // 1. Validate custom value shape if hash given:
        self.validate(&address)?;

        // 2. Attempt to decode custom value:
        let metadata = self.client.metadata_ref();
        let value = frame_decode::custom_values::decode_custom_value(
            address.name(),
            metadata,
            metadata.types(),
            Addr::Target::into_visitor(),
        )
        .map_err(CustomValueError::CouldNotDecodeCustomValue)?;

        Ok(value)
    }

    /// Access the bytes of a custom value by the address it is registered under.
    pub fn entry_bytes<Addr: Address>(&self, address: Addr) -> Result<Vec<u8>, CustomValueError> {
        // 1. Validate custom value shape if hash given:
        self.validate(&address)?;

        // 2. Return the underlying bytes:
        let custom_value = self
            .client
            .metadata_ref()
            .custom_value_info(address.name())
            .map_err(|e| CustomValueError::NotFound(e.not_found))?;
        Ok(custom_value.bytes.to_vec())
    }
}
