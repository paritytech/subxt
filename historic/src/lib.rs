//! `subxt-historic` is a library for working with non head-of-chain data on Substrate-based blockchains.

// TODO: Remove this when we're ready to release, and document everything!
#![allow(missing_docs)]

mod utils;

pub mod client;
pub mod config;
pub mod error;
pub mod extrinsics;
pub mod storage;

pub use client::{OfflineClient, OnlineClient};
pub use config::polkadot::PolkadotConfig;
pub use config::substrate::SubstrateConfig;
pub use error::Error;

/// External types and crates that may be useful.
pub mod ext {
    pub use futures::stream::{Stream, StreamExt};
}

/// Helper types that could be useful.
pub mod helpers {
    pub use crate::utils::{AnyResolver, AnyResolverError, AnyTypeId};
}
