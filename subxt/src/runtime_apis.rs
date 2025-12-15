// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes [`RuntimeApisClient`], which has methods for calling Runtime APIs.
//! It's created by calling [`crate::client::ClientAtBlock::runtime_apis()`].
//!
//! ```rust,no_run
//! pub use subxt::{OnlineClient, PolkadotConfig};
//!
//! let client = OnlineClient::new().await?;
//! let at_block = client.at_current_block().await?;
//!
//! let runtime_apis = at_block.runtime_apis();
//! ```

mod payload;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::RuntimeApiError;
use derive_where::derive_where;
use scale_decode::IntoVisitor;
use std::marker::PhantomData;

pub use payload::{DynamicPayload, Payload, StaticPayload, dynamic};

/// A client for working with Runtime APIs. See [the module docs](crate::runtime_apis) for more.
#[derive_where(Clone; Client)]
pub struct RuntimeApisClient<'atblock, T: Config, Client> {
    client: &'atblock Client,
    marker: PhantomData<T>,
}

impl<'atblock, T: Config, Client> RuntimeApisClient<'atblock, T, Client> {
    /// Create a new [`RuntimeApi`]
    pub(crate) fn new(client: &'atblock Client) -> Self {
        Self {
            client,
            marker: PhantomData,
        }
    }
}

impl<'atblock, T, Client> RuntimeApisClient<'atblock, T, Client>
where
    T: Config,
    Client: OfflineClientAtBlockT<T>,
{
    /// Run the validation logic against some runtime API payload you'd like to use. Returns `Ok(())`
    /// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
    /// Return an error if the payload was not valid or something went wrong trying to validate it (ie
    /// the runtime API in question do not exist at all)
    pub fn validate<P: Payload>(&self, payload: P) -> Result<(), RuntimeApiError> {
        let Some(hash) = payload.validation_hash() else {
            return Ok(());
        };

        let metadata = self.client.metadata_ref();
        let trait_name = payload.trait_name();
        let method_name = payload.method_name();

        let api_trait = metadata
            .runtime_api_trait_by_name(trait_name)
            .ok_or_else(|| RuntimeApiError::TraitNotFound(trait_name.to_string()))?;
        let api_method = api_trait.method_by_name(method_name).ok_or_else(|| {
            RuntimeApiError::MethodNotFound {
                trait_name: trait_name.to_string(),
                method_name: method_name.to_string(),
            }
        })?;

        if hash != api_method.hash() {
            Err(RuntimeApiError::IncompatibleCodegen)
        } else {
            Ok(())
        }
    }

    /// Return the name of the runtime API call from the payload.
    pub fn encode_name<P: Payload>(&self, payload: P) -> String {
        format!("{}_{}", payload.trait_name(), payload.method_name())
    }

    /// Return the encoded call args from a runtime API payload.
    pub fn encode_args<P: Payload>(&self, payload: P) -> Result<Vec<u8>, RuntimeApiError> {
        let metadata = self.client.metadata_ref();
        let value = frame_decode::runtime_apis::encode_runtime_api_inputs(
            payload.trait_name(),
            payload.method_name(),
            payload.args(),
            metadata,
            metadata.types(),
        )
        .map_err(RuntimeApiError::CouldNotEncodeInputs)?;

        Ok(value)
    }
}

impl<'atblock, T, Client> RuntimeApisClient<'atblock, T, Client>
where
    T: Config,
    Client: OnlineClientAtBlockT<T>,
{
    /// Execute a raw runtime API call. This returns the raw bytes representing the result
    /// of this call. The caller is responsible for decoding the result.
    pub async fn call_raw<'a>(
        &self,
        function: &'a str,
        call_parameters: Option<&'a [u8]>,
    ) -> Result<Vec<u8>, RuntimeApiError> {
        let client = &self.client;
        let block_hash = client.block_hash();
        let data = client
            .backend()
            .call(function, call_parameters, block_hash)
            .await
            .map_err(RuntimeApiError::CannotCallApi)?;

        Ok(data)
    }

    /// Execute a runtime API call.
    pub async fn call<P: Payload>(&self, payload: P) -> Result<P::ReturnType, RuntimeApiError> {
        let client = &self.client;
        let block_hash = client.block_hash();
        let metadata = client.metadata_ref();

        // Validate the runtime API payload hash against the compile hash from codegen.
        self.validate(&payload)?;

        // Encode the arguments of the runtime call.
        let call_name = self.encode_name(&payload);
        let call_args = self.encode_args(&payload)?;

        // Make the call.
        let bytes = client
            .backend()
            .call(&call_name, Some(call_args.as_slice()), block_hash)
            .await
            .map_err(RuntimeApiError::CannotCallApi)?;

        // Decode the response.
        let cursor = &mut &*bytes;
        let value = frame_decode::runtime_apis::decode_runtime_api_response(
            payload.trait_name(),
            payload.method_name(),
            cursor,
            metadata,
            metadata.types(),
            P::ReturnType::into_visitor(),
        )
        .map_err(RuntimeApiError::CouldNotDecodeResponse)?;

        Ok(value)
    }
}
