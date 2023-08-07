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
//! In most cases, the default [`crate::config::DefaultExtrinsicParams`] type will work here; it understands the "standard"
//! signed extensions that are in use, and allows the user to provide things like a tip, and set the extrinsic mortality via
//! [`crate::config::DefaultExtrinsicParamsBuilder`]. It will ensure that data is given in the correct order, and return an
//! error if the chain is using signed extensions that are unknown to it which require data.
//!
//! In the event that novel signed extensions are in use (or if you just wish to provide a different interface for users to
//! configure transactions), you can either implement an interface for the new signed extension and add it to the list of
//! configured ones, or you can create an entirely custom implementation of [`crate::config::ExtrinsicParams`] with whatever
//! arbitrary behaviour you require.
//!
//! ### Finding out which signed extensions a chain is using.
//!
//! This information can be obtained from the `SignedExtra` parameter of the `UncheckedExtrinsic` of your
//! parachain, which will be a tuple of signed extensions. It can also be obtained from the metadata (see
//! [`frame_metadata::v15::SignedExtensionMetadata`]).
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
//! `AdditionalSigned` column make up the "additional" data that we're expected to provide. The goal of an
//! [`crate::config::ExtrinsicParams`] impl then is to provide the appropriate (SCALE encoded) data for these.
//! If the [`crate::config::ExtrinsicParams`] impl needs additional data to be able to do this, it can use
//! the [`crate::config::ExtrinsicParams::OtherParams`] associated type to obtain it from the user.
//!
//! ### Implementing and adding new signed extensions to the config.
//!
//! As it happens, Statemint only uses signed extensions that Subxt is natively aware of via [`crate::config::DefaultExtrinsicParams`],
//! and so the default config will work just fine. If it did not, then you could implement [`crate::config::signed_extensions::SignedExtension`]
//! on some custom type. This could then be placed into a new set of signed extensions, and (optionally) some novel interface could be
//! constructed to make it easy for users to configure them when submitting transactions.
//!
//! Let's see what this looks like:
//!
//! ```rust,ignore
#![doc = include_str ! ("../../../examples/setup_config_signed_extension.rs")]
//! ```
//!
//! ### Implementing [`crate::config::ExtrinsicParams`] from scratch.
//!
//! Alternately, you are free to implement [`crate::config::ExtrinsicParams`] entirely from scratch if you know exactly what "extra" and`
//! "additional" data your node needs and would prefer to  craft your own interface. This should be somewhat of a last resort.
//!
//! Let's see what this looks like (this config won't work on any real node):
//!
//! ```rust,ignore
#![doc = include_str ! ("../../../examples/setup_config_custom.rs")]
//! ```
