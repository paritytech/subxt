# subxt &middot; ![build](https://github.com/paritytech/subxt/workflows/Rust/badge.svg) [![Latest Version](https://img.shields.io/crates/v/subxt.svg)](https://crates.io/crates/subxt) [![Documentation](https://docs.rs/subxt/badge.svg)](https://docs.rs/subxt)

A library to **sub**mit e**xt**rinsics to a [substrate](https://github.com/paritytech/substrate) node via RPC.

## Usage

See [examples](./examples).

## Integration Testing

Most tests require a running substrate node to communicate with. This is done by spawning an instance of the
substrate node per test. It requires an executable binary `substrate` at [`v3.0.0`](https://github.com/paritytech/substrate/releases/tag/v3.0.0) on your path.

This can be done by downloading the prebuilt binary:

```bash
curl "https://releases.parity.io/substrate/x86_64-debian:stretch/v3.0.0/substrate/substrate" --output substrate --location
chmod +x ./substrate
mv substrate ~/.local/bin
```

Or installed from source via cargo:

```bash
cargo install --git https://github.com/paritytech/substrate node-cli --tag=v3.0.0 --force
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
