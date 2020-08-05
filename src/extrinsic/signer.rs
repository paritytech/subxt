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

//! A library to **sub**mit e**xt**rinsics to a
//! [substrate](https://github.com/paritytech/substrate) node via RPC.

use super::{
    SignedExtra,
    SignedPayload,
    UncheckedExtrinsic,
};
use crate::runtimes::Runtime;
use codec::Encode;
use sp_core::{
    DeriveJunction,
    Pair,
};
use sp_runtime::traits::{
    IdentifyAccount,
    SignedExtension,
    Verify,
};
use std::{
    future::Future,
    pin::Pin,
};

/// Extrinsic signer.
pub trait Signer<T: Runtime>: Send + Sync {
    /// Returns the public key.
    fn public(&self) -> &<T::Signature as Verify>::Signer;

    /// Returns the account id.
    fn account_id(&self) -> &T::AccountId;

    /// Optionally returns a nonce.
    fn nonce(&self) -> Option<T::Index>;

    /// Sets the nonce.
    fn set_nonce(&mut self, nonce: T::Index);

    /// Increments the nonce.
    fn increment_nonce(&mut self);

    /// Takes an unsigned extrinsic and returns a signed extrinsic.
    ///
    /// Some signers may fail, for instance because the hardware on which the keys are located has
    /// refused the operation.
    fn sign_extrinsic(
        &self,
        extrinsic: SignedPayload<T>,
    ) -> Pin<Box<dyn Future<Output = Result<UncheckedExtrinsic<T>, String>> + Send + Sync>>;
}

pub trait DerivableSigner<T: Runtime>: Signer<T> {
    /// Derive signer.
    fn derive<I: Iterator<Item = DeriveJunction>>(
        &self,
        iter: I,
    ) -> Box<dyn DerivedSigner<T>>;
}

pub trait DerivedSigner<T: Runtime>: Signer<T> {
    /// Signs an arbitrary payload using a key derived with the soft junction.
    fn sign(
        &self,
        payload: &[u8],
    ) -> Pin<Box<dyn Future<Output = Result<T::Signature, String>>>>;
}

/// Extrinsic signer using a private key.
pub struct PairSigner<T: Runtime, P: Pair> {
    account_id: T::AccountId,
    public: <T::Signature as Verify>::Signer,
    nonce: Option<T::Index>,
    signer: P,
}

impl<T: Runtime, P: Pair> PairSigner<T, P>
where
    <T::Signature as Verify>::Signer:
        From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
{
    /// Creates a new `Signer` from a `Pair`.
    pub fn new(signer: P) -> Self {
        let public = <T::Signature as Verify>::Signer::from(signer.public());
        let account_id =
            <T::Signature as Verify>::Signer::from(signer.public()).into_account();
        Self {
            signer,
            public,
            account_id,
            nonce: None,
        }
    }
}

impl<T: Runtime, P: Pair> Signer<T> for PairSigner<T, P>
where
    T::AccountId: Into<T::Address>,
    <<T::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
        Send + Sync,
    <T::Signature as Verify>::Signer:
        From<P::Public> + IdentifyAccount<AccountId = T::AccountId> + Send + Sync,
    P::Signature: Into<T::Signature>,
{
    fn public(&self) -> &<T::Signature as Verify>::Signer {
        &self.public
    }

    fn account_id(&self) -> &T::AccountId {
        &self.account_id
    }

    fn nonce(&self) -> Option<T::Index> {
        self.nonce
    }

    fn set_nonce(&mut self, nonce: T::Index) {
        self.nonce = Some(nonce);
    }

    fn increment_nonce(&mut self) {
        self.nonce = self.nonce.map(|nonce| nonce + 1.into());
    }

    fn sign_extrinsic(
        &self,
        extrinsic: SignedPayload<T>,
    ) -> Pin<Box<dyn Future<Output = Result<UncheckedExtrinsic<T>, String>> + Send + Sync>>
    {
        let signature = extrinsic.using_encoded(|payload| self.signer.sign(payload));
        let (call, extra, _) = extrinsic.deconstruct();
        let extrinsic = UncheckedExtrinsic::<T>::new_signed(
            call,
            self.account_id.clone().into(),
            signature.into(),
            extra,
        );
        Box::pin(async move { Ok(extrinsic) })
    }
}

impl<T: Runtime, P: Pair> DerivableSigner<T> for PairSigner<T, P>
where
    T::AccountId: Into<T::Address>,
    <<T::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
        Send + Sync,
    <T::Signature as Verify>::Signer:
        From<P::Public> + IdentifyAccount<AccountId = T::AccountId> + Send + Sync,
    P::Signature: Into<T::Signature>,
{
    fn derive<I: Iterator<Item = DeriveJunction>>(
        &self,
        iter: I,
    ) -> Box<dyn DerivedSigner<T>> {
        Box::new(PairSigner::new(
            self.signer.derive(iter, None).map_err(|_| "").unwrap().0,
        ))
    }
}

impl<T: Runtime, P: Pair> DerivedSigner<T> for PairSigner<T, P>
where
    T::AccountId: Into<T::Address>,
    <<T::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
        Send + Sync,
    <T::Signature as Verify>::Signer:
        From<P::Public> + IdentifyAccount<AccountId = T::AccountId> + Send + Sync,
    P::Signature: Into<T::Signature>,
{
    fn sign(
        &self,
        payload: &[u8],
    ) -> Pin<Box<dyn Future<Output = Result<T::Signature, String>>>> {
        let signature = self.signer.sign(payload).into();
        Box::pin(async move { Ok(signature) })
    }
}
