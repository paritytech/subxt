// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Subxt is a library for interacting with Substrate based nodes. Using it looks something like this:
//!
//! ```rust,ignore
#![doc = include_str!("../examples/tx_basic.rs")]
//! ```
//!
//! Take a look at [the Subxt guide](book) to learn more about how to use Subxt.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(
    all(feature = "web", feature = "native"),
    not(any(feature = "web", feature = "native"))
))]
compile_error!("subxt: exactly one of the 'web' and 'native' features should be used.");

#[cfg(all(feature = "web", feature = "substrate-compat"))]
compile_error!("subxt: the 'substrate-compat' feature is not compatible with the 'web' feature.");

// The guide is here.
pub mod book;

// Suppress an unused dependency warning because tokio is
// only used in example code snippets at the time of writing.
#[cfg(test)]
mod only_used_in_docs_or_tests {
    use subxt_signer as _;
    use tokio as _;
}

// Suppress an unused dependency warning because tracing_subscriber is
// only used in example code snippets at the time of writing.
#[cfg(test)]
use tracing_subscriber as _;

// Used to enable the js feature for wasm.
#[cfg(feature = "web")]
#[allow(unused_imports)]
pub use getrandom as _;

pub mod backend;
pub mod blocks;
pub mod client;
pub mod constants;
pub mod custom_values;
pub mod error;
pub mod events;
pub mod runtime_api;
pub mod storage;
pub mod tx;
pub mod utils;

/// This module provides a [`Config`] type, which is used to define various
/// types that are important in order to speak to a particular chain.
/// [`SubstrateConfig`] provides a default set of these types suitable for the
/// default Substrate node implementation, and [`PolkadotConfig`] for a
/// Polkadot node.
pub mod config {
    pub use subxt_core::config::{
        polkadot, signed_extensions, substrate, BlockHash, Config, DefaultExtrinsicParams,
        DefaultExtrinsicParamsBuilder, ExtrinsicParams, ExtrinsicParamsEncoder, Hasher, Header,
        PolkadotConfig, PolkadotExtrinsicParams, RefineParams, RefineParamsData, SignedExtension,
        SubstrateConfig, SubstrateExtrinsicParams,
    };
    pub use subxt_core::error::ExtrinsicParamsError;
}

/// Types representing the metadata obtained from a node.
pub mod metadata {
    pub use subxt_core::metadata::{DecodeWithMetadata, EncodeWithMetadata, Metadata};
    // Expose metadata types under a sub module in case somebody needs to reference them:
    pub use subxt_metadata as types;
}

/// Submit dynamic transactions.
pub mod dynamic {
    pub use subxt_core::dynamic::{
        constant, runtime_api_call, storage, tx, At, DecodedValue, DecodedValueThunk, Value,
    };
}

// Internal helper macros
#[macro_use]
mod macros;

// Expose light client bits
cfg_unstable_light_client! {
    pub use subxt_lightclient as lightclient;
}

// Expose a few of the most common types at root,
// but leave most types behind their respective modules.
pub use crate::{
    client::{OfflineClient, OnlineClient},
    config::{Config, PolkadotConfig, SubstrateConfig},
    error::Error,
    metadata::Metadata,
};

/// Re-export external crates that are made use of in the subxt API.
pub mod ext {
    pub use codec;
    pub use frame_metadata;
    pub use futures;
    pub use scale_bits;
    pub use scale_decode;
    pub use scale_encode;
    pub use scale_value;
    pub use subxt_core;

    cfg_jsonrpsee! {
        pub use jsonrpsee;
    }

    cfg_substrate_compat! {
        pub use subxt_core::ext::{sp_runtime, sp_core};
    }
}

