// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::ConstantAddress;
use crate::{
    client::OfflineClientT,
    error::{Error, MetadataError},
    metadata::DecodeWithMetadata,
    Config,
};
use derivative::Derivative;

/// A client for accessing constants.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct ConstantsClient<T, Client> {
    client: Client,
    _marker: std::marker::PhantomData<T>,
}

impl<T, Client> ConstantsClient<T, Client> {
    /// Create a new [`ConstantsClient`].
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Config, Client: OfflineClientT<T>> ConstantsClient<T, Client> {
    /// Run the validation logic against some constant address you'd like to access. Returns `Ok(())`
    /// if the address is valid (or if it's not possible to check since the address has no validation hash).
    /// Return an error if the address was not valid or something went wrong trying to validate it (ie
    /// the pallet or constant in question do not exist at all).
    pub fn validate<Address: ConstantAddress>(&self, address: &Address) -> Result<(), Error> {
        if let Some(actual_hash) = address.validation_hash() {
            let expected_hash = self
                .client
                .metadata()
                .pallet_by_name_err(address.pallet_name())?
                .constant_hash(address.constant_name())
                .ok_or_else(|| {
                    MetadataError::ConstantNameNotFound(address.constant_name().to_owned())
                })?;
            if actual_hash != expected_hash {
                return Err(MetadataError::IncompatibleCodegen.into());
            }
        }
        Ok(())
    }

    /// Access the constant at the address given, returning the type defined by this address.
    /// This is probably used with addresses given from static codegen, although you can manually
    /// construct your own, too.
    pub fn at<Address: ConstantAddress>(
        &self,
        address: &Address,
    ) -> Result<Address::Target, Error> {
        let metadata = self.client.metadata();

        // 1. Validate constant shape if hash given:
        self.validate(address)?;

        // 2. Attempt to decode the constant into the type given:
        let constant = metadata
            .pallet_by_name_err(address.pallet_name())?
            .constant_by_name(address.constant_name())
            .ok_or_else(|| {
                MetadataError::ConstantNameNotFound(address.constant_name().to_owned())
            })?;
        let value = <Address::Target as DecodeWithMetadata>::decode_with_metadata(
            &mut constant.value(),
            constant.ty(),
            &metadata,
        )?;
        Ok(value)
    }
}
