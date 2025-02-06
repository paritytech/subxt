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
//! let mut state = vec![
//!     Json(1u8),
//!     Json(2u8),
//!     Json(3u8),
//! ];
//! 
//! // Define a mock client by providing some state (can be optional)
//! // and functions which intercept method and subscription calls and
//! // returns something back.
//! let mock_client = MockRpcClient::builder()
//!     .method_handler_once("foo", move |params| {
//!         // Return each item from our state, and then null afterwards.
//!         let val = state.pop()
//!         async move { Json(val) }
//!     })
//!     .subscription_handler("bar", |params, unsub| async move {
//!         // Arrays, vecs or an RpcSubscription can be returned here to
//!         // signal the set of values to be handed back on a subscription.
//!         vec![Json(1), Json(2), Json(3)]
//!     })
//!     .build();
//! 
//! // Build an RPC Client that can be used in Subxt or in conjunction with
//! // the RPC methods provided in this crate. 
//! let rpc_client = RpcClient::new(mock_client);
//! ```

use super::{RpcClientT, RawRpcFuture, RawRpcSubscription};
use crate::{Error, UserError};
use core::future::Future;
use futures::StreamExt;
use serde_json::value::RawValue;
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};

type MethodHandlerFnOnce = Box<dyn FnOnce(&str, Option<Box<serde_json::value::RawValue>>) -> RawRpcFuture<'static, Box<RawValue>> + Send + Sync + 'static>;
type SubscriptionHandlerFnOnce = Box<dyn FnOnce(&str, Option<Box<serde_json::value::RawValue>>, &str) -> RawRpcFuture<'static, RawRpcSubscription> + Send + Sync + 'static>;

type MethodHandlerFn = Box<dyn FnMut(&str, Option<Box<serde_json::value::RawValue>>) -> RawRpcFuture<'static, Box<RawValue>> + Send + Sync + 'static>;
type SubscriptionHandlerFn = Box<dyn FnMut(&str, Option<Box<serde_json::value::RawValue>>, &str) -> RawRpcFuture<'static, RawRpcSubscription> + Send + Sync + 'static>;

/// A builder to configure and build a new [`MockRpcClient`].
pub struct MockRpcClientBuilder {
    method_handlers_once: HashMap<String, VecDeque<MethodHandlerFnOnce>>,
    method_handlers: HashMap<String, MethodHandlerFn>,
    method_fallback: Option<MethodHandlerFn>,
    subscription_handlers_once: HashMap<String, VecDeque<SubscriptionHandlerFnOnce>>,
    subscription_handlers: HashMap<String, SubscriptionHandlerFn>,
    subscription_fallback: Option<SubscriptionHandlerFn>
}

impl  MockRpcClientBuilder {
    fn new() -> Self {
        MockRpcClientBuilder {
            method_handlers_once: HashMap::new(),
            method_handlers: HashMap::new(),
            method_fallback: None,
            subscription_handlers_once: HashMap::new(),
            subscription_handlers: HashMap::new(),
            subscription_fallback: None
        }
    }

    /// Add a handler for a specific RPC method. This is called exactly once, and multiple such calls for the same method can be
    /// added. Only when any calls registered with this have been used up is the method set by [`Self::method_handler`] called.
    pub fn method_handler_once<MethodHandler, MFut, MRes>(mut self, name: impl Into<String>, f: MethodHandler) -> Self 
    where
        MethodHandler: FnOnce(Option<Box<serde_json::value::RawValue>>) -> MFut + Send + Sync + 'static,
        MFut: Future<Output = MRes> + Send + 'static,
        MRes: IntoHandlerResponse,
    {
        let handler: MethodHandlerFnOnce = Box::new(move |_method: &str, params: Option<Box<serde_json::value::RawValue>>| {
            let fut = f(params);
            Box::pin(async move { fut.await.into_handler_response() })
        });
        self.method_handlers_once.entry(name.into()).or_default().push_back(handler);
        self
    }

    /// Add a handler for a specific RPC method.
    pub fn method_handler<MethodHandler, MFut, MRes>(mut self, name: impl Into<String>, mut f: MethodHandler) -> Self 
    where
        MethodHandler: FnMut(Option<Box<serde_json::value::RawValue>>) -> MFut + Send + Sync + 'static,
        MFut: Future<Output = MRes> + Send + 'static,
        MRes: IntoHandlerResponse,
    {
        let handler: MethodHandlerFn = Box::new(move |_method: &str, params: Option<Box<serde_json::value::RawValue>>| {
            let fut = f(params);
            Box::pin(async move { fut.await.into_handler_response() })
        });
        self.method_handlers.insert(name.into(), handler);
        self
    }

