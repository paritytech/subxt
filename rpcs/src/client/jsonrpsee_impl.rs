// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{RawRpcFuture, RawRpcSubscription, RpcClientT};
use crate::Error;
use futures::stream::{StreamExt, TryStreamExt};
use jsonrpsee::{
    core::{
        client::{Error as JsonrpseeError, Client, ClientT, SubscriptionClientT, SubscriptionKind},
        traits::ToRpcParams,
    },
    types::SubscriptionId,
};
use serde_json::value::RawValue;

struct Params(Option<Box<RawValue>>);

impl ToRpcParams for Params {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
        Ok(self.0)
    }
}

impl RpcClientT for Client {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        Box::pin(async move {
            let res = ClientT::request(self, method, Params(params))
                .await
                .map_err(error_to_rpc_error)?;
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
            let stream = SubscriptionClientT::subscribe::<Box<RawValue>, _>(
                self,
                sub,
                Params(params),
                unsub,
            )
            .await
            .map_err(error_to_rpc_error)?;

            let id = match stream.kind() {
                SubscriptionKind::Subscription(SubscriptionId::Str(id)) => {
                    Some(id.clone().into_owned())
                }
                _ => None,
            };

            let stream = stream
                .map_err(|e| Error::Client(Box::new(e)))
                .boxed();
            Ok(RawRpcSubscription { stream, id })
        })
    }
}

/// Convert a JsonrpseeError into the RPC error in this crate.
/// The main reason for this is to capture user errors so that
/// they can be represented/handled without casting. 
fn error_to_rpc_error(error: JsonrpseeError) -> Error {
    match error {
        JsonrpseeError::Call(e) => {
            Error::User(crate::UserError {
                code: e.code(),
                message: e.message().to_owned(),
                data: e.data().map(|d| d.to_owned())
            })
        },
        e => {
            Error::Client(Box::new(e))
        }
    }
}