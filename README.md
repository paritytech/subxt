# subxt &middot; ![build](https://github.com/paritytech/substrate-subxt/workflows/Rust/badge.svg) [![Latest Version](https://img.shields.io/crates/v/substrate-subxt.svg)](https://crates.io/crates/substrate-subxt) [![Documentation](https://docs.rs/substrate-subxt/badge.svg)](https://docs.rs/substrate-subxt)

A library to **sub**mit e**xt**rinsics to a [substrate](https://github.com/paritytech/substrate) node via RPC.

## Usage

See [examples](./examples).

## Integration Testing

Most tests require a running substrate node to communicate with. This is done by spawning an instance of the
substrate node per test. It requires an executable binary `substrate` at [`polkadot-v0.9.10`](https://github.com/paritytech/substrate/releases/tag/polkadot-v0.9.10) on your path.

This can be installed from source via cargo:

```bash
cargo install --git https://github.com/paritytech/substrate node-cli --tag=polkadot-v0.9.10 --force
```

**Alternatives**

[substrate-api-client](https://github.com/scs/substrate-api-client) provides similar functionality.

## Subxt Client
By default the client builder will connect to a full node via rpc. The `subxt-client` helps
embedding a light client directly. It can also be used to embed a full node. This is especially
useful for testing and ci.

#### License

<sup>
The entire code within this repository is licensed under the <a href="LICENSE">GPLv3</a>.
Please <a href="https://www.parity.io/contact/">contact us</a> if you have questions about the licensing of our
 products.
</sup>
