# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.35.1] - 2024-04-03

This is a small patch release that adds support for `BinaryHeap` in the codegen/subxt-cli
which wasn't supported by `scale-typegen`. This was discovered because the most recent metadata in polkadot
are now utilizing `BinaryHeap`.

## [0.35.0] - 2024-03-21

This release contains several fixes, adds `no_std` support to a couple of crates (`subxt-signer` and `subxt-metadata`) and introduces a few quality of life improvements, which I'll quickly cover:

### Reworked light client ([#1475](https://github.com/paritytech/subxt/pull/1475))

This PR reworks the light client interface. The "basic" usage of connecting to a parachain now looks like this:

```rust
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

use subxt::lightclient::LightClient;

// Instantiate a light client with the Polkadot relay chain given its chain spec.
let (lightclient, polkadot_rpc) = LightClient::relay_chain(POLKADOT_SPEC)?;
// Connect the light client to some parachain by giving a chain spec for it.
let asset_hub_rpc = lightclient.parachain(ASSET_HUB_SPEC)?;

// Now, we can create Subxt clients from these Smoldot backed RPC clients:
let polkadot_api = OnlineClient::<PolkadotConfig>::from_rpc_client(polkadot_rpc).await?;
let asset_hub_api = OnlineClient::<PolkadotConfig>::from_rpc_client(asset_hub_rpc).await?;
```

This interface mirrors the requirement that we must connect to a relay chain before we can connect to a parachain. It also moves the light client specific logic into an `RpcClientT` implementation, rather than exposing it as a `subxt::client::LightClient`.

### Typed Storage Keys ([#1419](https://github.com/paritytech/subxt/pull/1419))

This PR changes the storage interface so that, where possible, we now also decode the storage keys as well as the values when iterating over storage entries:

```rust
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

// Create a new API client, configured to talk to Polkadot nodes.
let api = OnlineClient::<PolkadotConfig>::new().await?;

// Build a storage query to iterate over account information.
let storage_query = polkadot::storage().system().account_iter();

// Get back an iterator of results (here, we are fetching 10 items at
// a time from the node, but we always iterate over one at a time).
let mut results = api.storage().at_latest().await?.iter(storage_query).await?;

while let Some(Ok(kv)) = results.next().await {
    // We used to get a tuple of key bytes + value. Now we get back a
    // `kv` struct containing the bytes and value as well as the actual
    // decoded keys:
    println!("Decoded key(s): {:?}", kv.keys);
    println!("Key bytes: 0x{}", hex::encode(&kv.key_bytes));
    println!("Value: {:?}", kv.value);
}
```

When using the static interface, keys come back as a tuple of values corresponding to the different hashers used in constructing the key. When using a dynamic interface, keys will be encoded/decoded from the type given so long as it implements `subxt::storage::StorageKey`, eg `Vec<scale_value::Value>`.

### Extrinsic Params Refinement ([#1439](https://github.com/paritytech/subxt/pull/1439))

Prior to this PR, one could configure extrinsic signed extensions by providing some params like so:

```rust
// Configure the transaction parameters; we give a small tip and set the
// transaction to live for 32 blocks from the `latest_block` above:
let tx_params = Params::new()
    .tip(1_000)
    .mortal(latest_block.header(), 32)
    .build();

let hash = api.tx().sign_and_submit(&tx, &from, tx_params).await?;
```

If you want to customize the account nonce, you'd use a different call like `create_signed_with_nonce` instead.

One of the downsides of the above approach is that, if you don't provide any explicit params, transactions will be immortal by default (because the signed extensions didn't have the information to do any better).

Now, with the help of a `RefineParams` trait, transactions will default to being mortal and living for 32 blocks unless an explicit mortality is provided as above.

One notable change is that the offline-only `create_signed_with_nonce` and `create_partial_signed_with_nonce` functions have lost the `_with_nonce` suffix. Since we can't discover nonce/mortality settings offline, you should now provide `Params` and set an explicit nonce (and mortality, if you like) when using these calls, otherwise the nonce will be set to 0 and the mortality to `Immortal`.

For a full list of changes, please see the following:

### Added

