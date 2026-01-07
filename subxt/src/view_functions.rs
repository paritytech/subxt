// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes [`ViewFunctionsClient`], which has methods for calling View Functions.
//! It's created by calling [`crate::client::ClientAtBlock::view_functions()`].

mod payload;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::ViewFunctionError;
use derive_where::derive_where;
use scale_decode::IntoVisitor;
use std::marker::PhantomData;

pub use payload::{DynamicPayload, Payload, StaticPayload, dynamic};

/// The name of the Runtime API call which can execute
const CALL_NAME: &str = "RuntimeViewFunction_execute_view_function";

/// A client for working with View Functions. See [the module docs](crate::view_functions) for more.
#[derive_where(Clone; Client)]
pub struct ViewFunctionsClient<'atblock, T: Config, Client> {
    client: &'atblock Client,
    marker: PhantomData<T>,
}

impl<'atblock, T: Config, Client> ViewFunctionsClient<'atblock, T, Client> {
    /// Create a new [`ViewFunctionsClient`]
    pub(crate) fn new(client: &'atblock Client) -> Self {
        Self {
            client,
            marker: PhantomData,
        }
    }
}

impl<'atblock, T, Client> ViewFunctionsClient<'atblock, T, Client>
where
    T: Config,
    Client: OfflineClientAtBlockT<T>,
{
    /// Run the validation logic against some View Function payload you'd like to use. Returns `Ok(())`
    /// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
    /// Return an error if the payload was not valid or something went wrong trying to validate it (ie
    /// the View Function in question do not exist at all)
    pub fn validate<Call: Payload>(&self, payload: Call) -> Result<(), ViewFunctionError> {
        let Some(hash) = payload.validation_hash() else {
            return Ok(());
        };

        let metadata = self.client.metadata_ref();
        let pallet_name = payload.pallet_name();
        let function_name = payload.function_name();

        let view_function = metadata
            .pallet_by_name(pallet_name)
            .ok_or_else(|| ViewFunctionError::PalletNotFound(pallet_name.to_string()))?
            .view_function_by_name(function_name)
            .ok_or_else(|| ViewFunctionError::ViewFunctionNotFound {
                pallet_name: pallet_name.to_string(),
                function_name: function_name.to_string(),
            })?;

        if hash != view_function.hash() {
            Err(ViewFunctionError::IncompatibleCodegen)
        } else {
            Ok(())
        }
    }

    /// Encode the bytes that will be passed to the "execute_view_function" Runtime API call,
    /// to execute the View Function represented by the given payload.
    pub fn encode_args<P: Payload>(&self, payload: P) -> Result<Vec<u8>, ViewFunctionError> {
        let metadata = self.client.metadata_ref();
        let inputs = frame_decode::view_functions::encode_view_function_inputs(
            payload.pallet_name(),
            payload.function_name(),
            payload.args(),
            metadata,
            metadata.types(),
        )
        .map_err(ViewFunctionError::CouldNotEncodeInputs)?;

        Ok(inputs)
    }
}

impl<'atblock, T, Client> ViewFunctionsClient<'atblock, T, Client>
where
    T: Config,
    Client: OnlineClientAtBlockT<T>,
{
    /// Execute a raw View function API call. This returns the raw bytes representing the result
    /// of this call. The caller is responsible for decoding the result.
    pub async fn call_raw(
        &self,
        call_parameters: Option<&[u8]>,
    ) -> Result<Vec<u8>, ViewFunctionError> {
        let client = &self.client;
        let block_hash = client.block_ref().hash();
        let data = client
            .backend()
            .call(CALL_NAME, call_parameters, block_hash)
            .await
            .map_err(ViewFunctionError::CannotCallApi)?;

        Ok(data)
    }

    /// Execute a View Function call.
    pub async fn call<P: Payload>(&self, payload: P) -> Result<P::ReturnType, ViewFunctionError> {
        let client = &self.client;
        let metadata = client.metadata_ref();
        let block_hash = client.block_ref().hash();

        // Validate the View Function payload hash against the compile hash from codegen.
        self.validate(&payload)?;

        // Assemble the data to call the "execute_view_function" runtime API, which
        // then calls the relevant view function.
        let call_args = self.encode_args(&payload)?;

        // Make the call.
        let bytes = client
            .backend()
            .call(CALL_NAME, Some(call_args.as_slice()), block_hash)
            .await
            .map_err(ViewFunctionError::CannotCallApi)?;

        // Decode the response.
        let cursor = &mut &*bytes;
        let value = frame_decode::view_functions::decode_view_function_response(
            payload.pallet_name(),
            payload.function_name(),
            cursor,
            metadata,
            metadata.types(),
            P::ReturnType::into_visitor(),
        )
        .map_err(ViewFunctionError::CouldNotDecodeResponse)?;

        Ok(value)
    }
}
