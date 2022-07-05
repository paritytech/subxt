// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides two clients that can be used to work with
//! transactions, storage and events. The [`OfflineClient`] works
//! entirely offline and can be passed to any function that doesn't
//! require network access. The [`OnlineClient`] requires network
//! access.

mod offline_client;
mod online_client;
mod events_client;
mod tx_client;
mod storage_client;
mod constants_client;

pub use offline_client::{
    OfflineClient,
    OfflineClientT,
};
pub use online_client::{
    OnlineClient,
    OnlineClientT,
};

/// This module contains a client for working with events.
pub mod events {
    pub use super::events_client::*;
}

/// This module contains a client for working with transactions.
pub mod tx {
    pub use super::tx_client::*;
}

/// This module contains a client for working with storage.
pub mod storage {
    pub use super::storage_client::*;
}

/// This module contains a client for working with constants.
pub mod constants {
    pub use super::constants_client::*;
}