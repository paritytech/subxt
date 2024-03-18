// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::ConstantAddress;
use crate::{client::OfflineClientT, error::Error, Config};
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
        let metadata = self.client.metadata();
        subxt_core::constants::validate_constant(&metadata, address).map_err(Error::from)
    }

    /// Access the constant at the address given, returning the type defined by this address.
    /// This is probably used with addresses given from static codegen, although you can manually
    /// construct your own, too.
    pub fn at<Address: ConstantAddress>(
        &self,
        address: &Address,
    ) -> Result<Address::Target, Error> {
        let metadata = self.client.metadata();
        subxt_core::constants::get_constant(&metadata, address).map_err(Error::from)
    }
}
