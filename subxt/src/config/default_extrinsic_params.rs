// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{signed_extensions, ExtrinsicParams};
use super::{Config, Header};

/// The default [`super::ExtrinsicParams`] implementation understands common signed extensions
/// and how to apply them to a given chain.
pub type DefaultExtrinsicParams<T> = signed_extensions::AnyOf<
    T,
    (
        signed_extensions::CheckSpecVersion,
        signed_extensions::CheckTxVersion,
        signed_extensions::CheckNonce,
        signed_extensions::CheckGenesis<T>,
        signed_extensions::CheckMortality<T>,
        signed_extensions::ChargeAssetTxPayment,
        signed_extensions::ChargeTransactionPayment,
    ),
>;

/// A builder that outputs the set of [`super::ExtrinsicParams::OtherParams`] required for
/// [`DefaultExtrinsicParams`]. This may expose methods that aren't applicable to the current
/// chain; such values will simply be ignored if so.
pub struct DefaultExtrinsicParamsBuilder<T: Config> {
    /// `None` means the tx will be immortal.
    mortality: Option<Mortality<T::Hash>>,
    /// `None` means we'll use the native token.
    tip_of_asset_id: Option<u32>,
    tip: u128,
    tip_of: u128,
}

struct Mortality<Hash> {
    /// Block hash that mortality starts from
    checkpoint_hash: Hash,
    /// Block number that mortality starts from (must
    // point to the same block as the hash above)
    checkpoint_number: u64,
    /// How many blocks the tx is mortal for
    period: u64,
}

impl<T: Config> Default for DefaultExtrinsicParamsBuilder<T> {
    fn default() -> Self {
        Self {
            mortality: None,
            tip: 0,
            tip_of: 0,
            tip_of_asset_id: None,
        }
    }
}

impl<T: Config> DefaultExtrinsicParamsBuilder<T> {
    /// Configure new extrinsic params. We default to providing no tip
    /// and using an immortal transaction unless otherwise configured
    pub fn new() -> Self {
        Default::default()
    }

    /// Make the transaction mortal, given a block header that it should be mortal from,
    /// and the number of blocks (roughly; it'll be rounded to a power of two) that it will
    /// be mortal for.
    pub fn mortal(mut self, from_block: &T::Header, for_n_blocks: u64) -> Self {
        self.mortality = Some(Mortality {
            checkpoint_hash: from_block.hash(),
            checkpoint_number: from_block.number().into(),
            period: for_n_blocks,
        });
        self
    }

    /// Make the transaction mortal, given a block number and block hash (which must both point to
    /// the same block) that it should be mortal from, and the number of blocks (roughly; it'll be
    /// rounded to a power of two) that it will be mortal for.
    ///
    /// Prefer to use [`DefaultExtrinsicParamsBuilder::mortal()`], which ensures that the block hash
    /// and number align.
    pub fn mortal_unchecked(
        mut self,
        from_block_number: u64,
        from_block_hash: T::Hash,
        for_n_blocks: u64,
    ) -> Self {
        self.mortality = Some(Mortality {
            checkpoint_hash: from_block_hash,
            checkpoint_number: from_block_number,
            period: for_n_blocks,
        });
        self
    }

    /// Provide a tip to the block author in the chain's native token.
    pub fn tip(mut self, tip: u128) -> Self {
        self.tip = tip;
        self.tip_of = tip;
        self.tip_of_asset_id = None;
        self
    }

    /// Provide a tip to the block auther using the token denominated by the `asset_id` provided. This
    /// is not applicable on chains which don't use the `ChargeAssetTxPayment` signed extension; in this
    /// case, no tip will be given.
    pub fn tip_of(mut self, tip: u128, asset_id: u32) -> Self {
        self.tip = 0;
        self.tip_of = tip;
        self.tip_of_asset_id = Some(asset_id);
        self
    }

    /// Build the extrinsic parameters.
    pub fn build(self) -> <DefaultExtrinsicParams<T> as ExtrinsicParams<T>>::OtherParams {
        let check_mortality_params = if let Some(mortality) = self.mortality {
            signed_extensions::CheckMortalityParams::mortal(
                mortality.period,
                mortality.checkpoint_number,
                mortality.checkpoint_hash,
            )
        } else {
            signed_extensions::CheckMortalityParams::immortal()
        };

        let charge_asset_tx_params = if let Some(asset_id) = self.tip_of_asset_id {
            signed_extensions::ChargeAssetTxPaymentParams::tip_of(self.tip, asset_id)
        } else {
            signed_extensions::ChargeAssetTxPaymentParams::tip(self.tip)
        };

        let charge_transaction_params =
            signed_extensions::ChargeTransactionPaymentParams::tip(self.tip);

        (
            (),
            (),
            (),
            (),
            check_mortality_params,
            charge_asset_tx_params,
            charge_transaction_params,
        )
    }
}
