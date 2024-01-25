// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Polkadot specific configuration

use super::{Config, DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder};

use super::SubstrateConfig;
pub use crate::utils::{AccountId32, MultiAddress, MultiSignature};
pub use primitive_types::{H256, U256};

/// Default set of commonly used types by Polkadot nodes.
pub enum PolkadotConfig {}

impl Config for PolkadotConfig {
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;
    type AssetId = u32;
}

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type PolkadotExtrinsicParams<T> = DefaultExtrinsicParams<T>;

/// A builder which leads to [`PolkadotExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type PolkadotExtrinsicParamsBuilder<T> = DefaultExtrinsicParamsBuilder<T>;
