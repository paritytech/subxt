// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::RpcError;
use futures::Stream;
use std::{future::Future, pin::Pin};

// Re-exporting for simplicity since it's used a bunch in the trait definition.
pub use serde_json::value::RawValue;

/// Any RPC client which implements this can be used in our [`super::Rpc`] type
/// to talk to a node.
///
/// This is a low level interface whose methods expect an already-serialized set of params,
/// and return an owned but still-serialized [`RawValue`], deferring deserialization to
/// the caller. This is the case because we want the methods to be object-safe (which prohibits
/// generics), and want to avoid any unnecessary allocations in serializing/deserializing
/// parameters.
///
/// # Panics
///
/// Implementations are free to panic if the `RawValue`'s passed to `request_raw` or
/// `subscribe_raw` are not JSON arrays. Internally, we ensure that this is always the case.
pub trait RpcClientT: Send + Sync + 'static {
    /// Make a raw request for which we expect a single response back from. Implementations
    /// should expect that the params will either be `None`, or be an already-serialized
    /// JSON array of parameters.
    ///
    /// See [`super::RpcParams`] and the [`super::rpc_params!`] macro for an example of how to
    /// construct the parameters.
    ///
    /// Prefer to use the interface provided on [`super::RpcClient`] where possible.
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RpcFuture<'a, Box<RawValue>>;

    /// Subscribe to some method. Implementations should expect that the params will
    /// either be `None`, or be an already-serialized JSON array of parameters.
    ///
    /// See [`super::RpcParams`] and the [`super::rpc_params!`] macro for an example of how to
    /// construct the parameters.
    ///
    /// Prefer to use the interface provided on [`super::RpcClient`] where possible.
    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RpcFuture<'a, RpcSubscription>;
}

/// A boxed future that is returned from the [`RpcClientT`] methods.
pub type RpcFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, RpcError>> + Send + 'a>>;

/// The RPC subscription returned from [`RpcClientT`]'s `subscription` method.
pub struct RpcSubscription {
    /// The subscription stream.
    pub stream: RpcSubscriptionStream,
    /// The ID associated with the subscription.
    pub id: Option<RpcSubscriptionId>,
}

/// The inner subscription stream returned from our [`RpcClientT`]'s `subscription` method.
pub type RpcSubscriptionStream =
    Pin<Box<dyn Stream<Item = Result<Box<RawValue>, RpcError>> + Send + 'static>>;

/// The ID associated with the [`RpcClientT`]'s `subscription`.
pub type RpcSubscriptionId = String;
