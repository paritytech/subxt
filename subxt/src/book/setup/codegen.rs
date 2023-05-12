// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Generating an interface
//!
//! The simplest way to use Subxt is to generate an interface to a chain that you'd like to interact
//! with. This generated interface allows you to build transactions and construct queries to access
//! data while leveraging the full type safety of the Rust compiler.
//!
//! ## The `#[subxt]` macro
//!
//! The most common way to generate the interface is to use the [`#[subxt]`](crate::subxt) macro.
//! Using this macro looks something like:
//!
//! ```rust,no_run
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_tiny.scale")]
//! pub mod polkadot {}
//! ```
//!
//! The macro takes a path to some node metadata, and uses that to generate the interface you'll use
//! to talk to it. [Go here](crate::subxt) to learn more about the options available to the macro.
//!
//! To obtain this metadata you'll need for the above, you can use the `subxt` CLI tool to download it
//! from a node. The tool can be installed via `cargo`:
//!
//! ```shell
//! cargo install subxt-cli
//! ```
//!
//! And then it can be used to fetch metadata and save it to a file:
//!
//! ```shell
//! # Download and save all of the metadata:
//! subxt metadata > metadata.scale
//! # Download and save only the pallets you want to generate an interface for:
//! subxt metadata --pallets Balances,System > metadata.scale
//! ```
//!
//! Explicitly specifying pallets will cause the tool to strip out all unnecessary metadata and type
//! information, making the bundle much smaller in the event that you only need to generate an
//! interface for a subset of the available pallets on the node.
//!
//! ## The CLI tool
//!
//! Using the [`#[subxt]`](crate::subxt) macro carries some downsides:
//!
//! - Using it to generate an interface will have a small impact on compile times (though much less of
//! one if you only need a few pallets).
//! - IDE support for autocompletion and documentation when using the macro interface can be poor.
//! - It's impossible to manually look at the generated code to understand and debug things.
//!
//! If these are an issue, you can manually generate the same code that the macro generates under the hood
//! by using the `subxt codegen` command:
//!
//! ```shell
//! # Install the CLI tool if you haven't already:
//! cargo install subxt-cli
//! # Generate and format rust code, saving it to `interface.rs`:
//! subxt codegen | rustfmt > interface.rs
//! ```
//!
//! Use `subxt codegen --help` for more options; many of the options available via the macro are
//! also available via the CLI tool, such as the ability to substitute generated types for others,
//! or strip out docs from the generated code.
//!
