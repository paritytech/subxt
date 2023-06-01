// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

// Dev note; I used the following command to normalize and wrap comments:
// rustfmt +nightly --config wrap_comments=true,comment_width=100,normalize_comments=true subxt/src/book/mod.rs
// It messed up comments in code blocks though, so be prepared to go and fix those.

//! # The Subxt Guide
//!
//! Subxt is a library for interacting with Substrate based nodes. It has a focus on **sub**mitting
//! e**xt**rinsics, hence the name, however it's also capable of reading blocks, storage, events and
//! constants from a node. The aim of this guide is to explain key concepts and get you started with
//! using Subxt.
//!
//! 1. [Features](#features-at-a-glance)
//! 2. [Limitations](#limitations)
//! 3. [Quick start](#quick-start)
//! 4. [Usage](#usage)
//!
//! ## Features at a glance
//!
//! Here's a quick overview of the features that Subxt has to offer:
//!
//! - Subxt allows you to generate a static, type safe interface to a node given some metadata; this
//!   allows you to catch many errors at compile time rather than runtime.
//! - Subxt also makes heavy use of node metadata to encode/decode the data sent to/from it. This
//!   allows it to target almost any node which can output the correct metadata, and allows it some
//!   flexibility in encoding and decoding things to account for cross-node differences.
//! - Subxt has a pallet-oriented interface, meaning that code you write to talk to some pallet on
//!   one node will often "Just Work" when pointed at different nodes that use the same pallet.
//! - Subxt can work offline; you can generate and sign transactions, access constants from node
//!   metadata and more, without a network connection. This is all checked at compile time, so you
//!   can be certain it won't try to establish a network connection if you don't want it to.
//! - Subxt can forego the statically generated interface and build transactions, storage queries
//!   and constant queries using data provided at runtime, rather than queries constructed
//!   statically.
//! - Subxt can be compiled to WASM to run in the browser, allowing it to back Rust based browser
//!   apps, or even bind to JS apps.
//!
//! ## Limitations
//!
//! In various places, you can provide a block hash to access data at a particular block, for
//! instance:
//!
//! - [`crate::storage::StorageClient::at`]
//! - [`crate::events::EventsClient::at`]
//! - [`crate::blocks::BlocksClient::at`]
//! - [`crate::runtime_api::RuntimeApiClient::at`]
//!
//! However, Subxt is (by default) only capable of properly working with blocks that were produced
//! after the most recent runtime update. This is because it uses the most recent metadata given
//! back by a node to encode and decode things. It's possible to decode older blocks produced by a
//! runtime that emits compatible (currently, V14) metadata by manually setting the metadata used by
//! the client using [`crate::client::OnlineClient::set_metadata()`].
//!
//! Subxt does not support working with blocks produced prior to the runtime update that introduces
//! V14 metadata. It may have some success decoding older blocks using newer metadata, but may also
//! completely fail to do so.
//!
//! ## Quick start
//!
//! Here is a simple but complete example of using Subxt to transfer some tokens from the example
//! accounts, Alice to Bob:
//!
//! ```rust,ignore
#![doc = include_str!("../../examples/tx_basic.rs")]
//! ```
//!
//! This example assumes that a Polkadot node is running locally (Subxt endeavors to support all
//! recent releases). Typically, to use Subxt to talk to some custom Substrate node (for example a
//! parachain node), you'll want to:
//!
//! 1. [Generate an interface](setup::codegen).
//! 2. [Configure and instantiate the client](setup::client).
//!
//! Follow the above links to learn more about each step.
//!
//! ## Usage
//!
//! Once Subxt is configured, the next step is interacting with a node. Follow the links
//! below to learn more about how to use Subxt for each of the following things:
//!
//! - [Transactions](usage::transactions): Subxt can build and submit transactions, wait until they are in
//!   blocks, and retrieve the associated events.
//! - [Storage](usage::storage): Subxt can query the node storage.
//! - [Events](usage::events): Subxt can read the events emitted for recent blocks.
//! - [Constants](usage::constants): Subxt can access the constant values stored in a node, which
//!   remain the same for a given runtime version.
//! - [Blocks](usage::blocks): Subxt can load recent blocks or subscribe to new/finalized blocks,
//!   reading the extrinsics, events and storage at these blocks.
//! - [Runtime APIs](usage::runtime_apis): Subxt can make calls into pallet runtime APIs to retrieve
//!   data.
//!
pub mod setup;
pub mod usage;
