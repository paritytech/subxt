// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a [`MockRpcClient`], which is useful for testing.
//! 
//! # Example
//! 
//! ```rust
//! use subxt_rpcs::client::{ RpcClient, MockRpcClient };
//! use subxt_rpcs::client::mock_rpc_client::Json;
//! 
//! let state = vec![
//!     Json(1u8),
//!     Json(2u8),
//!     Json(3u8),
//! ];
//! 
//! // Define a mock client by providing some state (can be optional)
//! // and functions which intercept method and subscription calls and
//! // return something back.
//! let mock_client = MockRpcClient::new(
//!     state,
//!     |state: &mut Vec<Json<_>>, method, params| {
//!         state.pop().unwrap()
//!     },
//!     |state: &mut _, sub, params, unsub| {
//!         vec![Json(1), Json(2), Json(3)]
//!     }
//! );
//! 
//! // Build an RPC Client that can be used in Subxt or in conjunction with
//! // the RPC methods provided in this crate. 
//! let rpc_client = RpcClient::new(mock_client);
//! ```

use super::{RpcClientT, RawRpcFuture, RawRpcSubscription};
use crate::Error;
use core::future::Future;
use serde_json::value::RawValue;
use std::sync::{Arc,Mutex};

type MethodHandlerFn<State> = Box<dyn Fn(&mut State, String, Option<Box<serde_json::value::RawValue>>) -> RawRpcFuture<'static, Box<RawValue>> + Send + Sync + 'static>;
type SubscriptionHandlerFn<State> = Box<dyn Fn(&mut State, String, Option<Box<serde_json::value::RawValue>>, String) -> RawRpcFuture<'static, RawRpcSubscription> + Send + Sync + 'static>;

/// A mock RPC client that responds programmatically to requests.
/// Useful for testing.
pub struct MockRpcClient<State> {
    state: Arc<Mutex<State>>,
    method_handler: MethodHandlerFn<State>,
    subscription_handler: SubscriptionHandlerFn<State>
}

impl <State: Send + 'static> MockRpcClient<State> {
    /// Create a [`MockRpcClient`] by providing a function to handle method calls
    /// and a function to handle subscription calls.
    pub fn new<MethodHandler, SubscriptionHandler, MA, SA>(
        state: State, 
        method_handler: MethodHandler, 
        subscription_handler: SubscriptionHandler
    ) -> MockRpcClient<State>
    where
        MethodHandler: IntoMethodHandler<State, MA>,
        SubscriptionHandler: IntoSubscriptionHandler<State, SA>,
    {
        MockRpcClient {
            state: Arc::new(Mutex::new(state)),
            method_handler: method_handler.into_method_handler(),
            subscription_handler: subscription_handler.into_subscription_handler()
        }
    }
}

impl <State: Send + 'static> RpcClientT for MockRpcClient<State> {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<serde_json::value::RawValue>>,
    ) -> RawRpcFuture<'a, Box<serde_json::value::RawValue>> {
        let mut s = self.state.lock().unwrap();
        (self.method_handler)(&mut *s, method.to_owned(), params)
    }
    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<serde_json::value::RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        let mut s = self.state.lock().unwrap();
        (self.subscription_handler)(&mut *s, sub.to_owned(), params, unsub.to_owned())
    }
}

// The below is all boilerplate to allow various types of functions, sync and async,
// and returning different types of arguments, are all able to be used as method and
// subscription handler functions.

/// Return responses wrapped in this to have them serialized to JSON. 
pub struct Json<T>(pub T);

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

/// Anything that can be a response to a subscription handler implements this.
pub trait IntoSubscriptionResponse {
    /// Convert self into a handler response.
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error>;
}

impl IntoSubscriptionResponse for Result<RawRpcSubscription, Error> {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        self
    }
}

impl <T: IntoHandlerResponse + Send + 'static> IntoSubscriptionResponse for Vec<T> {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        let iter = self.into_iter().map(|item| item.into_handler_response());
        Ok(RawRpcSubscription {
            stream: Box::pin(futures::stream::iter(iter)),
            id: None,
        })
    }
}

impl <T: IntoHandlerResponse + Send + 'static, const N: usize> IntoSubscriptionResponse for [T; N] {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        let iter = self.into_iter().map(|item| item.into_handler_response());
        Ok(RawRpcSubscription {
            stream: Box::pin(futures::stream::iter(iter)),
            id: None,
        })
    }
}

/// Anything that is a valid method handler implements this trait.
pub trait IntoMethodHandler<State, A> {
    /// Convert self into a method handler function.
    fn into_method_handler(self) -> MethodHandlerFn<State>;
}

impl <State, F, R> IntoMethodHandler<State, SyncMarker> for F 
where
    F: Fn(&mut State, String, Option<Box<serde_json::value::RawValue>>) -> R + Send + Sync + 'static,
    R: IntoHandlerResponse + Send + 'static,
{
    fn into_method_handler(self) -> MethodHandlerFn<State> {
        Box::new(move |state: &mut State, method: String, params: Option<Box<serde_json::value::RawValue>>| {
            let res = self(state, method, params);
            Box::pin(async move { res.into_handler_response() })
        })
    }
}

impl <State, F, Fut, R> IntoMethodHandler<State, AsyncMarker> for F 
where
    F: Fn(&mut State, String, Option<Box<serde_json::value::RawValue>>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: IntoHandlerResponse + Send + 'static,
{
    fn into_method_handler(self) -> MethodHandlerFn<State> {
        Box::new(move |state: &mut State, method: String, params: Option<Box<serde_json::value::RawValue>>| {
            let fut = self(state, method, params);
            Box::pin(async move { fut.await.into_handler_response() })
        })
    }
}

/// Anything that is a valid subscription handler implements this trait.
pub trait IntoSubscriptionHandler<State, A> {
    /// Convert self into a subscription handler function.
    fn into_subscription_handler(self) -> SubscriptionHandlerFn<State>;
}

impl <State, F, R> IntoSubscriptionHandler<State, SyncMarker> for F 
where
    F: Fn(&mut State, String, Option<Box<serde_json::value::RawValue>>, String) -> R + Send + Sync + 'static,
    R: IntoSubscriptionResponse + Send + 'static,
{
    fn into_subscription_handler(self) -> SubscriptionHandlerFn<State> {
        Box::new(move |state: &mut State, sub: String, params: Option<Box<serde_json::value::RawValue>>, unsub: String| {
            let res = self(state, sub, params, unsub);
            Box::pin(async move { res.into_subscription_response() })
        })
    }
}

impl <State, F, Fut, R> IntoSubscriptionHandler<State, AsyncMarker> for F 
where
    F: Fn(&mut State, String, Option<Box<serde_json::value::RawValue>>, String) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: IntoSubscriptionResponse + Send + 'static,
{
    fn into_subscription_handler(self) -> SubscriptionHandlerFn<State> {
        Box::new(move |state: &mut State, sub: String, params: Option<Box<serde_json::value::RawValue>>, unsub: String| {
            let fut = self(state, sub, params, unsub);
            Box::pin(async move { fut.await.into_subscription_response() })
        })
    }
}

#[doc(hidden)]
pub enum SyncMarker {}
#[doc(hidden)]
pub enum AsyncMarker {}