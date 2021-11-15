# subxt &middot; ![build](https://github.com/paritytech/subxt/workflows/Rust/badge.svg) [![Latest Version](https://img.shields.io/crates/v/subxt.svg)](https://crates.io/crates/subxt) [![Documentation](https://docs.rs/subxt/badge.svg)](https://docs.rs/subxt)

A library to **sub**mit e**xt**rinsics to a [substrate](https://github.com/paritytech/substrate) node via RPC.

### :warning: Health Warning :warning: considered *alpha* after recent changes, API still subject to change

#### See https://github.com/paritytech/subxt/issues/309 for an overview of outstanding issues.

## Usage

### Downloading metadata from a Substrate node

Use the [`subxt-cli`](./cli) tool to download the metadata for your target runtime from a node.

1. Install:
```bash
cargo install subxt-cli
```
2. Save the encoded metadata to a file:
```bash
subxt metadata -f bytes > metadata.scale
```

This defaults to querying the metadata of a locally running node on the default `http://localhost:9933/`. If querying
a different node then the `metadata` command accepts a `--url` argument.

### Generating the runtime API from the downloaded metadata

Declare a module and decorate it with the `subxt` attribute which points at the downloaded metadata for the 
target runtime:

```rust
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod node_runtime { }
```

**Important:** `runtime_metadata_path` resolves to a path relative to the directory where your crate's `Cargo.toml` 
resides ([`CARGO_MANIFEST_DIR`](https://doc.rust-lang.org/cargo/reference/environment-variables.html)), *not* relative to the source file.

### Initializing the API client

API is still a work in progress. See [examples](./examples) for the current usage.

### Querying Storage

API is still a work in progress. See [tests](./tests/integration/frame) for the current usage.

### Submitting Extrinsics

API is still a work in progress. See [examples](./examples/polkadot_balance_transfer.rs) for the current usage.

## Integration Testing

Most tests require a running substrate node to communicate with. This is done by spawning an instance of the
substrate node per test. It requires an executable binary `substrate` at [`polkadot-v0.9.10`](https://github.com/paritytech/substrate/releases/tag/polkadot-v0.9.10) on your path.

This can be installed from source via cargo:

```bash
cargo install --git https://github.com/paritytech/substrate node-cli --tag=polkadot-v0.9.10 --force
```

**Alternatives**

[substrate-api-client](https://github.com/scs/substrate-api-client) provides similar functionality.

#### License

<sup>
The entire code within this repository is licensed under the <a href="LICENSE">GPLv3</a>.
Please <a href="https://www.parity.io/contact/">contact us</a> if you have questions about the licensing of our
 products.
</sup>
