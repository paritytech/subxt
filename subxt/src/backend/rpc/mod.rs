// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! RPC types and client for interacting with a substrate node.
//!
//! These are used behind the scenes by Subxt backend implementations, for
//! example [`crate::backend::legacy::LegacyBackend`]. If you need an RPC client,
//! then you can manually instantiate one, and then hand it to Subxt if you'd like
//! to re-use it for the Subxt connection.
//!
//! - [`RpcClientT`] is the underlying dynamic RPC implementation. This provides
//!   the low level [`RpcClientT::request_raw`] and [`RpcClientT::subscribe_raw`]
//!   methods.
//! - [`RpcClient`] is then a slightly higher level wrapper around this, offering
//!   the [`RpcClient::request`] and [`RpcClient::subscribe`] methods.
//!
//! # Example
//!
//! Fetching the genesis hash.
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() {
//! use subxt::{
//!     config::SubstrateConfig,
//!     backend::rpc::RpcClient,
//!     backend::legacy::rpc_methods,
//! };
//!
//! // Instantiate a default RPC client pointing at some URL. Use types from
//! // SubstrateConfig where necessary.
//! let rpc_client = RpcClient::<SubstrateConfig>::from_url("ws://localhost:9944")
//!     .await
//!     .unwrap();
//!
//! // Use it to make RPC calls, here using the legacy genesis_hash method.
//! let genesis_hash = rpc_methods::genesis_hash(&rpc_client)
//!     .await
//!     .unwrap();
//!
//! println!("{genesis_hash}");
//! # }
//! ```

// Allow an `rpc.rs` file in the `rpc` folder to align better
// with other file names for their types.
#![allow(clippy::module_inception)]

#[cfg(feature = "jsonrpsee")]
mod jsonrpsee_impl;

mod rpc_client;
mod rpc_client_t;

pub use rpc_client_t::{
    RawValue, RpcClientT, RpcFuture, RpcSubscription, RpcSubscriptionId, RpcSubscriptionStream,
};

pub use rpc_client::{rpc_params, RpcClient, RpcParams, Subscription};

/// The default RPC client that's used (based on [`jsonrpsee`]).
#[cfg(feature = "jsonrpsee")]
pub async fn default_rpc_client<U: AsRef<str>>(url: U) -> Result<impl RpcClientT, crate::Error> {
    let client = jsonrpsee_helpers::client(url.as_ref())
        .await
        .map_err(|e| crate::error::RpcError::ClientError(Box::new(e)))?;
    Ok(client)
}

// helpers for a jsonrpsee specific OnlineClient.
#[cfg(all(feature = "jsonrpsee", feature = "native"))]
mod jsonrpsee_helpers {
    pub use jsonrpsee::{
        client_transport::ws::{Receiver, Sender, Url, WsTransportClientBuilder},
        core::{
            client::{Client, ClientBuilder},
            Error,
        },
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

// helpers for a jsonrpsee specific OnlineClient.
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
