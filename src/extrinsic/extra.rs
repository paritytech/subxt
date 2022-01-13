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

use crate::PhantomDataSendSync;
use codec::{
    Decode,
    Encode,
};
use derivative::Derivative;
use scale_info::TypeInfo;
use sp_runtime::{
    generic::Era,
    traits::{
        DispatchInfoOf,
        SignedExtension,
    },
    transaction_validity::TransactionValidityError,
};

use crate::Config;

/// Extra type.
// pub type Extra<T> = <<T as Config>::Extra as SignedExtra<T>>::Extra;

/// SignedExtra checks copied from substrate, in order to remove requirement to implement
/// substrate's `frame_system::Trait`

/// Ensure the runtime version registered in the transaction is the same as at present.
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the version, which is
/// returned via `additional_signed()`.

/// Ensure the runtime version registered in the transaction is the same as at present.
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct CheckSpecVersion<T: Config>(
    pub PhantomDataSendSync<T>,
    /// Local version to be used for `AdditionalSigned`
    #[codec(skip)]
    pub u32,
);

impl<T: Config> SignedExtension for CheckSpecVersion<T> {
    const IDENTIFIER: &'static str = "CheckSpecVersion";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = u32;
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(self.1)
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// Ensure the transaction version registered in the transaction is the same as at present.
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the version, which is
/// returned via `additional_signed()`.
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct CheckTxVersion<T: Config>(
    pub PhantomDataSendSync<T>,
    /// Local version to be used for `AdditionalSigned`
    #[codec(skip)]
    pub u32,
);

impl<T: Config> SignedExtension for CheckTxVersion<T> {
    const IDENTIFIER: &'static str = "CheckTxVersion";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = u32;
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(self.1)
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// Check genesis hash
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the genesis hash, which is
/// returned via `additional_signed()`.
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct CheckGenesis<T: Config>(
    pub PhantomDataSendSync<T>,
    /// Local genesis hash to be used for `AdditionalSigned`
    #[codec(skip)]
    pub T::Hash,
);

impl<T: Config> SignedExtension for CheckGenesis<T> {
    const IDENTIFIER: &'static str = "CheckGenesis";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = T::Hash;
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(self.1)
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// Check for transaction mortality.
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the genesis hash, which is
/// returned via `additional_signed()`. It assumes therefore `Era::Immortal` (The transaction is
/// valid forever)
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct CheckMortality<T: Config>(
    /// The default structure for the Extra encoding
    pub (Era, PhantomDataSendSync<T>),
    /// Local genesis hash to be used for `AdditionalSigned`
    #[codec(skip)]
    pub T::Hash,
);

impl<T: Config> SignedExtension for CheckMortality<T> {
    const IDENTIFIER: &'static str = "CheckMortality";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = T::Hash;
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(self.1)
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// Nonce check and increment to give replay protection for transactions.
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct CheckNonce<T: Config>(#[codec(compact)] pub T::Index);

impl<T: Config> SignedExtension for CheckNonce<T> {
    const IDENTIFIER: &'static str = "CheckNonce";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// Resource limit check.
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct CheckWeight<T: Config>(pub PhantomDataSendSync<T>);

impl<T: Config> SignedExtension for CheckWeight<T> {
    const IDENTIFIER: &'static str = "CheckWeight";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// Require the transactor pay for themselves and maybe include a tip to gain additional priority
/// in the queue.
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = ""),
    Default(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct ChargeTransactionPayment<T: Config>(
    #[codec(compact)] u128,
    pub PhantomDataSendSync<T>,
);

impl<T: Config> SignedExtension for ChargeTransactionPayment<T> {
    const IDENTIFIER: &'static str = "ChargeTransactionPayment";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// Require the transactor pay for themselves and maybe include a tip to gain additional priority
/// in the queue.
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = ""),
    Default(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct ChargeAssetTxPayment<T: Config> {
    /// The tip for the block author.
    #[codec(compact)]
    pub tip: u128,
    /// The asset with which to pay the tip.
    pub asset_id: Option<u32>,
    /// Marker for unused type parameter.
    pub marker: PhantomDataSendSync<T>,
}

impl<T: Config> SignedExtension for ChargeAssetTxPayment<T> {
    const IDENTIFIER: &'static str = "ChargeAssetTxPayment";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// Trait for implementing transaction extras for a runtime.
pub trait SignedExtra<T: Config>: SignedExtension {
    /// The type the extras.
    type Extra: SignedExtension + Send + Sync;
    /// The additional config parameters.
    type Parameters: Default + Send + Sync;

    /// Creates a new `SignedExtra`.
    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: T::Index,
        genesis_hash: T::Hash,
        additional_params: Self::Parameters,
    ) -> Self;

    /// Returns the transaction extra.
    fn extra(&self) -> Self::Extra;
}

/// Default `SignedExtra` for substrate runtimes.
#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(T))]
pub struct DefaultExtraWithTxPayment<T: Config, X> {
    spec_version: u32,
    tx_version: u32,
    nonce: T::Index,
    genesis_hash: T::Hash,
    marker: PhantomDataSendSync<X>,
}

impl<T, X> SignedExtra<T> for DefaultExtraWithTxPayment<T, X>
where
    T: Config,
    X: SignedExtension<AccountId = T::AccountId, Call = ()> + Default,
{
    type Extra = (
        CheckSpecVersion<T>,
        CheckTxVersion<T>,
        CheckGenesis<T>,
        CheckMortality<T>,
        CheckNonce<T>,
        CheckWeight<T>,
        X,
    );
    type Parameters = ();

    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: T::Index,
        genesis_hash: T::Hash,
        _params: Self::Parameters,
    ) -> Self {
        DefaultExtraWithTxPayment {
            spec_version,
            tx_version,
            nonce,
            genesis_hash,
            marker: PhantomDataSendSync::new(),
        }
    }

    fn extra(&self) -> Self::Extra {
        (
            CheckSpecVersion(PhantomDataSendSync::new(), self.spec_version),
            CheckTxVersion(PhantomDataSendSync::new(), self.tx_version),
            CheckGenesis(PhantomDataSendSync::new(), self.genesis_hash),
            CheckMortality(
                (Era::Immortal, PhantomDataSendSync::new()),
                self.genesis_hash,
            ),
            CheckNonce(self.nonce),
            CheckWeight(PhantomDataSendSync::new()),
            X::default(),
        )
    }
}

impl<T, X: SignedExtension<AccountId = T::AccountId, Call = ()> + Default> SignedExtension
    for DefaultExtraWithTxPayment<T, X>
where
    T: Config,
    X: SignedExtension,
{
    const IDENTIFIER: &'static str = "DefaultExtra";
    type AccountId = T::AccountId;
    type Call = ();
    type AdditionalSigned =
        <<Self as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned;
    type Pre = ();

    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        self.extra().additional_signed()
    }
    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Ok(())
    }
}

/// A default `SignedExtra` configuration, with [`ChargeTransactionPayment`] for tipping.
///
/// Note that this must match the `SignedExtra` type in the target runtime's extrinsic definition.
pub type DefaultExtra<T> = DefaultExtraWithTxPayment<T, ChargeTransactionPayment<T>>;