/// Generate a strongly typed API for interacting with a Substrate runtime from its metadata.
///
/// # Metadata
///
/// First, you'll need to get hold of some metadata for the node you'd like to interact with. One
/// way to do this is by using the `subxt` CLI tool:
///
/// ```bash
/// # Install the CLI tool:
/// cargo install subxt-cli
/// # Use it to download metadata (in this case, from a node running locally)
/// subxt metadata > polkadot_metadata.scale
/// ```
///
/// Run `subxt metadata --help` for more options.
///
/// # Basic usage
///
/// Annotate a Rust module with the `subxt` attribute referencing the aforementioned metadata file.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
/// )]
/// mod polkadot {}
/// ```
///
/// The `subxt` macro will populate the annotated module with all of the methods and types required
/// for interacting with the runtime that the metadata is in via Subxt.
///
/// # Configuration
///
/// This macro supports a number of attributes to configure what is generated:
///
/// ## `crate = "..."`
///
/// Use this attribute to specify a custom path to the `subxt_core` crate:
///
/// ```rust
/// # pub extern crate subxt_core;
/// # pub mod path { pub mod to { pub use subxt_core; } }
/// # fn main() {}
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
///     crate = "crate::path::to::subxt_core"
/// )]
/// mod polkadot {}
/// ```
///
/// This is useful if you write a library which uses this macro, but don't want to force users to depend on `subxt`
/// at the top level too. By default the path `::subxt` is used.
///
/// ## `substitute_type(path = "...", with = "...")`
///
/// This attribute replaces any reference to the generated type at the path given by `path` with a
/// reference to the path given by `with`.
///
/// ```rust
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
///     substitute_type(path = "sp_arithmetic::per_things::Perbill", with = "crate::Foo")
/// )]
/// mod polkadot {}
///
/// # #[derive(
/// #     scale_encode::EncodeAsType,
/// #     scale_decode::DecodeAsType,
/// #     codec::Encode,
/// #     codec::Decode,
/// #     Clone,
/// #     Debug,
/// # )]
/// // In reality this needs some traits implementing on
/// // it to allow it to be used in place of Perbill:
/// pub struct Foo(u32);
/// # impl codec::CompactAs for Foo {
/// #     type As = u32;
/// #     fn encode_as(&self) -> &Self::As {
/// #         &self.0
/// #     }
/// #     fn decode_from(x: Self::As) -> Result<Self, codec::Error> {
/// #         Ok(Foo(x))
/// #     }
/// # }
/// # impl From<codec::Compact<Foo>> for Foo {
/// #     fn from(v: codec::Compact<Foo>) -> Foo {
/// #         v.0
/// #     }
/// # }
/// # fn main() {}
/// ```
///
/// If the type you're substituting contains generic parameters, you can "pattern match" on those, and
/// make use of them in the substituted type, like so:
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
///     substitute_type(
///         path = "sp_runtime::multiaddress::MultiAddress<A, B>",
///         with = "::subxt::utils::Static<polkadot_sdk::sp_runtime::MultiAddress<A, B>>"
///     )
/// )]
/// mod polkadot {}
/// ```
///
/// The above is also an example of using the [`crate::utils::Static`] type to wrap some type which doesn't
/// on it's own implement [`scale_encode::EncodeAsType`] or [`scale_decode::DecodeAsType`], which are required traits
/// for any substitute type to implement by default.
///
/// ## `derive_for_all_types = "..."`
///
/// By default, all generated types derive a small set of traits. This attribute allows you to derive additional
/// traits on all generated types:
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
///     derive_for_all_types = "Eq, PartialEq"
/// )]
/// mod polkadot {}
/// ```
///
/// Any substituted types (including the default substitutes) must also implement these traits in order to avoid errors
/// here.
///
/// ## `derive_for_type(path = "...", derive = "...")`
///
/// Unlike the above, which derives some trait on every generated type, this attribute allows you to derive traits only
/// for specific types. Note that any types which are used inside the specified type may also need to derive the same traits.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
///     derive_for_all_types = "Eq, PartialEq",
///     derive_for_type(path = "frame_support::PalletId", derive = "Ord, PartialOrd"),
///     derive_for_type(path = "sp_runtime::ModuleError", derive = "Hash"),
/// )]
/// mod polkadot {}
/// ```
///
/// ## `runtime_metadata_insecure_url = "..."`
///
/// This attribute can be used instead of `runtime_metadata_path` and will tell the macro to download metadata from a node running
/// at the provided URL, rather than a node running locally. This can be useful in CI, but is **not recommended** in production code,
/// since it runs at compile time and will cause compilation to fail if the node at the given address is unavailable or unresponsive.
///
/// ```rust,ignore
/// #[subxt::subxt(
///     runtime_metadata_insecure_url = "wss://rpc.polkadot.io:443"
/// )]
/// mod polkadot {}
/// ```
///
/// ## `generate_docs`
///
/// By default, documentation is not generated via the macro, since IDEs do not typically make use of it. This attribute
/// forces documentation to be generated, too.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
///     generate_docs
/// )]
/// mod polkadot {}
/// ```
///
/// ## `runtime_types_only`
///
/// By default, the macro will generate various interfaces to make using Subxt simpler in addition with any types that need
/// generating to make this possible. This attribute makes the codegen only generate the types and not the Subxt interface.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
///     runtime_types_only
/// )]
/// mod polkadot {}
/// ```
///
/// ## `no_default_derives`
///
/// By default, the macro will add all derives necessary for the generated code to play nicely with Subxt. Adding this attribute
/// removes all default derives.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
///     runtime_types_only,
///     no_default_derives,
///     derive_for_all_types="codec::Encode, codec::Decode"
/// )]
/// mod polkadot {}
/// ```
///
/// **Note**: At the moment, you must derive at least one of `codec::Encode` or `codec::Decode` or `scale_encode::EncodeAsType` or
/// `scale_decode::DecodeAsType` (because we add `#[codec(..)]` attributes on some fields/types during codegen), and you must use this
/// feature in conjunction with `runtime_types_only` (or manually specify a bunch of defaults to make codegen work properly when
/// generating the subxt interfaces).
///
/// ## `unstable_metadata`
///
/// This attribute works only in combination with `runtime_metadata_insecure_url`. By default, the macro will fetch the latest stable
/// version of the metadata from the target node. This attribute makes the codegen attempt to fetch the unstable version of
/// the metadata first. This is **not recommended** in production code, since the unstable metadata a node is providing is likely
/// to be incompatible with Subxt.
///
/// ```rust,ignore
/// #[subxt::subxt(
///     runtime_metadata_insecure_url = "wss://rpc.polkadot.io:443",
///     unstable_metadata
/// )]
/// mod polkadot {}
/// ```
pub use subxt_macro::subxt;
