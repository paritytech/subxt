// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::{
    Error,
    RpcError
};
use std::{
    future::Future,
    pin::Pin,
};
use futures::{ Stream, StreamExt };
use serde::{ Serialize, de::DeserializeOwned };
use serde_json::value::RawValue;
use std::task::Poll;
use std::sync::Arc;

/// Any RPC client which implements this can be used in our [`super::Rpc`] type
/// to talk to a node.
///
/// This is a low level interface which expects an already-serialized set of params,
/// and returns an owned but still-serialized [`RawValue`], deferring deserialization to
/// the caller. This is the case because we want the methods to be object-safe (which prohibits
/// generics), and want to avoid any unnecessary allocations.
//
// Dev note: to avoid a proliferation of where clauses and generic types, we
// currently expect boxed futures/streams to be returned. This imposes a limit on
// implementations and forces an allocation, but is simpler for the library to
// work with.
pub trait RpcClientT: Send + Sync + 'static {
    fn request(&self, method: &str, params: Box<RawValue>) -> RpcResponse;
    fn subscribe(&self, sub: &str, params: Box<RawValue>, unsub: &str) -> RpcSubscription;
}

/// The response returned from our [`RpcClientT`] implementation's `request` method.
pub type RpcResponse = Pin<Box<dyn Future<Output = Result<Box<RawValue>, RpcError>>>>;

/// The response returned from our [`RpcClientT`] implementation's `subscribe` method.
pub type RpcSubscription = Pin<Box<dyn Future<Output = Result<RpcSubscriptionStream, RpcError>>>>;

/// The inner subscription stream returned from our [`RpcClientT`]'s `subscription` method.
pub type RpcSubscriptionStream = Pin<Box<dyn Stream<Item = Result<Box<RawValue>, RpcError>> + Send + Sync + 'static>>;


/// A concrete wrapper around an [`RpcClientT`] to add a layer of useful functionality on top.
pub struct RpcClient(Arc<dyn RpcClientT>);

impl RpcClient {
    pub(crate) fn new<R: RpcClientT>(client: R) -> Self {
        RpcClient(Arc::new(client))
    }

    /// Make an RPC request. Params are expected to be an array or tuple of 0 or more
    /// items which can be serialized.
    pub async fn request<P, Res>(&self, method: &str, params: P) -> Result<Res, Error>
    where
        P: Serialize,
        Res: DeserializeOwned
    {
        let param_string = serde_json::to_string(&params)
            .map_err(|e| RpcError(e.to_string()))?;
        let res = self.0.request(method, &param_string).await?;
        let val = serde_json::from_str(&res)?;
        Ok(val)
    }

    /// Subscribe to an RPC endpoint, providing the parameters and the method to call to
    /// unsubscribe from it again. Params are expected to be an array or tuple of 0 or more
    /// items which can be serialized.
    pub async fn subscribe<P, Res>(&self, sub: &str, params: P, unsub: &str) -> Result<Subscription<Res>, Error>
    where
        P: Serialize,
        Res: DeserializeOwned
    {
        let param_string = serde_json::to_string(&params)
            .map_err(|e| RpcError(e.to_string()))?;
        let sub = self.0.subscribe(sub, &param_string, unsub).await?;
        Ok(Subscription::new(sub))
    }
}

/// An RPC Subscription. This implements [`Stream`].
pub struct Subscription<Res> {
    inner: RpcSubscription,
    _marker: std::marker::PhantomData<Res>
}

impl <Res> std::marker::Unpin for Subscription<Res> {}

impl <Res> Subscription<Res> {
    fn new(inner: RpcSubscription) -> Self {
        Self { inner, _marker: std::marker::PhantomData }
    }
}

impl <Res: DeserializeOwned> Stream for Subscription<Res> {
    type Item = Result<Res, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let res = futures::ready!(self.inner.poll_next_unpin(cx));

        // Decode the inner RawValue to the type we're expecting and map
        // any errors to the right shape:
        res.map(|r| {
            r.map_err(|e| e.into()).and_then(|raw_val| {
                serde_json::from_str(&raw_val).map_err(|e| e.into())
            })
        });

        Poll::Ready(res)
    }
}
