// Copyright 2019-2023 Parity Technologies (UK) Ltd.
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
//! Custom values can be accessed via a [`CustomValuesClient`](crate::custom_values::CustomValuesClient).
//! The client exposes an `at` function by which a custom value can be fetched, given an address to this custom value.
//! An address can be as simple as the aforementioned __name__ as a [str]. This will return a dynamic value, that you can manually decode into the type you want.
//! Suppose, the custom types contain a value of type `Foo` under the name `"foo"` you can access it like in this example:
//!
//! ```rust,ignore
//! use subxt::{OnlineClient, PolkadotConfig, ext::{codec::Decode, scale_decode::DecodeAsType}};
//!
//! #[derive(Decode, DecodeAsType, Debug)]
//! struct Foo {
//!     n: u8,
//!     b: bool,
//! }
//!
//! let api = OnlineClient::<PolkadotConfig>::new().await?;
//! let custom_value_client = api.custom_values();
//! let foo_dynamic = custom_value_client.at("foo")?;
//! let foo: Foo = foo_dynamic.as_type()?;
//!
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
//! Note: Names of custom values are converted to __snake_case__ to produce a valid function name during code generation.
//! If there are multiple values where the names would be equal when converted to __snake_case__, functions might not be statically generated for some of them, because of naming conflicts.
//! Make sure names in the custom values of your metadata differ significantly.
