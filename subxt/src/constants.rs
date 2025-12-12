mod address;

use crate::client::OfflineClientAtBlockT;
use crate::config::Config;
use crate::error::ConstantError;
use frame_decode::constants::ConstantTypeInfo;
use scale_decode::IntoVisitor;
use std::marker::PhantomData;

pub use address::{Address, DynamicAddress, StaticAddress, dynamic};

/// A client for working with storage entries.
#[derive(Clone)]
pub struct ConstantsClient<'atblock, T, Client> {
    client: &'atblock Client,
    marker: PhantomData<T>,
}

impl<'atblock, T, Client> ConstantsClient<'atblock, T, Client> {
    pub(crate) fn new(client: &'atblock Client) -> Self {
        ConstantsClient {
            client,
            marker: PhantomData,
        }
    }
}

impl<'atblock, T: Config, Client: OfflineClientAtBlockT<T>> ConstantsClient<'atblock, T, Client> {
    /// Run the validation logic against some constant address you'd like to access. Returns `Ok(())`
    /// if the address is valid (or if it's not possible to check since the address has no validation hash).
    /// Return an error if the address was not valid or something went wrong trying to validate it (ie
    /// the pallet or constant in question do not exist at all).
    pub fn validate<Addr: Address>(&self, address: Addr) -> Result<(), ConstantError> {
        let metadata = self.client.metadata_ref();
        if let Some(actual_hash) = address.validation_hash() {
            let expected_hash = metadata
                .pallet_by_name(address.pallet_name())
                .ok_or_else(|| {
                    ConstantError::PalletNameNotFound(address.pallet_name().to_string())
                })?
                .constant_hash(address.constant_name())
                .ok_or_else(|| ConstantError::ConstantNameNotFound {
                    pallet_name: address.pallet_name().to_string(),
                    constant_name: address.constant_name().to_owned(),
                })?;
            if actual_hash != expected_hash {
                return Err(ConstantError::IncompatibleCodegen);
            }
        }
        Ok(())
    }

    /// Access the constant at the given address, returning the value defined by this address.
    pub fn entry<Addr: Address>(&self, address: Addr) -> Result<Addr::Target, ConstantError> {
        let metadata = self.client.metadata_ref();

        // 1. Validate constant shape if hash given:
        self.validate(&address)?;

        // 2. Attempt to decode the constant into the type given:
        let constant = frame_decode::constants::decode_constant(
            address.pallet_name(),
            address.constant_name(),
            metadata,
            metadata.types(),
            Addr::Target::into_visitor(),
        )
        .map_err(ConstantError::CouldNotDecodeConstant)?;

        Ok(constant)
    }

    /// Access the bytes of a constant by its address.
    pub fn entry_bytes<Addr: Address>(&self, address: Addr) -> Result<Vec<u8>, ConstantError> {
        // 1. Validate custom value shape if hash given:
        self.validate(&address)?;

        // 2. Return the underlying bytes:
        let constant = self
            .client
            .metadata_ref()
            .constant_info(address.pallet_name(), address.constant_name())
            .map_err(|e| ConstantError::ConstantInfoError(e.into_owned()))?;
        Ok(constant.bytes.to_vec())
    }
}
