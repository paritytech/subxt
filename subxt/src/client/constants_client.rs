// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::OfflineClientT;
use crate::Config;
use codec::Decode;
use crate::error::BasicError;
use crate::metadata::MetadataError;
use scale_value::{ Value, scale::TypeId };

/// A client for accessing constants.
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
    /// Access a constant, returning a [`Value`] type representing it.
    pub fn value_at(&self, pallet_name: &str, constant_name: &str) -> Result<Value<TypeId>, BasicError> {
        let metadata = self.client.metadata();
        let pallet = metadata.pallet(&pallet_name)?;
        let constant = pallet.constant(&constant_name)?;
        let val = scale_value::scale::decode_as_type(&mut &*constant.value, constant.ty.id(), metadata.types())?;
        Ok(val)
    }

    /// Access the constant at the address given, returning the type defined by this address.
    /// This is probably used with addresses given from static codegen, although you can manually
    /// construct your own, too.
    pub fn at<ReturnTy: Decode>(&self, address: &ConstantAddress<'_, ReturnTy>) -> Result<ReturnTy, BasicError> {
        let metadata = self.client.metadata();

        // 1. Validate constant shape if hash given:
        if let Some(actual_hash) = address.constant_hash {
            let expected_hash = metadata.constant_hash(&address.pallet_name, &address.constant_name)?;
            if actual_hash != expected_hash {
                return Err(MetadataError::IncompatibleMetadata.into());
            }
        }

        // 2. Attempt to decode the constant into the type given:
        let pallet = metadata.pallet(&address.pallet_name)?;
        let constant = pallet.constant(&address.constant_name)?;
        let value = codec::Decode::decode(&mut &constant.value[..])?;
        Ok(value)
    }

}

/// This is returned from constant accesses in the statically generated
/// code, and contains the information needed to find, validate and decode
/// the constant.
pub struct ConstantAddress<'a, ReturnTy> {
    pallet_name: &'a str,
    constant_name: &'a str,
    constant_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<ReturnTy>
}

impl <'a, ReturnTy> ConstantAddress<'a, ReturnTy> {
    /// Create a new [`ConstantAddress`] that will be validated
    /// against node metadata using the hash given.
    pub fn new_with_validation(pallet_name: &'a str, constant_name: &'a str, hash: [u8; 32]) -> Self {
        Self {
            pallet_name,
            constant_name,
            constant_hash: Some(hash),
            _marker: std::marker::PhantomData
        }
    }

    /// Do not validate this constant prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            pallet_name: self.pallet_name,
            constant_name: self.constant_name,
            constant_hash: None,
            _marker: self._marker
        }
    }
}