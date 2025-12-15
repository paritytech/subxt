// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # The book
//!
//! Subxt is a library for interacting with Substrate based chains. In the early days, it had a focus on
//! **sub**mitting e**xt**rinsics, hence the name, however it has since evolved into a full featured library for
//! interacting with many aspects of chains across the Polkadot Network.
//!
//! ## The Polkadot Network
//!
//! The Polkadot Network is a collection of interconnected Blockchains. Each chain can accept different
//! transactions and store different data, as well as fundamentally differing in other areas, such as the
//! size and format of account addresses, the data expected to be provided alongside transactions, and
//! even more fundamental properties such at the number of bytes used to represent a block number.
//!
//! Blockchains on the Polkadot network are essentially distributed databases:
//! - To write to a chain, users submit _transactions_. Transactions are packets of data that are submitted to a
//!   chain, usually _from_ a specific account. The result of executing these transactions (assuming everything was
//!   successful) is that one or more _storage entries_ in the blockchain will be updated.
//! - _Storage Entries_ are sets of values of a given shape. We can read these in order to see what the current
//!   state of affairs is.
//!
//! Transactions are appended to the blockchain in batches known as blocks, where each block points to the previous
//! one. Blocks are immutable and cannot be altered once added, and so the blockchain is essentially a big append-only
//! log of all of the transactions every submitted. Storage entries update at each block in response to the transactions
//! in it. Interactions with a blockchain happen _at_ a certain block; transactions are submitted to update the state
//! at a given block, and state can be read at a given block.
//!
//! Chains on the Polkadot network are typically created using the Substrate library. This library provides
//! various primitives and defaults which make it much simpler to build a new blockchain. Substrate based chains group the
//! functionality that they expose to users into _pallets_, where each pallet is a self contained module which contains
//! its own storage entries and accepts its own set of transactions. For example, the _Balances_ pallet would accept
//! transactions related to transferring tokens between users, and expose storage indicating which user has what tokens.
//!
//! Aside from transactions and storage entries, pallets also expose:
//! - _Constants_, which are fixed values at a given runtime version.
//! - _View Functions_, which are read-only functions that can be called and return some result.
//!
//! Outside of pallets, _Runtime APIs_ also exist, which are read-only functions that can be called and return some result.
//!
//! All of this logic lives inside the _runtime_ of a chain. An important aspect of Substrate based chains is that this
//! runtime can be upgraded. Runtime upgrades allow the functionality of a chain to be changed over time. This means
//! that the values that you can read and write from can change from one block to the next.
//!
//! In order to understand what interactions are possible at a specific runtime version, each runtime exposes
//! [_metadata_](https://github.com/paritytech/frame-metadata/). Metadata contains all of the information needed to
//! understand what interactions are possible at this runtime. The shape of metadata itself can also change, which
//! is why metadatas are versioned. Typically, we refer to metadata at version 14 or above as "modern" metadata, and
//! metadata older than this as "historic" or "legacy" metadata. In order to interact with blocks at runtimes which expose
//! historic metadata, additional type information needs to be provided by the user, as it was not present in the
//! metadata.
//!
//! ### TL;DR:
//! - Each chain can be configured differently.
//! - Transactions write to the blockchain, and update storage entries which can be read from.
//! - Reading and writing to a chain happens in the context of a specific block.
//! - Functionality is organized into _pallets_.
//! - This functionality can change over time as Runtime updates occur.
//! - Metadata describes what functionality is available for a given runtime.
//!
//! ## Interacting with the Polkadot Network
//!
//! Subxt is built for interacting with Substrate based chains on the Polkadot. The basic steps for using Subxt are:
//!
//! 0. (Optional) Generate an interface to the chain you wish to interact with. This provides type safe APIs.
//! 1. Create/instantiate some configuration for the chain you wish to interact with. Subxt provides a default
//!    [`crate::config::SubstrateConfig`] which works with most chains, or [`crate::config::PolkadotConfig`] which
//!    is configured specifically for the Polkadot Relay Chain.
//! 2. Create a _client_ for interacting with the chain, which consumes this configuration. typically, you'll create
//!    an [`crate::client::OnlineClient`] which will connect to the chain. It's also possible to create an
//!    [`crate::client::OfflineClient`] in the event that you want to avoid any network connection, although in this
//!    case you'll obviously have much more limited functionality available to you.
//! 3. Pick a block to work at. To work at the current block at the time of calling, you'd use
//!    [`crate::client::OnlineClient::at_current_block()`].
//! 4. Do things in the context of this block.
//!
//! Behind the scenes, Subxt takes are of things like:
//! - Downloading the metadata at the given blocks where needed.
//! - Ensuring that anything you try to do is actually valid at the given block.
//! - Encoding and decoding the various data sent back and forth.
//! - Translating older metadatas into a useful format
//!
//! See
#![doc = concat!("[the examples](https://github.com/paritytech/subxt/tree/", env!("SUBXT_REF"), "/subxt/examples)")]
//! for more.
