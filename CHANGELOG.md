# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
