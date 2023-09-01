// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module will expose a backend implementation based on the new APIs
//! described at <https://github.com/paritytech/json-rpc-interface-spec/>. See
//! [`rpc_methods`] for the raw API calls.
//!
//! # Warning
//!
//! Everything in this module is **unstable**, meaning that it could change without
//! warning at any time.

mod follow_stream;
mod follow_stream_unpin;

pub mod rpc_methods;

use crate::{config::Header, Config, Error};
use self::rpc_methods::TransactionStatus as RpcTransactionStatus;
use crate::backend::{
    rpc::RpcClient, Backend, BlockRef, RuntimeVersion, StorageResponse, StreamOf, StreamOfResults,
    TransactionStatus,
};

pub use rpc_methods::UnstableRpcMethods;

/// The unstable backend.
#[derive(Debug, Clone)]
pub struct UnstableBackend<T> {
    methods: UnstableRpcMethods<T>,
}

/// Driver for the unstable backend.
#[derive(Debug)]
pub struct UnstableBackendDriver<T> {
    methods: UnstableRpcMethods<T>,
}

// impl a chainhead_follow stream which will re-follow if any stop events are emitted, and otherwise
// just provide back all of the events etc from the stream. allow access to the subscription ID, and emit
// an "interrupted" and "resumed" message if these occur.
//
// impl a chainhead_follow_pinner stream which will wrap the above, and keep track of all block hashes that
// are announced (in a map + queue/linkedlist, each block being Best or Finalized). Any "interrupted" message clears all
// pinned blocks. We also send Unpin messages for all blocks over a certain age (older blocks in the queue).
// Expose an "unpin" method to allow blocks to be unpinned sooner than this by the user. Since we want to unpin things internally
// to this stream, we'll prolly store a list of futures that the stream will poll to completion, which can include Unpin ones.
// maybe also expose a function to allow things to submit to this list any other futures, if we find we want it.
//
// The BlockRef impl can then just retain a reference to this stream and register a block to be unpinned when dropped, letting the
// stream take it to completion internally.
//
// the UnstableBackendDriverInner should then poll this stream. This will sustain the follow subscription and the pinning
// logic should mean that the stream will run forever unless network issues. It will copy all messages received on the
// stream to any interested sub-streams, and expose a function to add new sub-streams.
//
// Then, for different backend calls, we create new sub streams. We can wait for the operationId to be returned from the initial
// method call, and then spawn a new stream which filters on results with that operationId. (we should provide a helper to make doing
// this super easy).
//
// We can also catch any Interruped/Resumed events, and could use this
// to either end the streams or provide an "interrupted" message back to users (depending on the case).

/// Instantiate a new backend which uses the unstable API methods. This returns
/// an interface to the backend, but also a driver which must be polled in order
/// for the backend to make progress.
// pub fn new(client: RpcClient) -> (UnstableBackend<T>, UnstableBackendDriver<T>) {
//     let methods = UnstableRpcMethods::new(client);

//     let backend = UnstableBackend {
//         methods: methods.clone(),
//     };
//     let driver = UnstableBackendDriver {
//         methods: methods
//     };

//     (backend, driver)
// }

impl<T: Config> UnstableBackend<T> {
}

impl<T: Config> super::sealed::Sealed for UnstableBackend<T> {}