//! # Connecting to a parachain with Subxt
//!
//! In this example we connect to a parachain and subscribe to its blocks.
//! There are many parachains on Polkadot and Kusama. Connecting to a parachain (or any other substrate based blockchain) with subxt requires 2 steps:
//!
//! 1. Fetching the chains metadata to generate a static interface via the subxt codegen.
//! 2. Creating a config struct that implements `subxt::Config` to give some type information
//! that is currently not covered by the metadata.
//!
//! We now show these steps in detail. As an example we use the
//! ["Statemint"](https://parachains.info/details/statemint) parachain, also known as "Asset Hub" that is currently (2023-06-26)
//! deployed on Polkadot and [Kusama (as "Statemine")](https://parachains.info/details/statemine).
//!
//! ## 1. Fetch the Metadata
//!
//! To fetch the metadata for the Statemint parachain, we need to have the URL of an RPC node.
//! We can find the "Asset Hub" entry, by looking through the sidebar on [Polkadot.js](https://polkadot.js.org/apps/).
//! We connect to the node ("via Parity"), which leads us to [this page](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpolkadot-asset-hub-rpc.polkadot.io#/explorer).
//! In the URL of the page we can already see the URL of the RPC endpoint of the node as a query parameter. It is also printed
//! to the Javascript console of the browser: `WS endpoint= wss://polkadot-asset-hub-rpc.polkadot.io`.
//!
//! Let's create a separate crate for the statically generated interface.
//! Keeping the metadata and the resulting generated code in a separate crate
//! avoids unnecessary recompiles, whenever you change something in the rest of your project.
//!
//! ```txt
//! cargo new metadata --lib
//! ```
//!
//! In the new crate we can now get the metadata via the [subxt cli](https://crates.io/crates/subxt-cli) tool.
//! It is important to specify the port as `:443` like so:
//! ```txt
//! subxt metadata  --url wss://polkadot-asset-hub-rpc.polkadot.io:443 > statemint_metadata.scale
//! ```
//! The metadata is saved as `statemint_metadata.scale`. We need to add subxt as a dependency with `cargo add subxt`
//! and then we can replace the code in lib.rs with the following:
//! ```
//! #[subxt::subxt(runtime_metadata_path = "statemint_metadata.scale")]
//! pub mod statemint {}
//! ```
//! We use the `metadata` crate as a dependency in our main project and have access to
//! all the pallets and calls of Ajuna via the exported module `metadata::statemint`.
//!
//! # 2. Create the Config
//!
//! To construct a config, we need to investigate which types Statemint uses as `AccountId`, `Hasher`, etc.
//! We need to take a look at the source code of Statemint and find out how it implements some substrate functionalities.
//! Statemint (Polkadot Asset Hub) is part of the [Cumulus Github repository](https://github.com/paritytech/cumulus).
//! The crate defining the parachains runtime can be found [here](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/assets/asset-hub-polkadot).
//!
//! ## 2.1 Create the config from scratch
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
//!	    frame_system::CheckNonZeroSender<Runtime>,
//!	    frame_system::CheckSpecVersion<Runtime>,
//!	    frame_system::CheckTxVersion<Runtime>,
//!	    frame_system::CheckGenesis<Runtime>,
//!	    frame_system::CheckEra<Runtime>,
//!	    frame_system::CheckNonce<Runtime>,
//!	    frame_system::CheckWeight<Runtime>,
//!	    pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>,
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
//! ## 2.2 Simplifying the config
//!
//! Now you should be able to create a config for a parachain from scratch and understand the details of its construction.
//! However this is quite a tedious process, so subxt provides some sane defaults that can make your life easier.
//! First, let's ditch the dependencies to [sp_core] and [sp_runtime].
//! We should not be forced to utilize these traits whenever we create a config for a chain.
//! That is why subxt provides some types that can function as drop in replacements for some types in [sp_core] and [sp_runtime].
//! You can use:
//! - [subxt::config::extrinsic_params::Era] instead of [sp_runtime::generic::Era]
//! - [subxt::utils::AccountId32] instead of [sp_core::crypto::AccountId32]
//! - [subxt::utils::MultiAddress] instead of [sp_runtime::MultiAddress]
//! - [subxt::utils::H160], [subxt::utils::H256] and [subxt::utils::H512] instead of [sp_core::H256], [sp_core::H160] and [sp_core::H512]
//! - [subxt::hash] instead of [sp_runtime::MultiAddress]
//! - [subxt::config::substrate::Era] instead of [sp_runtime::generic::Era]
//! - [subxt::config::substrate::SubstrateHeader] instead of [sp_runtime::generic::Header]
//! - [subxt::config::substrate::AssetTip] and [subxt::config::polkadot::PlainTip] instead of [pallet_transaction_payment::ChargeTransactionPayment]
//! - [subxt::config::substrate::BlakeTwo256] instead of [sp_core::Blake2Hasher] or [sp_runtime::traits::BlakeTwo256]
//! - [subxt::config::substrate::DigestItem] and [subxt::config::substrate::Digest] instead of types from [sp_runtime::generic::digest]
//! - [subxt::config::substrate::ConsensusEngineId] instead of [sp_runtime::ConsensusEngineId]
//!
//! With these optimizations, our config can look like the `StatemintConfig2` below.
//!
//! ## 2.3. Use the Substrate default config as much as possible
//!
//! Because most substrate based chains share a great deal of types, _subxt_ already provides two configs:
//! - [subxt::SubstrateConfig] configured for the default [substrate node template](https://github.com/substrate-developer-hub/substrate-node-template)
//! - [subxt::PolkadotConfig] configured for the [polkadot node implementation](https://github.com/paritytech/polkadot)
//!
//! Even those two just differ in the type of the `Tip` and `MultiAddress`.
//! Statemint (Polkadot Asset Hub) seems to match the Polkadot config in almost all points, except for the `ExtrinsicParams`.
//! Here the tips follow the same structure as for the default substrate node.
//! We can now simply build the config for Statemint from the building blocks provided
//! by [subxt::SubstrateConfig] and [subxt::PolkadotConfig] as shown below in `StatemintConfig3`.
//!
//! All configs we constructed, `StatemintConfig`, `StatemintConfig2` and `StatemintConfig3` should behave in the same way.
//! All three ways are valid for constructing a config. Choose one depending on your use case.
//!
use codec::Encode;
use futures::StreamExt;
use subxt::config::ExtrinsicParams;
use subxt::{Config, OnlineClient, PolkadotConfig, SubstrateConfig};

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct ChargeAssetTxPayment {
    #[codec(compact)]
    tip: u128,
    asset_id: Option<u32>,
}

