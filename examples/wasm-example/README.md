# wasm-example

This is a small WASM app using the Yew UI framework to showcase how to use Subxt's features in a WASM environment.

To run the app locally we first install Trunk, a WASM bundler:

```
cargo install --locked trunk
```

You need to have a local polkadot/substrate node with it's JSON-RPC HTTP server running at 127.0.0.1:9933 in order for the examples to be working.
If you have a `polkadot` binary already, running this should be sufficient:

```
polkadot --dev
```

Then, in another terminal, run the app locally with:

```
trunk serve --open
```

