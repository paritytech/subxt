# parachain-example

This example showcases working with Subxt and Zombienet to try out connecting to a locally deployed parachain, here
["Statemint"](https://parachains.info/details/statemint), also known as "Asset Hub".

## Running the example

### 1. Install `zombienet`

[Zombienet](https://github.com/paritytech/zombienet) is a tool for quickly spinning up a (local) blockchain
network. We will use it to start up a local Asset Hub for us.

Please follow the install guide in the [zombienet github repo](https://github.com/paritytech/zombienet) to
install it.

### 2. `polkadot`

We need a relay chain. Build the polkadot binary from the [polkadot github repo](https://github.com/paritytech/polkadot)
and install it in your path:

```txt
git clone https://github.com/paritytech/polkadot.git
cd polkadot
cargo build --release
cargo install --path .
```

### 3. `polkadot-parachain`

The Asset Hub is part of the [cumulus github repo](https://github.com/paritytech/cumulus), an SDK for developing
parachains. Building the cumulus workspace produces a binary called `polkadot-parachain` which can be used to run
Asset Hub nodes.

```txt
git clone https://github.com/paritytech/cumulus.git
cd cumulus
cargo build --release
cargo install --path .
```

### 4. Run the parachain locally

With these binaries installed, Zombienet can now get the parachain running locally from a configuration file, `asset-hub-zombienet.toml`
in this case. We need to have at least 2 validator nodes running via the `polkadot` binary, and an Asset Hub node running via the
`polkadot-parachain` binary. Zombienet starts these up, and gets the parachain registered with the validator nodes for us. To do that,
run:

```txt
zombienet -p native spawn asset-hub-zombienet.toml
```

Zombienet uses Kubernetes by default, but we can use it without Kubernetes by providing the `-p native` flag.

You might have noticed that we use `chain = "rococo-local"` in the `asset-hub-zombienet.toml` file for the relay chain. This is just to
make the epoch time shorter and should have no effect on your interactions with the parachain. Polkadot / Kusama / Rococo have different
epoch times of `24h` / `2h` / `2min` respectively.

### 5. Run the example

The parachain is only registered after the first epoch. So after the previous step, we need to wait 2 minutes until the parachain becomes
interactive and produces blocks. At this point, we can run:

```
cargo run --bin
```

To run our example code.

## Dev notes

We can obtain the metadata for Statemint via the [subxt cli](https://crates.io/crates/subxt-cli) tool, like so:

```txt
subxt metadata  --url wss://polkadot-asset-hub-rpc.polkadot.io:443 > statemint_metadata.scale
```

It is important to explicitly specify the port as `443`.

One way to find a suitable URL to obtain this from is by looking through the sidebar on [Polkadot.js](https://polkadot.js.org/apps/)
to find the Asset Hub entry, and seeing which RPC node URLs it uses.