////////////////////////////////////////////////////////////
// First Config (verbose and detailed)
////////////////////////////////////////////////////////////

pub enum StatemintConfig {}

impl Config for StatemintConfig {
    type Index = u32;
    type Hash = sp_core::H256;
    type AccountId = sp_core::crypto::AccountId32;
    type Address = sp_runtime::MultiAddress<Self::AccountId, ()>;
    type Signature = sp_runtime::MultiSignature;
    type Hasher = sp_runtime::traits::BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, Self::Hasher>;
    type ExtrinsicParams = StatemintExtrinsicParams;
}

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct StatemintExtrinsicParams {
    extra_params: StatemintExtraParams,
    additional_params: StatemintAdditionalParams,
}

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct StatemintExtraParams {
    era: sp_runtime::generic::Era,
    nonce: u32,
    charge: ChargeAssetTxPayment,
}

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct StatemintAdditionalParams {
    spec_version: u32,
    tx_version: u32,
    genesis_hash: sp_core::H256,
    mortality_hash: sp_core::H256,
}

impl ExtrinsicParams<<StatemintConfig as Config>::Index, <StatemintConfig as Config>::Hash>
    for StatemintExtrinsicParams
{
    /// mortality hash, era, charge
    type OtherParams = (
        sp_core::H256,
        sp_runtime::generic::Era,
        ChargeAssetTxPayment,
    );

    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: <StatemintConfig as Config>::Index,
        genesis_hash: <StatemintConfig as Config>::Hash,
        other_params: Self::OtherParams,
    ) -> Self {
        let (mortality_hash, era, charge) = other_params;

        let extra_params = StatemintExtraParams { era, nonce, charge };

        let additional_params = StatemintAdditionalParams {
            spec_version,
            tx_version,
            genesis_hash,
            mortality_hash,
        };
        Self {
            extra_params,
            additional_params,
        }
    }

    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.extra_params.encode_to(v);
    }

    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.additional_params.encode_to(v);
    }
}

