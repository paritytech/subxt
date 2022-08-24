// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    RpcClientT,
    RpcSubscriptionStream,
};
use crate::error::Error;
use futures::{
    Stream,
    StreamExt,
};
use serde::{
    de::DeserializeOwned,
    Serialize,
};
use serde_json::value::RawValue;
use std::{
    pin::Pin,
    sync::Arc,
    task::Poll,
};

/// A concrete wrapper around an [`RpcClientT`] to add a layer of useful functionality on top.
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
    pub async fn request<Res: DeserializeOwned>(
        &self,
        method: &str,
        params: RpcParams,
    ) -> Result<Res, Error> {
        let res = self.0.request_raw(method, params.build()).await?;
        let val = serde_json::from_str(res.get())?;
        Ok(val)
    }

    /// Subscribe to an RPC endpoint, providing the parameters and the method to call to
    /// unsubscribe from it again. Params are expected to be an array or tuple of 0 or more
    /// items which can be serialized.
    pub async fn subscribe<Res: DeserializeOwned>(
        &self,
        sub: &str,
        params: RpcParams,
        unsub: &str,
    ) -> Result<Subscription<Res>, Error> {
        let sub = self.0.subscribe_raw(sub, params.build(), unsub).await?;
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

/// Create some [`RpcParams`] to pass to our [`RpcClient`].
#[macro_export]
macro_rules! rpc_params {
    ($($p:expr), *) => {{
        // May be unused if empty; no params.
        #[allow(unused_mut)]
        let mut params = $crate::rpc::RpcParams::new();
        // loop here to allow breaking early with an error.
        loop {
            $(
                if let Err(e) = params.push($p) {
                    break Err(e)
                }
            )*
            break Ok::<$crate::rpc::RpcParams, $crate::error::Error>(params)
        }
    }}
}
pub use rpc_params;

/// This represents the parameters passed to an [`RpcClient`], and exists to
/// enforce that parameters are provided in the correct format.
#[derive(Debug, Clone, Default)]
pub struct RpcParams(Vec<u8>);

impl RpcParams {
    /// Create a new empty set of [`RpcParams`].
    pub fn new() -> Self {
        // Empty params still need 2 bytes for "[]".
        Self(Vec::with_capacity(2))
    }
    /// Push a parameter into our [`RpcParams`]. This serializes it.
    pub fn push<P: Serialize>(&mut self, param: P) -> Result<(), Error> {
        if self.0.is_empty() {
            self.0.push(b'[');
        } else {
            self.0.push(b',')
        }
        serde_json::to_writer(&mut self.0, &param)?;
        Ok(())
    }
    /// Build a [`RawValue`] from our params.
    fn build(mut self) -> Box<RawValue> {
        if self.0.is_empty() {
            self.0.push(b'[');
        }
        self.0.push(b']');
        let s = String::from_utf8(self.0).expect("JSON is valid utf8");
        RawValue::from_string(s).expect("Should be valid JSON")
    }
}

/// A generic RPC Subscription. This implements [`Stream`].
pub struct Subscription<Res> {
    inner: RpcSubscriptionStream,
    _marker: std::marker::PhantomData<Res>,
}

impl<Res> std::fmt::Debug for Subscription<Res> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("inner", &"RpcSubscriptionStream")
            .field("_marker", &self._marker)
            .finish()
    }
}

impl<Res> Subscription<Res> {
    fn new(inner: RpcSubscriptionStream) -> Self {
        Self {
            inner,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Res: DeserializeOwned> Subscription<Res> {
    /// Wait for the next item from the subscription.
    pub async fn next(&mut self) -> Option<Result<Res, Error>> {
        StreamExt::next(self).await
    }
}

impl<Res> std::marker::Unpin for Subscription<Res> {}

impl<Res: DeserializeOwned> Stream for Subscription<Res> {
    type Item = Result<Res, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
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
