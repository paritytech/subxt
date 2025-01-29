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
//! // returns something back.
//! let mock_client = MockRpcClient::new(
//!     state,
//!     |state, method, params| {
//!         // We'll panic if an RPC method is called more than 3 times:
//!         let val = state.pop().unwrap();
//!         async move { val }
//!     },
//!     |state, sub, params, unsub| {
//!         // Arrays, vecs or an RpcSubscription can be returned here to
//!         // signal the set of values to be handed back on a subscription.
//!         let vals = vec![Json(1), Json(2), Json(3)];
//!         async move { vals }
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
    /// Create a [`MockRpcClient`] by providing some state (which will be mutably available 
    /// to each function), a function to handle method calls, and a function to handle 
    /// subscription calls.
    pub fn new<MethodHandler, MFut, MRes, SubscriptionHandler, SFut, SRes>(
        state: State, 
        method_handler: MethodHandler, 
        subscription_handler: SubscriptionHandler
    ) -> MockRpcClient<State>
    where
        MethodHandler: Fn(&mut State, String, Option<Box<serde_json::value::RawValue>>) -> MFut + Send + Sync + 'static,
        MFut: Future<Output = MRes> + Send + 'static,
        MRes: IntoHandlerResponse,
        SubscriptionHandler: Fn(&mut State, String, Option<Box<serde_json::value::RawValue>>, String) -> SFut + Send + Sync + 'static,
        SFut: Future<Output = SRes> + Send + 'static,
        SRes: IntoSubscriptionResponse,
    {
        MockRpcClient {
            state: Arc::new(Mutex::new(state)),
            method_handler: Box::new(move |state: &mut State, method: String, params: Option<Box<serde_json::value::RawValue>>| {
                let fut = method_handler(state, method, params);
                Box::pin(async move { fut.await.into_handler_response() })
            }),
            subscription_handler: Box::new(move |state: &mut State, sub: String, params: Option<Box<serde_json::value::RawValue>>, unsub: String| {
                let fut = subscription_handler(state, sub, params, unsub);
                Box::pin(async move { fut.await.into_subscription_response() })
            })
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
