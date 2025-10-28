// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::Payload;
use crate::{
    backend::BlockRef,
    client::OnlineClientT,
    config::{Config, HashFor},
    error::ViewFunctionError,
};
use derive_where::derive_where;
use std::{future::Future, marker::PhantomData};

/// Execute View Function calls.
#[derive_where(Clone; Client)]
pub struct ViewFunctionsApi<T: Config, Client> {
    client: Client,
    block_ref: BlockRef<HashFor<T>>,
    _marker: PhantomData<T>,
}

impl<T: Config, Client> ViewFunctionsApi<T, Client> {
    /// Create a new [`ViewFunctionsApi`]
    pub(crate) fn new(client: Client, block_ref: BlockRef<HashFor<T>>) -> Self {
        Self {
            client,
            block_ref,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> ViewFunctionsApi<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Run the validation logic against some View Function payload you'd like to use. Returns `Ok(())`
    /// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
    /// Return an error if the payload was not valid or something went wrong trying to validate it (ie
    /// the View Function in question do not exist at all)
    pub fn validate<Call: Payload>(&self, payload: Call) -> Result<(), ViewFunctionError> {
        subxt_core::view_functions::validate(payload, &self.client.metadata()).map_err(Into::into)
    }

    /// Execute a View Function call.
    pub fn call<Call: Payload>(
        &self,
        payload: Call,
    ) -> impl Future<Output = Result<Call::ReturnType, ViewFunctionError>> + use<Call, Client, T>
    {
        let client = self.client.clone();
        let block_hash = self.block_ref.hash();
        // Ensure that the returned future doesn't have a lifetime tied to api.view_functions(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let metadata = client.metadata();

            // Validate the View Function payload hash against the compile hash from codegen.
            subxt_core::view_functions::validate(&payload, &metadata)?;

            // Assemble the data to call the "execute_view_function" runtime API, which
            // then calls the relevant view function.
            let call_name = subxt_core::view_functions::CALL_NAME;
            let call_args = subxt_core::view_functions::call_args(&payload, &metadata)?;

            // Make the call.
            let bytes = client
                .backend()
                .call(call_name, Some(call_args.as_slice()), block_hash)
                .await
                .map_err(ViewFunctionError::CannotCallApi)?;

            // Decode the response.
            let value =
                subxt_core::view_functions::decode_value(&mut &*bytes, &payload, &metadata)?;
            Ok(value)
        }
    }
}
