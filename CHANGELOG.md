# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
