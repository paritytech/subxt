The `polkadot_metadata_full.scale` can be obtained by pointing subxt to a polkadot node. For that just run this command in the root of the repository:

```
cargo run --bin subxt metadata --url wss://rpc.polkadot.io:443 > artifacts/polkadot_metadata_full.scale
```

It contains the full metadata including all pallets. From this file, two other stripped down versions are created that contain only a subset of pallets:

-   `polkadot_metadata_tiny.scale` contains no pallets at all.
-   `polkadot_metadata_small.scale` contains only the pallets balances, staking, system and multisig.

They are generated running the following commands from the root of the repository:

```
cargo run --bin subxt metadata --file artifacts/polkadot_metadata_full.scale --pallets "" > artifacts/polkadot_metadata_tiny.scale

cargo run --bin subxt metadata --file artifacts/polkadot_metadata_full.scale --pallets "Balances,Staking,System,Multisig" > artifacts/polkadot_metadata_small.scale
```
