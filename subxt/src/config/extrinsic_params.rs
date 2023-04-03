// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains a trait which controls the parameters that must
//! be provided in order to successfully construct an extrinsic. A basic
//! implementation of the trait is provided ([`BaseExtrinsicParams`]) which is
//! used by the provided Substrate and Polkadot configuration.

use crate::{utils::Encoded, Config};
use codec::{Compact, Decode, Encode};
use core::fmt::Debug;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

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
/// Prefer to use [`super::substrate::SubstrateExtrinsicParamsBuilder`] for a version of this
/// tailored towards Substrate, or [`super::polkadot::PolkadotExtrinsicParamsBuilder`] for a
/// version tailored to Polkadot.
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
            mortality_checkpoint: other_params.mortality_checkpoint.unwrap_or(genesis_hash),
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

// Dev note: This and related bits taken from `sp_runtime::generic::Era`
/// An era to describe the longevity of a transaction.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Era {
    /// The transaction is valid forever. The genesis hash must be present in the signed content.
    Immortal,

    /// Period and phase are encoded:
    /// - The period of validity from the block hash found in the signing material.
    /// - The phase in the period that this transaction's lifetime begins (and, importantly,
    /// implies which block hash is included in the signature material). If the `period` is
    /// greater than 1 << 12, then it will be a factor of the times greater than 1<<12 that
    /// `period` is.
    ///
    /// When used on `FRAME`-based runtimes, `period` cannot exceed `BlockHashCount` parameter
    /// of `system` module.
    Mortal(Period, Phase),
}

/// Era period
pub type Period = u64;

/// Era phase
pub type Phase = u64;

// E.g. with period == 4:
// 0         10        20        30        40
// 0123456789012345678901234567890123456789012
//              |...|
//    authored -/   \- expiry
// phase = 1
// n = Q(current - phase, period) + phase
impl Era {
    /// Create a new era based on a period (which should be a power of two between 4 and 65536
    /// inclusive) and a block number on which it should start (or, for long periods, be shortly
    /// after the start).
    ///
    /// If using `Era` in the context of `FRAME` runtime, make sure that `period`
    /// does not exceed `BlockHashCount` parameter passed to `system` module, since that
    /// prunes old blocks and renders transactions immediately invalid.
    pub fn mortal(period: u64, current: u64) -> Self {
        let period = period
            .checked_next_power_of_two()
            .unwrap_or(1 << 16)
            .clamp(4, 1 << 16);
        let phase = current % period;
        let quantize_factor = (period >> 12).max(1);
        let quantized_phase = phase / quantize_factor * quantize_factor;

        Self::Mortal(period, quantized_phase)
    }

    /// Create an "immortal" transaction.
    pub fn immortal() -> Self {
        Self::Immortal
    }
}

// Both copied from `sp_runtime::generic::Era`; this is the wire interface and so
// it's really the most important bit here.
impl Encode for Era {
    fn encode_to<T: codec::Output + ?Sized>(&self, output: &mut T) {
        match self {
            Self::Immortal => output.push_byte(0),
            Self::Mortal(period, phase) => {
                let quantize_factor = (*period >> 12).max(1);
                let encoded = (period.trailing_zeros() - 1).clamp(1, 15) as u16
                    | ((phase / quantize_factor) << 4) as u16;
                encoded.encode_to(output);
            }
        }
    }
}
impl Decode for Era {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let first = input.read_byte()?;
        if first == 0 {
            Ok(Self::Immortal)
        } else {
            let encoded = first as u64 + ((input.read_byte()? as u64) << 8);
            let period = 2 << (encoded % (1 << 4));
            let quantize_factor = (period >> 12).max(1);
            let phase = (encoded >> 4) * quantize_factor;
            if period >= 4 && phase < period {
                Ok(Self::Mortal(period, phase))
            } else {
                Err("Invalid period and phase".into())
            }
        }
    }
}