    /// Add a fallback handler to handle any methods not handled by a specific handler.
    pub fn method_fallback<MethodHandler, MFut, MRes>(mut self, mut f: MethodHandler) -> Self 
    where
        MethodHandler: FnMut(String, Option<Box<serde_json::value::RawValue>>) -> MFut + Send + Sync + 'static,
        MFut: Future<Output = MRes> + Send + 'static,
        MRes: IntoHandlerResponse,
    {
        let handler: MethodHandlerFn = Box::new(move |method: &str, params: Option<Box<serde_json::value::RawValue>>| {
            let fut = f(method.to_owned(), params);
            Box::pin(async move { fut.await.into_handler_response() })
        });
        self.method_fallback = Some(handler);
        self
    }

    /// Add a handler for a specific RPC subscription.
    pub fn subscription_handler_once<SubscriptionHandler, SFut, SRes>(mut self, name: impl Into<String>, f: SubscriptionHandler) -> Self 
    where
        SubscriptionHandler: FnOnce(Option<Box<serde_json::value::RawValue>>, String) -> SFut + Send + Sync + 'static,
        SFut: Future<Output = SRes> + Send + 'static,
        SRes: IntoSubscriptionResponse,
    {
        let handler: SubscriptionHandlerFnOnce = Box::new(move |_sub: &str, params: Option<Box<serde_json::value::RawValue>>, unsub: &str| {
            let fut = f(params, unsub.to_owned());
            Box::pin(async move { fut.await.into_subscription_response() })
        });
        self.subscription_handlers_once.entry(name.into()).or_default().push_back(handler);
        self
    }

    /// Add a handler for a specific RPC subscription.
    pub fn subscription_handler<SubscriptionHandler, SFut, SRes>(mut self, name: impl Into<String>, mut f: SubscriptionHandler) -> Self 
    where
        SubscriptionHandler: FnMut(Option<Box<serde_json::value::RawValue>>, String) -> SFut + Send + Sync + 'static,
        SFut: Future<Output = SRes> + Send + 'static,
        SRes: IntoSubscriptionResponse,
    {
        let handler: SubscriptionHandlerFn = Box::new(move |_sub: &str, params: Option<Box<serde_json::value::RawValue>>, unsub: &str| {
            let fut = f(params, unsub.to_owned());
            Box::pin(async move { fut.await.into_subscription_response() })
        });
        self.subscription_handlers.insert(name.into(), handler);
        self
    }

    /// Add a fallback handler to handle any subscriptions not handled by a specific handler.
    pub fn subscription_fallback<SubscriptionHandler, SFut, SRes>(mut self, mut f: SubscriptionHandler) -> Self 
    where
        SubscriptionHandler: FnMut(String, Option<Box<serde_json::value::RawValue>>, String) -> SFut + Send + Sync + 'static,
        SFut: Future<Output = SRes> + Send + 'static,
        SRes: IntoSubscriptionResponse,
    {
        let handler: SubscriptionHandlerFn = Box::new(move |sub: &str, params: Option<Box<serde_json::value::RawValue>>, unsub: &str| {
            let fut = f(sub.to_owned(), params, unsub.to_owned());
            Box::pin(async move { fut.await.into_subscription_response() })
        });
        self.subscription_fallback = Some(handler);
        self
    }

    /// Construct a [`MockRpcClient`] given some state which will be mutably available to each of the handlers.
    pub fn build(self) -> MockRpcClient {
        MockRpcClient { 
            method_handlers_once: Arc::new(Mutex::new(self.method_handlers_once)),
            method_handlers: Arc::new(Mutex::new(self.method_handlers)), 
            method_fallback: self.method_fallback.map(|f| Arc::new(Mutex::new(f))),
            subscription_handlers_once: Arc::new(Mutex::new(self.subscription_handlers_once)), 
            subscription_handlers: Arc::new(Mutex::new(self.subscription_handlers)), 
            subscription_fallback: self.subscription_fallback.map(|f| Arc::new(Mutex::new(f))),
        }
    }
}

/// A mock RPC client that responds programmatically to requests.
/// Useful for testing.
pub struct MockRpcClient {
    // These are all accessed for just long enough to call the method. The method
    // returns a future, but the method call itself isn't held for long.
    method_handlers_once: Arc<Mutex<HashMap<String, VecDeque<MethodHandlerFnOnce>>>>,
    method_handlers: Arc<Mutex<HashMap<String, MethodHandlerFn>>>,
    method_fallback: Option<Arc<Mutex<MethodHandlerFn>>>,
    subscription_handlers_once: Arc<Mutex<HashMap<String, VecDeque<SubscriptionHandlerFnOnce>>>>,
    subscription_handlers: Arc<Mutex<HashMap<String, SubscriptionHandlerFn>>>,
    subscription_fallback: Option<Arc<Mutex<SubscriptionHandlerFn>>>,
}

