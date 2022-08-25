// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::RpcError;
use futures::Stream;
use serde_json::value::RawValue;
use std::{
    future::Future,
    pin::Pin,
};

/// Any RPC client which implements this can be used in our [`super::Rpc`] type
/// to talk to a node.
///
/// This is a low level interface whose methods expect an already-serialized set of params,
/// and return an owned but still-serialized [`RawValue`], deferring deserialization to
/// the caller. This is the case because we want the methods to be object-safe (which prohibits
/// generics), and want to avoid any unnecessary allocations in serializing/deserializing
/// parameters.
pub trait RpcClientT: Send + Sync + 'static {
    /// Make a raw request for which we expect a single response back from. Implementations
    /// should expect that the params will either be `None`, or be an already-serialized
    /// JSON array of parameters.
    ///
    /// This is a lower level method and is not expected to be manually called; have
    /// a look at [`crate::rpc::RpcClient`] for the higher level wrapper that is used
    /// to interact with this.
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RpcFuture<'a, Box<RawValue>>;

    /// Subscribe to some method. Implementations should expect that the params will
    /// either be `None`, or be an already-serialized JSON array of parameters.
    ///
    /// This is a lower level method and is not expected to be manually called; have
    /// a look at [`crate::rpc::RpcClient`] for the higher level wrapper that is used
    /// to interact with this.
    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RpcFuture<'a, RpcSubscription>;
}

/// A boxed future that is returned from the [`RpcClientT`] methods.
pub type RpcFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, RpcError>> + Send + 'a>>;

/// The inner subscription stream returned from our [`RpcClientT`]'s `subscription` method.
pub type RpcSubscription =
    Pin<Box<dyn Stream<Item = Result<Box<RawValue>, RpcError>> + Send + 'static>>;
