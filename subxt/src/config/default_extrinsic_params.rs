// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::config::transaction_extensions::CheckMortalityParams;

use super::{Config, HashFor};
use super::{ExtrinsicParams, transaction_extensions};

/// The default [`super::ExtrinsicParams`] implementation understands common signed extensions
/// and how to apply them to a given chain.
pub type DefaultExtrinsicParams<T> = transaction_extensions::AnyOf<
    T,
    (
        transaction_extensions::VerifySignature<T>,
        transaction_extensions::CheckSpecVersion,
        transaction_extensions::CheckTxVersion,
        transaction_extensions::CheckNonce,
        transaction_extensions::CheckGenesis<T>,
        transaction_extensions::CheckMortality<T>,
        transaction_extensions::ChargeAssetTxPayment<T>,
        transaction_extensions::ChargeTransactionPayment,
        transaction_extensions::CheckMetadataHash,
    ),
>;

/// A builder that outputs the set of [`super::ExtrinsicParams::Params`] required for
/// [`DefaultExtrinsicParams`]. This may expose methods that aren't applicable to the current
/// chain; such values will simply be ignored if so.
pub struct DefaultExtrinsicParamsBuilder<T: Config> {
    /// `None` means the tx will be immortal, else it's mortality is described.
    mortality: transaction_extensions::CheckMortalityParams<T>,
    /// `None` means the nonce will be automatically set.
    nonce: Option<u64>,
    /// `None` means we'll use the native token.
    tip_of_asset_id: Option<T::AssetId>,
    tip_of: u128,
    /// A fallback tip used when no Asset ID is given (or the chain doesn't support it).
    tip: u128,
}

impl<T: Config> Default for DefaultExtrinsicParamsBuilder<T> {
    fn default() -> Self {
        Self {
            mortality: CheckMortalityParams::<T>::default(),
            tip: 0,
            tip_of: 0,
            tip_of_asset_id: None,
            nonce: None,
        }
    }
}

impl<T: Config> DefaultExtrinsicParamsBuilder<T> {
    /// Configure new extrinsic params. We default to providing no tip
    /// and using an immortal transaction unless otherwise configured
    pub fn new() -> Self {
        Default::default()
    }

    /// Make the transaction immortal, meaning it will never expire. This means that it could, in
    /// theory, be pending for a long time and only be included many blocks into the future.
    pub fn immortal(mut self) -> Self {
        self.mortality = transaction_extensions::CheckMortalityParams::<T>::immortal();
        self
    }

    /// Make the transaction mortal, given a number of blocks it will be mortal for from
    /// the current block at the time of submission.
    ///
    /// # Warning
    ///
    /// This will ultimately return an error if used for creating extrinsic offline, because we need
    /// additional information in order to set the mortality properly.
    ///
    /// When creating offline transactions, you must use [`Self::mortal_from_unchecked`] instead to set
    /// the mortality. This provides all of the necessary information which we must otherwise be online
    /// in order to obtain.
    pub fn mortal(mut self, for_n_blocks: u64) -> Self {
        self.mortality = transaction_extensions::CheckMortalityParams::<T>::mortal(for_n_blocks);
        self
    }

    /// Configure a transaction that will be mortal for the number of blocks given, and from the
    /// block details provided. Prefer to use [`Self::mortal()`] where possible, which prevents
    /// the block number and hash from being misaligned.
    pub fn mortal_from_unchecked(
        mut self,
        for_n_blocks: u64,
        from_block_n: u64,
        from_block_hash: HashFor<T>,
    ) -> Self {
        self.mortality = transaction_extensions::CheckMortalityParams::mortal_from_unchecked(
            for_n_blocks,
            from_block_n,
            from_block_hash,
        );
        self
    }

    /// Provide a specific nonce for the submitter of the extrinsic
    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    /// Provide a tip to the block author in the chain's native token.
    pub fn tip(mut self, tip: u128) -> Self {
        self.tip = tip;
        self.tip_of = tip;
        self.tip_of_asset_id = None;
        self
    }

    /// Provide a tip to the block author using the token denominated by the `asset_id` provided. This
    /// is not applicable on chains which don't use the `ChargeAssetTxPayment` signed extension; in this
    /// case, you can also call [`Self::tip`] to configure a tip in the native asset in case this is not
    /// applicable.
    pub fn tip_of(mut self, tip: u128, asset_id: T::AssetId) -> Self {
        self.tip_of = tip;
        self.tip_of_asset_id = Some(asset_id);
        self
    }

    /// Build the extrinsic parameters.
    pub fn build(self) -> <DefaultExtrinsicParams<T> as ExtrinsicParams<T>>::Params {
        let check_mortality_params = self.mortality;

        let charge_asset_tx_params = if let Some(asset_id) = self.tip_of_asset_id {
            transaction_extensions::ChargeAssetTxPaymentParams::tip_of(self.tip_of, asset_id)
        } else {
            transaction_extensions::ChargeAssetTxPaymentParams::tip(self.tip_of)
        };

        let charge_transaction_params =
            transaction_extensions::ChargeTransactionPaymentParams::tip(self.tip);

        let check_nonce_params = if let Some(nonce) = self.nonce {
            transaction_extensions::CheckNonceParams::with_nonce(nonce)
        } else {
            transaction_extensions::CheckNonceParams::from_chain()
        };

        (
            (),
            (),
            (),
            check_nonce_params,
            (),
            check_mortality_params,
            charge_asset_tx_params,
            charge_transaction_params,
            (),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_default<T: Default>(_t: T) {}

    #[test]
    fn params_are_default() {
        let params = DefaultExtrinsicParamsBuilder::<crate::config::PolkadotConfig>::new().build();
        assert_default(params)
    }
}