impl MockRpcClient {
    /// Construct a new [`MockRpcClient`]
    pub fn builder() -> MockRpcClientBuilder {
        MockRpcClientBuilder::new()
    }
}

impl RpcClientT for MockRpcClient {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<serde_json::value::RawValue>>,
    ) -> RawRpcFuture<'a, Box<serde_json::value::RawValue>> {
        // Remove and call a one-time handler if any exist.
        let mut handlers_once = self.method_handlers_once.lock().unwrap();
        if let Some(handlers) = handlers_once.get_mut(method) {
            if let Some(handler) = handlers.pop_front() {
                return handler(method, params)
            }
        }
        drop(handlers_once);

        // Call a specific handler for the method if one is found.
        let mut handlers = self.method_handlers.lock().unwrap();
        if let Some(handler) = handlers.get_mut(method) {
            return handler(method, params)
        }
        drop(handlers);
        
        // Call a fallback handler if one exists
        if let Some(handler) = &self.method_fallback {
            let mut handler = handler.lock().unwrap();
            return handler(method, params)
        }

        // Else, method not found.
        Box::pin(async move { Err(UserError::method_not_found().into()) })
    }
    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<serde_json::value::RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        // Remove and call a one-time handler if any exist.
        let mut handlers_once = self.subscription_handlers_once.lock().unwrap();
        if let Some(handlers) = handlers_once.get_mut(sub) {
            if let Some(handler) = handlers.pop_front() {
                return handler(sub, params, unsub)
            }
        }
        drop(handlers_once);

        // Call a specific handler for the subscrpition if one is found.
        let mut handlers = self.subscription_handlers.lock().unwrap();
        if let Some(handler) = handlers.get_mut(sub) {
            return handler(sub, params, unsub)
        }
        drop(handlers);
        
        // Call a fallback handler if one exists
        if let Some(handler) = &self.subscription_fallback {
            let mut handler = handler.lock().unwrap();
            return handler(sub, params, unsub)
        }
        
        // Else, method not found.
        Box::pin(async move { Err(UserError::method_not_found().into()) })
    }
}

/// State passed to each handler.
pub struct StateHolder<T>(Arc<tokio::sync::Mutex<T>>);

/// A guard for state that is being accessed.
pub struct StateHolderGuard<'a, T>(tokio::sync::MutexGuard<'a, T>);

impl <T> StateHolder<T> {
    /// Get the inner state
    pub async fn get(&self) -> StateHolderGuard<T> {
        StateHolderGuard(self.0.lock().await)
    }

    /// Set the inner state to a new value, returning the old state.
    pub async fn set(&self, new: T) -> T {
        let mut guard = self.0.lock().await;
        std::mem::replace(&mut *guard, new)
    }

    /// Update the inner state, returning the old state.
    pub async fn update<F: FnOnce(&T) -> T>(&self, f: F) -> T {
        let mut guard = self.0.lock().await;
        let new = f(&guard);
        std::mem::replace(&mut *guard, new)
    }
}

impl <'a, T> core::ops::Deref for StateHolderGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
impl <'a, T> core::ops::DerefMut for StateHolderGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

/// Return responses wrapped in this to have them serialized to JSON. 
pub struct Json<T>(pub T);

impl Json<serde_json::Value> {
    /// Create a [`Json<serde_json::Value>`] from some serializable value.
    /// Useful when value types are heterogenous.
    pub fn value_of<T: serde::Serialize>(item: T) -> Self {
        Json(serde_json::to_value(item).expect("item cannot be converted to a serde_json::Value"))
    }
}

/// Anything that can be converted into a valid handler response implements this.
pub trait IntoHandlerResponse {
    /// Convert self into a handler response.
    fn into_handler_response(self) -> Result<Box<RawValue>, Error>;
}

impl <T: IntoHandlerResponse> IntoHandlerResponse for Result<T, Error> {
    fn into_handler_response(self) -> Result<Box<RawValue>, Error> {
        self.and_then(|val| val.into_handler_response())
    }
}

