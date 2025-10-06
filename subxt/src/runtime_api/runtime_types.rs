// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::Payload;
use crate::{
    backend::BlockRef,
    client::OnlineClientT,
    config::{Config, HashFor},
    error::RuntimeApiError,
};
use derive_where::derive_where;
use std::{future::Future, marker::PhantomData};

/// Execute runtime API calls.
#[derive_where(Clone; Client)]
pub struct RuntimeApi<T: Config, Client> {
    client: Client,
    block_ref: BlockRef<HashFor<T>>,
    _marker: PhantomData<T>,
}

impl<T: Config, Client> RuntimeApi<T, Client> {
    /// Create a new [`RuntimeApi`]
    pub(crate) fn new(client: Client, block_ref: BlockRef<HashFor<T>>) -> Self {
        Self {
            client,
            block_ref,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> RuntimeApi<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Run the validation logic against some runtime API payload you'd like to use. Returns `Ok(())`
    /// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
    /// Return an error if the payload was not valid or something went wrong trying to validate it (ie
    /// the runtime API in question do not exist at all)
    pub fn validate<Call: Payload>(&self, payload: &Call) -> Result<(), RuntimeApiError> {
        subxt_core::runtime_api::validate(payload, &self.client.metadata()).map_err(Into::into)
    }

    /// Execute a raw runtime API call. This returns the raw bytes representing the result
    /// of this call. The caller is responsible for decoding the result.
    pub fn call_raw<'a>(
        &self,
        function: &'a str,
        call_parameters: Option<&'a [u8]>,
    ) -> impl Future<Output = Result<Vec<u8>, RuntimeApiError>> + use<'a, Client, T> {
        let client = self.client.clone();
        let block_hash = self.block_ref.hash();
        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let data = client
                .backend()
                .call(function, call_parameters, block_hash)
                .await
                .map_err(RuntimeApiError::CannotCallApi)?;
            Ok(data)
        }
    }

    /// Execute a runtime API call.
    pub fn call<Call: Payload>(
        &self,
        payload: Call,
    ) -> impl Future<Output = Result<Call::ReturnType, RuntimeApiError>> + use<Call, Client, T> {
        let client = self.client.clone();
        let block_hash = self.block_ref.hash();
        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let metadata = client.metadata();

            // Validate the runtime API payload hash against the compile hash from codegen.
            subxt_core::runtime_api::validate(&payload, &metadata)?;

            // Encode the arguments of the runtime call.
            let call_name = subxt_core::runtime_api::call_name(&payload);
            let call_args = subxt_core::runtime_api::call_args(&payload, &metadata)?;

            // Make the call.
            let bytes = client
                .backend()
                .call(&call_name, Some(call_args.as_slice()), block_hash)
                .await
                .map_err(RuntimeApiError::CannotCallApi)?;

            // Decode the response.
            let value = subxt_core::runtime_api::decode_value(&mut &*bytes, &payload, &metadata)?;
            Ok(value)
        }
    }
}
