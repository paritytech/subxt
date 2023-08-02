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

# signing example

For the signing example, we use the `@polkadot/extension-dapp` NPM package to talk to wallets loaded as browser extensions. In order to sign and submit the transaction using the `polkadot --dev` node we spawned above, you'll need to create a dev account in your wallet of choice. Use the recovery phrase `bottom drive obey lake curtain smoke basket hold race lonely fit walk` and the derivation path `//Alice` to create a dev account that can be used.