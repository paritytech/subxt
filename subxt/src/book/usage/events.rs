// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Events
//!
//! In the process of adding extrinsics to a block, they are executed. When extrinsics are executed,
//! they normally produce events describing what's happening (at the very least, an event dictating whether
//! the extrinsic has succeeded or failed). The node may also emit some events of its own as the block is
//! processed.
//!
//! Events live in a single location in node storage which is overwritten at each block. Normal nodes tend to
//! keep a snapshot of the state at a small number of previous blocks, so you can sometimes access
//! older events by using [`crate::events::EventsClient::at()`] and providing an older block hash.
//!
//! When we submit transactions using Subxt, methods like [`crate::tx::TxProgress::wait_for_finalized_success()`]
//! return [`crate::blocks::ExtrinsicEvents`], which can be used to iterate and inspect the events produced
//! by that transaction being executed. We can also access _all_ of the events produced in a single block using one
//! of these two interfaces:
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use subxt::client::OnlineClient;
//! use subxt::config::PolkadotConfig;
//!
//! // Create client:
//! let client = OnlineClient::<PolkadotConfig>::new().await?;
//!
//! // Get events from the latest block (use .at() to specify a block hash):
//! let events = client.blocks().at_latest().await?.events().await?;
//! // We can use this shorthand too:
//! let events = client.events().at_latest().await?;
//! # Ok(())
//! # }
//! ```
//!
//! Once we've loaded our events, we can iterate all events or search for specific events via
//! methods like [`crate::events::Events::iter()`] and [`crate::events::Events::find()`]. See
//! [`crate::events::Events`] and [`crate::events::EventDetails`] for more information.
//!
//! ## Example
//!
//! Here's an example which puts this all together:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/events.rs")]
//! ```
//!