impl <T: IntoHandlerResponse> IntoHandlerResponse for Option<T> {
    fn into_handler_response(self) -> Result<Box<RawValue>, Error> {
        self.ok_or_else(|| UserError::method_not_found().into())
            .and_then(|val| val.into_handler_response())
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

impl IntoHandlerResponse for core::convert::Infallible {
    fn into_handler_response(self) -> Result<Box<RawValue>, Error> {
        match self {}
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

// A tuple of a subscription plus some string is treated as a subscription with that string ID.
impl <T: IntoSubscriptionResponse, S: Into<String>> IntoSubscriptionResponse for (T, S) {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        self.0
            .into_subscription_response()
            .map(|mut r| {
                r.id = Some(self.1.into());
                r
            })
    }
}

impl <T: IntoHandlerResponse + Send + 'static> IntoSubscriptionResponse for tokio::sync::mpsc::Receiver<T> {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        struct IntoStream<T>(tokio::sync::mpsc::Receiver<T>);
        impl <T> futures::Stream for IntoStream<T> {
            type Item = T;
            fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
                self.0.poll_recv(cx)
            }
        }

        Ok(RawRpcSubscription {
            stream: Box::pin(IntoStream(self).map(|item| item.into_handler_response())),
            id: None,
        })
    }
}
impl <T: IntoHandlerResponse + Send + 'static> IntoSubscriptionResponse for tokio::sync::mpsc::UnboundedReceiver<T> {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        struct IntoStream<T>(tokio::sync::mpsc::UnboundedReceiver<T>);
        impl <T> futures::Stream for IntoStream<T> {
            type Item = T;
            fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
                self.0.poll_recv(cx)
            }
        }

        Ok(RawRpcSubscription {
            stream: Box::pin(IntoStream(self).map(|item| item.into_handler_response())),
            id: None,
        })
    }
}

impl IntoSubscriptionResponse for RawRpcSubscription {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        Ok(self)
    }
}

impl <T: IntoSubscriptionResponse> IntoSubscriptionResponse for Result<T, Error> {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        self.and_then(|res| res.into_subscription_response())
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

impl <T: IntoSubscriptionResponse + Send + 'static> IntoSubscriptionResponse for Option<T> {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        match self {
            Some(sub) => {
                sub.into_subscription_response()
            },
            None => {
                Ok(RawRpcSubscription {
                    stream: Box::pin(futures::stream::empty()),
                    id: None,
                }) 
            }
        }
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

impl IntoSubscriptionResponse for core::convert::Infallible {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        match self {}
    }
}

/// Send the first items and then the second items back on a subscription;
/// If any one of the responses is an error, we'll return the error.
/// If one response has an ID and the other doesn't, we'll use that ID.
pub struct AndThen<A, B>(pub A, pub B);

impl <A: IntoSubscriptionResponse, B: IntoSubscriptionResponse> IntoSubscriptionResponse for AndThen<A, B> {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        let a_responses = self.0.into_subscription_response();
        let b_responses = self.1.into_subscription_response();

        match (a_responses, b_responses) {
            (Err(a), _) => {
                Err(a)
            },
            (_, Err(b)) => {
                Err(b)
            },
            (Ok(mut a), Ok(b)) => {
                a.stream = Box::pin(a.stream.chain(b.stream));
                a.id = a.id.or(b.id);
                Ok(a)
            }
        }
    }
}

/// Send back either one response or the other.
pub enum Either<A, B> {
    /// The first possibility.
    A(A),
    /// The second possibility.
    B(B)
}

impl <A: IntoHandlerResponse, B: IntoHandlerResponse> IntoHandlerResponse for Either<A, B> {
    fn into_handler_response(self) -> Result<Box<RawValue>, Error> {
        match self {
            Either::A(a) => a.into_handler_response(),
            Either::B(b) => b.into_handler_response(),
        }
    }
}

