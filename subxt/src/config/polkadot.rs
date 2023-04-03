// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Polkadot specific configuration

use codec::Encode;

use super::extrinsic_params::{BaseExtrinsicParams, BaseExtrinsicParamsBuilder};

/// Default set of commonly used types by Polkadot nodes.
pub type PolkadotConfig = super::WithExtrinsicParams<
    super::SubstrateConfig,
    PolkadotExtrinsicParams<super::SubstrateConfig>,
>;

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type PolkadotExtrinsicParams<T> = BaseExtrinsicParams<T, PlainTip>;

/// A builder which leads to [`PolkadotExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type PolkadotExtrinsicParamsBuilder<T> = BaseExtrinsicParamsBuilder<T, PlainTip>;

// Because Era is one of the args to our extrinsic params.
pub use super::extrinsic_params::Era;

/// A tip payment.
#[derive(Copy, Clone, Debug, Default, Encode)]
pub struct PlainTip {
    #[codec(compact)]
    tip: u128,
}

impl PlainTip {
    /// Create a new tip of the amount provided.
    pub fn new(amount: u128) -> Self {
        PlainTip { tip: amount }
    }
}

impl From<u128> for PlainTip {
    fn from(n: u128) -> Self {
        PlainTip::new(n)
    }
}
