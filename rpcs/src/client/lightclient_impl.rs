// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{RawRpcFuture, RawRpcSubscription, RpcClientT};
use crate::Error;
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
                .await?;

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
                .await?;

            let id = Some(sub.id().to_owned());
            let stream = sub
                .map_err(|e| Error::Client(Box::new(e)))
                .boxed();

            Ok(RawRpcSubscription { id, stream })
        })
    }
}

impl From<LightClientRpcError> for Error {
    fn from(err: LightClientRpcError) -> Error {
        match err {
            LightClientRpcError::JsonRpcError(e) => {
                // If the error is a typical user error, report it as such, else
                // just wrap the error into a ClientError.
                let Ok(user_error) = e.try_deserialize() else {
                    return Error::Client(Box::<CoreError>::from(e))
                };
                Error::User(user_error)
            },
            LightClientRpcError::SmoldotError(e) => Error::Client(Box::<CoreError>::from(e)),
            LightClientRpcError::BackgroundTaskDropped => Error::Client(Box::<CoreError>::from("Smoldot background task was dropped")),
        }
    }
}

type CoreError = dyn core::error::Error + Send + Sync + 'static;