// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    RpcClientT,
    RpcResponse,
    RpcSubscription,
};
use serde_json::value::{ Value, RawValue };
use crate::error::RpcError;
use jsonrpsee::{
    core::client::{ ClientT, SubscriptionClientT, Client },
    types::ParamsSer,
};

impl RpcClientT for Client {
    fn request(&self, method: &str, params: Box<RawValue>) -> RpcResponse {
        Box::pin(async move {
            let params = prep_params_for_jsonrpsee(&params)?;
            let res = ClientT::request(self, method, Some(params))
                .await
                .map_err(|e| RpcError(e.to_string()))?;
            Ok(res)
        })
    }

    fn subscribe(&self, sub: &str, params: Box<RawValue>, unsub: &str) -> RpcSubscription {
        Box::pin(async move {
            let params = prep_params_for_jsonrpsee(&params)?;
            let res = SubscriptionClientT::subscribe(self, sub, Some(params), unsub)
                .await
                .map_err(|e| RpcError(e.to_string()))?;
            Ok(res)
        })
    }
}

// This is ugly; we have to encode to Value's to be compat with the jsonrpc interface.
// Remove and simplify this once something like https://github.com/paritytech/jsonrpsee/issues/862 is in:
fn prep_params_for_jsonrpsee(params: &RawValue) -> Result<ParamsSer<'static>, RpcError> {
    let val = serde_json::to_value(&params).expect("RawValue guarantees valid JSON");
    let arr = match val {
        Value::Array(arr) => Ok(arr),
        _ => RpcError(format!("RPC Params are expected to be an array but got {params}"))
    }?;
    Ok(ParamsSer::Array(arr))
}