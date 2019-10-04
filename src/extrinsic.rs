// Copyright 2019 Parity Technologies (UK) Ltd.
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

use crate::srml::{
    balances::Balances,
    system::System,
};
use parity_scale_codec::{
    Codec,
    Decode,
    Encode,
};
use runtime_primitives::{
    generic::{
        Era,
        SignedPayload,
        UncheckedExtrinsic,
    },
    traits::SignedExtension,
    transaction_validity::TransactionValidityError,
};
use std::marker::PhantomData;
use substrate_primitives::Pair;

/// SignedExtra checks copied from substrate, in order to remove requirement to implement
/// substrate's `srml_system::Trait`

/// Ensure the runtime version registered in the transaction is the same as at present.
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the version, which is
/// returned via `additional_signed()`.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckVersion<T: System + Send + Sync>(
    PhantomData<T>,
    /// Local version to be used for `AdditionalSigned`
    #[codec(skip)]
    u32,
);

impl<T> SignedExtension for CheckVersion<T>
where
    T: System + Send + Sync,
{
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
pub struct CheckGenesis<T: System + Send + Sync>(
    PhantomData<T>,
    /// Local genesis hash to be used for `AdditionalSigned`
    #[codec(skip)]
    T::Hash,
);

impl<T> SignedExtension for CheckGenesis<T>
where
    T: System + Send + Sync,
{
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
pub struct CheckEra<T: System + Send + Sync>(
    /// The default structure for the Extra encoding
    (Era, PhantomData<T>),
    /// Local genesis hash to be used for `AdditionalSigned`
    #[codec(skip)]
    T::Hash,
);

impl<T> SignedExtension for CheckEra<T>
where
    T: System + Send + Sync,
{
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
pub struct CheckNonce<T: System + Send + Sync>(#[codec(compact)] T::Index);

impl<T> SignedExtension for CheckNonce<T>
where
    T: System + Send + Sync,
{
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
pub struct CheckWeight<T: System + Send + Sync>(PhantomData<T>);

impl<T> SignedExtension for CheckWeight<T>
where
    T: System + Send + Sync,
{
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
pub struct TakeFees<T: Balances>(#[codec(compact)] T::Balance);

impl<T> SignedExtension for TakeFees<T>
where
    T: Balances + Send + Sync,
{
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
pub struct CheckBlockGasLimit<T: System + Send + Sync>(PhantomData<T>);

impl<T> SignedExtension for CheckBlockGasLimit<T>
where
    T: System + Send + Sync,
{
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

pub trait SignedExtra<T> {
    type Extra: SignedExtension;

    fn extra(&self) -> Self::Extra;
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct DefaultExtra<T: System> {
    version: u32,
    nonce: T::Index,
    genesis_hash: T::Hash,
}

impl<T: System + Balances + Send + Sync> DefaultExtra<T> {
    pub fn new(version: u32, nonce: T::Index, genesis_hash: T::Hash) -> Self {
        DefaultExtra {
            version,
            nonce,
            genesis_hash,
        }
    }
}

impl<T: System + Balances + Send + Sync> SignedExtra<T> for DefaultExtra<T> {
    type Extra = (
        CheckVersion<T>,
        CheckGenesis<T>,
        CheckEra<T>,
        CheckNonce<T>,
        CheckWeight<T>,
        TakeFees<T>,
        CheckBlockGasLimit<T>,
    );

    fn extra(&self) -> Self::Extra {
        (
            CheckVersion(PhantomData, self.version),
            CheckGenesis(PhantomData, self.genesis_hash),
            CheckEra((Era::Immortal, PhantomData), self.genesis_hash),
            CheckNonce(self.nonce),
            CheckWeight(PhantomData),
            TakeFees(<T as Balances>::Balance::default()),
            CheckBlockGasLimit(PhantomData),
        )
    }
}

impl<T: System + Balances + Send + Sync> SignedExtension for DefaultExtra<T> {
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

pub fn create_and_sign<T: System + Send + Sync, C, P, E>(
    signer: P,
    call: C,
    extra: E,
) -> Result<
    UncheckedExtrinsic<T::Address, C, P::Signature, <E as SignedExtra<T>>::Extra>,
    TransactionValidityError,
>
where
    P: Pair,
    P::Public: Into<T::Address>,
    P::Signature: Codec,
    C: Encode,
    E: SignedExtra<T> + SignedExtension,
{
    let raw_payload = SignedPayload::new(call, extra.extra())?;
    let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
    let (call, extra, _) = raw_payload.deconstruct();

    Ok(UncheckedExtrinsic::new_signed(
        call,
        signer.public().into(),
        signature.into(),
        extra,
    ))
}