impl <A: IntoSubscriptionResponse, B: IntoSubscriptionResponse> IntoSubscriptionResponse for Either<A, B> {
    fn into_subscription_response(self) -> Result<RawRpcSubscription, Error> {
        match self {
            Either::A(a) => a.into_subscription_response(),
            Either::B(b) => b.into_subscription_response(),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{RpcClient, rpc_params};
    use super::*;

    #[tokio::test]
    async fn test_method_params() {
        let rpc_client = MockRpcClient::builder()
            .method_handler("foo", |params| async {
                Json(params)
            })
            .build();

        let rpc_client = RpcClient::new(rpc_client);

        // We get back whatever params we give
        let res: (i32,i32,i32) = rpc_client.request("foo", rpc_params![1, 2, 3]).await.unwrap();
        assert_eq!(res, (1,2,3));

        let res: (String,) = rpc_client.request("foo", rpc_params!["hello"]).await.unwrap();
        assert_eq!(res, ("hello".to_owned(),));
    }

    #[tokio::test]
    async fn test_method_handler_then_fallback() {
        let rpc_client = MockRpcClient::builder()
            .method_handler("foo", |_params| async {
                Json(1)
            })
            .method_fallback(|name, _params| async {
                Json(name)
            })
            .build();

        let rpc_client = RpcClient::new(rpc_client);

        // Whenever we call "foo", we get 1 back.
        for i in [1,1,1,1] {
            let res: i32 = rpc_client.request("foo", rpc_params![]).await.unwrap();
            assert_eq!(res, i);
        }

        // Whenever we call anything else, we get the name of the method back
        for name in ["bar", "wibble", "steve"] {
            let res: String = rpc_client.request(name, rpc_params![]).await.unwrap();
            assert_eq!(res, name);
        }
    }

    #[tokio::test]
    async fn test_method_once_then_handler() {
        let rpc_client = MockRpcClient::builder()
            .method_handler_once("foo", |_params| async {
                Json(1)
            })
            .method_handler("foo", |_params| async {
                Json(2)
            })
            .build();

        let rpc_client = RpcClient::new(rpc_client);

        // Check that we call the "once" one time and then the second after that.
        for i in [1,2,2,2,2] {
            let res: i32 = rpc_client.request("foo", rpc_params![]).await.unwrap();
            assert_eq!(res, i);
        }
    }

    #[tokio::test]
    async fn test_method_once() {
        let rpc_client = MockRpcClient::builder()
            .method_handler_once("foo", |_params| async {
                Json(1)
            })
            .method_handler_once("foo", |_params| async {
                Json(2)
            })
            .method_handler_once("foo", |_params| async {
                Json(3)
            })
            .build();

        let rpc_client = RpcClient::new(rpc_client);

        // Check that each method is only called once, in the right order.
        for i in [1,2,3] {
            let res: i32 = rpc_client.request("foo", rpc_params![]).await.unwrap();
            assert_eq!(res, i);
        }

        // Check that we get a "method not found" error afterwards.
        let err = rpc_client.request::<i32>("foo", rpc_params![]).await.unwrap_err();
        let not_found_code = UserError::method_not_found().code;
        assert!(matches!(err, Error::User(u) if u.code == not_found_code));
    }

    #[tokio::test]
    async fn test_subscription_once_then_handler_then_fallback() {
        let rpc_client = MockRpcClient::builder()
            .subscription_handler_once("foo", |_params, _unsub| async {
                vec![Json(0), Json(0)]
            })
            .subscription_handler("foo", |_params, _unsub| async {
                vec![Json(1), Json(2), Json(3)]
            })
            .subscription_fallback(|_name, _params, _unsub| async {
                vec![Json(4)]
            })
            .build();

        let rpc_client = RpcClient::new(rpc_client);

        // "foo" returns 0,0 the first time it's subscribed to
        let sub = rpc_client.subscribe::<i32>("foo", rpc_params![], "unsub").await.unwrap();
        let res: Vec<i32> = sub.map(|i| i.unwrap()).collect().await;
        assert_eq!(res, vec![0,0]);

        // then, "foo" returns 1,2,3 in subscription every other time
        for _ in 1..5 {
            let sub = rpc_client.subscribe::<i32>("foo", rpc_params![], "unsub").await.unwrap();
            let res: Vec<i32> = sub.map(|i| i.unwrap()).collect().await;
            assert_eq!(res, vec![1,2,3]);
        }

        // anything else returns 4
        let sub = rpc_client.subscribe::<i32>("bar", rpc_params![], "unsub").await.unwrap();
        let res: Vec<i32> = sub.map(|i| i.unwrap()).collect().await;
        assert_eq!(res, vec![4]);
    }

    #[tokio::test]
    async fn test_subscription_and_then_with_channel() {
        let (tx, rx) = tokio::sync::mpsc::channel(10);

        let rpc_client = MockRpcClient::builder()
            .subscription_handler_once("foo", move |_params, _unsub| async move {
                AndThen(
                    // These should be sent first..
                    vec![Json(1), Json(2), Json(3)],
                    // .. and then anything the channel is handing back.
                    rx
                )
            })
            .build();

        let rpc_client = RpcClient::new(rpc_client);

        // Send a few values down the channel to be handed back in "foo" subscription:
        tokio::spawn(async move {
            for i in 4..=6 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                tx.send(Json(i)).await.unwrap();
            }
        });

        // Expect all values back:
        let sub = rpc_client.subscribe::<i32>("foo", rpc_params![], "unsub").await.unwrap();
        let res: Vec<i32> = sub.map(|i| i.unwrap()).collect().await;
        assert_eq!(res, vec![1,2,3,4,5,6]);
    }
}