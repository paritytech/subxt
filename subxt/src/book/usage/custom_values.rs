// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Custom Values
//!
//! Substrate-based chains can expose custom values in their metadata.
//! Each of these values:
//!
//! - can be accessed by a unique __name__.
//! - refers to a concrete __type__ stored in the metadata.
//! - contains a scale encoded __value__ of that type.
//!
//! ## Getting a custom value
//!
//! First, you must construct an address to access a custom value. This can be either:
//! - a raw [`str`] which assumes the return type to be the dynamic [`crate::dynamic::Value`] type,
//! - created via [`dynamic`](crate::custom_values::dynamic) function whereby you set the return type
//!   that you want back,
//! - created via statically generated addresses as part of the `#[subxt]` macro which define the return type.
//!
//! With an address, use [`at`](crate::custom_values::CustomValuesClient::at) to access and decode specific values, and
//! [`bytes_at`](crate::custom_values::CustomValuesClient::bytes_at) to access the raw bytes.
//!
//! ## Examples
//!
//! Dynamically accessing a custom value using a [`str`] to select which one:
//!
//! ```rust,ignore
//! use subxt::{OnlineClient, PolkadotConfig, ext::scale_decode::DecodeAsType};
//! use subxt::dynamic::Value;
//!
//! let api = OnlineClient::<PolkadotConfig>::new().await?;
//! let custom_value_client = api.custom_values();
//! let foo: Value = custom_value_client.at("foo")?;
//! ```
//!
//! Use the [`dynamic`](crate::custom_values::dynamic) function to select the return type:
//!
//! ```rust,ignore
//! use subxt::{OnlineClient, PolkadotConfig, ext::scale_decode::DecodeAsType};
//!
//! #[derive(Decode, DecodeAsType, Debug)]
//! struct Foo {
//!     n: u8,
//!     b: bool,
//! }
//!
//! let api = OnlineClient::<PolkadotConfig>::new().await?;
//! let custom_value_client = api.custom_values();
//! let custom_value_addr = subxt::custom_values::dynamic::<Foo>("foo");
//! let foo: Foo = custom_value_client.at(&custom_value_addr)?;
//! ```
//!
//! Alternatively we also provide a statically generated api for custom values:
//!
//! ```rust,ignore
//! #[subxt::subxt(runtime_metadata_path = "some_metadata.scale")]
//! pub mod interface {}
//!
//! let static_address = interface::custom().foo();
//!
//! let api = OnlineClient::<PolkadotConfig>::new().await?;
//! let custom_value_client = api.custom_values();
//!
//! // Now the `at()` function already decodes the value into the Foo type:
//! let foo = custom_value_client.at(&static_address)?;
//! ```         
//!
