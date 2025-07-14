//! `subxt-historic` is a library for working with non head-of-chain data on Substrate-based blockchains.

// TODO: Remove this when we're ready to release, and document everything!
#![allow(missing_docs)]

mod utils;

pub mod config;
pub mod client;
pub mod error;
pub mod extrinsics;