- Reworked light client ([#1475](https://github.com/paritytech/subxt/pull/1475))
- `no_std` compatibility for `subxt-signer` ([#1477](https://github.com/paritytech/subxt/pull/1477))
- Typed Storage Keys ([#1419](https://github.com/paritytech/subxt/pull/1419))
- Extrinsic Params Refinement ([#1439](https://github.com/paritytech/subxt/pull/1439))
- Make storage_page_size for the LegacyBackend configurable ([#1458](https://github.com/paritytech/subxt/pull/1458))
- `no_std` compatibility for `subxt-metadata` ([#1401](https://github.com/paritytech/subxt/pull/1401))
- Experimental `reconnecting-rpc-client` ([#1396](https://github.com/paritytech/subxt/pull/1396))

### Changed

- `scale-type-resolver` integration ([#1460](https://github.com/paritytech/subxt/pull/1460))
- subxt: Derive `std::cmp` traits for subxt payloads and addresses ([#1429](https://github.com/paritytech/subxt/pull/1429))
- CLI: Return error on wrongly specified type paths ([#1397](https://github.com/paritytech/subxt/pull/1397))
- rpc v2: chainhead support multiple finalized block hashes in `FollowEvent::Initialized` ([#1476](https://github.com/paritytech/subxt/pull/1476))
- rpc v2: rename transaction to transactionWatch ([#1399](https://github.com/paritytech/subxt/pull/1399))

### Fixed

- Avoid a panic in case we try decoding naff bytes ([#1444](https://github.com/paritytech/subxt/pull/1444))
- Fix error mapping to wrong transaction status ([#1445](https://github.com/paritytech/subxt/pull/1445))
- Update DispatchError to match latest in polkadot-sdk ([#1442](https://github.com/paritytech/subxt/pull/1442))
- Handle errors when fetching storage keys from Unstablebackend ([#1440](https://github.com/paritytech/subxt/pull/1440))
- Swap type aliases around to be semantically correct ([#1441](https://github.com/paritytech/subxt/pull/1441))

## [0.34.0] - 2024-01-23

This release introduces a bunch of features that make subxt easier to use. Let's look at a few of them.

### Codegen - Integrating [`scale-typegen`](https://github.com/paritytech/scale-typegen) and adding type aliases ([#1249](https://github.com/paritytech/subxt/pull/1249))

We rewrote the code generation functionality of subxt and outsourced it to the new [`scale-typegen`](https://github.com/paritytech/scale-typegen) crate, which serves a more general purpose.

Since a lot of types used in substrate are rich with generics, this release introduces type aliases into the generated code.
A type alias is generated for the arguments/keys or each call, storage entry, and runtime API method ([#1249](https://github.com/paritytech/subxt/pull/1249)).

### Macro - Errors for misspecified type paths ([#1339](https://github.com/paritytech/subxt/pull/1339))

The subxt macro provides attributes to specify custom derives, attributes, and type substitutions on a per-type basis.
Previously we did not verify that the provided type paths are part of the metadata. This is now fixed:
If you provide an invalid type path, the macro will tell you so. It also suggests similar type paths, you might have meant instead.

```rust
#[subxt::subxt(
    runtime_metadata_path = "metadata.scale",
    derive_for_type(path = "Junctions", derive = "Clone")
)]
pub mod polkadot {}
```

This gives you a compile-time error like this:

```md
Type `Junctions` does not exist at path `Junctions`

A type with the same name is present at:
xcm::v3::junctions::Junctions
xcm::v2::multilocation::Junctions
```

### Macro - Recursive derives and attributes ([#1379](https://github.com/paritytech/subxt/pull/1379))

Previously adding derives on a type containing other types was also cumbersome, see this example:

```rust
#[subxt::subxt(
    runtime_metadata_path = "metadata.scale",
    derive_for_type(path = "xcm::v2::multilocation::MultiLocation", derive = "Clone"),
    derive_for_type(path = "xcm::v2::multilocation::Junctions", derive = "Clone"),
    derive_for_type(path = "xcm::v2::junction::Junction", derive = "Clone"),
    derive_for_type(path = "xcm::v2::NetworkId", derive = "Clone"),
    derive_for_type(path = "xcm::v2::BodyId", derive = "Clone"),
    derive_for_type(path = "xcm::v2::BodyPart", derive = "Clone"),
    derive_for_type(
        path = "bounded_collections::weak_bounded_vec::WeakBoundedVec",
        derive = "Clone"
    )
)]
pub mod polkadot {}
```

We introduced a `recursive` flag for custom derives and attributes that automatically inserts the specified derives on all child types:

```rust
#[subxt::subxt(
    runtime_metadata_path = "metadata.scale",
    derive_for_type(path = "xcm::v2::multilocation::MultiLocation", derive = "Clone", recursive),
)]
pub mod polkadot {}
```

### Subxt CLI - New features and usability improvements ([#1290](https://github.com/paritytech/subxt/pull/1290), [#1336](https://github.com/paritytech/subxt/pull/1336), and [#1379](https://github.com/paritytech/subxt/pull/1379))

Our CLI tool now allows you to explore runtime APIs and events ([#1290](https://github.com/paritytech/subxt/pull/1290)). We also fully integrated with [`scale-typegen-description`](https://github.com/paritytech/scale-typegen/tree/master/description), a crate that can describe types in a friendly way and provide type examples. The output is also color-coded to be easier on the eyes. Get started with these commands:

```sh
# Show details about a runtime API call:
subxt explore --url wss://westend-rpc.polkadot.io api StakingAPI nominations_quota
# Execute a runtime API call from the CLI:
subxt explore --url wss://westend-rpc.polkadot.io api core version -e
# Discover what events a pallet can emit:
subxt explore --url wss://westend-rpc.polkadot.io pallet Balances events
```

All CLI commands that take some metadata via `--file` or `--url`, can now also read the metadata directly from `stdin` with `--file -` ([#1336](https://github.com/paritytech/subxt/pull/1336)).
This allows you to pipe in metadata from other processes like in this command chain:
```sh
parachain-node export-metadata | subxt codegen --file - | rustfmt > main.rs
```

Similar to the macro, the `subxt codegen` command can now also use `recursive` flags:
```sh
subxt codegen --derive-for-type xcm::v2::multilocation::MultiLocation=Clone,recursive
subxt codegen --attributes-for-type "xcm::v2::multilocation::MultiLocation=#[myerror],recursive"
```

### Minor changes and things to be aware of

- Using insecure connections is now an explicit opt-in in many places ([#1309](https://github.com/paritytech/subxt/pull/1309))
- When decoding extrinsics from a block into a static type, we now return it's details (e.g. signature, signed extensions, raw bytes) alongside the statically decoded extrinsic itself ([#1376](https://github.com/paritytech/subxt/pull/1376))

We also made a few fixes and improvements around the unstable backend and the lightclient, preparing them for more stable usage in the future.

### Added

- Errors for misspecified type paths + suggestions ([#1339](https://github.com/paritytech/subxt/pull/1339))
- CLI: Recursive derives and attributes ([#1379](https://github.com/paritytech/subxt/pull/1379))
- CLI: Explore runtime APIs and events, colorized outputs, scale-typegen integration for examples ([#1290](https://github.com/paritytech/subxt/pull/1290))
- Add chainflip to real world usage section of README  ([#1351](https://github.com/paritytech/subxt/pull/1351))
- CLI: Allow using `--file -` to read metadata from stdin ([#1336](https://github.com/paritytech/subxt/pull/1336))
- Codegen: Generate type aliases for better API ergonomics ([#1249](https://github.com/paritytech/subxt/pull/1249))

### Changed

- Return Pending rather than loop around if no new finalized hash in submit_transaction ([#1378](https://github.com/paritytech/subxt/pull/1378))
- Return `ExtrinsicDetails` alongside decoded static extrinsics ([#1376](https://github.com/paritytech/subxt/pull/1376))
- Improve Signed Extension and Block Decoding Examples/Book ([#1357](https://github.com/paritytech/subxt/pull/1357))
- Use `scale-typegen` as a backend for the codegen ([#1260](https://github.com/paritytech/subxt/pull/1260))
- Using insecure connections is now opt-in ([#1309](https://github.com/paritytech/subxt/pull/1309))

### Fixed

- Ensure lightclient chainSpec is at least one block old ([#1372](https://github.com/paritytech/subxt/pull/1372))
- Typo fix in docs ([#1370](https://github.com/paritytech/subxt/pull/1370))
- Don't unpin blocks that may show up again ([#1368](https://github.com/paritytech/subxt/pull/1368))
- Runtime upgrades in unstable backend ([#1348](https://github.com/paritytech/subxt/pull/1348))
- Generate docs for feature gated items ([#1332](https://github.com/paritytech/subxt/pull/1332))
- Backend: Remove only finalized blocks from the event window ([#1356](https://github.com/paritytech/subxt/pull/1356))
- Runtime updates: wait until upgrade on chain ([#1321](https://github.com/paritytech/subxt/pull/1321))
- Cache extrinsic events ([#1327](https://github.com/paritytech/subxt/pull/1327))

## [0.33.0] - 2023-12-06

This release makes a bunch of small QoL improvements and changes. Let's look at the main ones.

### Add support for configuring multiple chains ([#1238](https://github.com/paritytech/subxt/pull/1238))

The light client support previously provided a high level interface for connecting to single chains (ie relay chains). This PR exposes a "low level" interface which allows smoldot (the light client implementation we use) to be configured somewhat more arbitrarily, and then converted into a valid subxt `OnlineClient` to be used.

See [this example](https://github.com/paritytech/subxt/blob/418c3afc923cacd17501f374fdee0d8f588e14fd/subxt/examples/light_client_parachains.rs) for more on how to do this.

We'll likely refine this over time and add a slightly higher level interface to make common operations much easier to do.

### Support decoding signed extensions ([#1209](https://github.com/paritytech/subxt/pull/1209) and [#1235](https://github.com/paritytech/subxt/pull/1235))

This PR makes it possible to decode the signed extensions in extrinsics. This looks something like:

```rust
let api = OnlineClient::<PolkadotConfig>::new().await?;

// Get blocks; here we just subscribe to them:
let mut blocks_sub = api.blocks().subscribe_finalized().await?;

while let Some(block) = blocks_sub.next().await {
    let block = block?;

    // Fetch the extrinsics in the block:
    let extrinsics = block.extrinsics().await?;

    // Iterate over them:
    for extrinsic in extrinsics.iter() {

        // Returns None if the extrinsic isn't signed, so no signed extensions:
        let Some(signed_exts) = extrinsic.signed_extensions() else {
            continue;
        };

        // We can ask for a couple of common values, None if not found:
        println!("Tip: {:?}", signed_exts.tip());
        println!("Nonce: {:?}", signed_exts.tip());

        // Or we can find and decode into a static signed extension type
        // (Err if we hit a decode error first, then None if it's not found):
        if let Ok(Some(era)) = signed_exts.find::<CheckMortality<PolkadotConfig>>() {
            println!("Era: {era:?}");
        }

        // Or we can iterate over the signed extensions to work with them:
        for signed_ext in signed_exts {
            println!("Signed Extension name: {}", signed_ext.name());

            // We can try to statically decode each one:
            if let Ok(Some(era)) = signed_ext.as_signed_extension::<CheckMortality<PolkadotConfig>>() {
                println!("Era: {era:?}");
            }

            // Or we can dynamically decode it into a `scale_value::Value`:
            if let Ok(value) = signed_ext.value() {
                println!("Decoded extension: {value}");
            }
        }
    }
}
```

See the API docs for more.

### ChargeAssetTxPayment: Add support for generic AssetId

Still on the topic of signed extensions, the `ChargeAssetTxPayment` extension was previously not able to be used with a generic AssetId, which prohibited it from being used on the Asset Hub (which uses a `MultiLocation` instead). To address this, we added an `AssetId` type to our `subxt::Config`, which can now be configured.

One example of doing that [can be found here](https://github.com/paritytech/subxt/blob/master/subxt/examples/setup_config_custom.rs).

This example uses a generated `MultiLocation` type to be used as the `AssetId`. Currently it requires a rather hideous set of manual clones like so:

```rust
#[subxt::subxt(
    runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
    derive_for_type(path = "xcm::v2::multilocation::MultiLocation", derive = "Clone"),
    derive_for_type(path = "xcm::v2::multilocation::Junctions", derive = "Clone"),
    derive_for_type(path = "xcm::v2::junction::Junction", derive = "Clone"),
    derive_for_type(path = "xcm::v2::NetworkId", derive = "Clone"),
    derive_for_type(path = "xcm::v2::BodyId", derive = "Clone"),
    derive_for_type(path = "xcm::v2::BodyPart", derive = "Clone"),
    derive_for_type(
        path = "bounded_collections::weak_bounded_vec::WeakBoundedVec",
        derive = "Clone"
    )
)]
```

This is something we plan to address in the next version of Subxt.

### Change SignedExtension matching logic ([#1283](https://github.com/paritytech/subxt/pull/1283))

Before this release, each signed extension had a unique name (`SignedExtension::NAME`). We'd use this name to figure out which signed extensions to apply for a given chain inside the `signed_extensions::AnyOf` type.

However, we recently ran into a new signed extension in Substrate called `SkipCheckIfFeeless`. This extension would wrap another signed extension, but maintained its own name. It has since been "hidden" from the public Substrate interface again, but a result of encountering this is that we have generalised the way that we "match" on signed extensions, so that we can be smarter about it going forwards.

So now, for a given signed extension, we go from:

```rust
impl<T: Config> SignedExtension<T> for ChargeAssetTxPayment<T> {
    const NAME: &'static str = "ChargeAssetTxPayment";
    type Decoded = Self;
}
```

To:

```rust
impl<T: Config> SignedExtension<T> for ChargeAssetTxPayment<T> {
    type Decoded = Self;
    fn matches(identifier: &str, type_id: u32, types: &PortableRegistry) -> bool {
        identifier == "ChargeAssetTxPayment"
    }
}
```

On the whole, we continue matching by name, as in the example above, but this allows an author to inspect the type of the signed extension (and subtypes of it) too if they want the signed extension to match (and thus be used) only in certain cases.

### Remove `wait_for_in_block` helper method ([#1237](https://github.com/paritytech/subxt/pull/1237))

One can no longer use `tx.wait_for_in_block` to wait for a transaction to enter a block. The reason for this removal is that, especially when we migrate to the new `chainHead` APIs, we will no longer be able to reliably obtain any details about the block that the transaction made it into.

In other words, the following sort of thing would often fail:

```rust
tx.wait_for_in_block()
  .await?
  .wait_for_success()
  .await?;
```

The reason for this is that the block announced in the transaction status may not have been "pinned" yet in the new APIs. In the old APIs, errors would occasionally be encountered because the block announced may have been pruned by the time we ask for details for it. Overall; having an "unreliable" higher level API felt like a potential foot gun.

That said, you can still achieve the same via the lower level APIs like so:

```rust
while let Some(status) = tx.next().await {
    match status? {
        TxStatus::InBestBlock(tx_in_block) | TxStatus::InFinalizedBlock(tx_in_block) => {
            // now, we can attempt to work with the block, eg:
            tx_in_block.wait_for_success().await?;
        },
        TxStatus::Error { message } | TxStatus::Invalid { message } | TxStatus::Dropped { message } => {
            // Handle any errors:
            println!("Error submitting tx: {message}");
        },
        // Continue otherwise:
        _ => continue,
    }
}
```

### Subxt-codegen: Tidy crate interface ([#1225](https://github.com/paritytech/subxt/pull/1225))

The `subxt-codegen` crate has always been a bit of a mess because it wasn't really supposed to be used outside of the subxt crates, which had led to issues like https://github.com/paritytech/subxt/issues/1211.

This PR tidies up the interface to that crate so that it's much easier now to programmatically generate the Subxt interface. Now, we have three properly supported ways to do this, depending on your needs:

1. Using the `#[subxt]` macro.
2. Using the `subxt codegen` CLI command.
3. Programmatically via the `subxt-codegen` crate.

Each method aims to expose a similar and consistent set of options.

If you were previously looking to use parts of the type generation logic to, for instance, generate runtime types but not the rest of the Subxt interface, then the https://github.com/paritytech/scale-typegen crate will aim to fill this role eventually.

That sums up the most significant changes. A summary of all of the relevant changes is as follows:

### Added

- CLI: Add command to fetch chainSpec and optimize its size ([#1278](https://github.com/paritytech/subxt/pull/1278))
- Add legacy RPC usage example ([#1279](https://github.com/paritytech/subxt/pull/1279))
- impl RpcClientT for `Arc<T>` and `Box<T>` ([#1277](https://github.com/paritytech/subxt/pull/1277))
- RPC: Implement legacy RPC system_account_next_index ([#1250](https://github.com/paritytech/subxt/pull/1250))
- Lightclient: Add support for configuring multiple chains ([#1238](https://github.com/paritytech/subxt/pull/1238))
- Extrinsics: Allow static decoding of signed extensions ([#1235](https://github.com/paritytech/subxt/pull/1235))
- Extrinsics: Support decoding signed extensions ([#1209](https://github.com/paritytech/subxt/pull/1209))
- ChargeAssetTxPayment: Add support for generic AssetId (eg `u32` or `MultiLocation`) ([#1227](https://github.com/paritytech/subxt/pull/1227))
- Add Clone + Debug on Payloads/Addresses, and compare child storage results ([#1203](https://github.com/paritytech/subxt/pull/1203))

### Changed

- Lightclient: Update smoldot to `0.14.0` and smoldot-light to `0.12.0` ([#1307](https://github.com/paritytech/subxt/pull/1307))
- Cargo: Switch to workspace lints ([#1299](https://github.com/paritytech/subxt/pull/1299))
- Update substrate-* and signer-related dependencies ([#1297](https://github.com/paritytech/subxt/pull/1297))
- Change SignedExtension matching logic and remove SkipCheckIfFeeless bits ([#1283](https://github.com/paritytech/subxt/pull/1283))
- Update the README with the new location of node-cli ([#1282](https://github.com/paritytech/subxt/pull/1282))
- Generalize `substrate-compat` impls to accept any valid hasher/header impl ([#1265](https://github.com/paritytech/subxt/pull/1265))
- Extrinsics: Remove `wait_for_in_block` helper method ([#1237](https://github.com/paritytech/subxt/pull/1237))
- Subxt-codegen: Tidy crate interface ([#1225](https://github.com/paritytech/subxt/pull/1225))
- Lightclient: Update usage docs ([#1223](https://github.com/paritytech/subxt/pull/1223))
- Wee tidy to subxt-signer flags ([#1200](https://github.com/paritytech/subxt/pull/1200))
- Batch fetching storage values again to improve performance ([#1199](https://github.com/paritytech/subxt/pull/1199))
- Add `subxt` feature in `subxt-signer` crate to default features ([#1193](https://github.com/paritytech/subxt/pull/1193))

### Fixed

- Trimmed metadata hash comparison fails in `is_codegen_valid_for`  ([#1306](https://github.com/paritytech/subxt/pull/1306))
- Sync tx submission with chainHead_follow ([#1305](https://github.com/paritytech/subxt/pull/1305))
- Storage: Fix partial key storage iteration ([#1298](https://github.com/paritytech/subxt/pull/1298))
- Lightclient: Fix wasm socket closure called after being dropped ([#1289](https://github.com/paritytech/subxt/pull/1289))
- Fix parachain example ([#1228](https://github.com/paritytech/subxt/pull/1228))

## [0.32.1] - 2023-10-05

This is a patch release, mainly to deploy the fix [#1191](https://github.com/paritytech/subxt/pull/1191), which resolves an issue around codegen when runtime API definitions have an argument name "_".

We also expose an API, `api.blocks().at(block_hash).account_nonce(account_id)`, which allows one to obtain the account nonce for some account at any block hash, and not just at the latest finalized block hash as is possible via `api.tx().account_nonce(..)`.

The main changes are:

- fix for when runtime API field name is _  ([#1191](https://github.com/paritytech/subxt/pull/1191))
- allow getting account nonce at arbitrary blocks, too  ([#1182](https://github.com/paritytech/subxt/pull/1182))
- chore: improve some error messages  ([#1183](https://github.com/paritytech/subxt/pull/1183))

## [0.32.0] - 2023-09-27

This is a big release that adds quite a lot, and also introduces some slightly larger breaking changes. Let's look at the main changes:

### The `Backend` trait and the `UnstableBackend` and `LegacyBackend` impls.

See [#1126](https://github.com/paritytech/subxt/pull/1126), [#1137](https://github.com/paritytech/subxt/pull/1137) and [#1161](https://github.com/paritytech/subxt/pull/1161) for more information.

The overarching idea here is that we want Subxt to be able to continue to support talking to nodes/light-clients using the "legacy" RPC APIs that are currently available, but we _also_ want to be able to support using only [the new RPC APIs](https://paritytech.github.io/json-rpc-interface-spec/) once they are stabilized.

Until now, the higher level APIs in Subxt all had access to the RPCs and could call whatever they needed. Now, we've abstracted away which RPCs are called (or even that RPCs are used at all) behind a `subxt::backend::Backend` trait. Higher level APIs no longer have access to RPC methods and instead have access to the current `Backend` implementation. We then added two `Backend` implementations:

- `subxt::backend::legacy::LegacyBackend`: This uses the "legacy" RPCs, as we've done to date, to obtain the information we need. This is still the default backend that Subxt will use.
- `subxt::backend::unstable::UnstableBackend`: This backend relies on the new (and currently still unstable) `chainHead` based RPC APIs to obtain the information we need. This could break at any time as the RPC methods update, until they are fully stabilized. One day, this will be the default backend.

One of the significant differences between backends is that the `UnstableBackend` can only fetch further information about blocks that are "pinned", ie that we have signalled are still in use. To that end, the backend now hands back `BlockRef`s instead of plain block hashes. As long as a `BlockRef` exists for some block, the backend (and node) will attempt to keep it available. Thus, Subxt will keep hold of these internally as needed, and also allows you to obtain them from a `Block` with `block.reference()`, in case you need to try and hold on to any blocks for longer.

One of the main breaking changes here is in how you can access and call RPC methods.

Previously, you could access them directly from the Subxt client, since it exposed the RPC methods itself, eg:

```rust
let genesis_hash = client.rpc().genesis_hash().await?;
```

Now, the client only knows about a `Backend` (ie it has a `.backend()` method instead of `.rpc()`), and doesn't know about RPCs, but you can still manually create an `RpcClient` to call RPC methods like so:

```rust
use subxt::{
    config::SubstrateConfig,
    backend::rpc::RpcClient,
    backend::legacy::LegacyRpcMethods,
};

// Instantiate an RPC client pointing at some URL.
let rpc_client = RpcClient::from_url("ws://localhost:9944").await?;

// We could also call unstable RPCs with `backend::unstable::UnstableRpcMethods`:
let rpc_methods = LegacyRpcMethods::<SubstrateConfig>::new(rpc_client);

// Use it to make RPC calls, here calling the legacy genesis_hash method.
let genesis_hash = rpc_methods.genesis_hash().await?
```

If you'd like to share a single client for RPCs and Subxt usage, you can clone this RPC client and run `OnlineClient::<SubstrateConfig>::from_rpc_client(rpc_client)` to create a Subxt client using it.

Another side effect of this change is that RPC related things have moved from `subxt::rpc::*` to `subxt::backend::rpc::*` and some renaming has happened along the way.

A number of smaller breaking changes have also been made in order to expose details that are compatible with both sets of RPCs, and to generally move Subxt towards working well with the new APIs and exposing things in a consistent way:

- The storage methods `fetch_keys` is renamed to `fetch_raw_keys` (this just for consistency with `fetch_raw`).
- The storage method `iter` no longer accepts a `page_size` argument, and each item returned is now an `Option<Result<(key, val)>>` instead of a `Result<Option<(key, val)>>` (we now return a valid `Stream` implementation for storage entry iteration). See [this example](https://github.com/paritytech/subxt/blob/cd5060a5a08c9bd73477477cd2cadc16015e77bf/subxt/examples/storage_iterating.rs#L18).
- The events returned when you manually watch a transaction have changed in order to be consistent with the new RPC APIs (the new events [can be seen here](https://github.com/paritytech/subxt/blob/cd5060a5a08c9bd73477477cd2cadc16015e77bf/subxt/src/tx/tx_progress.rs#L203)), and `next_item` => `next`. If you rely on higher level calls like `sign_and_submit_then_watch`, nothing has changed.
- Previously, using `.at_latest()` in various places would mean that calls would run against the latest _best_ block. Now, all such calls will run against the latest _finalized_ block. The latest best block is subject to changing or being pruned entirely, and can differ between nodes.
- `.at(block_hash)` should continue to work as-is, but can now also accept a `BlockRef`, to keep the relevant block around while you're using the associated APIs.
- To fetch the extrinsics in a block, you used to call `block.body().await?.extrinsics()`. This has now been simplified to `block.extrinsics().await?`.

### Making `ExtrinsicParams` more flexible with `SignedExtension`s.

See [#1107](https://github.com/paritytech/subxt/pull/1107) for more information.

When configuring Subxt to work against a given chain, you needed to configure the `ExtrinsicParams` associated type to encode exactly what was required by the chain when submitting transactions. This could be difficult to get right.

Now, we have "upgraded" the `ExtrinsicParams` trait to give it access to metadata, so that it can be smarter about how to encode the correct values. We've also added a `subxt::config::SignedExtension` trait, and provided implementations of it for all of the "standard" signed extensions (though [we have a little work to do still](https://github.com/paritytech/subxt/issues/1162)).

How can you use `SignedExtension`s? Well, `subxt::config::signed_extensions::AnyOf<T, Params>` is a type which can accept any tuple of `SignedExtension`s, and itself implements `ExtrinsicParams`. It's smart, and will use the metadata to know which of the signed extensions that you provided to actually use on a given chain. So, `AnyOf` makes it easy to compose whichever `SignedExtension`s you need to work with a chain.

Finally, we expose `subxt::config::{ DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder }`; the former just uses `AnyOf` to automatically use any of the "standard" signed extensions as needed, and the latter provided a nice builder interface to configure any parameters for them. This is now the default type used in `SubstrateConfig` and `PolkadotConfig`, so long story short: those configurations (and particularly their `ExtrinsicParams`) are more likely to _Just Work_ now across default chains.

[See this example](https://github.com/paritytech/subxt/blob/cd5060a5a08c9bd73477477cd2cadc16015e77bf/subxt/examples/setup_config_signed_extension.rs) for how to create and use custom signed extensions, or [this example](https://github.com/paritytech/subxt/blob/cd5060a5a08c9bd73477477cd2cadc16015e77bf/subxt/examples/setup_config_custom.rs) for how to implement custom `ExtrinsicParams` if you'd prefer to ignore `SignedExtension`s entirely.

As a result of using the new `DefaultExtrinsicParams` in `SubstrateConfig` and `PolkadotConfig`, the interface to configure transactions has changed (and in fact been generally simplified). Configuring a mortal transaction with a small tip ƒor instance used to look like:

```rust
use subxt::config::polkadot::{Era, PlainTip, PolkadotExtrinsicParamsBuilder as Params};

let tx_params = Params::new()
    .tip(PlainTip::new(1_000))
    .era(Era::mortal(32, latest_block.header().number()), latest_block.header().hash());

let hash = api.tx().sign_and_submit(&tx, &from, tx_params).await?;
```

And now it will look like this:

```rust
use subxt::config::polkadot::PolkadotExtrinsicParamsBuilder as Params;

let tx_params = Params::new()
    .tip(1_000)
    .mortal(latest_block.header(), 32)
    .build();

let hash = api.tx().sign_and_submit(&tx, &from, tx_params).await?;
```

Check the docs for `PolkadotExtrinsicParamsBuilder` and the `ExtrinsicParams` trait for more information.

### Storage: Allow iterating storage entries at different depths

See ([#1079](https://github.com/paritytech/subxt/pull/1079)) for more information.

Previously, we could statically iterate over the root of some storage map using something like:

```rust
// Build a storage query to iterate over account information.
let storage_query = polkadot::storage().system().account_root();

// Get back an iterator of results (here, we are fetching 10 items at
// a time from the node, but we always iterate over one at a time).
let mut results = api.storage().at_latest().await?.iter(storage_query, 10).await?;
```

Now, the suffix `_root` has been renamed to `_iter`, and if the storage entry is for instance a double map (or greater depth), we'll also now generate `_iter2`, `iter3` and so on, each accepting the keys needed to access the map at that depth to iterate the remainder. The above example now becomes:

```rust
// Build a storage query to iterate over account information.
let storage_query = polkadot::storage().system().account_iter();

// Get back an iterator of results
let mut results = api.storage().at_latest().await?.iter(storage_query).await?;
```

Note also that the pagination size no longer needs to be provided; that's handled internally by the relevant `Backend`.

### Custom values

This is not a breaking change, but just a noteworthy addition; see [#1106](https://github.com/paritytech/subxt/pull/1106), [#1117](https://github.com/paritytech/subxt/pull/1117) and [#1147](https://github.com/paritytech/subxt/pull/1147) for more information.

V15 metadata allows chains to insert arbitrary information into a new "custom values" hashmap ([see this](https://github.com/paritytech/frame-metadata/blob/0e90489c8588d48b55779f1c6b93216346ecc8a9/frame-metadata/src/v15.rs#L306)). Subxt has now added APIs to allow accessing these custom values a little like how constants can be accessed.

Dynamically accessing custom values looks a bit like this:

```rust
// Obtain the raw bytes for some entry:
let custom_value_bytes: Vec<u8> = client.custom_values().bytes_at("custom-value-name")?;

// Obtain a representation of the value that we can attempt to decode:
let custom_value = client.custom_values().at("custom-value-name")?;

// Decode it into a runtime Value if possible:
let value: Value = custom_value.to_value()?;
// Or attempt to decode it into a specific type:
let value: Foo = custom_value.as_type()?;
```

We can also use codegen to statically access values, which makes use of validation and returns a known type whenever possible, for the added compile time safety this brings:

```rust
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod runtime {}

// The generated interface also exposes any custom values with known types and sensible names:
let value_addr = runtime::custom().custom_value_name();

// We can use this address to access and decode the relevant value from metadata:
let static_value = client.custom_values().at(&value_addr)?;
// Or just ask for the bytes for it:
let static_value_bytes = client.custom_values().bytes_at(&value_addr)?;
```

That sums up the most significant changes. All of the key commits in this release can be found here:

### Added

- `UnstableBackend`: Add a chainHead based backend implementation ([#1161](https://github.com/paritytech/subxt/pull/1161))
- `UnstableBackend`: Expose the chainHead RPCs ([#1137](https://github.com/paritytech/subxt/pull/1137))
- Introduce Backend trait to allow different RPC (or other) backends to be implemented ([#1126](https://github.com/paritytech/subxt/pull/1126))
- Custom Values: Fixes and tests for "custom values" ([#1147](https://github.com/paritytech/subxt/pull/1147))
- Custom Values: Add generated APIs to statically access custom values in metadata ([#1117](https://github.com/paritytech/subxt/pull/1117))
- Custom Values: Support dynamically accessing custom values in metadata ([#1106](https://github.com/paritytech/subxt/pull/1106))
- Add `storage_version()` and `runtime_wasm_code()` to storage ([#1111](https://github.com/paritytech/subxt/pull/1111))
- Make ExtrinsicParams more flexible, and introduce signed extensions ([#1107](https://github.com/paritytech/subxt/pull/1107))

### Changed

- `subxt-codegen`: Add "web" feature for WASM compilation that works with `jsonrpsee` ([#1175](https://github.com/paritytech/subxt/pull/1175))
- `subxt-codegen`: support compiling to WASM ([#1154](https://github.com/paritytech/subxt/pull/1154))
- CI: Use composite action to avoid dupe use-substrate code ([#1177](https://github.com/paritytech/subxt/pull/1177))
- Add custom `Debug` impl for `DispatchError` to avoid huge metadata output ([#1153](https://github.com/paritytech/subxt/pull/1153))
- Remove unused start_key that new RPC API may not be able to support ([#1148](https://github.com/paritytech/subxt/pull/1148))
- refactor(rpc): Use the default port if one isn't provided ([#1122](https://github.com/paritytech/subxt/pull/1122))
- Storage: Support iterating over NMaps with partial keys ([#1079](https://github.com/paritytech/subxt/pull/1079))

### Fixed

- metadata: Generate runtime outer enums if not present in V14 ([#1174](https://github.com/paritytech/subxt/pull/1174))
- Remove "std" feature from `sp-arithmetic` to help substrate compat. ([#1155](https://github.com/paritytech/subxt/pull/1155))
- integration-tests: Increase the number of events we'll wait for ([#1152](https://github.com/paritytech/subxt/pull/1152))
- allow 'latest' metadata to be returned from the fallback code ([#1127](https://github.com/paritytech/subxt/pull/1127))
- chainHead: Propagate results on the `chainHead_follow` ([#1116](https://github.com/paritytech/subxt/pull/1116))


## [0.31.0] - 2023-08-02

This is a small release whose primary goal is to bump the versions of `scale-encode`, `scale-decode` and `scale-value` being used, to benefit from recent changes in those crates.

`scale-decode` changes how compact values are decoded as part of [#1103](https://github.com/paritytech/subxt/pull/1103). A compact encoded struct should now be properly decoded into a struct of matching shape (which implements `DecodeAsType`). This will hopefully resolve issues around structs like `Perbill`. When decoding the SCALE bytes for such types into `scale_value::Value`, the `Value` will now be a composite type wrapping a value, and not just the value.

We've also figured out how to sign extrinsics using browser wallets when a Subxt app is compiled to WASM; see [#1067](https://github.com/paritytech/subxt/pull/1067) for more on that!

The key commits:

### Added

- Add browser extension signing example ([#1067](https://github.com/paritytech/subxt/pull/1067))

### Changed

- Bump to latest scale-encode/decode/value and fix test running ([#1103](https://github.com/paritytech/subxt/pull/1103))
- Set minimum supported `rust-version` to `1.70` ([#1097](https://github.com/paritytech/subxt/pull/1097))

### Fixed

- Tests: support 'substrate-node' too and allow multiple binary paths ([#1102](https://github.com/paritytech/subxt/pull/1102))


## [0.30.1] - 2023-07-25

This patch release fixes a small issue whereby using `runtime_metadata_url` in the Subxt macro would still attempt to download unstable metadata, which can fail at the moment if the chain has not updated to stable V15 metadata yet (which has a couple of changes from the last unstable version). Note that you're generally encouraged to use `runtime_metadata_path` instead, which does not have this issue.

### Fixes

- codegen: Fetch and decode metadata version then fallback ([#1092](https://github.com/paritytech/subxt/pull/1092))


## [0.30.0] - 2023-07-24

This release beings with it a number of exciting additions. Let's cover a few of the most significant ones:

### Light client support (unstable)

This release adds support for light clients using Smoldot, both when compiling native binaries and when compiling to WASM to run in a browser environment. This is unstable for now while we continue testing it and work on making use of the new RPC APIs.

Here's how to use it:

```rust
use subxt::{
    client::{LightClient, LightClientBuilder},
    PolkadotConfig
};
use subxt_signer::sr25519::dev;

// Create a light client:
let api = LightClient::<PolkadotConfig>::builder()
    // You can also pass a chain spec directly using `build`, which is preferred:
    .build_from_url("ws://127.0.0.1:9944")
    .await?;

// Working with the interface is then the same as before:
let dest = dev::bob().public_key().into();
let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 10_000);
let events = api
    .tx()
    .sign_and_submit_then_watch_default(&balance_transfer_tx, &dev::alice())
    .await?
    .wait_for_finalized_success()
    .await?;
```

At the moment you may encounter certain things that don't work; please file an issue if you do!

### V15 Metadata

This release stabilizes the metadata V15 interface, which brings a few changes but primarily allows you to interact with Runtime APIs via an ergonomic Subxt interface:

```rust
// We can use the static interface to interact in a type safe way:
#[subxt::subxt(runtime_metadata_path = "path/to/metadata.scale")]
pub mod polkadot {}

let runtime_call = polkadot::apis()
    .metadata()
    .metadata_versions();

// Or we can use the dynamic interface like so:
use subxt::dynamic::Value;

let runtime_call = subxt::dynamic::runtime_api_call(
    "Metadata",
    "metadata_versions",
    Vec::<Value<()>>::new()
);
```

This is no longer behind a feature flag, but if the chain you're connecting to doesn't use V15 metadata yet then the above will be unavailable.

### `subxt-signer`

The new `subxt-signer` crate provides the ability to sign transactions using either sr25519 or ECDSA. It's WASM compatible, and brings in fewer dependencies than using `sp_core`/`sp_keyring` does, while having an easy to use interface. Here's an example of signing a transaction using it:

```rust
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

let api = OnlineClient::<PolkadotConfig>::new().await?;

// Build the extrinsic; a transfer to bob:
let dest = dev::bob().public_key().into();
let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 10_000);

// Sign and submit the balance transfer extrinsic from Alice:
let from = dev::alice();
let events = api
    .tx()
    .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
    .await?
    .wait_for_finalized_success()
    .await?;
```

Dev keys should only be used for tests since they are publicly known. Actual keys can be generated from URIs, phrases or raw entropy, and derived using soft/hard junctions:

```rust
use subxt_signer::{ SecretUri, sr25519::Keypair };
use std::str::FromStr;

// From a phrase (see `bip39` crate on generating phrases):
let phrase = bip39::Mnemonic::parse(phrase).unwrap();
let keypair = Keypair::from_phrase(&phrase, Some("Password")).unwrap();

// Or from a URI:
let uri = SecretUri::from_str("//Alice").unwrap();
let keypair = Keypair::from_uri(&uri).unwrap();

// Deriving a new key from an existing one:
let keypair = keypair.derive([
    DeriveJunction::hard("Alice"),
    DeriveJunction::soft("stash")
]);
```

### Breaking changes

A few small breaking changes have occurred:

- There is no longer a need for an `Index` associated type in your `Config` implementations; we now work it out dynamically where needed.
- The "substrate-compat" feature flag is no longer enabled by default. `subxt-signer` added native signing support and can be used instead of bringing in Substrate dependencies to sign transactions now. You can still enable this feature flag as before to make use of them if needed.
    - **Note:** Be aware that Substrate crates haven't been published in a while and have fallen out of date, though. This will be addressed eventually, and when it is we can bring the Substrate crates back uptodate here.

For anything else that crops up, the compile errors and API docs will hopefully point you in the right direction, but please raise an issue if not.

For a full list of changes, see below:

### Added

- Example: How to connect to parachain ([#1043](https://github.com/paritytech/subxt/pull/1043))
- ECDSA Support in signer ([#1064](https://github.com/paritytech/subxt/pull/1064))
- Add `subxt_signer` crate for native & WASM compatible signing ([#1016](https://github.com/paritytech/subxt/pull/1016))
- Add light client platform WASM compatible ([#1026](https://github.com/paritytech/subxt/pull/1026))
- light-client: Add experimental light-client support ([#965](https://github.com/paritytech/subxt/pull/965))
- Add `diff` command to CLI tool to visualize metadata changes ([#1015](https://github.com/paritytech/subxt/pull/1015))
- CLI: Allow output to be written to file ([#1018](https://github.com/paritytech/subxt/pull/1018))

### Changed

- Remove `substrate-compat` default feature flag ([#1078](https://github.com/paritytech/subxt/pull/1078))
- runtime API: Substitute `UncheckedExtrinsic` with custom encoding ([#1076](https://github.com/paritytech/subxt/pull/1076))
- Remove `Index` type from Config trait ([#1074](https://github.com/paritytech/subxt/pull/1074))
- Utilize Metadata V15 ([#1041](https://github.com/paritytech/subxt/pull/1041))
- chain_getBlock extrinsics encoding ([#1024](https://github.com/paritytech/subxt/pull/1024))
- Make tx payload details public ([#1014](https://github.com/paritytech/subxt/pull/1014))
- CLI tool tests ([#977](https://github.com/paritytech/subxt/pull/977))
- Support NonZero numbers ([#1012](https://github.com/paritytech/subxt/pull/1012))
- Get account nonce via state_call ([#1002](https://github.com/paritytech/subxt/pull/1002))
- add `#[allow(rustdoc::broken_intra_doc_links)]` to subxt-codegen ([#998](https://github.com/paritytech/subxt/pull/998))

### Fixed

- remove parens in hex output for CLI tool ([#1017](https://github.com/paritytech/subxt/pull/1017))
- Prevent bugs when reusing type ids in hashing ([#1075](https://github.com/paritytech/subxt/pull/1075))
- Fix invalid generation of types with >1 generic parameters ([#1023](https://github.com/paritytech/subxt/pull/1023))
- Fix jsonrpsee web features ([#1025](https://github.com/paritytech/subxt/pull/1025))
- Fix codegen validation when Runtime APIs are stripped ([#1000](https://github.com/paritytech/subxt/pull/1000))
- Fix hyperlink ([#994](https://github.com/paritytech/subxt/pull/994))
- Remove invalid redundant clone warning ([#996](https://github.com/paritytech/subxt/pull/996))

## [0.29.0] - 2023-06-01

This is another big release for Subxt with a bunch of awesome changes. Let's talk about some of the notable ones:

### A new guide

This release will come with overhauled documentation and examples which is much more comprehensive than before, and goes into much more detail on each of the main areas that Subxt can work in.

Check out [the documentation](https://docs.rs/subxt/latest/subxt/) for more. We'll continue to build on this with some larger examples, too, going forwards. ([#968](https://github.com/paritytech/subxt/pull/968)) is particularly cool as it's our first example showcasing Subxt working with Yew and WASM; it'll be extended with more documentation and things in the next release.

### A more powerful CLI tool: an `explore` command.

The CLI tool has grown a new command, `explore`. Point it at a node and use `explore` to get information about the calls, constants and storage of a node, with a helpful interface that allows you to progressively dig into each of these areas!

### Support for (unstable) V15 metadata and generating a Runtime API interface

One of the biggest changes in this version is that, given (unstable) V15 metadata, Subxt can now generate a nice interface to make working with Runtime APIs as easy as building extrinsics or storage queries. This is currently unstable until the V15 metadata format is stabilised, and so will break as we introduce more tweaks to the metadata format. We hope to stabilise V15 metadata soon; [see this](https://forum.polkadot.network/t/stablising-v15-metadata/2819) for more information. At this point, we'll stabilize support in Subxt.

### Support for decoding extrinsics

Up until now, you were able to retrieve the bytes for extrinsics, but weren't able to use Subxt to do much with those bytes.

Now, we expose several methods to decode extrinsics that work much like decoding events:

```rust
#[subxt::subxt(runtime_metadata_path = "polkadot_metadata.scale")]
pub mod polkadot {}

// Get some block:
let block = api.blocks().at_latest().await?;

// Find and decode a specific extrinsic in the block:
let remark = block.find::<polkadot::system::calls::Remark>()?;

// Iterate the extrinsics in the block:
for ext in block.iter() {
    // Decode a specific extrinsic into the call data:
    let remark = ext.as_extrinsic::<polkadot::system::calls::Remark>()?;
    // Decode any extrinsic into an enum containing the call data:
    let extrinsic = ext.as_root_extrinsic::<polkadot::Call>()?;
}
```

### New Metadata Type ([#974](https://github.com/paritytech/subxt/pull/974))

Previously, the `subxt_metadata` crate was simply a collection of functions that worked directly on `frame_metadata` types. Then, in `subxt`, we had a custom metadata type which wrapped this to provide the interface needed by various Subxt internals and traits.

Now, the `subxt_metadata` crate exposes our own `Metadata` type which can be decoded from the same wire format as the `frame_metadata` types we used to use. This type is now used throughout Subxt, as well as in the `codegen` stuff, and provides a single unified interface for working with metadata that is independent of the actual underlying metadata version we're using.

This shouldn't lead to breakages in most code, but if you need to load metadata for an `OfflineClient` you might previously have done this:

```rust
use subxt::ext::frame_metadata::RuntimeMetadataPrefixed;
use subxt::metadata::Metadata;

let metadata = RuntimeMetadataPrefixed::decode(&mut &*bytes).unwrap();
let metadata = Metadata::try_from(metadata).unwrap();
```

But now you'd do this:

```rust
use subxt::metadata::Metadata;

let metadata = Metadata::decode(&mut &*bytes).unwrap();
```

Otherwise, if you implement traits like `TxPayload` directly, you'll need to tweak the implementations to use the new `Metadata` type, which exposes everything you used to be able to get hold of but behind a slightly different interface.

### Removing `as_pallet_event` method ([#953](https://github.com/paritytech/subxt/pull/953))

In an effort to simplify the number of ways we have to decode events, `as_pallet_event` was removed. You can achieve a similar thing by calling `as_root_event`, which will decode _any_ event that the static interface knows about into an outer enum of pallet names to event names. if you only care about a specific event, you can match on this enum to look for events from a specific pallet.

Another reason that `as_pallet_event` was removed was that it could potentially decode events from the wrong pallets into what you're looking for, if the event shapes happened to line up, which was a potential foot gun.

### Added `as_root_error` for decoding errors.

Much like we can call `as_root_extrinsic` or `as_root_event` to decode extrinsics and events into a top level enum, we've also added `as_root_error` to do the same for errors and help to make this interface consistent across the board.

Beyond these, there's a bunch more that's been added, fixed and changes. A full list of the notable changes in this release are as follows:

### Added

- Add topics to `EventDetails` ([#989](https://github.com/paritytech/subxt/pull/989))
- Yew Subxt WASM examples ([#968](https://github.com/paritytech/subxt/pull/968))
- CLI subxt explore commands ([#950](https://github.com/paritytech/subxt/pull/950))
- Retain specific runtime APIs ([#961](https://github.com/paritytech/subxt/pull/961))
- Subxt Guide ([#890](https://github.com/paritytech/subxt/pull/890))
- Partial fee estimates for SubmittableExtrinsic ([#910](https://github.com/paritytech/subxt/pull/910))
- Add ability to opt out from default derives and attributes ([#925](https://github.com/paritytech/subxt/pull/925))
- add no_default_substitutions to the macro and cli ([#936](https://github.com/paritytech/subxt/pull/936))
- extrinsics: Decode extrinsics from blocks ([#929](https://github.com/paritytech/subxt/pull/929))
- Metadata V15: Generate Runtime APIs ([#918](https://github.com/paritytech/subxt/pull/918)) and ([#947](https://github.com/paritytech/subxt/pull/947))
- impl Header and Hasher for some substrate types behind the "substrate-compat" feature flag ([#934](https://github.com/paritytech/subxt/pull/934))
- add `as_root_error` for helping to decode ModuleErrors ([#930](https://github.com/paritytech/subxt/pull/930))

### Changed

- Update scale-encode, scale-decode and scale-value to latest ([#991](https://github.com/paritytech/subxt/pull/991))
- restrict sign_with_address_and_signature interface ([#988](https://github.com/paritytech/subxt/pull/988))
- Introduce Metadata type ([#974](https://github.com/paritytech/subxt/pull/974)) and ([#978](https://github.com/paritytech/subxt/pull/978))
- Have a pass over metadata validation ([#959](https://github.com/paritytech/subxt/pull/959))
- remove as_pallet_extrinsic and as_pallet_event ([#953](https://github.com/paritytech/subxt/pull/953))
- speed up ui tests. ([#944](https://github.com/paritytech/subxt/pull/944))
- cli: Use WS by default instead of HTTP ([#954](https://github.com/paritytech/subxt/pull/954))
- Upgrade to `syn 2.0` ([#875](https://github.com/paritytech/subxt/pull/875))
- Move all deps to workspace toml ([#932](https://github.com/paritytech/subxt/pull/932))
- Speed up CI ([#928](https://github.com/paritytech/subxt/pull/928)) and ([#926](https://github.com/paritytech/subxt/pull/926))
- metadata: Use v15 internally ([#912](https://github.com/paritytech/subxt/pull/912))
- Factor substrate node runner into separate crate ([#913](https://github.com/paritytech/subxt/pull/913))
- Remove need to import parity-scale-codec to use subxt macro ([#907](https://github.com/paritytech/subxt/pull/907))

### Fixed

- use blake2 for extrinsic hashing ([#921](https://github.com/paritytech/subxt/pull/921))
- Ensure unique types in codegen ([#967](https://github.com/paritytech/subxt/pull/967))
- use unit type in polkadot config ([#943](https://github.com/paritytech/subxt/pull/943))


## [0.28.0] - 2023-04-11

This is a fairly significant change; what follows is a description of the main changes to be aware of:

### Unify how we encode and decode static and dynamic types ([#842](https://github.com/paritytech/subxt/pull/842))

Prior to this, static types generated by codegen (ie subxt macro) would implement `Encode` and `Decode` from the `parity-scale-codec` library. This meant that they woule be encoded-to and decoded-from based on their shape. Dynamic types (eg the `subxt::dynamic::Value` type) would be encoded and decoded based on the node metadata instead.

This change makes use of the new `scale-encode` and `scale-decode` crates to auto-implement `EncodeAsType` and `DecodeAsType` on all of our static types. These traits allow types to take the node metadata into account when working out how best to encode and decode into them. By using metadata, we can be much more flexible/robust about how to encode/decode various types (as an example, nested transactions will now be portable across runtimes). Additionally, we can merge our codepaths for static and dynamic encoding/decoding, since both static and dynamic types can implement these traits. Read [the PR description](https://github.com/paritytech/subxt/pull/842) for more info.

A notable impact of this is that any types you wish to substitute when performing codegen (via the CLI tool or `#[subxt]` macro) must also implement `EncodeAsType` and `DecodeAsType` too. Substrate types, for instance, generally do not. To work around this, [#886](https://github.com/paritytech/subxt/pull/886) introduces a `Static` type and enhances the type substitution logic so that you're able to wrap any types which only implement `Encode` and `Decode` to work (note that you lose out on the improvements from `EncodeAsType` and `DecodeAsType` when you do this):

```rust
#[subxt::subxt(
    runtime_metadata_path = "/path/to/metadata.scale",
    substitute_type(
        type = "sp_runtime::multiaddress::MultiAddress<A, B>",
        with = "::subxt::utils::Static<::sp_runtime::multiaddress::MultiAddress<A, B>>"
    )
)]
pub mod node_runtime {}
```

So, if you want to substitute in Substrate types, wrap them in `::subxt::utils::Static` in the type substitution, as above. [#886](https://github.com/paritytech/subxt/pull/886) also generally improves type substitution so that you can substitute the generic params in nested types, since it's required in the above.

Several types have been renamed as a result of this unification (though they aren't commonly made explicit use of). Additionally, to obtain the bytes from a storage address, instead of doing:

```rust
let addr_bytes = storage_address.to_bytes()
```

You must now do:

```rust
let addr_bytes = cxt.client().storage().address_bytes(&storage_address).unwrap();
```

This is because the address on it's own no longer requires as much static information, and relies more heavily now on the node metadata to encode it to bytes.

### Expose Signer payload ([#861](https://github.com/paritytech/subxt/pull/861))

This is not a breaking change, but notable in that is adds `create_partial_signed_with_nonce` and `create_partial_signed` to the `TxClient` to allow you to break extrinsic creation into two steps:

1. building a payload, and then
2. when a signature is provided, getting back an extrinsic ready to be submitted.

This allows a signer payload to be obtained from Subxt, handed off to some external application, and then once a signature has been obtained, that can be passed back to Subxt to complete the creation of an extrinsic. This opens the door to using browser wallet extensions, for instance, to sign Subxt payloads.

### Stripping unneeded pallets from metadata ([#879](https://github.com/paritytech/subxt/pull/879))

This is not a breaking change, but adds the ability to use the Subxt CLI tool to strip out all but some named list of pallets from a metadata bundle. Aside from allowing you to store a significantly smaller metadata bundle with only the APIs you need in it, it will also lead to faster codegen, since there's much less of it to do.

Use a command like `subxt metadata --pallets Balances,System` to select specific pallets. You can provide an existing metadata file to take that and strip it, outputting a smaller bundle. Alternately it will grab the metadata from a local node and strip that before outputting.

### Dispatch error changes ([#878](https://github.com/paritytech/subxt/pull/878))

The `DispatchError` returned from either attempting to submit an extrinsic, or from calling `.dry_run()` has changed. It's now far more complete with respect to the information it returns in each case, and the interface has been tidied up. Changes include:

- For `ModuleError`'s, instead of `err.pallet` and `err.error`, you can obtain error details using `let details = err.details()?` and then `details.pallet()` and `details.error()`.
- `DryRunResult` is now a custom enum with 3 states, `Success`, `DispatchError` or `TransactionValidityError`. The middle of these contains much more information than previously.
- Errors in general have been marked `#[non_exahustive]` since they could grow and change at any time. (Owing to our use of `scale-decode` internally, we are not so contrained when it comes to having precise variant indexes or anything now, and can potentially deprecate rather than remove old variants as needed).
- On a lower level, the `rpc.dry_run()` RPC call now returns the raw dry run bytes which can then be decoded with the help of metadata into our `DryRunResult`.

### Extrinsic submission changes ([#897](https://github.com/paritytech/subxt/pull/897))

It was found by [@furoxr](https://github.com/furoxr) that Substrate nodes will stop sending transaction progress events under more circumstances than we originally expected. Thus, now calls like `wait_for_finalized()` and `wait_for_in_block()` will stop waiting for events when any of the following is sent from the node:

- `Usurped`
- `Finalized`
- `FinalityTimeout`
- `Invalid`
- `Dropped`

Previously we'd only close the subscription and stop waiting when we saw a `Finalized` or `FinalityTimeout` event. Thanks for digging into this [@furoxr](https://github.com/furoxr)!

### Add `at_latest()` method ([#900](https://github.com/paritytech/subxt/pull/900) and [#904](https://github.com/paritytech/subxt/pull/904))

A small breaking change; previously we had `.at(None)` or `.at(Some(block_hash))` methods in a few places to obtain things at either the latest block or some specific block hash.

This API has been clarified; we now have `.at_latest()` to obtain the thing at the latest block, or `.at(block_hash)` (note; no more option) to obtain the thing at some fixed block hash. In a few instances this has allowed us to ditch the `async` from the `.at()` call.

That covers the larger changes in this release. For more details, have a look at all of the notable PRs since the last release here:

### Added

- added at_latest ([#900](https://github.com/paritytech/subxt/pull/900) and [#904](https://github.com/paritytech/subxt/pull/904))
- Metadata: Retain a subset of metadata pallets ([#879](https://github.com/paritytech/subxt/pull/879))
- Expose signer payload to allow external signing ([#861](https://github.com/paritytech/subxt/pull/861))
- Add ink! as a user of `subxt` ([#837](https://github.com/paritytech/subxt/pull/837))
- codegen: Add codegen error ([#841](https://github.com/paritytech/subxt/pull/841))
- codegen: allow documentation to be opted out of ([#843](https://github.com/paritytech/subxt/pull/843))
- re-export `sp_core` and `sp_runtime` ([#853](https://github.com/paritytech/subxt/pull/853))
- Allow generating only runtime types in subxt macro ([#845](https://github.com/paritytech/subxt/pull/845))
- Add 'Static' type and improve type substitution codegen to accept it ([#886](https://github.com/paritytech/subxt/pull/886))

### Changed

- Improve Dispatch Errors ([#878](https://github.com/paritytech/subxt/pull/878))
- Use scale-encode and scale-decode to encode and decode based on metadata ([#842](https://github.com/paritytech/subxt/pull/842))
- For smoldot: support deserializing block number in header from hex or number ([#863](https://github.com/paritytech/subxt/pull/863))
- Bump Substrate dependencies to latest ([#905](https://github.com/paritytech/subxt/pull/905))

### Fixed

- wait_for_finalized behavior if the tx dropped, usurped or invalid ([#897](https://github.com/paritytech/subxt/pull/897))


## [0.27.1] - 2023-02-15

### Added

- Add `find_last` for block types ([#825](https://github.com/paritytech/subxt/pull/825))

## [0.27.0] - 2023-02-13

This is a fairly small release, primarily to bump substrate dependencies to their latest versions.

The main breaking change is fairly small: [#804](https://github.com/paritytech/subxt/pull/804). Here, the `BlockNumber` associated type has been removed from `Config` entirely, since it wasn't actually needed anywhere in Subxt. Additionally, the constraints on each of those associated types in `Config` were made more precise, primarily to tidy things up (but this should result in types more easily being able to meet the requirements here). If you use custom `Config`, the fix is simply to remove the `BlockNumber` type. If you also use the `Config` trait in your own functions and depend on those constraints, you may be able to define a custom `MyConfig` type which builds off `Config` and adds back any additional bounds that you want.

Note worthy PRs merged since the last release:

### Added

- Add find last function ([#821](https://github.com/paritytech/subxt/pull/821))
- Doc: first item is current version comment ([#817](https://github.com/paritytech/subxt/pull/817))

### Changed

- Remove unneeded Config bounds and BlockNumber associated type ([#804](https://github.com/paritytech/subxt/pull/804))


## [0.26.0] - 2023-01-24

This release adds a number of improvements, most notably:

- We make Substrate dependencies optional ([#760](https://github.com/paritytech/subxt/pull/760)), which makes WASM builds both smaller and more reliable. To do this, we re-implement some core types like `AccountId32`, `MultiAddress` and `MultiSignature` internally.
- Allow access to storage entries ([#774](https://github.com/paritytech/subxt/pull/774)) and runtime API's ([#777](https://github.com/paritytech/subxt/pull/777)) from some block. This is part of a move towards a more "block centric" interface, which will better align with the newly available `chainHead` style RPC interface.
- Add RPC methods for the new `chainHead` style interface (see https://paritytech.github.io/json-rpc-interface-spec/). These are currently unstable, but will allow users to start experimenting with this new API if their nodes support it.
- More advanced type substitution is now possible in the codegen interface ([#735](https://github.com/paritytech/subxt/pull/735)).

This release introduces a number of breaking changes that can be generally be fixed with mechanical tweaks to your code. The notable changes are described below.

### Make Storage API more Block-centric

See [#774](https://github.com/paritytech/subxt/pull/774). This PR makes the Storage API more consistent with the Events API, and allows access to it from a given block as part of a push to provide a more block centric API that will hopefully be easier to understand, and will align with the new RPC `chainHead` style RPC interface.

Before, your code will look like:

```rust
let a = api.storage().fetch(&staking_bonded, None).await?;
```

After, it should look like:

```rust
let a = api.storage().at(None).await?.fetch(&staking_bonded).await?;
```

Essentially, the final parameter for choosing which block to call some method at has been moved out of the storage method itself and is now provided to instantiate the storage API, either explicitly via an `.at(optional_block_hash)` as above, or implicitly when calling `block.storage()` to access the same storage methods for some block.

An alternate way to access the same storage (primarily used if you have subscribed to blocks or otherwise are working with some block) now is:

```rust
let block = api.blocks().at(None).await?
let a = block.storage().fetch(&staking_bonded, None).await?;
```

### More advanced type substitution in codegen

See [#735](https://github.com/paritytech/subxt/pull/735). Previously, you could perform basic type substitution like this:

```rust
#[subxt::subxt(runtime_metadata_path = "../polkadot_metadata.scale")]
pub mod node_runtime {
    #[subxt::subxt(substitute_type = "sp_arithmetic::per_things::Foo")]
    use crate::Foo;
}
```

This example would use `crate::Foo` every time an `sp_arithmetic::per_things::Foo` was encountered in the codegen. However, this was limited; the substitute type had to have the name number and order of generic parameters for this to work.

We've changed the interface above into:

```rust
#[subxt::subxt(
    runtime_metadata_path = "../polkadot_metadata.scale",
    substitute_type(
        type = "sp_arithmetic::per_things::Foo<A, B, C>",
        with = "crate::Foo<C>"
    )
)]
pub mod node_runtime {}
```

In this example, we can (optionally) specify the generic parameters we expect to see on the original type ("type"), and then of those, decide which should be present on the substitute type ("with"). If no parameters are provided at all, we'll get the same behaviour as before. This allows much more flexibility when defining substitute types.

### Optional Substrate dependencies

See [#760](https://github.com/paritytech/subxt/pull/760). Subxt now has a "substrate-compat" feature (enabled by default, and disabled for WASM builds). At present, enabling this feature simply exposes the `PairSigner` (which was always available before), allowing transactions to be signed via Substrate signer logic (as before). When disabled, you (currently) must bring your own signer implementation, but in return we can avoid bringing in a substantial number of Substrate dependencies in the process.

Regardless, this change also tidied up and moved various bits and pieces around to be consistent with this goal. To address some common moves, previously we'd have:

```rust
use subxt::{
    ext::{
        sp_core::{ sr25519, Pair },
        sp_runtime::{ AccountId32, generic::Header },
    },
    tx::{
        Era,
        PlainTip,
        PolkadotExtrinsicParamsBuilder
    }
};
```

And now this would look more like:

```rust
// `sp_core` and `sp_runtime` are no longer exposed via `ext`; add the crates yourself at matching versions to use:
use sp_core::{
    sr25519,
    Pair,
};
use subxt::{
    // You'll often want to use the "built-in" `AccountId32` now instead of the `sp_runtime` version:
    utils::AccountId32,
    // traits used in our `Config` trait are now provided directly in this module:
    config::Header,
    // Polkadot and Substrate specific Config types are now in the relevant Config section:
    config::polkadot::{
        Era,
        PlainTip,
        PolkadotExtrinsicParamsBuilder
    }
}
```

Additionally, the `type Hashing` in the `Config` trait is now called `Hasher`, to clarify what it is, and types returned directly from the RPC calls now all live in `crate::rpc::types`, rather than sometimes living in Substrate crates.

Some other note worthy PRs that were merged since the last release:

### Added

- Add block-centric Storage API ([#774](https://github.com/paritytech/subxt/pull/774))
- Add `chainHead` RPC methods ([#766](https://github.com/paritytech/subxt/pull/766))
- Allow for remapping type parameters in type substitutions ([#735](https://github.com/paritytech/subxt/pull/735))
- Add ability to set custom metadata etc on OnlineClient ([#794](https://github.com/paritytech/subxt/pull/794))
- Add `Cargo.lock` for deterministic builds ([#795](https://github.com/paritytech/subxt/pull/795))
- Add API to execute runtime calls ([#777](https://github.com/paritytech/subxt/pull/777))
- Add bitvec-like generic support to the scale-bits type for use in codegen ([#718](https://github.com/paritytech/subxt/pull/718))
- Add `--derive-for-type` to cli ([#708](https://github.com/paritytech/subxt/pull/708))

### Changed

- rename subscribe_to_updates() to updater() ([#792](https://github.com/paritytech/subxt/pull/792))
- Expose `Update` ([#791](https://github.com/paritytech/subxt/pull/791))
- Expose version info in CLI tool with build-time obtained git hash ([#787](https://github.com/paritytech/subxt/pull/787))
- Implement deserialize on AccountId32 ([#773](https://github.com/paritytech/subxt/pull/773))
- Codegen: Preserve attrs and add #[allow(clippy::all)] ([#784](https://github.com/paritytech/subxt/pull/784))
- make ChainBlockExtrinsic cloneable ([#778](https://github.com/paritytech/subxt/pull/778))
- Make sp_core and sp_runtime dependencies optional, and bump to latest ([#760](https://github.com/paritytech/subxt/pull/760))
- Make verbose rpc error display ([#758](https://github.com/paritytech/subxt/pull/758))
- rpc: Expose the `subscription ID` for `RpcClientT` ([#733](https://github.com/paritytech/subxt/pull/733))
- events: Fetch metadata at arbitrary blocks ([#727](https://github.com/paritytech/subxt/pull/727))

### Fixed

- Fix decoding events via `.as_root_event()` and add test ([#767](https://github.com/paritytech/subxt/pull/767))
- Retain Rust code items from `mod` decorated with `subxt` attribute ([#721](https://github.com/paritytech/subxt/pull/721))


## [0.25.0] - 2022-11-16

This release resolves the `parity-util-mem crate` several version guard by updating substrate related dependencies which makes
it possible to have other substrate dependencies in tree again along with subxt.

In addition the release has several API improvements in the dynamic transaction API along with that subxt now compiles down to WASM.

Notable PRs merged:

### Added

- Add getters for `Module` ([#697](https://github.com/paritytech/subxt/pull/697))
- add wasm support ([#700](https://github.com/paritytech/subxt/pull/700))
- Extend the new `api.blocks()` to be the primary way to subscribe and fetch blocks/extrinsics/events ([#691](https://github.com/paritytech/subxt/pull/691))
- Add runtime_metadata_url to pull metadata directly from a node ([#689](https://github.com/paritytech/subxt/pull/689))
- Implement `BlocksClient` for working with blocks ([#671](https://github.com/paritytech/subxt/pull/671))
- Allow specifying the `subxt` crate path for generated code ([#664](https://github.com/paritytech/subxt/pull/664))
- Allow taking out raw bytes from a SubmittableExtrinsic ([#683](https://github.com/paritytech/subxt/pull/683))
- Add DecodedValueThunk to allow getting bytes back from dynamic queries ([#680](https://github.com/paritytech/subxt/pull/680))

### Changed

- Update substrate crates ([#709](https://github.com/paritytech/subxt/pull/709))
- Make working with nested queries a touch easier ([#714](https://github.com/paritytech/subxt/pull/714))
- Upgrade to scale-info 2.3 and fix errors ([#704](https://github.com/paritytech/subxt/pull/704))
- No need to entangle Signer and nonce now ([#702](https://github.com/paritytech/subxt/pull/702))
- error: `RpcError` with custom client error ([#694](https://github.com/paritytech/subxt/pull/694))
- into_encoded() for consistency ([#685](https://github.com/paritytech/subxt/pull/685))
- make subxt::Config::Extrinsic Send ([#681](https://github.com/paritytech/subxt/pull/681))
- Refactor CLI tool to give room for growth ([#667](https://github.com/paritytech/subxt/pull/667))
- expose jsonrpc-core client ([#672](https://github.com/paritytech/subxt/pull/672))
- Upgrade clap to v4 ([#678](https://github.com/paritytech/subxt/pull/678))


## [0.24.0] - 2022-09-22

This release has a bunch of smaller changes and fixes. The breaking changes are fairly minor and should be easy to address if encountered. Notable additions are:
- Allowing the underlying RPC implementation to be swapped out ([#634](https://github.com/paritytech/subxt/pull/634)). This makes `jsonrpsee` an optional dependency, and opens the door for Subxt to be integrated into things like light clients, since we can decide how to handle RPC calls.
- A low level "runtime upgrade" API is exposed, giving more visibility into when node updates happen in case your application needs to handle them.
- `scale-value` and `scale-decode` dependencies are bumped. The main effect of this is that `bitvec` is no longer used under the hood in the core of Subxt, which helps to remove one hurdle on the way to being able to compile it to WASM.

Notable PRs merged:

### Added

- feat: add low-level `runtime upgrade API` ([#657](https://github.com/paritytech/subxt/pull/657))
- Add accessor for `StaticTxPayload::call_data` ([#660](https://github.com/paritytech/subxt/pull/660))
- Store type name of a field in event metadata, and export EventFieldMetadata ([#656](https://github.com/paritytech/subxt/pull/656) and [#654](https://github.com/paritytech/subxt/pull/654))
- Allow generalising over RPC implementation ([#634](https://github.com/paritytech/subxt/pull/634))
- Add conversion and default functions for `NumberOrHex` ([#636](https://github.com/paritytech/subxt/pull/636))
- Allow creating/submitting unsigned transactions, too. ([#625](https://github.com/paritytech/subxt/pull/625))
- Add Staking Miner and Introspector to usage list ([#647](https://github.com/paritytech/subxt/pull/647))

### Changed

- Bump scale-value and scale-decode ([#659](https://github.com/paritytech/subxt/pull/659))
- Tweak 0.23 notes and add another test for events ([#618](https://github.com/paritytech/subxt/pull/618))
- Specialize metadata errors ([#633](https://github.com/paritytech/subxt/pull/633))
- Simplify the TxPayload trait a little ([#638](https://github.com/paritytech/subxt/pull/638))
- Remove unnecessary `async` ([#645](https://github.com/paritytech/subxt/pull/645))
- Use 'sp_core::Hxxx' for all hash types ([#623](https://github.com/paritytech/subxt/pull/623))

### Fixed

- Fix `history_depth` testing ([#662](https://github.com/paritytech/subxt/pull/662))
- Fix codegen for `codec::Compact` as type parameters ([#651](https://github.com/paritytech/subxt/pull/651))
- Support latest substrate release ([#653](https://github.com/paritytech/subxt/pull/653))


## [0.23.0] - 2022-08-11

This is one of the most significant releases to date in Subxt, and carries with it a number of significant breaking changes, but in exchange, a number of significant improvements. The most significant PR is [#593](https://github.com/paritytech/subxt/pull/593); the fundamental change that this makes is to separate creating a query/transaction/address from submitting it. This gives us flexibility when creating queries; they can be either dynamically or statically generated, but also flexibility in our client, enabling methods to be exposed for online or offline use.

The best place to look to get a feel for what's changed, aside from the documentation itself, is the `examples` folder. What follows are some examples of the changes you'll need to make, which all follow a similar pattern:

### Submitting a transaction

Previously, we'd build a client which is tied to the static codegen, and then use the client to build and submit a transaction like so:

```rust
let api = ClientBuilder::new()
    .build()
    .await?
    .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<_>>>();

let balance_transfer = api
    .tx()
    .balances()
    .transfer(dest, 10_000)?
    .sign_and_submit_then_watch_default(&signer)
    .await?
    .wait_for_finalized_success()
    .await?;
```

Now, we build a transaction separately (in this case, using static codegen to guide us as before) and then submit it to a client like so:

``` rust
let api = OnlineClient::<PolkadotConfig>::new().await?;

let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 10_000);

let balance_transfer = api
    .tx()
    .sign_and_submit_then_watch_default(&balance_transfer_tx, &signer)
    .await?
    .wait_for_finalized_success()
    .await?;
```

See the `examples/examples/submit_and_watch.rs` example for more.

### Fetching a storage entry

Previously, we build and submit a storage query in one step:

```rust
let api = ClientBuilder::new()
    .build()
    .await?
    .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

let entry = api.storage().staking().bonded(&addr, None).await;
```

Now, we build the storage query separately and submit it to the client:

```rust
let api = OnlineClient::<PolkadotConfig>::new().await?;

let staking_bonded = polkadot::storage().staking().bonded(&addr);

let entry = api.storage().fetch(&staking_bonded, None).await;
```

Note that previously, the generated code would do the equivalent of `fetch_or_default` if possible, or `fetch` if no default existed. You must now decide whether to:
- fetch an entry, returning `None` if it's not found (`api.storage().fetch(..)`), or
- fetch an entry, returning the default if it's not found (`api.storage().fetch_or_default(..)`).

The static types will protect you against using `fetch_or_default` when no such default exists, and so the recommendation is to try changing all storage requests to use `fetch_or_default`, falling back to using `fetch` where doing so leads to compile errors.

See `examples/examples/concurrent_storage_requests.rs` for an example of fetching entries.

### Iterating over storage entries

Previously:

```rust
let api = ClientBuilder::new()
    .build()
    .await?
    .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

let mut iter = api
    .storage()
    .xcm_pallet()
    .version_notifiers_iter(None)
    .await?;

while let Some((key, value)) = iter.next().await? {
    // ...
}
```

Now, as before, building the storage query to iterate over is separate from using it:

```rust
let api = OnlineClient::<PolkadotConfig>::new().await?;

let key_addr = polkadot::storage()
    .xcm_pallet()
    .version_notifiers_root();

let mut iter = api
    .storage()
    .iter(key_addr, 10, None).await?;

while let Some((key, value)) = iter.next().await? {
    // ...
}
```

Note that the `_root()` suffix on generated storage queries accesses the root entry at that address,
and is available when the address is a map that can be iterated over. By not appending `_root()`, you'll
be asked to provide the values needed to access a specific entry in the map.

See the `examples/examples/storage_iterating.rs` example for more.

### Accessing constants

Before, we'd build a client and use the client to select and query a constant:

```rust
let api = ClientBuilder::new()
    .build()
    .await?
    .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

let existential_deposit = api
    .constants()
    .balances()
    .existential_deposit()?;
```

Now, similar to the other examples, we separately build a constant _address_ and provide that address to the client to look it up:

```rust
let api = OnlineClient::<PolkadotConfig>::new().await?;

let address = polkadot::constants()
    .balances()
    .existential_deposit();

let existential_deposit = api.constants().at(&address)?;
```

See the `examples/examples/fetch_constants.rs` example for more.

### Subscribing to events

Event subscriptions themselves are relatively unchanged (although the data you can access/get back has changed a little). Before:

```rust
let api = ClientBuilder::new()
    .build()
    .await?
    .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

let mut event_sub = api.events().subscribe().await?;

while let Some(events) = event_sub.next().await {
    // ...
}
```

Now, we simply swap the client out for our new one, and the rest is similar:

```rust
let api = OnlineClient::<PolkadotConfig>::new().await?;

let mut event_sub = api.events().subscribe().await?;

while let Some(events) = event_sub.next().await {
    // ...
}
```

Note that when working with a single event, the method `event.bytes()` previously returned just the bytes associated with the event fields. Now, `event.bytes()` returns _all_ of the bytes associated with the event. There is a separate method, `event.field_bytes()`, that returns the bytes for just the fields in the event. This change will **not** lead to a compile error, and so it's worth keeping an eye out for any uses of `.bytes()` to update them to `.field_bytes()`.

See the `examples/examples/subscribe_all_events.rs` example for more.


The general pattern, as seen above, is that we break apart constructing a query/address and using it. You can now construct queries dynamically instead and forego all static codegen by using the functionality exposed in the `subxt::dynamic` module instead.

Other smaller breaking changes have happened, but they should be easier to address by following compile errors.

For more details about all of the changes, the full commit history since the last release is as follows:

### Added

- Expose the extrinsic hash from TxProgress ([#614](https://github.com/paritytech/subxt/pull/614))
- Add support for `ws` in `subxt-cli` ([#579](https://github.com/paritytech/subxt/pull/579))
- Expose the SCALE encoded call data of an extrinsic ([#573](https://github.com/paritytech/subxt/pull/573))
- Validate absolute path for `substitute_type` ([#577](https://github.com/paritytech/subxt/pull/577))

### Changed

- Rework Subxt API to support offline and dynamic transactions ([#593](https://github.com/paritytech/subxt/pull/593))
- Use scale-decode to help optimise event decoding ([#607](https://github.com/paritytech/subxt/pull/607))
- Decode raw events using scale_value and return the decoded Values, too ([#576](https://github.com/paritytech/subxt/pull/576))
- dual license ([#590](https://github.com/paritytech/subxt/pull/590))
- Don't hash constant values; only their types ([#587](https://github.com/paritytech/subxt/pull/587))
- metadata: Exclude `field::type_name` from metadata validation ([#595](https://github.com/paritytech/subxt/pull/595))
- Bump Swatinem/rust-cache from 1.4.0 to 2.0.0 ([#597](https://github.com/paritytech/subxt/pull/597))
- Update jsonrpsee requirement from 0.14.0 to 0.15.1 ([#603](https://github.com/paritytech/subxt/pull/603))


## [0.22.0] - 2022-06-20

With this release, subxt can subscribe to the node's runtime upgrades to ensure that the metadata is updated and
extrinsics are properly constructed.

We have also made some slight API improvements to make in the area of storage keys, and thanks to an external contribution we now support dry running transactions before submitting them.

This release also improves the documentation, adds UI tests, and defaults the `subxt-cli` to return metadata
bytes instead of the JSON format.

### Fixed

- Handle `StorageEntry` empty keys ([#565](https://github.com/paritytech/subxt/pull/565))
- Fix documentation examples ([#568](https://github.com/paritytech/subxt/pull/568))
- Fix cargo clippy ([#548](https://github.com/paritytech/subxt/pull/548))
- fix: Find substrate port on different log lines ([#536](https://github.com/paritytech/subxt/pull/536))

### Added

- Followup test for checking propagated documentation ([#514](https://github.com/paritytech/subxt/pull/514))
- feat: refactor signing in order to more easily be able to dryrun ([#547](https://github.com/paritytech/subxt/pull/547))
- Add subxt documentation ([#546](https://github.com/paritytech/subxt/pull/546))
- Add ability to iterate over N map storage keys ([#537](https://github.com/paritytech/subxt/pull/537))
- Subscribe to Runtime upgrades for proper extrinsic construction ([#513](https://github.com/paritytech/subxt/pull/513))

### Changed
- Move test crates into a "testing" folder and add a ui (trybuild) test and ui-test helpers ([#567](https://github.com/paritytech/subxt/pull/567))
- Update jsonrpsee requirement from 0.13.0 to 0.14.0 ([#566](https://github.com/paritytech/subxt/pull/566))
- Make storage futures only borrow client, not self, for better ergonomics ([#561](https://github.com/paritytech/subxt/pull/561))
- Bump actions/checkout from 2 to 3 ([#557](https://github.com/paritytech/subxt/pull/557))
- Deny unused crate dependencies ([#549](https://github.com/paritytech/subxt/pull/549))
- Implement `Clone` for the generated `RuntimeApi` ([#544](https://github.com/paritytech/subxt/pull/544))
- Update color-eyre requirement from 0.5.11 to 0.6.1 ([#540](https://github.com/paritytech/subxt/pull/540))
- Update jsonrpsee requirement from 0.12.0 to 0.13.0 ([#541](https://github.com/paritytech/subxt/pull/541))
- Update artifacts and polkadot.rs and change CLI to default bytes ([#533](https://github.com/paritytech/subxt/pull/533))
- Replace `log` with `tracing` and record extrinsic info ([#535](https://github.com/paritytech/subxt/pull/535))
- Bump jsonrpsee ([#528](https://github.com/paritytech/subxt/pull/528))


## [0.21.0] - 2022-05-02

This release adds static metadata validation, via comparing the statically generated API with the target node's runtime
metadata. This implies a breaking change in the subxt API, as the user receives an error when interacting with an
incompatible API at the storage, call, and constant level.

The `subxt-cli` can check the compatibility of multiple runtime nodes, either full metadata compatibility or
compatibility at the pallet level.

Users can define custom derives for specific generated types of the API via adding the `derive_for_type` configuration
to the `subxt` attribute.

The metadata documentation is propagated to the statically generated API.

Previously developers wanting to build the subxt crate needed the `substrate` binary dependency in their local
environment. This restriction is removed via moving the integration tests to a dedicated crate.

The number of dependencies is reduced for individual subxt crates.

### Fixed
- test-runtime: Add exponential backoff ([#518](https://github.com/paritytech/subxt/pull/518))

### Added

- Add custom derives for specific generated types ([#520](https://github.com/paritytech/subxt/pull/520))
- Static Metadata Validation ([#478](https://github.com/paritytech/subxt/pull/478))
- Propagate documentation to runtime API ([#511](https://github.com/paritytech/subxt/pull/511))
- Add `tidext` in real world usage ([#508](https://github.com/paritytech/subxt/pull/508))
- Add system health rpc ([#510](https://github.com/paritytech/subxt/pull/510))

### Changed
- Put integration tests behind feature flag ([#515](https://github.com/paritytech/subxt/pull/515))
- Use minimum amount of dependencies for crates ([#524](https://github.com/paritytech/subxt/pull/524))
- Export `BaseExtrinsicParams` ([#516](https://github.com/paritytech/subxt/pull/516))
- bump jsonrpsee to v0.10.1 ([#504](https://github.com/paritytech/subxt/pull/504))


## [0.20.0] - 2022-04-06

The most significant change in this release is how we create and sign extrinsics, and how we manage the
"additional" and "extra" data that is attached to them. See https://github.com/paritytech/subxt/issues/477, and the
associated PR https://github.com/paritytech/subxt/pull/490 for a more detailed look at the code changes.

If you're targeting a node with compatible additional and extra transaction data to Substrate or Polkadot, the main
change you'll have to make is to import and use `subxt::PolkadotExtrinsicParams` or `subxt::SubstrateExtrinsicParams`
instead of `subxt::DefaultExtra` (depending on what node you're compatible with), and then use `sign_and_submit_default`
instead of `sign_and_submit` when making a call. Now, `sign_and_submit` accepts a second argument which allows these
parameters (such as mortality and tip payment) to be customized. See `examples/balance_transfer_with_params.rs` for a
small usage example.

If you're targeting a node which involves custom additional and extra transaction data, you'll need to implement the
trait `subxt::extrinsic::ExtrinsicParams`, which determines the parameters that can be provided to `sign_and_submit`, as
well as how to encode these into the "additional" and "extra" data needed for a transaction. Have a look at
`subxt/src/extrinsic/params.rs` for the trait definition and Substrate/Polkadot implementations. The aim with this change
is to make it easier to customise this for your own chains, and provide a simple way to provide values at runtime.

### Fixed

- Test utils: parse port from substrate binary output to avoid races ([#501](https://github.com/paritytech/subxt/pull/501))
- Rely on the kernel for port allocation ([#498](https://github.com/paritytech/subxt/pull/498))

### Changed

- Export ModuleError for downstream matching ([#499](https://github.com/paritytech/subxt/pull/499))
- Bump jsonrpsee to v0.9.0 ([#496](https://github.com/paritytech/subxt/pull/496))
- Use tokio instead of async-std in tests/examples ([#495](https://github.com/paritytech/subxt/pull/495))
- Read constants from metadata at runtime ([#494](https://github.com/paritytech/subxt/pull/494))
- Handle `sp_runtime::ModuleError` substrate updates ([#492](https://github.com/paritytech/subxt/pull/492))
- Simplify creating and signing extrinsics ([#490](https://github.com/paritytech/subxt/pull/490))
- Add `dev_getBlockStats` RPC ([#489](https://github.com/paritytech/subxt/pull/489))
- scripts: Hardcode github subxt pull link for changelog consistency ([#482](https://github.com/paritytech/subxt/pull/482))


## [0.19.0] - 2022-03-21

### Changed

- Return events from blocks skipped over during Finalization, too ([#473](https://github.com/paritytech/subxt/pull/473))
- Use RPC call to get account nonce ([#476](https://github.com/paritytech/subxt/pull/476))
- Add script to generate release changelog based on commits ([#465](https://github.com/paritytech/subxt/pull/465))
- README updates ([#472](https://github.com/paritytech/subxt/pull/472))
- Make EventSubscription and FilterEvents Send-able ([#471](https://github.com/paritytech/subxt/pull/471))


## [0.18.1] - 2022-03-04

# Fixed

- Remove unused `sp_version` dependency to fix duplicate `parity-scale-codec` deps ([#466](https://github.com/paritytech/subxt/pull/466))


## [0.18.0] - 2022-03-02

### Added

- Expose method to fetch nonce via `Client` ([#451](https://github.com/paritytech/subxt/pull/451))

### Changed

- Reference key storage api ([#447](https://github.com/paritytech/subxt/pull/447))
- Filter one or multiple events by type from an EventSubscription ([#461](https://github.com/paritytech/subxt/pull/461))
- New Event Subscription API ([#442](https://github.com/paritytech/subxt/pull/442))
- Distinct handling for N fields + 1 hasher vs N fields + N hashers ([#458](https://github.com/paritytech/subxt/pull/458))
- Update scale-info and parity-scale-codec requirements ([#462](https://github.com/paritytech/subxt/pull/462))
- Substitute BTreeMap/BTreeSet generated types for Vec ([#459](https://github.com/paritytech/subxt/pull/459))
- Obtain DispatchError::Module info dynamically ([#453](https://github.com/paritytech/subxt/pull/453))
- Add hardcoded override to ElectionScore ([#455](https://github.com/paritytech/subxt/pull/455))
- DispatchError::Module is now a tuple variant in latest Substrate ([#439](https://github.com/paritytech/subxt/pull/439))
- Fix flaky event subscription test ([#450](https://github.com/paritytech/subxt/pull/450))
- Improve documentation ([#449](https://github.com/paritytech/subxt/pull/449))
- Export `codegen::TypeGenerator` ([#444](https://github.com/paritytech/subxt/pull/444))
- Fix conversion of `Call` struct names to UpperCamelCase ([#441](https://github.com/paritytech/subxt/pull/441))
- Update release documentation with dry-run ([#435](https://github.com/paritytech/subxt/pull/435))


## [0.17.0] - 2022-02-04

### Added

- introduce jsonrpsee client abstraction + kill HTTP support. ([#341](https://github.com/paritytech/subxt/pull/341))
- Get event context on EventSubscription ([#423](https://github.com/paritytech/subxt/pull/423))

### Changed

- Add more tests for events.rs/decode_and_consume_type ([#430](https://github.com/paritytech/subxt/pull/430))
- Update substrate dependencies ([#429](https://github.com/paritytech/subxt/pull/429))
- export RuntimeError struct ([#427](https://github.com/paritytech/subxt/pull/427))
- remove unused PalletError struct ([#425](https://github.com/paritytech/subxt/pull/425))
- Move Subxt crate into a subfolder ([#424](https://github.com/paritytech/subxt/pull/424))
- Add release checklist ([#418](https://github.com/paritytech/subxt/pull/418))


## [0.16.0] - 2022-02-01

*Note*: This is a significant release which introduces support for V14 metadata and macro based codegen, as well as making many breaking changes to the API.

### Changed

- Log debug message for JSON-RPC response ([#415](https://github.com/paritytech/subxt/pull/415))
- Only convert struct names to camel case for Call variant structs ([#412](https://github.com/paritytech/subxt/pull/412))
- Parameterize AccountData ([#409](https://github.com/paritytech/subxt/pull/409))
- Allow decoding Events containing BitVecs ([#408](https://github.com/paritytech/subxt/pull/408))
- Custom derive for cli ([#407](https://github.com/paritytech/subxt/pull/407))
- make storage-n-map fields public too ([#404](https://github.com/paritytech/subxt/pull/404))
- add constants api to codegen ([#402](https://github.com/paritytech/subxt/pull/402))
- Expose transaction::TransactionProgress as public ([#401](https://github.com/paritytech/subxt/pull/401))
- add interbtc-clients to real world usage section ([#397](https://github.com/paritytech/subxt/pull/397))
- Make own version of RuntimeVersion to avoid mismatches ([#395](https://github.com/paritytech/subxt/pull/395))
- Use the generated DispatchError instead of the hardcoded Substrate one ([#394](https://github.com/paritytech/subxt/pull/394))
- Remove bounds on Config trait that aren't strictly necessary ([#389](https://github.com/paritytech/subxt/pull/389))
- add crunch to readme ([#388](https://github.com/paritytech/subxt/pull/388))
- fix remote example ([#386](https://github.com/paritytech/subxt/pull/386))
- fetch system chain, name and version ([#385](https://github.com/paritytech/subxt/pull/385))
- Fix compact event field decoding ([#384](https://github.com/paritytech/subxt/pull/384))
- fix: use index override when decoding enums in events ([#382](https://github.com/paritytech/subxt/pull/382))
- Update to jsonrpsee 0.7 and impl Stream on TransactionProgress ([#380](https://github.com/paritytech/subxt/pull/380))
- Add links to projects using subxt ([#376](https://github.com/paritytech/subxt/pull/376))
- Use released substrate dependencies ([#375](https://github.com/paritytech/subxt/pull/375))
- Configurable Config and Extra types ([#373](https://github.com/paritytech/subxt/pull/373))
- Implement pre_dispatch for SignedExtensions ([#370](https://github.com/paritytech/subxt/pull/370))
- Export TransactionEvents ([#363](https://github.com/paritytech/subxt/pull/363))
- Rebuild test-runtime if substrate binary is updated ([#362](https://github.com/paritytech/subxt/pull/362))
- Expand the subscribe_and_watch example ([#361](https://github.com/paritytech/subxt/pull/361))
- Add TooManyConsumers variant to track latest sp-runtime addition ([#360](https://github.com/paritytech/subxt/pull/360))
- Implement new API for sign_and_submit_then_watch ([#354](https://github.com/paritytech/subxt/pull/354))
- Simpler dependencies ([#353](https://github.com/paritytech/subxt/pull/353))
- Refactor type generation, remove code duplication ([#352](https://github.com/paritytech/subxt/pull/352))
- Make system properties an arbitrary JSON object, plus CI fixes ([#349](https://github.com/paritytech/subxt/pull/349))
- Fix a couple of CI niggles ([#344](https://github.com/paritytech/subxt/pull/344))
- Add timestamp pallet test ([#340](https://github.com/paritytech/subxt/pull/340))
- Add nightly CI check against latest substrate. ([#335](https://github.com/paritytech/subxt/pull/335))
- Ensure metadata is in sync with running node during tests ([#333](https://github.com/paritytech/subxt/pull/333))
- Update to jsonrpsee 0.5.1 ([#332](https://github.com/paritytech/subxt/pull/332))
- Update substrate and hardcoded default ChargeAssetTxPayment extension ([#330](https://github.com/paritytech/subxt/pull/330))
- codegen: fix compact unnamed fields ([#327](https://github.com/paritytech/subxt/pull/327))
- Check docs and run clippy on PRs ([#326](https://github.com/paritytech/subxt/pull/326))
- Additional parameters for SignedExtra ([#322](https://github.com/paritytech/subxt/pull/322))
- fix: also processess initialize and finalize events in event subscription ([#321](https://github.com/paritytech/subxt/pull/321))
- Release initial versions of subxt-codegen and subxt-cli ([#320](https://github.com/paritytech/subxt/pull/320))
- Add some basic usage docs to README. ([#319](https://github.com/paritytech/subxt/pull/319))
- Update jsonrpsee ([#317](https://github.com/paritytech/subxt/pull/317))
- Add missing cargo metadata fields for new crates ([#311](https://github.com/paritytech/subxt/pull/311))
- fix: keep processing a block's events after encountering a dispatch error ([#310](https://github.com/paritytech/subxt/pull/310))
- Codegen: enum variant indices ([#308](https://github.com/paritytech/subxt/pull/308))
- fix extrinsics retracted ([#307](https://github.com/paritytech/subxt/pull/307))
- Add utility pallet tests ([#300](https://github.com/paritytech/subxt/pull/300))
- fix metadata constants ([#299](https://github.com/paritytech/subxt/pull/299))
- Generate runtime API from metadata ([#294](https://github.com/paritytech/subxt/pull/294))
- Add NextKeys and QueuedKeys for session module ([#291](https://github.com/paritytech/subxt/pull/291))
- deps: update jsonrpsee 0.3.0 ([#289](https://github.com/paritytech/subxt/pull/289))
- deps: update jsonrpsee 0.2.0 ([#285](https://github.com/paritytech/subxt/pull/285))
- deps: Reorg the order of deps ([#284](https://github.com/paritytech/subxt/pull/284))
- Expose the rpc client in Client ([#267](https://github.com/paritytech/subxt/pull/267))
- update jsonrpsee to 0.2.0-alpha.6 ([#266](https://github.com/paritytech/subxt/pull/266))
- Remove funty pin, upgrade codec ([#265](https://github.com/paritytech/subxt/pull/265))
- Use async-trait ([#264](https://github.com/paritytech/subxt/pull/264))
- [jsonrpsee http client]: support tokio1 & tokio02. ([#263](https://github.com/paritytech/subxt/pull/263))
- impl `From<Arc<WsClient>>` and `From<Arc<HttpClient>>` ([#257](https://github.com/paritytech/subxt/pull/257))
- update jsonrpsee ([#251](https://github.com/paritytech/subxt/pull/251))
- return none if subscription returns early ([#250](https://github.com/paritytech/subxt/pull/250))


## [0.15.0] - 2021-03-15

### Added
- implement variant of subscription that returns finalized storage changes - [#237](https://github.com/paritytech/subxt/pull/237)
- implement session handling for unsubscribe in subxt-client - [#242](https://github.com/paritytech/subxt/pull/242)

### Changed
- update jsonrpsee [#251](https://github.com/paritytech/subxt/pull/251)
- return none if subscription returns early [#250](https://github.com/paritytech/subxt/pull/250)
- export ModuleError and RuntimeError for downstream usage - [#246](https://github.com/paritytech/subxt/pull/246)
- rpc client methods should be public for downstream usage - [#240](https://github.com/paritytech/subxt/pull/240)
- re-export WasmExecutionMethod for downstream usage - [#239](https://github.com/paritytech/subxt/pull/239)
- integration with jsonrpsee v2 - [#214](https://github.com/paritytech/subxt/pull/214)
- expose wasm execution method on subxt client config - [#230](https://github.com/paritytech/subxt/pull/230)
- Add hooks to register event types for decoding - [#227](https://github.com/paritytech/subxt/pull/227)
- Substrate 3.0 - [#232](https://github.com/paritytech/subxt/pull/232)


## [0.14.0] - 2021-02-05

- Refactor event type decoding and declaration [#221](https://github.com/paritytech/subxt/pull/221)
- Add Balances Locks [#197](https://github.com/paritytech/subxt/pull/197)
- Add event Phase::Initialization [#215](https://github.com/paritytech/subxt/pull/215)
- Make type explicit [#217](https://github.com/paritytech/subxt/pull/217)
- Upgrade dependencies, bumps substrate to 2.0.1 [#219](https://github.com/paritytech/subxt/pull/219)
- Export extra types [#212](https://github.com/paritytech/subxt/pull/212)
- Enable retrieval of constants from rutnime metadata [#207](https://github.com/paritytech/subxt/pull/207)
- register type sizes for u64 and u128 [#200](https://github.com/paritytech/subxt/pull/200)
- Remove some substrate dependencies to improve compile time [#194](https://github.com/paritytech/subxt/pull/194)
- propagate 'RuntimeError's to 'decode_raw_bytes' caller [#189](https://github.com/paritytech/subxt/pull/189)
- Derive `Clone` for `PairSigner` [#184](https://github.com/paritytech/subxt/pull/184)


## [0.13.0]

- Make the contract call extrinsic work [#165](https://github.com/paritytech/subxt/pull/165)
- Update to Substrate 2.0.0 [#173](https://github.com/paritytech/subxt/pull/173)
- Display RawEvent data in hex [#168](https://github.com/paritytech/subxt/pull/168)
- Add SudoUncheckedWeightCall [#167](https://github.com/paritytech/subxt/pull/167)
- Add Add SetCodeWithoutChecksCall [#166](https://github.com/paritytech/subxt/pull/166)
- Improve contracts pallet tests [#163](https://github.com/paritytech/subxt/pull/163)
- Make Metadata types public [#162](https://github.com/paritytech/subxt/pull/162)
- Fix option decoding and add basic sanity test [#161](https://github.com/paritytech/subxt/pull/161)
- Add staking support [#160](https://github.com/paritytech/subxt/pull/161)
- Decode option event arg [#158](https://github.com/paritytech/subxt/pull/158)
- Remove unnecessary Sync bound [#172](https://github.com/paritytech/subxt/pull/172)


## [0.12.0]

- Only return an error if the extrinsic failed. [#156](https://github.com/paritytech/subxt/pull/156)
- Update to rc6. [#155](https://github.com/paritytech/subxt/pull/155)
- Different assert. [#153](https://github.com/paritytech/subxt/pull/153)
- Add a method to fetch an unhashed key, close #100 [#152](https://github.com/paritytech/subxt/pull/152)
- Fix port number. [#151](https://github.com/paritytech/subxt/pull/151)
- Implement the `concat` in `twox_64_concat` [#150](https://github.com/paritytech/subxt/pull/150)
- Storage map iter [#148](https://github.com/paritytech/subxt/pull/148)


## [0.11.0]

- Fix build error, wabt 0.9.2 is yanked [#146](https://github.com/paritytech/subxt/pull/146)
- Rc5 [#143](https://github.com/paritytech/subxt/pull/143)
- Refactor: extract functions and types for creating extrinsics [#138](https://github.com/paritytech/subxt/pull/138)
- event subscription example [#140](https://github.com/paritytech/subxt/pull/140)
- Document the `Call` derive macro [#137](https://github.com/paritytech/subxt/pull/137)
- Document the #[module] macro [#135](https://github.com/paritytech/subxt/pull/135)
- Support authors api. [#134](https://github.com/paritytech/subxt/pull/134)


## [0.10.1] - 2020-06-19

- Release client v0.2.0 [#133](https://github.com/paritytech/subxt/pull/133)


## [0.10.0] - 2020-06-19

- Upgrade to substrate rc4 release [#131](https://github.com/paritytech/subxt/pull/131)
- Support unsigned extrinsics. [#130](https://github.com/paritytech/subxt/pull/130)


## [0.9.0] - 2020-06-25

- Events sub [#126](https://github.com/paritytech/subxt/pull/126)
- Improve error handling in proc-macros, handle DispatchError etc. [#123](https://github.com/paritytech/subxt/pull/123)
- Support embedded full/light node clients. [#91](https://github.com/paritytech/subxt/pull/91)
- Zero sized types [#121](https://github.com/paritytech/subxt/pull/121)
- Fix optional store items. [#120](https://github.com/paritytech/subxt/pull/120)
- Make signing fallable and asynchronous [#119](https://github.com/paritytech/subxt/pull/119)


## [0.8.0] - 2020-05-26

- Update to Substrate release candidate [#116](https://github.com/paritytech/subxt/pull/116)
- Update to alpha.8 [#114](https://github.com/paritytech/subxt/pull/114)
- Refactors the api [#113](https://github.com/paritytech/subxt/pull/113)


## [0.7.0] - 2020-05-13

- Split subxt [#102](https://github.com/paritytech/subxt/pull/102)
- Add support for RPC `state_getReadProof` [#106](https://github.com/paritytech/subxt/pull/106)
- Update to substrate alpha.7 release [#105](https://github.com/paritytech/subxt/pull/105)
- Double map and plain storage support, introduce macros [#93](https://github.com/paritytech/subxt/pull/93)
- Raw payload return SignedPayload struct [#92](https://github.com/paritytech/subxt/pull/92)


## [0.6.0] - 2020-04-15

- Raw extrinsic payloads in Client [#83](https://github.com/paritytech/subxt/pull/83)
- Custom extras [#89](https://github.com/paritytech/subxt/pull/89)
- Wrap and export BlockNumber [#87](https://github.com/paritytech/subxt/pull/87)
- All substrate dependencies upgraded to `alpha.6`


## [0.5.0] - 2020-03-25

- First release
- All substrate dependencies upgraded to `alpha.5`
