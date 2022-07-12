// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::ConstantAddress;
use crate::{
    Config,
    client::OfflineClientT,
    error::BasicError,
    metadata::{
        MetadataError,
        DecodeWithMetadata,
    },
};
use derivative::Derivative;

/// A client for accessing constants.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct ConstantsClient<T, Client> {
    client: Client,
    _marker: std::marker::PhantomData<T>,
}

impl <T, Client> ConstantsClient<T, Client> {
    /// Create a new [`ConstantsClient`].
    pub fn new(client: Client) -> Self {
        Self { client, _marker: std::marker::PhantomData }
    }
}

impl <T: Config, Client: OfflineClientT<T>> ConstantsClient<T, Client> {
    /// Access the constant at the address given, returning the type defined by this address.
    /// This is probably used with addresses given from static codegen, although you can manually
    /// construct your own, too.
    pub fn at<ReturnTy: DecodeWithMetadata>(&self, address: &ConstantAddress<'_, ReturnTy>) -> Result<ReturnTy::Target, BasicError> {
        let metadata = self.client.metadata();

        // 1. Validate constant shape if hash given:
        if let Some(actual_hash) = address.validation_hash() {
            let expected_hash = metadata.constant_hash(address.pallet_name(), address.constant_name())?;
            if actual_hash != expected_hash {
                return Err(MetadataError::IncompatibleMetadata.into());
            }
        }

        // 2. Attempt to decode the constant into the type given:
        let pallet = metadata.pallet(address.pallet_name())?;
        let constant = pallet.constant(address.constant_name())?;
        let value = ReturnTy::decode_with_metadata(&mut &*constant.value, constant.ty.id(), &metadata)?;
        Ok(value)
    }
}
