// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a [`MockRpcClient`], which is useful for testing.

use super::{RpcClientT, RawRpcFuture, RawRpcSubscription};
use crate::Error;
use core::future::Future;
use serde_json::value::RawValue;

type MethodHandlerFn = Box<dyn Fn(&str, Option<Box<serde_json::value::RawValue>>) -> RawRpcFuture<'static, Box<RawValue>> + Send + Sync + 'static>;
type SubscriptionHandlerFn = Box<dyn Fn(&str, Option<Box<serde_json::value::RawValue>>, &str) -> RawRpcFuture<'static, RawRpcSubscription> + Send + Sync + 'static>;

/// A mock RPC client that responds programmatically to requests.
/// Useful for testing.
pub struct MockRpcClient {
    method_handler: MethodHandlerFn,
    subscription_handler: SubscriptionHandlerFn
}

impl MockRpcClient {
    /// Create a [`MockRpcClient`] by providing a function to handle method calls
    /// and a function to handle subscription calls.
    pub fn from_handlers<M, S, MA, SA>(method_handler: M, subscription_handler: S) -> MockRpcClient 
    where 
        M: IntoMethodHandler<MA>,
        S: IntoSubscriptionHandler<SA>,
    {
        MockRpcClient {
            method_handler: method_handler.into_method_handler(),
            subscription_handler: subscription_handler.into_subscription_handler()
        }
    }
}

impl RpcClientT for MockRpcClient {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<serde_json::value::RawValue>>,
    ) -> RawRpcFuture<'a, Box<serde_json::value::RawValue>> {
        (self.method_handler)(method, params)
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<serde_json::value::RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        (self.subscription_handler)(sub, params, unsub)
    }
}

/// Return responses wrapped in this to have them serialized to JSON. 
pub struct Json<T>(T);

/// Anything that can be converted into a valid handler response implements this.
pub trait IntoHandlerResponse {
    /// Convert self into a handler response.
    fn into_handler_response(self) -> Result<Box<RawValue>, Error>;
}

impl <T: serde::Serialize> IntoHandlerResponse for Result<T, Error> {
    fn into_handler_response(self) -> Result<Box<RawValue>, Error> {
        self.and_then(|val| serialize_to_raw_value(&val))
    }
}

impl IntoHandlerResponse for Box<RawValue> {
    fn into_handler_response(self) -> Result<Box<RawValue>, Error> {
        Ok(self)
    }
}

impl IntoHandlerResponse for serde_json::Value {
    fn into_handler_response(self) -> Result<Box<RawValue>, Error> {
        serialize_to_raw_value(&self)
    }
}

impl <T: serde::Serialize> IntoHandlerResponse for Json<T> {
    fn into_handler_response(self) -> Result<Box<RawValue>, Error> {
        serialize_to_raw_value(&self.0)
    }
}

fn serialize_to_raw_value<T: serde::Serialize>(val: &T) -> Result<Box<RawValue>, Error> {
    let res = serde_json::to_string(val).map_err(Error::Deserialization)?;
    let raw_value = RawValue::from_string(res).map_err(Error::Deserialization)?;
    Ok(raw_value)
}

/// Anything that is a valid method handler implements this trait.
pub trait IntoMethodHandler<A> {
    /// Convert self into a method handler function.
    fn into_method_handler(self) -> MethodHandlerFn;
}

enum SyncMethodHandler {}
impl <F, R> IntoMethodHandler<SyncMethodHandler> for F 
where
    F: Fn(&str, Option<Box<serde_json::value::RawValue>>) -> R + Send + Sync + 'static,
    R: IntoHandlerResponse + Send + 'static,
{
    fn into_method_handler(self) -> MethodHandlerFn {
        Box::new(move |method: &str, params: Option<Box<serde_json::value::RawValue>>| {
            let res = self(method, params);
            Box::pin(async move { res.into_handler_response() })
        })
    }
}

enum AsyncMethodHandler {}
impl <F, Fut, R> IntoMethodHandler<AsyncMethodHandler> for F 
where
    F: Fn(&str, Option<Box<serde_json::value::RawValue>>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: IntoHandlerResponse + Send + 'static,
{
    fn into_method_handler(self) -> MethodHandlerFn {
        Box::new(move |method: &str, params: Option<Box<serde_json::value::RawValue>>| {
            let fut = self(method, params);
            Box::pin(async move { fut.await.into_handler_response() })
        })
    }
}

/// Anything that is a valid subscription handler implements this trait.
pub trait IntoSubscriptionHandler<A> {
    /// Convert self into a subscription handler function.
    fn into_subscription_handler(self) -> SubscriptionHandlerFn;
}

enum SyncSubscriptionHandler {}
impl <F, R> IntoMethodHandler<SyncMethodHandler> for F 
where
    F: Fn(&str, Option<Box<serde_json::value::RawValue>>) -> R + Send + Sync + 'static,
    R: IntoHandlerResponse + Send + 'static,
{
    fn into_method_handler(self) -> MethodHandlerFn {
        Box::new(move |method: &str, params: Option<Box<serde_json::value::RawValue>>| {
            let res = self(method, params);
            Box::pin(async move { res.into_handler_response() })
        })
    }
}