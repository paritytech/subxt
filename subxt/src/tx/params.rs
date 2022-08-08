// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    utils::Encoded,
    Config,
};
use codec::{
    Compact,
    Encode,
};
use core::fmt::Debug;
use derivative::Derivative;

// We require Era as a param below, so make it available from here.
pub use sp_runtime::generic::Era;

/// This trait allows you to configure the "signed extra" and
/// "additional" parameters that are signed and used in transactions.
/// see [`BaseExtrinsicParams`] for an implementation that is compatible with
/// a Polkadot node.
pub trait ExtrinsicParams<Index, Hash>: Debug + 'static {
    /// These parameters can be provided to the constructor along with
    /// some default parameters that `subxt` understands, in order to
    /// help construct your [`ExtrinsicParams`] object.
    type OtherParams;

    /// Construct a new instance of our [`ExtrinsicParams`]
    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: Index,
        genesis_hash: Hash,
        other_params: Self::OtherParams,
    ) -> Self;

    /// This is expected to SCALE encode the "signed extra" parameters
    /// to some buffer that has been provided. These are the parameters
    /// which are sent along with the transaction, as well as taken into
    /// account when signing the transaction.
    fn encode_extra_to(&self, v: &mut Vec<u8>);

    /// This is expected to SCALE encode the "additional" parameters
    /// to some buffer that has been provided. These parameters are _not_
    /// sent along with the transaction, but are taken into account when
    /// signing it, meaning the client and node must agree on their values.
    fn encode_additional_to(&self, v: &mut Vec<u8>);
}

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for the default substrate node.
pub type SubstrateExtrinsicParams<T> = BaseExtrinsicParams<T, AssetTip>;

/// A builder which leads to [`SubstrateExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type SubstrateExtrinsicParamsBuilder<T> = BaseExtrinsicParamsBuilder<T, AssetTip>;

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a polkadot node.
pub type PolkadotExtrinsicParams<T> = BaseExtrinsicParams<T, PlainTip>;

/// A builder which leads to [`PolkadotExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type PolkadotExtrinsicParamsBuilder<T> = BaseExtrinsicParamsBuilder<T, PlainTip>;

/// An implementation of [`ExtrinsicParams`] that is suitable for constructing
/// extrinsics that can be sent to a node with the same signed extra and additional
/// parameters as a Polkadot/Substrate node. The way that tip payments are specified
/// differs between Substrate and Polkadot nodes, and so we are generic over that in
/// order to support both here with relative ease.
///
/// If your node differs in the "signed extra" and "additional" parameters expected
/// to be sent/signed with a transaction, then you can define your own type which
/// implements the [`ExtrinsicParams`] trait.
#[derive(Derivative)]
#[derivative(Debug(bound = "Tip: Debug"))]
pub struct BaseExtrinsicParams<T: Config, Tip: Debug> {
    era: Era,
    nonce: T::Index,
    tip: Tip,
    spec_version: u32,
    transaction_version: u32,
    genesis_hash: T::Hash,
    mortality_checkpoint: T::Hash,
    marker: std::marker::PhantomData<T>,
}

/// This builder allows you to provide the parameters that can be configured in order to
/// construct a [`BaseExtrinsicParams`] value. This implements [`Default`], which allows
/// [`BaseExtrinsicParams`] to be used with convenience methods like `sign_and_submit_default()`.
///
/// Prefer to use [`SubstrateExtrinsicParamsBuilder`] for a version of this tailored towards
/// Substrate, or [`PolkadotExtrinsicParamsBuilder`] for a version tailored to Polkadot.
#[derive(Derivative)]
#[derivative(
    Debug(bound = "Tip: Debug"),
    Clone(bound = "Tip: Clone"),
    Copy(bound = "Tip: Copy"),
    PartialEq(bound = "Tip: PartialEq")
)]
pub struct BaseExtrinsicParamsBuilder<T: Config, Tip> {
    era: Era,
    mortality_checkpoint: Option<T::Hash>,
    tip: Tip,
}

impl<T: Config, Tip: Default> BaseExtrinsicParamsBuilder<T, Tip> {
    /// Instantiate the default set of [`BaseExtrinsicParamsBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the [`Era`], which defines how long the transaction will be valid for
    /// (it can be either immortal, or it can be mortal and expire after a certain amount
    /// of time). The second argument is the block hash after which the transaction
    /// becomes valid, and must align with the era phase (see the [`Era::Mortal`] docs
    /// for more detail on that).
    pub fn era(mut self, era: Era, checkpoint: T::Hash) -> Self {
        self.era = era;
        self.mortality_checkpoint = Some(checkpoint);
        self
    }

    /// Set the tip you'd like to give to the block author
    /// for this transaction.
    pub fn tip(mut self, tip: impl Into<Tip>) -> Self {
        self.tip = tip.into();
        self
    }
}

impl<T: Config, Tip: Default> Default for BaseExtrinsicParamsBuilder<T, Tip> {
    fn default() -> Self {
        Self {
            era: Era::Immortal,
            mortality_checkpoint: None,
            tip: Tip::default(),
        }
    }
}

impl<T: Config, Tip: Debug + Encode + 'static> ExtrinsicParams<T::Index, T::Hash>
    for BaseExtrinsicParams<T, Tip>
{
    type OtherParams = BaseExtrinsicParamsBuilder<T, Tip>;

    fn new(
        // Provided from subxt client:
        spec_version: u32,
        transaction_version: u32,
        nonce: T::Index,
        genesis_hash: T::Hash,
        // Provided externally:
        other_params: Self::OtherParams,
    ) -> Self {
        BaseExtrinsicParams {
            era: other_params.era,
            mortality_checkpoint: other_params
                .mortality_checkpoint
                .unwrap_or(genesis_hash),
            tip: other_params.tip,
            nonce,
            spec_version,
            transaction_version,
            genesis_hash,
            marker: std::marker::PhantomData,
        }
    }

    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        let nonce: u64 = self.nonce.into();
        let tip = Encoded(self.tip.encode());
        (self.era, Compact(nonce), tip).encode_to(v);
    }

    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        (
            self.spec_version,
            self.transaction_version,
            self.genesis_hash,
            self.mortality_checkpoint,
        )
            .encode_to(v);
    }
}

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

/// A tip payment made in the form of a specific asset.
#[derive(Copy, Clone, Debug, Default, Encode)]
pub struct AssetTip {
    #[codec(compact)]
    tip: u128,
    asset: Option<u32>,
}

impl AssetTip {
    /// Create a new tip of the amount provided.
    pub fn new(amount: u128) -> Self {
        AssetTip {
            tip: amount,
            asset: None,
        }
    }

    /// Designate the tip as being of a particular asset class.
    /// If this is not set, then the native currency is used.
    pub fn of_asset(mut self, asset: u32) -> Self {
        self.asset = Some(asset);
        self
    }
}

impl From<u128> for AssetTip {
    fn from(n: u128) -> Self {
        AssetTip::new(n)
    }
}
