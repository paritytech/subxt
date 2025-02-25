// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{client::OfflineClientT, error::Error, Config};
use derive_where::derive_where;
use subxt_core::constants::address::Address;

/// A client for accessing constants.
#[derive_where(Clone; Client)]
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
    pub fn validate<Addr: Address>(&self, address: &Addr) -> Result<(), Error> {
        let metadata = self.client.metadata();
        subxt_core::constants::validate(address, &metadata).map_err(Error::from)
    }

    /// Access the constant at the address given, returning the type defined by this address.
    /// This is probably used with addresses given from static codegen, although you can manually
    /// construct your own, too.
    pub fn at<Addr: Address>(&self, address: &Addr) -> Result<Addr::Target, Error> {
        let metadata = self.client.metadata();
        subxt_core::constants::get(address, &metadata).map_err(Error::from)
    }
}
