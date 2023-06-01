// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Blocks
//!
//! The [blocks API](crate::blocks::BlocksClient) in Subxt unifies many of the other interfaces, and
//! allows you to:
//!
//! - Access information about specific blocks (see [`crate::blocks::BlocksClient::at()`] and
//!   [`crate::blocks::BlocksClient::at_latest()`]).
//! - Subscribe to [all](crate::blocks::BlocksClient::subscribe_all()),
//!   [best](crate::blocks::BlocksClient::subscribe_best()) or
//!   [finalized](crate::blocks::BlocksClient::subscribe_finalized()) blocks as they are produced.
//!   Prefer to subscribe to finalized blocks unless you know what you're doing.
//!
//! In either case, you'll end up with [`crate::blocks::Block`]'s, from which you can access various
//! information about the block, such a the [header](crate::blocks::Block::header()), [block
//! number](crate::blocks::Block::number()) and [body](crate::blocks::Block::body()).
//! [`crate::blocks::Block`]'s also provide shortcuts to other Subxt APIs that will operate at the
//! given block:
//!
//! - [storage](crate::blocks::Block::storage()),
//! - [events](crate::blocks::Block::events())
//! - [runtime APIs](crate::blocks::Block::runtime_api())
//!
//! Aside from these links to other Subxt APIs, the main thing that we can do here is iterate over and
//! decode the extrinsics in a block body.
//!
//! ## Example
//!
//! Given a block, you can [download the block body](crate::blocks::Block::body()) and iterate over
//! the extrinsics stored within it using [`crate::blocks::BlockBody::extrinsics()`]. From there, you
//! can decode the extrinsics and access various details, including the associated events:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/blocks_subscribing.rs")]
//! ```
//!
