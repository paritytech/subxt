// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{client::OnlineClientT, error::Error, metadata::DecodeWithMetadata, Config};
use codec::Decode;
use derivative::Derivative;
use std::{future::Future, marker::PhantomData};

use super::RuntimeApiPayload;

/// Execute runtime API calls.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct RuntimeApi<T: Config, Client> {
    client: Client,
    block_hash: T::Hash,
    _marker: PhantomData<T>,
}

impl<T: Config, Client> RuntimeApi<T, Client> {
    /// Create a new [`RuntimeApi`]
    pub(crate) fn new(client: Client, block_hash: T::Hash) -> Self {
        Self {
            client,
            block_hash,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> RuntimeApi<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Execute a raw runtime API call.
    pub fn call_raw<'a, Res: Decode>(
        &self,
        function: &'a str,
        call_parameters: Option<&'a [u8]>,
    ) -> impl Future<Output = Result<Res, Error>> + 'a {
        let client = self.client.clone();
        let block_hash = self.block_hash;
        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let data: Res = client
                .rpc()
                .state_call(function, call_parameters, Some(block_hash))
                .await?;
            Ok(data)
        }
    }

    /// Execute a runtime API call.
    pub fn call<Call: RuntimeApiPayload>(
        &self,
        payload: Call,
    ) -> impl Future<Output = Result<Call::ReturnType, Error>> {
        let client = self.client.clone();
        let block_hash = self.block_hash;
        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let metadata = client.metadata();
            let function = payload.fn_name();

            // Check if the function is present in the runtime metadata.
            let fn_metadata = metadata.runtime_fn(function)?;
            // Return type ID used for dynamic decoding.
            let return_id = fn_metadata.return_id();

            // Validate the runtime API payload hash against the compile hash from codegen.
            if let Some(details) = payload.validation_details() {
                let runtime_hash = metadata
                    .runtime_api_hash(fn_metadata.trait_name(), fn_metadata.method_name())?;

                if details.hash != runtime_hash {
                    return Err(
                        crate::metadata::MetadataError::IncompatibleRuntimeApiMetadata(
                            fn_metadata.trait_name().into(),
                            fn_metadata.method_name().into(),
                        )
                        .into(),
                    );
                }
            }

            // Encode the arguments of the runtime call.
            // For static payloads (codegen) this is pass-through, bytes are not altered.
            // For dynamic payloads this relies on `scale_value::encode_as_fields_to`.
            let params = payload.encode_args(&metadata)?;

            let bytes = client
                .rpc()
                .state_call_raw(function, Some(params.as_slice()), Some(block_hash))
                .await?;

            let value = <Call::ReturnType as DecodeWithMetadata>::decode_with_metadata(
                &mut &bytes[..],
                return_id,
                &metadata,
            )?;
            Ok(value)
        }
    }
}
