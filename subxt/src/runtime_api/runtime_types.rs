// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::PayloadT;
use crate::{
    backend::{BackendExt, BlockRef},
    client::OnlineClientT,
    error::Error,
    Config,
};
use codec::Decode;
use derive_where::derive_where;
use std::{future::Future, marker::PhantomData};

/// Execute runtime API calls.
#[derive_where(Clone; Client)]
pub struct RuntimeApi<T: Config, Client> {
    client: Client,
    block_ref: BlockRef<T::Hash>,
    _marker: PhantomData<T>,
}

impl<T: Config, Client> RuntimeApi<T, Client> {
    /// Create a new [`RuntimeApi`]
    pub(crate) fn new(client: Client, block_ref: BlockRef<T::Hash>) -> Self {
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
    pub fn validate<Call: PayloadT>(&self, payload: &Call) -> Result<(), Error> {
        subxt_core::runtime_api::validate(&self.client.metadata(), payload).map_err(Into::into)
    }

    /// Execute a raw runtime API call.
    pub fn call_raw<'a, Res: Decode>(
        &self,
        function: &'a str,
        call_parameters: Option<&'a [u8]>,
    ) -> impl Future<Output = Result<Res, Error>> + 'a {
        let client = self.client.clone();
        let block_hash = self.block_ref.hash();
        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let data: Res = client
                .backend()
                .call_decoding(function, call_parameters, block_hash)
                .await?;
            Ok(data)
        }
    }

    /// Execute a runtime API call.
    pub fn call<Call: PayloadT>(
        &self,
        payload: Call,
    ) -> impl Future<Output = Result<Call::ReturnType, Error>> {
        let client = self.client.clone();
        let block_hash = self.block_ref.hash();
        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let metadata = client.metadata();

            // Validate the runtime API payload hash against the compile hash from codegen.
            subxt_core::runtime_api::validate(&metadata, &payload)?;

            // Encode the arguments of the runtime call.
            let call_name = subxt_core::runtime_api::call_name(&payload);
            let call_args = subxt_core::runtime_api::call_args(&metadata, &payload)?;

            // Make the call.
            let bytes = client
                .backend()
                .call(&call_name, Some(call_args.as_slice()), block_hash)
                .await?;

            // Decode the response.
            let value = subxt_core::runtime_api::decode_value(&metadata, &payload, &mut &*bytes)?;
            Ok(value)
        }
    }
}