////////////////////////////////////////////////////////////
// Second Config (just using subxt types)
////////////////////////////////////////////////////////////

pub enum StatemintConfig2 {}

impl Config for StatemintConfig2 {
    type Index = u32;
    type Hash = subxt::utils::H256;
    type AccountId = subxt::utils::AccountId32;
    type Address = subxt::utils::MultiAddress<Self::AccountId, ()>;
    type Signature = subxt::utils::MultiSignature;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type Header = subxt::config::substrate::SubstrateHeader<u32, Self::Hasher>;
    type ExtrinsicParams = StatemintExtrinsicParams;
}

#[derive(Encode, Debug, Clone)]
pub struct StatemintExtrinsicParams2 {
    extra_params: StatemintExtraParams2,
    additional_params: StatemintAdditionalParams2,
}

#[derive(Encode, Debug, Clone)]
pub struct StatemintExtraParams2 {
    era: subxt::config::extrinsic_params::Era,
    nonce: u32,
    charge: subxt::config::substrate::AssetTip,
}

#[derive(Encode, Debug, Clone)]
pub struct StatemintAdditionalParams2 {
    spec_version: u32,
    tx_version: u32,
    genesis_hash: subxt::utils::H256,
    mortality_hash: subxt::utils::H256,
}

impl ExtrinsicParams<<StatemintConfig2 as Config>::Index, <StatemintConfig2 as Config>::Hash>
    for StatemintExtrinsicParams2
{
    /// mortality hash, era, charge
    type OtherParams = (
        subxt::utils::H256,
        subxt::config::extrinsic_params::Era,
        subxt::config::substrate::AssetTip,
    );

    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: <StatemintConfig2 as Config>::Index,
        genesis_hash: <StatemintConfig2 as Config>::Hash,
        other_params: Self::OtherParams,
    ) -> Self {
        let (mortality_hash, era, charge) = other_params;

        let extra_params = StatemintExtraParams2 { era, nonce, charge };

        let additional_params = StatemintAdditionalParams2 {
            spec_version,
            tx_version,
            genesis_hash,
            mortality_hash,
        };
        Self {
            extra_params,
            additional_params,
        }
    }

    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.extra_params.encode_to(v);
    }

    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.additional_params.encode_to(v);
    }
}

////////////////////////////////////////////////////////////
// Third Config (using the Substrate and Polkadot Config)
////////////////////////////////////////////////////////////

pub enum StatemintConfig3 {}

impl Config for StatemintConfig3 {
    type Index = <PolkadotConfig as Config>::Index;
    type Hash = <PolkadotConfig as Config>::Hash;
    type AccountId = <PolkadotConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <PolkadotConfig as Config>::Signature;
    type Hasher = <PolkadotConfig as Config>::Hasher;
    type Header = <PolkadotConfig as Config>::Header;
    // this is the only difference:
    type ExtrinsicParams = <SubstrateConfig as Config>::ExtrinsicParams;
}

/// In this example you can just switch out `StatemintConfig` for `StatemintConfig2` or `StatemintConfig3` and the behavior should be the same.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_endpoint = "wss://polkadot-asset-hub-rpc.polkadot.io:443";

    // here we use the config:
    let api = OnlineClient::<StatemintConfig>::from_url(rpc_endpoint).await?;

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;

    // For each block, print a bunch of information about it:
    while let Some(block) = blocks_sub.next().await {
        let block = block?;

        let block_number = block.header().number;
        let block_hash = block.hash();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        let body = block.body().await?;
        for ext in body.extrinsics().iter() {
            let ext = ext?;
            let idx = ext.index();
            let events = ext.events().await?;
            // here we make use of the generated metadata code:
            let decoded_ext = ext.as_root_extrinsic::<metadata::statemint::Call>();

            println!("    Extrinsic #{idx}:");
            println!("      Bytes: {}", ext.bytes().len());
            println!("      Decoded: {decoded_ext:?}");
            println!("      Events:");

            for evt in events.iter() {
                let evt = evt?;

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();
                let event_values = evt.field_values()?;

                println!("        {pallet_name}_{event_name}");
                println!("          {}", event_values);
            }
        }
    }

    Ok(())
}
