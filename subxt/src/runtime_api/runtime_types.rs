// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    client::OnlineClientT,
    error::Error,
    Config,
};
use codec::Decode;
use derivative::Derivative;
use std::{
    future::Future,
    marker::PhantomData,
};

use super::{
    runtime_payload::{
        DynamicRuntimeApiPayload,
        RuntimeApiPayload,
    },
    RuntimeAPIPayload,
};

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
    pub fn call_raw<'a>(
        &self,
        function: &'a str,
        call_parameters: Option<&'a [u8]>,
    ) -> impl Future<Output = Result<Vec<u8>, Error>> + 'a {
        let client = self.client.clone();
        let block_hash = self.block_hash;
        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let data = client
                .rpc()
                .state_call(function, call_parameters, Some(block_hash))
                .await?;
            Ok(data.0)
        }
    }

    /// Execute a runtime API call for the given payload.
    pub fn call<ReturnTy: Decode>(
        &self,
        payload: RuntimeAPIPayload<ReturnTy>,
    ) -> impl Future<Output = Result<ReturnTy, Error>> {
        let client = self.client.clone();
        let block_hash = self.block_hash;

        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let payload = payload;
            let function = payload.func_name();
            let call_parameters = Some(payload.param_data());
            println!("StaticCall: {:?}", call_parameters);

            let data = client
                .rpc()
                .state_call(function, call_parameters, Some(block_hash))
                .await?;

            let result: ReturnTy = Decode::decode(&mut &data.0[..])?;
            Ok(result)
        }
    }

    /// Dynamic call.
    pub fn dyn_call<DynCall>(
        &self,
        payload: DynCall,
    ) -> impl Future<Output = Result<DynCall::Target, Error>>
    where
        DynCall: RuntimeApiPayload,
        <DynCall as RuntimeApiPayload>::Target: Decode,
    {
        let client = self.client.clone();
        let block_hash = self.block_hash;

        // Ensure that the returned future doesn't have a lifetime tied to api.runtime_api(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let payload = payload;
            let function = payload.func_name();

            let bytes = payload.encode_args(&client.metadata())?;
            let call_parameters = Some(bytes.as_slice());
            println!("DynCall: {:?}", call_parameters);

            let data = client
                .rpc()
                .state_call(&function, call_parameters, Some(block_hash))
                .await?;

            let result: DynCall::Target = Decode::decode(&mut &data.0[..])?;
            Ok(result)
        }
    }
}
