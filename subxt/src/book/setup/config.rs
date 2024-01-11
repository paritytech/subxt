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
//! Some chains may use config that is not compatible with our [`PolkadotConfig`](crate::config::PolkadotConfig) or
//! [`SubstrateConfig`](crate::config::SubstrateConfig).
//!
//! We now walk through creating a custom [`crate::config::Config`] for a parachain, using the
//! ["Statemint"](https://parachains.info/details/statemint) parachain, also known as "Asset Hub", as an example. It
//! is currently (as of 2023-06-26) deployed on Polkadot and [Kusama (as "Statemine")](https://parachains.info/details/statemine).
//!
//! To construct a valid [`crate::config::Config`] implementation, we need to find out which types to use for `AccountId`, `Hasher`, etc.
//! For this, we need to take a look at the source code of Statemint, which is currently a part of the [Cumulus Github repository](https://github.com/paritytech/cumulus).
//! The crate defining the asset hub runtime can be found [here](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/assets/asset-hub-polkadot).
//!
//! ## `AccountId`, `Hash`, `Hasher` and `Header`
//!
//! For these config types, we need to find out where the parachain runtime implements the `frame_system::Config` trait.
//! Look for a code fragment like `impl frame_system::Config for Runtime { ... }` In the source code.
//! For Statemint it looks like [this](https://github.com/paritytech/cumulus/blob/e2b7ad2061824f490c08df27a922c64f50accd6b/parachains/runtimes/assets/asset-hub-polkadot/src/lib.rs#L179)
//! at the time of writing. The `AccountId`, `Hash` and `Header` types of the [frame_system::pallet::Config](https://docs.rs/frame-system/latest/frame_system/pallet/trait.Config.html)
//! correspond to the ones we want to use in our Subxt [crate::Config]. In the Case of Statemint (Asset Hub) they are:
//!
//! - AccountId: `sp_core::crypto::AccountId32`
//! - Hash: `sp_core::H256`
//! - Hasher (type `Hashing` in [frame_system::pallet::Config](https://docs.rs/frame-system/latest/frame_system/pallet/trait.Config.html)): `sp_runtime::traits::BlakeTwo256`
//! - Header: `sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>`
//!
//! Subxt has its own versions of some of these types in order to avoid needing to pull in Substrate dependencies:
//!
//! - `sp_core::crypto::AccountId32` can be swapped with [`crate::utils::AccountId32`].
//! - `sp_core::H256` is a re-export which subxt also provides as [`crate::config::substrate::H256`].
//! - `sp_runtime::traits::BlakeTwo256` can be swapped with [`crate::config::substrate::BlakeTwo256`].
//! - `sp_runtime::generic::Header` can be swapped with [`crate::config::substrate::SubstrateHeader`].
//!
//! Having a look at how those types are implemented can give some clues as to how to implement other custom types that
//! you may need to use as part of your config.
//!
//! ## `Address`, `Signature`
//!
//! A Substrate runtime is typically constructed by using the [frame_support::construct_runtime](https://docs.rs/frame-support/latest/frame_support/macro.construct_runtime.html) macro.
//! In this macro, we need to specify the type of an `UncheckedExtrinsic`. Most of the time, the `UncheckedExtrinsic` will be of the type
//! `sp_runtime::generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>`.
//! The generic parameters `Address` and `Signature` specified when declaring the `UncheckedExtrinsic` type
//! are the types for `Address` and `Signature` we should use with our [crate::Config] implementation. This information can
//! also be obtained from the metadata (see [`frame_metadata::v15::ExtrinsicMetadata`]). In case of Statemint (Polkadot Asset Hub)
//! we see the following types being used in `UncheckedExtrinsic`:
//!
//! - Address: `sp_runtime::MultiAddress<Self::AccountId, ()>`
//! - Signature: `sp_runtime::MultiSignature`
//!
//! As above, Subxt has its own versions of these types that can be used instead to avoid pulling in Substrate dependencies.
//! Using the Subxt versions also makes interacting with generated code (which uses them in some places) a little nicer:
//!
//! - `sp_runtime::MultiAddress` can be swapped with [`crate::utils::MultiAddress`].
//! - `sp_runtime::MultiSignature` can be swapped with [`crate::utils::MultiSignature`].
//!
//! ## ExtrinsicParams
//!
//! Chains each have a set of "signed extensions" configured. Signed extensions provide a means to extend how transactions
//! work. Each signed extension can potentially encode some "extra" data which is sent along with a transaction, as well as some
//! "additional" data which is included in the transaction signer payload, but not transmitted along with the transaction. On
//! a node, signed extensions can then perform additional checks on the submitted transactions to ensure their validity.
//!
//! The `ExtrinsicParams` config type expects to be given an implementation of the [`crate::config::ExtrinsicParams`] trait.
//! Implementations of the [`crate::config::ExtrinsicParams`] trait are handed some parameters from Subxt itself, and can
//! accept arbitrary `OtherParams` from users, and are then expected to provide this "extra" and "additional" data when asked
//! via the required [`crate::config::ExtrinsicParamsEncoder`] impl.
//!
//! **In most cases, the default [crate::config::DefaultExtrinsicParams] type will work**: it understands the "standard"
//! signed extensions that are in use, and allows the user to provide things like a tip, and set the extrinsic mortality via
//! [`crate::config::DefaultExtrinsicParamsBuilder`]. It will use the chain metadata to decide which signed extensions to use
//! and in which order. It will return an error if the chain uses a signed extension which it doesn't know how to handle.
//!
//! If the chain uses novel signed extensions (or if you just wish to provide a different interface for users to configure
//! transactions), you can either:
//!
//! 1. Implement a new signed extension and add it to the list.
//! 2. Implement [`crate::config::DefaultExtrinsicParams`] from scratch.
//!
//! See below for examples of each.
//!
//! ### Finding out which signed extensions a chain is using.
//!
//! In either case, you'll want to find out which signed extensions a chain is using. This information can be obtained from
//! the `SignedExtra` parameter of the `UncheckedExtrinsic` of your parachain, which will be a tuple of signed extensions.
//! It can also be obtained from the metadata (see [`frame_metadata::v15::SignedExtensionMetadata`]).
//!
//! For statemint, the signed extensions look like
//! [this](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/assets/asset-hub-polkadot/src/lib.rs#L779):
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
//! Each element of the `SignedExtra` tuple implements [codec::Encode] and `sp_runtime::traits::SignedExtension`
//! which has an associated type `AdditionalSigned` that also implements [codec::Encode]. Let's look at the underlying types
//! for each tuple element. All zero-sized types have been replaced by `()` for simplicity.
//!
//! | tuple element                                                                                                             | struct type                                                                                                                                              | `AdditionalSigned` type          |
//! | ------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- |
//! | [`frame_system::CheckNonZeroSender`](https://docs.rs/frame-system/latest/frame_system/struct.CheckNonZeroSender.html)     | ()                                                                                                                                                       | ()                               |
//! | [`frame_system::CheckSpecVersion`](https://docs.rs/frame-system/latest/frame_system/struct.CheckSpecVersion.html)         | ()                                                                                                                                                       | [u32]                            |
//! | [`frame_system::CheckTxVersion`](https://docs.rs/frame-system/latest/frame_system/struct.CheckTxVersion.html)             | ()                                                                                                                                                       | [u32]                            |
//! | [`frame_system::CheckGenesis`](https://docs.rs/frame-system/latest/frame_system/struct.CheckGenesis.html)                 | ()                                                                                                                                                       | `Config::Hash` = `sp_core::H256` |
//! | [`frame_system::CheckMortality`](https://docs.rs/frame-system/latest/frame_system/struct.CheckMortality.html)             | `sp_runtime::generic::Era`                                                                                                                               | `Config::Hash` = `sp_core::H256` |
//! | [`frame_system::CheckNonce`](https://docs.rs/frame-system/latest/frame_system/struct.CheckNonce.html)                     | `frame_system::pallet::Config::Index` = u32                                                                                                              | ()                               |
//! | [`frame_system::CheckWeight`](https://docs.rs/frame-system/latest/frame_system/struct.CheckWeight.html)                   | ()                                                                                                                                                       | ()                               |
//! | [`frame_system::ChargeAssetTxPayment`](https://docs.rs/frame-system/latest/frame_system/struct.ChargeAssetTxPayment.html) | [pallet_asset_tx_payment::ChargeAssetTxPayment](https://docs.rs/pallet-asset-tx-payment/latest/pallet_asset_tx_payment/struct.ChargeAssetTxPayment.html) | ()                               |
//!
//! All types in the `struct type` column make up the "extra" data that we're expected to provide. All types in the
//! `AdditionalSigned` column make up the "additional" data that we're expected to provide. This information will be useful
//! whether we want to implement [`crate::config::SignedExtension`] for a signed extension, or implement
//! [`crate::config::ExtrinsicParams`] from scratch.
//!
//! As it happens, all of the signed extensions in the table are either already exported in [`crate::config::signed_extensions`],
//! or they hand back no "additional" or "extra" data. In both of these cases, the default `ExtrinsicParams` configuration will
//! work out of the box.
//!
//! ### Implementing and adding new signed extensions to the config
//!
//! If you do need to implement a novel signed extension, then you can implement [`crate::config::signed_extensions::SignedExtension`]
//! on a custom type and place it into a new set of signed extensions, like so:
//!
//! ```rust,ignore
#![doc = include_str ! ("../../../examples/setup_config_signed_extension.rs")]
//! ```
//!
//! ### Implementing [`crate::config::ExtrinsicParams`] from scratch
//!
//! Alternately, you are free to implement [`crate::config::ExtrinsicParams`] entirely from scratch if you know exactly what "extra" and`
//! "additional" data your node needs and would prefer to craft your own interface.
//!
//! Let's see what this looks like (this config won't work on any real node):
//!
//! ```rust,ignore
#![doc = include_str ! ("../../../examples/setup_config_custom.rs")]
//! ```
//!
//! ### Using a type from the metadata as a config parameter
//!
//! You can also use types that are generated from chain metadata as type parameters of the Config trait.
//! Just make sure all trait bounds are satisfied. This can often be achieved by using custom derives with the subxt macro.
//! For example, the AssetHub Parachain, expects tips to include a `MultiLocation`, which is a type we can draw from the metadata.
//!
//! This example shows, what using the `MultiLocation` struct as part of your config would look like in subxt:
//!
//! ```rust,ignore
#![doc = include_str ! ("../../../examples/setup_config_assethub.rs")]
//! ```
