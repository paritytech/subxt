// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use codec::{
    Codec,
    Decode,
    Encode,
};
use core::{
    fmt::Debug,
    marker::PhantomData,
};
use sp_core::Pair;
use sp_runtime::{
    generic::{
        Era,
        SignedPayload,
        UncheckedExtrinsic,
    },
    traits::{
        IdentifyAccount,
        SignedExtension,
        Verify,
    },
    transaction_validity::TransactionValidityError,
};

use crate::frame::{
    balances::Balances,
    system::System,
};

/// SignedExtra checks copied from substrate, in order to remove requirement to implement
/// substrate's `frame_system::Trait`

/// Ensure the runtime version registered in the transaction is the same as at present.
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the version, which is
/// returned via `additional_signed()`.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckVersion<T: System>(
    pub PhantomData<T>,
    /// Local version to be used for `AdditionalSigned`
    #[codec(skip)]
    pub u32,
);

impl<T> SignedExtension for CheckVersion<T>
where
    T: System + Clone + Debug + Eq + Send + Sync,
{
    const IDENTIFIER: &'static str = "CheckVersion";
    type AccountId = u64;
    type Call = ();
    type AdditionalSigned = u32;
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(self.1)
    }
}

/// Check genesis hash
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the genesis hash, which is
/// returned via `additional_signed()`.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckGenesis<T: System>(
    pub PhantomData<T>,
    /// Local genesis hash to be used for `AdditionalSigned`
    #[codec(skip)]
    pub T::Hash,
);

impl<T> SignedExtension for CheckGenesis<T>
where
    T: System + Clone + Debug + Eq + Send + Sync,
{
    const IDENTIFIER: &'static str = "CheckGenesis";
    type AccountId = u64;
    type Call = ();
    type AdditionalSigned = T::Hash;
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(self.1)
    }
}

/// Check for transaction mortality.
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the genesis hash, which is
/// returned via `additional_signed()`. It assumes therefore `Era::Immortal` (The transaction is
/// valid forever)
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckEra<T: System>(
    /// The default structure for the Extra encoding
    pub (Era, PhantomData<T>),
    /// Local genesis hash to be used for `AdditionalSigned`
    #[codec(skip)]
    pub T::Hash,
);

impl<T> SignedExtension for CheckEra<T>
where
    T: System + Clone + Debug + Eq + Send + Sync,
{
    const IDENTIFIER: &'static str = "CheckEra";
    type AccountId = u64;
    type Call = ();
    type AdditionalSigned = T::Hash;
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(self.1)
    }
}

/// Nonce check and increment to give replay protection for transactions.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckNonce<T: System>(#[codec(compact)] pub T::Index);

impl<T> SignedExtension for CheckNonce<T>
where
    T: System + Clone + Debug + Eq + Send + Sync,
{
    const IDENTIFIER: &'static str = "CheckNonce";
    type AccountId = u64;
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

/// Resource limit check.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckWeight<T: System>(pub PhantomData<T>);

impl<T> SignedExtension for CheckWeight<T>
where
    T: System + Clone + Debug + Eq + Send + Sync,
{
    const IDENTIFIER: &'static str = "CheckWeight";
    type AccountId = u64;
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

/// Require the transactor pay for themselves and maybe include a tip to gain additional priority
/// in the queue.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct ChargeTransactionPayment<T: Balances>(#[codec(compact)] pub T::Balance);

impl<T> SignedExtension for ChargeTransactionPayment<T>
where
    T: Balances + Clone + Debug + Eq + Send + Sync,
{
    const IDENTIFIER: &'static str = "ChargeTransactionPayment";
    type AccountId = u64;
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

/// Checks if a transaction would exhausts the block gas limit.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckBlockGasLimit<T: System>(pub PhantomData<T>);

impl<T> SignedExtension for CheckBlockGasLimit<T>
where
    T: System + Clone + Debug + Eq + Send + Sync,
{
    const IDENTIFIER: &'static str = "CheckBlockGasLimit";
    type AccountId = u64;
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();
    fn additional_signed(
        &self,
    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

/// Trait for implementing transaction extras for a runtime.
pub trait SignedExtra<T: System> {
    /// The type the extras.
    type Extra: SignedExtension;

    /// Creates a new `SignedExtra`.
    fn new(version: u32, nonce: T::Index, genesis_hash: T::Hash) -> Self;

    /// Returns the transaction extra.
    fn extra(&self) -> Self::Extra;
}

/// Default `SignedExtra` for substrate runtimes.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct DefaultExtra<T: System> {
    version: u32,
    nonce: T::Index,
    genesis_hash: T::Hash,
}

impl<T: System + Balances + Clone + Debug + Eq + Send + Sync> SignedExtra<T>
    for DefaultExtra<T>
{
    type Extra = (
        CheckVersion<T>,
        CheckGenesis<T>,
        CheckEra<T>,
        CheckNonce<T>,
        CheckWeight<T>,
        ChargeTransactionPayment<T>,
        CheckBlockGasLimit<T>,
    );

    fn new(version: u32, nonce: T::Index, genesis_hash: T::Hash) -> Self {
        DefaultExtra {
            version,
            nonce,
            genesis_hash,
        }
    }

    fn extra(&self) -> Self::Extra {
        (
            CheckVersion(PhantomData, self.version),
            CheckGenesis(PhantomData, self.genesis_hash),
            CheckEra((Era::Immortal, PhantomData), self.genesis_hash),
            CheckNonce(self.nonce),
            CheckWeight(PhantomData),
            ChargeTransactionPayment(<T as Balances>::Balance::default()),
            CheckBlockGasLimit(PhantomData),
        )
    }
}

impl<T: System + Balances + Clone + Debug + Eq + Send + Sync> SignedExtension
    for DefaultExtra<T>
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
}

pub(crate) fn create_and_sign<T: System + Send + Sync, C, P, S, E>(
    signer: P,
    call: C,
    extra: E,
) -> Result<
    UncheckedExtrinsic<T::Address, C, S, <E as SignedExtra<T>>::Extra>,
    TransactionValidityError,
>
where
    P: Pair,
    S: Verify + Codec + From<P::Signature>,
    S::Signer: From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
    C: Encode,
    E: SignedExtra<T> + SignedExtension,
    T::Address: From<T::AccountId>,
{
    let raw_payload = SignedPayload::new(call, extra.extra())?;
    let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
    let (call, extra, _) = raw_payload.deconstruct();
    let account_id = S::Signer::from(signer.public()).into_account();

    Ok(UncheckedExtrinsic::new_signed(
        call,
        account_id.into(),
        signature.into(),
        extra,
    ))
}
