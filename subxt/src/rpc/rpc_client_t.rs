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
/// This is a low level interface which expects an already-serialized set of params,
/// and returns an owned but still-serialized [`RawValue`], deferring deserialization to
/// the caller. This is the case because we want the methods to be object-safe (which prohibits
/// generics), and want to avoid any unnecessary allocations.
// Dev note: to avoid a proliferation of where clauses and generic types, we
// currently expect boxed futures/streams to be returned. This imposes a limit on
// implementations and forces an allocation, but is simpler for the library to
// work with.
pub trait RpcClientT: Send + Sync + 'static {
    /// Make a raw request for which we expect a single response back from. The params will
    /// be provided in the form of a pre-encoded JSON array.
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Box<RawValue>,
    ) -> RpcFuture<'a, Box<RawValue>>;

    /// Subscribe to some method. The params will be provided in the form of a pre-encoded JSON array,
    /// and the "unsub" param tells the underlying client which method is expected to be called to unsubscribe.
    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Box<RawValue>,
        unsub: &'a str,
    ) -> RpcFuture<'a, RpcSubscription>;
}

/// A boxed future that is returned from the [`RpcClientT`] methods.
pub type RpcFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, RpcError>> + Send + 'a>>;

/// The inner subscription stream returned from our [`RpcClientT`]'s `subscription` method.
pub type RpcSubscription =
    Pin<Box<dyn Stream<Item = Result<Box<RawValue>, RpcError>> + Send + 'static>>;
