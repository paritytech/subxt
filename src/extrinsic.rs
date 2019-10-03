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
    traits::{
        SignedExtension,
        StaticLookup,
    },
    transaction_validity::TransactionValidityError,
};
use std::marker::PhantomData;
use substrate_primitives::Pair;

/// SignedExtra checks copied from substrate, in order to remove requirement to implement
/// substrate's `srml_system::Trait`

macro_rules! impl_signed_extensions {
    ( $( ($name:ident, $mod:ident, $ty:ty, $self:ident, $f:ident, $res:expr, $fmt:expr) ),* ) => {
        $(
            impl<T> SignedExtension for $name<T> where T: $mod + Send + Sync {
                type AccountId = u64;
                type AdditionalSigned = $ty;
                type Call = ();
                type Pre = ();
                fn additional_signed(&$self) -> Result<$ty, TransactionValidityError> {
                    Ok($res)
                }
            }

            impl<T> std::fmt::Debug for $name<T> where T: $mod + Send + Sync {
                fn fmt(&$self, $f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    $fmt
                }
            }
        )*
    }
}

impl_signed_extensions!(
    (CheckVersion, System, u32, self, f, self.1, self.1.fmt(f)),
    (
        CheckGenesis,
        System,
        T::Hash,
        self,
        f,
        self.1,
        self.1.fmt(f)
    ),
    (CheckEra, System, T::Hash, self, f, self.1, self.1.fmt(f)),
    (CheckNonce, System, (), self, f, (), self.0.fmt(f)),
    (CheckWeight, System, (), self, _f, (), Ok(())),
    (TakeFees, Balances, (), self, f, (), self.0.fmt(f)),
    (CheckBlockGasLimit, System, (), self, _f, (), Ok(()))
);

/// Ensure the runtime version registered in the transaction is the same as at present.
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the version, which is
/// returned via `additional_signed()`.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct CheckVersion<T: System + Send + Sync>(
    PhantomData<T>,
    /// Local version to be used for `AdditionalSigned`
    #[codec(skip)]
    u32,
);

/// Check genesis hash
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the genesis hash, which is
/// returned via `additional_signed()`.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct CheckGenesis<T: System + Send + Sync>(
    PhantomData<T>,
    /// Local genesis hash to be used for `AdditionalSigned`
    #[codec(skip)]
    T::Hash,
);

/// Check for transaction mortality.
///
/// # Note
///
/// This is modified from the substrate version to allow passing in of the genesis hash, which is
/// returned via `additional_signed()`. It assumes therefore `Era::Immortal` (The transaction is
/// valid forever)
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct CheckEra<T: System + Send + Sync>(
    /// The default structure for the Extra encoding
    (Era, PhantomData<T>),
    /// Local genesis hash to be used for `AdditionalSigned`
    #[codec(skip)]
    T::Hash,
);

/// Nonce check and increment to give replay protection for transactions.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct CheckNonce<T: System + Send + Sync>(#[codec(compact)] T::Index);

/// Resource limit check.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct CheckWeight<T: System + Send + Sync>(PhantomData<T>);

/// Require the transactor pay for themselves and maybe include a tip to gain additional priority
/// in the queue.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct TakeFees<T: Balances>(#[codec(compact)] T::Balance);

/// Checks if a transaction would exhausts the block gas limit.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct CheckBlockGasLimit<T: System + Send + Sync>(PhantomData<T>);

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
    UncheckedExtrinsic<
        <T::Lookup as StaticLookup>::Source,
        C,
        P::Signature,
        <E as SignedExtra<T>>::Extra,
    >,
    TransactionValidityError,
>
where
    P: Pair,
    P::Public: Into<<T::Lookup as StaticLookup>::Source>,
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
