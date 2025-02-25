// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # The Subxt client.
//!
//! The client forms the entry point to all of the Subxt APIs. Every client implements one or
//! both of [`crate::client::OfflineClientT`] and [`crate::client::OnlineClientT`].
//!
//! Subxt ships with three clients which implement one or both of traits:
//! - An [online client](crate::client::OnlineClient).
//! - An [offline client](crate::client::OfflineClient).
//! - A light client (which is currently still unstable).
//!
//! In theory it's possible for users to implement their own clients, although this isn't generally
//! expected.
//!
//! The provided clients are all generic over the [`crate::config::Config`] that they accept, which
//! determines how they will interact with the chain.
//!
//! In the case of the [`crate::OnlineClient`], we have various ways to instantiate it:
//!
//! - [`crate::OnlineClient::new()`] to connect to a node running locally. This uses the default Subxt
//!   backend, and the default RPC client.
//! - [`crate::OnlineClient::from_url()`] to connect to a node at a specific URL. This uses the default Subxt
//!   backend, and the default RPC client.
//! - [`crate::OnlineClient::from_rpc_client()`] to instantiate the client with a [`crate::backend::rpc::RpcClient`].
//! - [`crate::OnlineClient::from_backend()`] to instantiate Subxt using a custom backend. Currently there
//!   is just one backend, [`crate::backend::legacy::LegacyBackend`]. This backend can be instantiated from
//!   a [`crate::backend::rpc::RpcClient`].
//!
//! [`crate::backend::rpc::RpcClient`] can itself be instantiated from anything that implements the low level
//! [`crate::backend::rpc::RpcClientT`] trait; this allows you to decide how Subxt will attempt to talk to a node
//! if you'd prefer something other default client. We use this approach under the hood to implement the light client.
//!
//! ## Examples
//!
//! Most of the other examples will instantiate a client. Here are a couple of examples for less common
//! cases.
//!
//! ### Writing a custom [`crate::backend::rpc::RpcClientT`] implementation:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/setup_client_custom_rpc.rs")]
//! ```
//!
//! ### Creating an [`crate::OfflineClient`]:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/setup_client_offline.rs")]
//! ```
//!
