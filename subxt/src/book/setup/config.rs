//! # Creating a Config
//!
//! Subxt requires you to provide a type implementing [`crate::config::Config`] in order to connect to a node.
//! The [`crate::config::Config`] trait for the most part mimics the `frame_system::Config` trait.
//! For most use cases, you can just use one of the following Configs shipped with Subxt:
//!
//! - [`PolkadotConfig`](crate::config::PolkadotConfig) for talking to Polkadot nodes, and
//! - [`SubstrateConfig`](crate::config::SubstrateConfig) for talking to generic nodes built with Substrate.
//!
//! # How to create a Config for a custom chain?
//!
//! We now show in depth how a subxt Config can be created for a parachain, using the ["Statemint"](https://parachains.info/details/statemint) parachain,
//! also known as "Asset Hub" as an example. It is currently (as of 2023-06-26)
//! deployed on Polkadot and [Kusama (as "Statemine")](https://parachains.info/details/statemine).
//! We also provide a separate example on [how to interact with the parachain](https://github.com/paritytech/subxt/tree/master/examples/parachain-example).
//!
//! To construct a config, we need to investigate which types Statemint uses as `AccountId`, `Hasher`, etc.
//! We need to take a look at the source code of Statemint and find out how it implements some substrate functionalities.
//! Statemint (Polkadot Asset Hub) is part of the [Cumulus Github repository](https://github.com/paritytech/cumulus).
//! The crate defining the parachains runtime can be found [here](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/assets/asset-hub-polkadot).
//!
//! ## 1. Create the config from scratch
//!
//! First we want to create the config from scratch to understand all the details.
//!
//! ### AccountId, Index, Hash, Hasher and Header
//! We need to check, where the parachains runtime implements the frame_system::Config trait.
//! Look for a code fragment like `impl frame_system::Config for Runtime { ... }` In the source code.
//! For Statemint we find it [here](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/assets/asset-hub-polkadot/src/lib.rs#L179).
//! The `AccountId`, `Index`, `Hash` and `Header` types of the [frame_system::Config] should also be the ones
//! we want to use for implementing [subxt::Config].
//! In the Case of Statemint (Asset Hub) they are:
//!
//! - AccountId: [sp_core::crypto::AccountId32]
//! - Index: [u32]
//! - Hash: [sp_core::H256]
//! - Hasher (type `Hashing` in [frame_system::Config]): [sp_runtime::traits::BlakeTwo256]
//! - Header: [sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>](sp_runtime::generic::Header)
//!
//! ### Address, Signature, ExtrinsicParams
//! A Substrate runtime is typically constructed by using the [frame_support::construct_runtime] macro.
//! In this macro, we need to specify the type of an `UncheckedExtrinsic`. Most of the time, the `UncheckedExtrinsic` will be of the type
//! [sp_runtime::generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>](sp_runtime::generic::UncheckedExtrinsic).
//! The generic parameters `Address` and `Signature` specified when declaring the `UncheckedExtrinsic` type
//! are the types for `Address` and `Signature` we should use when implementing the [subxt::Config] trait.
//! In case of Statemint (Polkadot Asset Hub) we see the following types being used in `UncheckedExtrinsic`:
//!
//! - Address: [sp_runtime::MultiAddress<Self::AccountId, ()>](sp_runtime::MultiAddress)
//! - Signature: [sp_runtime::MultiSignature]
//!
//! #### ExtrinsicParams
//! The `ExtrinsicParams` type is the most complicated to set up, but it can be derived from
//! the `SignedExtra` parameter of the `UncheckedExtrinsic` of your parachain. The `SignedExtra` parameter is a tuple in most cases,
//! where each field of the tuple corresponds to 0-1 fields in the `ExtrinsicParams` struct we want to create.
//! It looks like [this](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/assets/asset-hub-polkadot/src/lib.rs#L779) for the Statemint parachain:
//!
//! ```rs
//! pub type SignedExtra = (
//!     frame_system::CheckNonZeroSender<Runtime>,
//!     frame_system::CheckSpecVersion<Runtime>,
//!     frame_system::CheckTxVersion<Runtime>,
//!     frame_system::CheckGenesis<Runtime>,
//!     frame_system::CheckEra<Runtime>,
//!     frame_system::CheckNonce<Runtime>,
//!     frame_system::CheckWeight<Runtime>,
//!     pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>,
//! );
//! ```
//!
//! The `ExtrinsicParams` struct is basically a collection of fields that can be grouped into two categories:
//! _extra parameters_ and _additional parameters_. _Extra parameters_ are taken into account when
//! signing a transaction and sent along with it, while _additional parameters_ are only used during
//! the signing step but _not_ sent along with the transaction.
//!
//! Each element of the `SignedExtra` tuple implements [parity_scale_codec::codec::Encode] and [sp_runtime::traits::SignedExtension]
//! which has an associated type `AdditionalSigned` that also implements [parity_scale_codec::codec::Encode]. Let's look at the underlying types for each tuple element.
//! All zero-sized types have been replaced by `()` for simplicity.
//!
//! | tuple element                        | struct type                                     | `AdditionalSigned` type          |
//! | ------------------------------------ | ----------------------------------------------- | ---------------------------------|
//! | [frame_system::CheckNonZeroSender]   | ()                                              | ()                               |
//! | [frame_system::CheckSpecVersion]     | ()                                              | [u32]                            |
//! | [frame_system::CheckTxVersion]       | ()                                              | [u32]                            |
//! | [frame_system::CheckGenesis]         | ()                                              | `Config::Hash` = [sp_core::H256] |
//! | [frame_system::CheckMortality]       | [sp_runtime::generic::Era]                      | `Config::Hash` = [sp_core::H256] |
//! | [frame_system::CheckNonce]           | [Config::Index] = [u32]                         | ()                               |
//! | [frame_system::CheckWeight]          | ()                                              | ()                               |
//! | [frame_system::ChargeAssetTxPayment] | [pallet_asset_tx_payment::ChargeAssetTxPayment] | ()                               |
//!
//! All types in the `struct type` column make up our _extra parameters_.
//! We can put them together into a struct, ignoring all zero-sized types.
//! The types of the last column make up our _additional parameters_. Here
//! we can also ignore the zero-sized types. We can name the fields however we like,
//! however their order in the struct should match the order in the `SignedExtra` tuple (see table).
//! Beware that some numbers might be compact encoded, when encoding the struct, some might not be.
//! Please check the exact struct type definition here. For example the definition for CheckNonce is
//! `CheckNonce<T: Config>(#[codec(compact)] pub T::Index)`. We can see that the `#[codec(compact)`
//! attribute tells us that the `u32` value needs to be compact encoded.
//!
//! Because the `pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>` struct requires us to pass the
//! struct of our `Runtime` as a generic argument, we decide to just recreate the struct here. This prohibits us
//! from pulling in tons of unneeded dependencies. This is a pattern that you might see often,
//! especially because substrate crates and their types are quite big, clunky and entangled sometimes.
//!
//! With all these types collected, we can create two structs: `StatemintExtraParams` and `StatemintAdditionalParams`.
//! Then we combine them into a `StatemintExtrinsicParams` struct for which we implement the [subxt::config::ExtrinsicParams] trait, see below.
//! Now we have all the parts we need to create our config `StatemintConfig` and implment the [subxt::Config] trait on it.
//! Note that StatemintConfig is an empty enum, an _uninhabited type_ that is never means to be instantiated
//! but just gives type information to various interfaces of _subxt_.
//!
//! ```rust,ignore
#![doc = include_str ! ("../../../../examples/parachain-example/src/statemint_config_verbose.rs")]
//! ```
//!
//! ## 2. Simplifying the config
//!
//! Now you should be able to create a config for a parachain from scratch and understand the details of its construction.
//! However this is quite a tedious process, so subxt provides some sane defaults that can make your life easier.
//! First, let's ditch the dependencies to [sp_core] and [sp_runtime].
//! We should not be forced to utilize these traits whenever we create a config for a chain.
//! That is why subxt provides some types that can function as drop in replacements for some types in [sp_core] and [sp_runtime].
//! You can use:
//! - [crate::config::extrinsic_params::Era] instead of [sp_runtime::generic::Era]
//! - [crate::utils::AccountId32] instead of [sp_core::crypto::AccountId32]
//! - [crate::utils::MultiAddress] instead of [sp_runtime::MultiAddress]
//! - [crate::utils::H160], [crate::utils::H256] and [crate::utils::H512] instead of [sp_core::H256], [sp_core::H160] and [sp_core::H512]
//! - [crate::utils::MultiAddress] instead of [sp_runtime::MultiAddress]
//! - [crate::config::substrate::Era] instead of [sp_runtime::generic::Era]
//! - [crate::config::substrate::SubstrateHeader] instead of [sp_runtime::generic::Header]
//! - [crate::config::substrate::AssetTip] and [crate::config::polkadot::PlainTip] instead of [pallet_transaction_payment::ChargeTransactionPayment](https://docs.rs/pallet-transaction-payment/latest/pallet_transaction_payment/struct.ChargeTransactionPayment.html)
//! - [crate::config::substrate::BlakeTwo256] instead of [sp_core::Blake2Hasher] or [sp_runtime::traits::BlakeTwo256]
//! - [crate::config::substrate::DigestItem] and [crate::config::substrate::Digest] instead of types from [sp_runtime::generic::digest]
//! - [crate::config::substrate::ConsensusEngineId] instead of [sp_runtime::ConsensusEngineId]
//!
//! With these optimizations, our config can look like this:
//!
//! ```rust,ignore
#![doc = include_str ! ("../../../../examples/parachain-example/src/statemint_config_with_subxt_types.rs")]
//! ```
//!
//! ## 3. Use the [`PolkadotConfig`](crate::PolkadotConfig) and [`SubstrateConfig`](crate::SubstrateConfig) to compose a Config
//!
//! Because most substrate based chains share a great deal of types, _subxt_ already provides two configs:
//! - [`SubstrateConfig`](crate::SubstrateConfig) configured for the default [substrate node template](https://github.com/substrate-developer-hub/substrate-node-template)
//! - [`PolkadotConfig`](crate::PolkadotConfig) configured for the [polkadot node implementation](https://github.com/paritytech/polkadot)
//!
//! Even those two just differ in the type of the `Tip` and `MultiAddress`.
//! Statemint (Polkadot Asset Hub) seems to match the Polkadot config in almost all points, except for the `ExtrinsicParams`.
//! Here the tips follow the same structure as for the default substrate node.
//! We can now simply build the config for Statemint from the building blocks provided
//! by [`PolkadotConfig`](crate::PolkadotConfig) and [`SubstrateConfig`](crate::SubstrateConfig) as shown below.
//!
//! ```rust,ignore
#![doc = include_str ! ("../../../../examples/parachain-example/src/statemint_config_composed.rs")]
//! ```
