// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This crate provides a low level RPC interface to Substrate based nodes.

#[cfg(any(
    all(feature = "web", feature = "native"),
    not(any(feature = "web", feature = "native"))
))]
compile_error!("subxt-rpcs: exactly one of the 'web' and 'native' features should be used.");


mod macros;

pub mod utils;
pub mod client;
pub mod methods;

// Expose the most common things at the top level:
pub use client::{ RpcClient, RpcClientT };
pub use methods::{ ChainHeadRpcMethods, LegacyRpcMethods };

/// Configuration used by some of the RPC methods to determine the shape of
/// some of the inputs or responses. 
pub trait RpcConfig {
    /// The block header type.
    type Header: Header;
    /// The block hash type.
    type Hash: BlockHash;
    /// The Account ID type.
    type AccountId: AccountId;
}

/// A trait which is applied to any type that is a valid block header.
pub trait Header: std::fmt::Debug + codec::Decode + serde::de::DeserializeOwned {}
impl <T> Header for T where T: std::fmt::Debug + codec::Decode + serde::de::DeserializeOwned {}

/// A trait which is applied to any type that is a valid block hash.
pub trait BlockHash: serde::de::DeserializeOwned + serde::Serialize {}
impl <T> BlockHash for T where T: serde::de::DeserializeOwned + serde::Serialize {}

/// A trait which is applied to any type that is a valid Account ID.
pub trait AccountId: serde::Serialize {}
impl <T> AccountId for T where T: serde::Serialize {}

#[cfg(feature = "subxt-core")]
mod impl_config {
    use super::*;
    impl <T> RpcConfig for T where T: subxt_core::Config {
        type Header = T::Header;
        type Hash = T::Hash;
        type AccountId = T::AccountId;
    }
}

/// This encapsulates any errors that could be emitted in this crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    // Dev note: We need the error to be safely sent between threads
    // for `subscribe_to_block_headers_filling_in_gaps` and friends.
    /// An error coming from the underlying RPC Client.
    #[error("RPC error: client error: {0}")]
    Client(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// The connection was lost and automatically reconnected. Reconnecting clients
    /// should only emit this when they plan to try reconnecting internally, in which
    /// case they should buffer anycalls made to them until they have reconnected and
    /// then send them off in the order given.
    #[error("RPC error: the connection was lost ({0}); reconnect automatically initiated")]
    DisconnectedWillReconnect(String),
    /// Cannot deserialize the response.
    #[error("RPC error: cannot deserialize response: {0}")]
    Deserialization(serde_json::Error),
    /// Cannot SCALE decode some part of the response.
    #[error("RPC error: cannot SCALE decode some part of the response: {0}")]
    Decode(codec::Error),
    /// The requested URL is insecure.
    #[error("RPC error: insecure URL: {0}")]
    InsecureUrl(String),
}
