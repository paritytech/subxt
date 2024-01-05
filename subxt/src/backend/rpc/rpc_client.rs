// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{RawRpcSubscription, RpcClientT};
use crate::error::Error;
use futures::{Stream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::RawValue;
use std::{pin::Pin, sync::Arc, task::Poll};

/// A concrete wrapper around an [`RpcClientT`] which provides some higher level helper methods,
/// is cheaply cloneable, and can be handed to things like [`crate::client::OnlineClient`] to
/// instantiate it.
#[derive(Clone)]
pub struct RpcClient {
    client: Arc<dyn RpcClientT>,
}

impl RpcClient {
    #[cfg(feature = "jsonrpsee")]
    /// Create a default RPC client pointed at some URL, currently based on [`jsonrpsee`].
    pub async fn from_url<U: AsRef<str>>(url: U) -> Result<Self, Error> {
        let client = jsonrpsee_helpers::client(url.as_ref())
            .await
            .map_err(|e| crate::error::RpcError::ClientError(Box::new(e)))?;
        Ok(Self::new(client))
    }

    /// Create a new [`RpcClient`] from an arbitrary [`RpcClientT`] implementation.
    pub fn new<R: RpcClientT>(client: R) -> Self {
        RpcClient {
            client: Arc::new(client),
        }
    }

    /// Make an RPC request, given a method name and some parameters.
    ///
    /// See [`RpcParams`] and the [`rpc_params!`] macro for an example of how to
    /// construct the parameters.
    pub async fn request<Res: DeserializeOwned>(
        &self,
        method: &str,
        params: RpcParams,
    ) -> Result<Res, Error> {
        let res = self.client.request_raw(method, params.build()).await?;
        let val = serde_json::from_str(res.get())?;
        Ok(val)
    }

    /// Subscribe to an RPC endpoint, providing the parameters and the method to call to
    /// unsubscribe from it again.
    ///
    /// See [`RpcParams`] and the [`rpc_params!`] macro for an example of how to
    /// construct the parameters.
    pub async fn subscribe<Res: DeserializeOwned>(
        &self,
        sub: &str,
        params: RpcParams,
        unsub: &str,
    ) -> Result<RpcSubscription<Res>, Error> {
        let sub = self
            .client
            .subscribe_raw(sub, params.build(), unsub)
            .await?;
        Ok(RpcSubscription::new(sub))
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
        &*self.client
    }
}

/// Create some [`RpcParams`] to pass to our [`RpcClient`]. [`RpcParams`]
/// simply enforces that parameters handed to our [`RpcClient`] methods
/// are the correct shape.
///
/// As with the [`serde_json::json!`] macro, this will panic if you provide
/// parameters which cannot successfully be serialized to JSON.
///
/// # Example
///
/// ```rust
/// use subxt::backend::rpc::{ rpc_params, RpcParams };
///
/// // If you provide no params you get `None` back
/// let params: RpcParams = rpc_params![];
/// assert!(params.build().is_none());
///
/// // If you provide params you get `Some<Box<RawValue>>` back.
/// let params: RpcParams = rpc_params![1, true, "foo"];
/// assert_eq!(params.build().unwrap().get(), "[1,true,\"foo\"]");
/// ```
#[macro_export]
macro_rules! rpc_params {
    ($($p:expr), *) => {{
        // May be unused if empty; no params.
        #[allow(unused_mut)]
        let mut params = $crate::backend::rpc::RpcParams::new();
        $(
            params.push($p).expect("values passed to rpc_params! must be serializable to JSON");
        )*
        params
    }}
}
pub use rpc_params;

/// This represents the parameters passed to an [`RpcClient`], and exists to
/// enforce that parameters are provided in the correct format.
///
/// Prefer to use the [`rpc_params!`] macro for simpler creation of these.
///
/// # Example
///
/// ```rust
/// use subxt::backend::rpc::RpcParams;
///
/// let mut params = RpcParams::new();
/// params.push(1).unwrap();
/// params.push(true).unwrap();
/// params.push("foo").unwrap();
///
/// assert_eq!(params.build().unwrap().get(), "[1,true,\"foo\"]");
/// ```
#[derive(Debug, Clone, Default)]
pub struct RpcParams(Vec<u8>);

impl RpcParams {
    /// Create a new empty set of [`RpcParams`].
    pub fn new() -> Self {
        Self(Vec::new())
    }
    /// Push a parameter into our [`RpcParams`]. This serializes it to JSON
    /// in the process, and so will return an error if this is not possible.
    pub fn push<P: Serialize>(&mut self, param: P) -> Result<(), Error> {
        if self.0.is_empty() {
            self.0.push(b'[');
        } else {
            self.0.push(b',')
        }
        serde_json::to_writer(&mut self.0, &param)?;
        Ok(())
    }
    /// Build a [`RawValue`] from our params, returning `None` if no parameters
    /// were provided.
    pub fn build(mut self) -> Option<Box<RawValue>> {
        if self.0.is_empty() {
            None
        } else {
            self.0.push(b']');
            let s = unsafe { String::from_utf8_unchecked(self.0) };
            Some(RawValue::from_string(s).expect("Should be valid JSON"))
        }
    }
}

/// A generic RPC Subscription. This implements [`Stream`], and so most of
/// the functionality you'll need to interact with it comes from the
/// [`StreamExt`] extension trait.
pub struct RpcSubscription<Res> {
    inner: RawRpcSubscription,
    _marker: std::marker::PhantomData<Res>,
}

impl<Res> std::fmt::Debug for RpcSubscription<Res> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcSubscription")
            .field("inner", &"RawRpcSubscription")
            .field("_marker", &self._marker)
            .finish()
    }
}

