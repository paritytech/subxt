// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes [`ViewFunctionsClient`], which has methods for calling View Functions.
//! It's created by calling [`crate::client::ClientAtBlock::view_functions()`].

mod payload;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::{ViewFunctionDispatchError, ViewFunctionError};
use crate::metadata::Metadata;
use codec::{Decode, Encode};
use derive_where::derive_where;
use scale_decode::IntoVisitor;
use std::marker::PhantomData;

pub use payload::{DynamicPayload, Payload, StaticPayload, dynamic};

/// The name of the Runtime API call which can execute
const CALL_NAME: &str = "RuntimeViewFunction_execute_view_function";

/// The length of the query ID which identifies a View Function.
const QUERY_ID_LEN: usize = 32;

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
        let mut call_args = frame_decode::view_functions::encode_view_function_inputs(
            payload.pallet_name(),
            payload.function_name(),
            payload.args(),
            metadata,
            metadata.types(),
        )
        .map_err(ViewFunctionError::CouldNotEncodeInputs)?;

        // The above gives back the 32 byte query ID followed by the encoded inputs, but
        // the Runtime API takes the inputs as a single `Vec<u8>` argument, so they need
        // to be length prefixed.
        let inputs = call_args.split_off(QUERY_ID_LEN);
        inputs.encode_to(&mut call_args);

        Ok(call_args)
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
        decode_response(&payload, &bytes, metadata)
    }
}

/// Decode the bytes returned from the "execute_view_function" Runtime API call. This
/// returns `Result<Vec<u8>, ViewFunctionDispatchError>`, where the `Vec<u8>` contains
/// the encoded output of the View Function.
fn decode_response<P: Payload>(
    payload: &P,
    bytes: &[u8],
    metadata: &Metadata,
) -> Result<P::ReturnType, ViewFunctionError> {
    let output = Result::<Vec<u8>, ViewFunctionDispatchError>::decode(&mut &*bytes)
        .map_err(ViewFunctionError::CouldNotDecodeDispatchResult)?
        .map_err(ViewFunctionError::DispatchError)?;

    let cursor = &mut &*output;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::{OfflineClient, OfflineClientAtBlock};
    use crate::config::SubstrateConfig;
    use crate::config::substrate::SpecVersionForRange;
    use assert_matches::assert_matches;
    use std::sync::Arc;

    const QUERY_ID: [u8; 32] = [1; 32];

    /// The call enum of the `Test` pallet in [`test_metadata`].
    #[allow(dead_code, non_camel_case_types)]
    #[derive(scale_info::TypeInfo)]
    enum TestCall {
        call,
    }

    /// Metadata with a `Test` pallet exposing a `get(a: u32) -> u64` View Function.
    fn test_metadata() -> Metadata {
        use frame_metadata::v16;
        use scale_info::meta_type;

        v16::RuntimeMetadataV16::new(
            vec![v16::PalletMetadata {
                name: "Test",
                storage: None,
                calls: Some(v16::PalletCallMetadata {
                    ty: meta_type::<TestCall>(),
                    deprecation_info: v16::EnumDeprecationInfo::nothing_deprecated(),
                }),
                event: None,
                constants: Vec::new(),
                error: None,
                associated_types: Vec::new(),
                view_functions: vec![v16::PalletViewFunctionMetadata {
                    id: QUERY_ID,
                    name: "get",
                    inputs: vec![v16::FunctionParamMetadata {
                        name: "a",
                        ty: meta_type::<u32>(),
                    }],
                    output: meta_type::<u64>(),
                    docs: Vec::new(),
                    deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                }],
                index: 0,
                docs: Vec::new(),
                deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
            }],
            v16::ExtrinsicMetadata {
                versions: vec![4],
                address_ty: meta_type::<u8>(),
                call_ty: meta_type::<TestCall>(),
                signature_ty: meta_type::<u8>(),
                transaction_extensions_by_version: [(0u8, Vec::new())].into_iter().collect(),
                transaction_extensions: Vec::new(),
            },
            Vec::new(),
            v16::OuterEnums {
                call_enum_ty: meta_type::<TestCall>(),
                event_enum_ty: meta_type::<()>(),
                error_enum_ty: meta_type::<()>(),
            },
            v16::CustomMetadata {
                map: Default::default(),
            },
        )
        .try_into()
        .expect("can build valid metadata")
    }

    fn test_client() -> OfflineClientAtBlock<SubstrateConfig> {
        let config = SubstrateConfig::builder()
            .set_metadata_for_spec_versions([(0, Arc::new(test_metadata()))])
            .set_spec_version_for_block_ranges([SpecVersionForRange {
                block_range: 0..u64::MAX,
                spec_version: 0,
                transaction_version: 0,
            }])
            .build();

        OfflineClient::new_with_config(config)
            .at_block(0u64)
            .unwrap()
    }

    #[test]
    fn encode_args_length_prefixes_inputs() {
        let client = test_client();
        let payload = dynamic::<_, u64>("Test", "get", (7u32,));

        let call_args = client.view_functions().encode_args(&payload).unwrap();

        // The query ID, then the inputs encoded as a `Vec<u8>`.
        let mut expected = QUERY_ID.to_vec();
        7u32.encode().encode_to(&mut expected);
        assert_eq!(call_args, expected);
    }

    #[test]
    fn decode_response_unwraps_dispatch_result() {
        let metadata = test_metadata();
        let payload = dynamic::<_, u64>("Test", "get", (7u32,));
        let bytes = Ok::<Vec<u8>, ()>(42u64.encode()).encode();

        let value = decode_response(&payload, &bytes, &metadata).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn decode_response_returns_dispatch_errors() {
        let metadata = test_metadata();
        let payload = dynamic::<_, u64>("Test", "get", (7u32,));

        // `Err(ViewFunctionDispatchError::NotFound(QUERY_ID))`:
        let mut bytes = vec![1, 1];
        bytes.extend_from_slice(&QUERY_ID);

        assert_matches!(
            decode_response(&payload, &bytes, &metadata),
            Err(ViewFunctionError::DispatchError(
                ViewFunctionDispatchError::NotFound(id)
            )) if id == QUERY_ID
        );
    }
}
