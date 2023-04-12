// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/*!
# Generating an interface

The simplest way to use Subxt is to generate an interface to a chain that you'd like to interact with. This generated interface allows you to build transactions and construct queries to access data while leveraging the full type safety of the Rust compiler.

## The `#[subxt]` macro

The simplest way to generate an interface to use is via the [`#[subxt]`](crate::subxt!) macro. Using this macro looks something like:

```rust,no_run
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}
```

The macro takes a path to some node metadata, and uses that to generate the interface you'll use to talk to it.

The simplest way to do obtain this metadata is to use the `subxt` CLI tool to download it from a node. The tool can be installed via `cargo`:

```shell
cargo install subxt-cli
```

And then, use it to fetch metadata and save it to a file:

```shell
# Download and save all of the metadata:
subxt metadata > metadata.scale
# Download and save only the pallets you want to generate an interface for:
subxt metadata --pallets Balances,System > metadata.scale
```

Explicitly specifying pallets will cause the tool to strip out all unnecessary metadata and type information, making the bundle much smaller in the event that you only need to generate an interface for a subset of the available pallets on the node.


## The CLI tool

Using the [`#[subxt]`](crate::subxt!) macro carries some downsides, notably that using it to generate an interface will have an impact on compile times (though much less of one if you only need a few pallets), and that editor looking tends to not be very good at autocompleting and providing documentation for the generated interface. Additionally, you can't peer into the generated code and see what's going on if you use the macro.

If you'd like to manually generate the same code that the macro generates under the hood, you can use the `subxt codegen` command:

```rust
# Install the CLI tool if you haven't already:
cargo install subxt-cli
# Generate and format rust code, saving it to `interface.rs`:
subxt codegen | rustfmt > interface.rs
```

Use `subxt codegen --help` for more options; many of the options available via the macro are also available via the CLI tool, such as the abliity to substitute generated types for others, or strip out docs from the generated code.

*/