impl<Res> RpcSubscription<Res> {
    /// Creates a new [`RpcSubscription`].
    pub fn new(inner: RawRpcSubscription) -> Self {
        Self {
            inner,
            _marker: std::marker::PhantomData,
        }
    }

    /// Obtain the ID associated with this subscription.
    pub fn subscription_id(&self) -> Option<&str> {
        self.inner.id.as_deref()
    }
}

impl<Res: DeserializeOwned> RpcSubscription<Res> {
    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<Result<Res, Error>> {
        StreamExt::next(self).await
    }
}

impl<Res> std::marker::Unpin for RpcSubscription<Res> {}

impl<Res: DeserializeOwned> Stream for RpcSubscription<Res> {
    type Item = Result<Res, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let res = futures::ready!(self.inner.stream.poll_next_unpin(cx));

        // Decode the inner RawValue to the type we're expecting and map
        // any errors to the right shape:
        let res = res.map(|r| {
            r.map_err(|e| e.into())
                .and_then(|raw_val| serde_json::from_str(raw_val.get()).map_err(|e| e.into()))
        });

        Poll::Ready(res)
    }
}

// helpers for a jsonrpsee specific RPC client.
#[cfg(all(feature = "jsonrpsee", feature = "native"))]
mod jsonrpsee_helpers {
    pub use jsonrpsee::{
        client_transport::ws::{Receiver, Sender, Url, WsTransportClientBuilder},
        core::{client::Client, Error},
    };

    /// Build WS RPC client from URL
    pub async fn client(url: &str) -> Result<Client, Error> {
        let (sender, receiver) = ws_transport(url).await?;
        Ok(Client::builder()
            .max_buffer_capacity_per_subscription(4096)
            .build_with_tokio(sender, receiver))
    }

    async fn ws_transport(url: &str) -> Result<(Sender, Receiver), Error> {
        let url = Url::parse(url).map_err(|e| Error::Transport(e.into()))?;
        WsTransportClientBuilder::default()
            .build(url)
            .await
            .map_err(|e| Error::Transport(e.into()))
    }
}

// helpers for a jsonrpsee specific RPC client.
#[cfg(all(feature = "jsonrpsee", feature = "web", target_arch = "wasm32"))]
mod jsonrpsee_helpers {
    pub use jsonrpsee::{
        client_transport::web,
        core::{
            client::{Client, ClientBuilder},
            Error,
        },
    };

    /// Build web RPC client from URL
    pub async fn client(url: &str) -> Result<Client, Error> {
        let (sender, receiver) = web::connect(url)
            .await
            .map_err(|e| Error::Transport(e.into()))?;
        Ok(ClientBuilder::default()
            .max_buffer_capacity_per_subscription(4096)
            .build_with_wasm(sender, receiver))
    }
}
