// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This crate provides a low level RPC interface to Substrate based nodes.
//!
//! See the [`client`] module for a [`client::RpcClient`] which is driven by implementations
//! of [`client::RpcClientT`] (several of which are provided behind feature flags).
//!
//! See the [`methods`] module for structs which implement sets of concrete RPC calls for
//! communicating with Substrate based nodes. These structs are all driven by a [`client::RpcClient`].
//!
//! The RPC clients/methods here are made use of in `subxt`. Enabling the `subxt` feature flag ensures
//! that all Subxt configurations are also valid RPC configurations.
//!
//! The provided RPC client implementations can be used natively (with the default `native` feature
//! flag) or in WASM based web apps (with the `web` feature flag).

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(
    all(feature = "web", feature = "native"),
    not(any(feature = "web", feature = "native"))
))]
compile_error!("subxt-rpcs: exactly one of the 'web' and 'native' features should be used.");

mod macros;

pub mod client;
pub mod methods;
pub mod utils;

// Used to enable the js feature for wasm.
#[cfg(feature = "web")]
#[allow(unused_imports)]
pub use getrandom as _;

// Expose the most common things at the top level:
pub use client::{RpcClient, RpcClientT};
pub use methods::{ChainHeadRpcMethods, LegacyRpcMethods};

/// Configuration used by some of the RPC methods to determine the shape of
/// some of the inputs or responses.
pub trait RpcConfig {
    /// The block header type.
    type Header: Header;
    /// The block hash type.
    type Hash: Hash;
    /// The Account ID type.
    type AccountId: AccountId;
}

/// A trait which is applied to any type that is a valid block header.
pub trait Header: std::fmt::Debug + codec::Decode + serde::de::DeserializeOwned {}
impl<T> Header for T where T: std::fmt::Debug + codec::Decode + serde::de::DeserializeOwned {}

/// A trait which is applied to any type that is a valid block hash.
pub trait Hash: serde::de::DeserializeOwned + serde::Serialize {}
impl<T> Hash for T where T: serde::de::DeserializeOwned + serde::Serialize {}

/// A trait which is applied to any type that is a valid Account ID.
pub trait AccountId: serde::Serialize {}
impl<T> AccountId for T where T: serde::Serialize {}

// When the subxt feature is enabled, ensure that any valid `subxt::Config`
// is also a valid `RpcConfig`.
#[cfg(feature = "subxt")]
mod impl_config {
    use super::*;
    use subxt_core::config::HashFor;

    impl<T> RpcConfig for T
    where
        T: subxt_core::Config,
    {
        type Header = T::Header;
        type Hash = HashFor<T>;
        type AccountId = T::AccountId;
    }
}

/// This encapsulates any errors that could be emitted in this crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An error which indicates a user fault.
    #[error("User error: {0}")]
    User(#[from] UserError),
    // Dev note: We need the error to be safely sent between threads
    // for `subscribe_to_block_headers_filling_in_gaps` and friends.
    /// An error coming from the underlying RPC Client.
    #[error("RPC error: client error: {0}")]
    Client(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// The connection was lost and the client will automatically reconnect. Clients
    /// should only emit this if they are internally reconnecting, and will buffer any
    /// calls made to them in the meantime until the connection is re-established.
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

impl Error {
    /// Is the error the `DisconnectedWillReconnect` variant? This should be true
    /// only if the underlying `RpcClient` implementation was disconnected and is
    /// automatically reconnecting behind the scenes.
    pub fn is_disconnected_will_reconnect(&self) -> bool {
        matches!(self, Error::DisconnectedWillReconnect(_))
    }
}

/// This error should be returned when the user is at fault making a call,
/// for instance because the method name was wrong, parameters invalid or some
/// invariant not upheld. Implementations of [`RpcClientT`] should turn any such
/// errors into this, so that they can be handled appropriately. By contrast,
/// [`Error::Client`] is emitted when the underlying RPC Client implementation
/// has some problem that isn't user specific (eg network issues or similar).
#[derive(Debug, Clone, serde::Deserialize, thiserror::Error)]
#[serde(deny_unknown_fields)]
pub struct UserError {
    /// Code
    pub code: i32,
    /// Message
    pub message: String,
    /// Optional data
    pub data: Option<Box<serde_json::value::RawValue>>,
}

impl UserError {
    /// Returns a standard JSON-RPC "method not found" error.
    pub fn method_not_found() -> UserError {
        UserError {
            code: -32601,
            message: "Method not found".to_owned(),
            data: None,
        }
    }
}

impl core::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", &self.message, &self.code)
    }
}
