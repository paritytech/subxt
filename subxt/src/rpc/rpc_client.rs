// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::{
    Error,
    RpcError
};
use std::pin::Pin;
use futures::{ Stream, StreamExt };
use serde::{ Serialize, de::DeserializeOwned };
use serde_json::value::RawValue;
use std::task::Poll;
use std::sync::Arc;
use super::{
    RpcClientT,
    RpcSubscriptionStream,
};

/// A concrete wrapper around an [`RpcClientT`] to add a layer of useful functionality on top.
//
// Dev note: These would be an Ext trait or methods on the `RpcClientT` trait, but
// we run into issues with async fns not supported in traits and object safety issues,
// which we can avoid by having this concrete wrapper type.
#[derive(Clone)]
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
        let params_value = RawValue::from_string(param_string)
            .expect("Just serialized so must be valid JSON");

        let res = self.0.request_raw(method, params_value).await?;
        let val = serde_json::from_str(res.get())?;
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
        let params_value = RawValue::from_string(param_string)
            .expect("Just serialized so must be valid JSON");

        let sub = self.0.subscribe_raw(sub, params_value, unsub).await?;
        Ok(Subscription::new(sub))
    }
}

impl std::fmt::Debug for RpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RpcClient").finish()
    }
}

impl std::ops::Deref for RpcClient {
    type Target = dyn RpcClientT;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

/// A generic RPC Subscription. This implements [`Stream`].
pub struct Subscription<Res> {
    inner: RpcSubscriptionStream,
    _marker: std::marker::PhantomData<Res>
}

impl <Res> std::fmt::Debug for Subscription<Res> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("inner", &"RpcSubscriptionStream")
            .field("_marker", &self._marker)
            .finish()
    }
}

impl <Res> std::marker::Unpin for Subscription<Res> {}

impl <Res> Subscription<Res> {
    fn new(inner: RpcSubscriptionStream) -> Self {
        Self { inner, _marker: std::marker::PhantomData }
    }
}

impl <Res: DeserializeOwned> Subscription<Res> {
    /// Wait for the next item from the subscription.
    pub async fn next(&mut self) -> Option<Result<Res, Error>> {
        StreamExt::next(self).await
    }
}

impl <Res: DeserializeOwned> Stream for Subscription<Res> {
    type Item = Result<Res, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let res = futures::ready!(self.inner.poll_next_unpin(cx));

        // Decode the inner RawValue to the type we're expecting and map
        // any errors to the right shape:
        let res = res.map(|r| {
            r.map_err(|e| e.into()).and_then(|raw_val| {
                serde_json::from_str(raw_val.get()).map_err(|e| e.into())
            })
        });

        Poll::Ready(res)
    }
}
