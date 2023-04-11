// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/*!
# Events

In the process of adding extrinsics to a block, they are executed. When extrinsics are executed, they may produce events describing what's happening, but additionally the node may add emit some events of its own as the blook is processed. Events live in a single location in node storage which is overwritten at each block.

When we submit extrinsics using Subxt, methods like [`crate::tx::TxProgress::wait_for_finalized_success()`] return [`crate::blocks::ExtrinsicEvents`], which can be used to iterate and inspect the events produced for a specific extrinsic. We can also access _all_ of the events produced in a single block using one of these two interfaces:

```rust,no_run
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use subxt::client::OnlineClient;
use subxt::config::PolkadotConfig;

// Create client:
let client = OnlineClient::<PolkadotConfig>::new().await?;

// Get events from the latest block:
let events = client.blocks().at_latest().await?.events().await?;
// We can use this shorthand too:
let events = client.events().at_latest().await?;
# Ok(())
# }
```

Once we've loaded our events, we can iterate all events or search for specific events via methods like [`crate::events::Events::iter()`] and [`crate::events::Events::find()`]. See [`crate::events::Events`] and [`crate::events::EventDetails`] for more information.

## Example

Here's an example which puts this all together:

*/
//! ```rust,ignore
#![doc = include_str!("../../../../examples/examples/events.rs")]
//! ```
/*!

*/
