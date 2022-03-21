// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use codec::{
    Encode,
};
use sp_runtime::{
    generic::Era,
};
use codec::Compact;

use crate::Config;

/// This trait allows you to configure the "signed extra" and
/// "additional" parameters that are signed and used in transactions.
/// see [`DefaultExtra`] for an implementation that is compatible with
/// a Polkadot node.
pub trait ExtrinsicParams<T: Config> {
    /// Thexe parameters can be provided to the constructor along with
    /// some default parameters that `subxt` understands, in order to
    /// help construct your [`ExtrinsicParams`] object.
    type OtherParams;

    /// Construct a new instance of our [`ExtrinsicParams`]
    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: T::Index,
        genesis_hash: T::Hash,
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
/// parameters as a Polkadot node. If your node differs in the "signed extra" and
/// "additional" parameters expected to be sent/signed with a transaction, then you'll
/// need to define your own struct which implements [`ExtrinsicParams`] and provides
/// back the custom extra and additional parameters you require.
pub struct DefaultExtra<T: Config> {
    era: Era,
    nonce: T::Index,
    tip: u128,
    spec_version: u32,
    transaction_version: u32,
    genesis_hash: T::Hash,
    mortality_checkpoint: T::Hash,
    marker: std::marker::PhantomData<T>
}

/// The set of parameters which can be provided in order to customise our [`DefaultExtra`]
/// values. These implement [`Default`] so that [`DefaultExtra`] can be used with
/// convenience methods like [`crate::Client::sign_and_submit_default`].
pub struct DefaultExtraParams<T: Config> {
    era: Era,
    mortality_checkpoint: Option<T::Hash>,
    tip: u128,
}

impl <T: Config> DefaultExtraParams<T> {
    /// Instantiate the default set of [`DefaultExtraParams`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the [`Era`] and era checkpoint.
    pub fn era(mut self, era: Era, checkpoint: T::Hash) -> Self {
        self.era = era;
        self.mortality_checkpoint = Some(checkpoint);
        self
    }

    /// Set the tip you'd like to give to the block author
    /// for this transaction.
    pub fn tip<Tip: Into<u128>>(mut self, tip: Tip) -> Self {
        self.tip = tip.into();
        self
    }
}

impl <T: Config> Default for DefaultExtraParams<T> {
    fn default() -> Self {
        Self {
            era: Era::Immortal,
            mortality_checkpoint: None,
            tip: 0
        }
    }
}

impl <T: Config> ExtrinsicParams<T> for DefaultExtra<T> {
    type OtherParams = DefaultExtraParams<T>;

    fn new(
        // Provided from subxt client:
        spec_version: u32,
        transaction_version: u32,
        nonce: T::Index,
        genesis_hash: T::Hash,
        // Provided externally:
        other_params: Self::OtherParams,
    ) -> Self {
        DefaultExtra {
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
        (self.era, Compact(nonce), Compact(self.tip)).encode_to(v);
    }

    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        (
            self.spec_version,
            self.transaction_version,
            self.genesis_hash,
            self.mortality_checkpoint
        ).encode_to(v);
    }
}
