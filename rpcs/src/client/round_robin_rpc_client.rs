// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a [`RoundRobinRpcClient`], which is useful for load balancing
//! requests across multiple RPC clients.
//! 
//! # Example
//! 
//! ```rust,no_run
//! # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
//! use subxt_rpcs::client::{RpcClient, RoundRobinRpcClient, jsonrpsee_client};
//! 
//! // Construct some RpcClients (we'll make some jsonrpsee clients here, but
//! // you could use anything which implements `RpcClientT`).
//! let client1 = jsonrpsee_client("http://localhost:8080").await.unwrap();
//! let client2 = jsonrpsee_client("http://localhost:8081").await.unwrap();
//! let client3 = jsonrpsee_client("http://localhost:8082").await.unwrap();
//! 
//! let round_robin_client = RoundRobinRpcClient::new(vec![client1, client2, client3]);
//! 
//! // Build an RPC Client that can be used in Subxt or in conjunction with
//! // the RPC methods provided in this crate. 
//! let rpc_client = RpcClient::new(round_robin_client);
//! # Ok(())
//! # }
//! ```

use super::{ RpcClientT, RawRpcFuture, RawRpcSubscription };
use std::sync::{ Arc, atomic::{ AtomicUsize, Ordering } };

/// A simple RPC client which is provided a set of clients on initialization and
/// will round-robin through them for each request.
#[derive(Clone, Debug)]
pub struct RoundRobinRpcClient<Client> {
    inner: Arc<RoundRobinRpcClientInner<Client>>,
}

#[derive(Debug)]
struct RoundRobinRpcClientInner<Client> {
    clients: Vec<Client>,
    next_index: AtomicUsize,
}

impl <Client: RpcClientT> RoundRobinRpcClient<Client> {
    /// Create a new `RoundRobinRpcClient` with the given clients.
    /// 
    /// # Panics
    /// 
    /// Panics if the `clients` vector is empty.
    pub fn new(clients: Vec<Client>) -> Self {
        assert!(!clients.is_empty(), "At least one client must be provided");
        Self {
            inner: Arc::new(RoundRobinRpcClientInner {
                clients,
                next_index: AtomicUsize::new(0),
            }),
        }
    }

    fn next_client(&self) -> &Client {
        let idx = self.next_index();
        &self.inner.clients[idx]
    }

    fn next_index(&self) -> usize {
        // Note: fetch_add wraps on overflow so no need to handle this.
        self.inner.next_index.fetch_add(1, Ordering::Relaxed) % self.inner.clients.len()
    }
}

impl <Client: RpcClientT> RpcClientT for RoundRobinRpcClient<Client> {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<serde_json::value::RawValue>>,
    ) -> RawRpcFuture<'a, Box<serde_json::value::RawValue>> {
        let client = self.next_client();
        client.request_raw(method, params)
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<serde_json::value::RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        let client = self.next_client();
        client.subscribe_raw(sub, params, unsub)
    }
}