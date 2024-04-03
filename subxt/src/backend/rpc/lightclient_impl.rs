// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{RawRpcFuture, RawRpcSubscription, RpcClientT};
use crate::error::RpcError;
use futures::stream::{StreamExt, TryStreamExt};
use serde_json::value::RawValue;
use subxt_lightclient::{LightClientRpc, LightClientRpcError};

impl RpcClientT for LightClientRpc {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        Box::pin(async move {
            let res = self.request(method.to_owned(), params)
                .await
                .map_err(lc_err_to_rpc_err)?;

            Ok(res)
        })
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        Box::pin(async move {
            let sub = self.subscribe(sub.to_owned(), params, unsub.to_owned())
                .await
                .map_err(lc_err_to_rpc_err)?;

            let id = Some(sub.id().to_owned());
            let stream = sub
                .map_err(|e| RpcError::ClientError(Box::new(e)))
                .boxed();

            Ok(RawRpcSubscription { id, stream })
        })
    }
}

fn lc_err_to_rpc_err(err: LightClientRpcError) -> RpcError {
    match err {
        LightClientRpcError::JsonRpcError(e) => RpcError::ClientError(Box::new(e)),
        LightClientRpcError::SmoldotError(e) => RpcError::RequestRejected(e),
        LightClientRpcError::BackgroundTaskDropped => RpcError::SubscriptionDropped,
    }
}