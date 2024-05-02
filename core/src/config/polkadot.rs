// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Polkadot specific configuration

use super::{Config, DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder};

use crate::config::SubstrateConfig;
pub use crate::utils::{AccountId32, MultiAddress, MultiSignature};
pub use primitive_types::{H256, U256};

/// Default set of commonly used types by Polkadot nodes.
// Note: The trait implementations exist just to make life easier,
// but shouldn't strictly be necessary since users can't instantiate this type.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PolkadotConfig {}

impl Config for PolkadotConfig {
    // coming from: System::Config
    type Hash = <SubstrateConfig as Config>::Hash; // Done
    type AccountId = <SubstrateConfig as Config>::AccountId; // Done
    type Hasher = <SubstrateConfig as Config>::Hasher; // Done

    // coming from <runtime as traits::Block>::Extrinsic type
    type Address = MultiAddress<Self::AccountId, ()>; // Done
    type Signature = <SubstrateConfig as Config>::Signature; // Done

    // coming from <runtime as traits::Block>::Header type
    type Header = <SubstrateConfig as Config>::Header; // Done

    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;

    // coming from Assets::Config (interested in foreign Assets specifically)
    type AssetId = u32; // Done
}

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type PolkadotExtrinsicParams<T> = DefaultExtrinsicParams<T>;

/// A builder which leads to [`PolkadotExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type PolkadotExtrinsicParamsBuilder<T> = DefaultExtrinsicParamsBuilder<T>;
