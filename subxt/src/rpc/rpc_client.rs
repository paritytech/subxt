// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::RpcError;
use std::{
    future::Future,
    pin::Pin,
};
use futures::Stream;

/// Any RPC client which implements this can be used in our [`super::Rpc`] type
/// to talk to a node.
//
// Dev note: to avoid a proliferation of where clauses and generic types, we
// currently expect boxed futures/streams to be returned. This imposes a limit on
// implementations and forces an allocation, but is simpler for the library to
// work with.
pub trait RpcClientT: Send + Sync + 'static {
    fn request<P, I, R>(&self, method: &str, params: P) -> RpcResponse<R>
    where
        P: IntoIterator<Item = I>,
        I: serde::Serialize,
        R: serde::de::DeserializeOwned;

    fn subscribe<P, I, R>(&self, sub: &str, params: P, unsub: &str) -> RpcSubscription<R>
        where
            P: IntoIterator<Item = I>,
            I: serde::Serialize,
            R: serde::de::DeserializeOwned;
}

/// A subscription returned from our [`RpcClientT`] implementation.
pub type RpcSubscription<R> = Pin<Box<dyn Stream<Item = Result<R, RpcError>> + Send + Sync + 'static>>;

/// The response returned from our [`RpcClientT`] implementation.
pub type RpcResponse<R> = Pin<Box<dyn Future<Output = Result<R, RpcError>>>>;