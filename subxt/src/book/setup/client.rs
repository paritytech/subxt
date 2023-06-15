// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Configuring the Subxt client
//!
//! Subxt ships with two clients, an [offline client](crate::client::OfflineClient) and an [online
//! client](crate::client::OnlineClient). These are backed by the traits
//! [`crate::client::OfflineClientT`] and [`crate::client::OnlineClientT`], so in theory it's
//! possible for users to implement their own clients, although this isn't generally expected.
//!
//! Both clients are generic over a [`crate::config::Config`] trait, which is the way that we give
//! the client certain information about how to interact with a node that isn't otherwise available
//! or possible to include in the node metadata. Subxt ships out of the box with two default
//! implementations:
//!
//! - [`crate::config::PolkadotConfig`] for talking to Polkadot nodes, and
//! - [`crate::config::SubstrateConfig`] for talking to generic nodes built with Substrate.
//!
//! The latter will generally work in many cases, but will need modifying if the chain you'd like to
//! connect to has altered any of the details mentioned in [the trait](`crate::config::Config`).
//!
//! In the case of the [`crate::OnlineClient`], we have a few options to instantiate it:
//!
//! - [`crate::OnlineClient::new()`] to connect to a node running locally.
//! - [`crate::OnlineClient::from_url()`] to connect to a node at a specific URL.
//! - [`crate::OnlineClient::from_rpc_client()`] to instantiate the client with a custom RPC
//!   implementation.
//!
//! The latter accepts anything that implements the low level [`crate::rpc::RpcClientT`] trait; this
//! allows you to decide how Subxt will attempt to talk to a node if you'd prefer something other
//! than the provided interfaces.
//!
//! ## Examples
//!
//! Defining some custom config based off the default Substrate config:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/setup_client_custom_config.rs")]
//! ```
//!
//! Writing a custom [`crate::rpc::RpcClientT`] implementation:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/setup_client_custom_rpc.rs")]
//! ```
//!
//! Creating an [`crate::OfflineClient`]:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/setup_client_offline.rs")]
//! ```
//!
