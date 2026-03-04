<p align="center">
  <img src="./logo.svg" alt="subxt logo" width="420" />
</p>

<p align="center">
  <a href="https://github.com/paritytech/subxt/actions/workflows/rust.yml"><img src="https://github.com/paritytech/subxt/actions/workflows/rust.yml/badge.svg" alt="build"></a>
  <a href="https://crates.io/crates/subxt"><img src="https://img.shields.io/crates/v/subxt.svg" alt="Latest Version"></a>
  <a href="https://docs.rs/subxt"><img src="https://docs.rs/subxt/badge.svg" alt="Documentation"></a>
</p>

Subxt is a library for interacting with chains in the [Polkadot](https://github.com/paritytech/polkadot-sdk) network. It can:

- Submit Extrinsics (this is where the name comes from).
- Access information at any block (eg storage values, constants, Runtime APIs, View Functions).
- Subscribe to new blocks (and then do the above at them).
- Do all of the above via a safe, statically typed interface or via a flexible dynamic interface.
- Do most of the above via a built-in light client to interact with chains trustlessly.
- Compile to WASM and run [entirely in the browser](./examples/wasm-example), or be [called via FFI](./examples/ffi-example) in many other languages.
- Be used entirely offline to provide a subset of the available functionality.

## Usage

Take a look at the [single-file examples](./subxt/examples) folder or the [project based examples](./examples) folder for various smaller or
larger `subxt` usage examples, or [read the docs](https://docs.rs/subxt/latest/subxt) to learn more.

## Example

The "hello world" example of Subxt is submitting a transaction. This is what it looks like:

```rust
use subxt::{Error, OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "/path/to/polkadot_rc_metadata.scale")]
mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Almost all actions are performed at an explicit block. Here we use
    // the current block at the time of running this.
    let at_block = api.at_current_block().await?;

    // Build a balance transfer extrinsic.
    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::transactions()
        .balances()
        .transfer_allow_death(dest, 10_000);

    // Submit the balance transfer extrinsic from Alice, and wait for it 
    // to be successful and in a finalized block. We get back the extrinsic 
    // events if all is well.
    let from = dev::alice();
    let events = at_block
        .transactions()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
        .await?
        .wait_for_finalized_success()
        .await?;

    // (Optional) we can look for a specific event to learn more about 
    // the submission.
    if let Some(event) = events.find_first::<polkadot::balances::events::Transfer>() {
        println!("Balance transfer success: {event:?}");
    }

    Ok(())
}
```

## Real world usage

Please add your project to this list via a PR.

- [cargo-contract](https://github.com/paritytech/cargo-contract/) CLI for interacting with Wasm smart contracts.
- [xcm-cli](https://github.com/ascjones/xcm-cli) CLI for submitting XCM messages.
- [phala-pherry](https://github.com/Phala-Network/phala-blockchain/tree/master/standalone/pherry) The relayer between Phala blockchain and the off-chain Secure workers.
- [crunch](https://github.com/turboflakes/crunch) CLI to claim staking rewards in batch every Era or X hours for substrate-based chains.
- [interbtc-clients](https://github.com/interlay/interbtc-clients) Client implementations for the interBTC parachain; notably the Vault / Relayer and Oracle.
- [tidext](https://github.com/tidelabs/tidext) Tidechain client with Stronghold signer.
- [staking-miner-v2](https://github.com/paritytech/staking-miner-v2) Submit NPos election solutions and get rewards.
- [polkadot-introspector](https://github.com/paritytech/polkadot-introspector) Tools for monitoring Polkadot nodes.
- [ink!](https://github.com/paritytech/ink) Smart contract language that uses `subxt` for allowing developers to conduct [End-to-End testing](https://use.ink/basics/contract-testing/end-to-end-e2e-testing) of their contracts.
- [Chainflip](https://github.com/chainflip-io/chainflip-backend) A decentralised exchange for native cross-chain swaps.
- [Hyperbridge](https://github.com/polytope-labs/hyperbridge) A hyperscalable coprocessor for verifiable cross-chain interoperability.
- [pop CLI](https://github.com/r0gue-io/pop-cli) The all-in-one tool for Polkadot development.

## Alternatives

If you're working in TypeScript / JavaScript, [polkadot-api](https://github.com/polkadot-api/polkadot-api) is an excellent and actively developed alternative.

#### License

The entire code within this repository is dual licensed under the _GPL-3.0_ or _Apache-2.0_ licenses. See [the LICENSE](./LICENSE) file for more details.

Please <a href="https://www.parity.io/contact/">contact us</a> if you have questions about the licensing of our products.
