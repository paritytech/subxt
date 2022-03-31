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

//! A library to **sub**mit e**xt**rinsics to a
//! [substrate](https://github.com/paritytech/substrate) node via RPC.

use crate::Config;
use sp_core::Pair;
use sp_runtime::traits::{
    IdentifyAccount,
    Verify,
};

/// Signing transactions requires a [`Signer`]. This is responsible for
/// providing the "from" account that the transaction is being signed by,
/// as well as actually signing a SCALE encoded payload. Optionally, a
/// signer can also provide the nonce for the transaction to use.
pub trait Signer<T: Config> {
    /// Optionally returns a nonce.
    fn nonce(&self) -> Option<T::Index>;

    /// Return the "from" account ID.
    fn account_id(&self) -> &T::AccountId;

    /// Return the "from" address.
    fn address(&self) -> T::Address;

    /// Takes a signer payload for an extrinsic, and returns a signature based on it.
    ///
    /// Some signers may fail, for instance because the hardware on which the keys are located has
    /// refused the operation.
    fn sign(&self, signer_payload: &[u8]) -> T::Signature;
}

/// A [`Signer`] implementation that can be constructed from an [`Pair`].
#[derive(Clone, Debug)]
pub struct PairSigner<T: Config, P: Pair> {
    account_id: T::AccountId,
    nonce: Option<T::Index>,
    signer: P,
}

impl<T, P> PairSigner<T, P>
where
    T: Config,
    T::Signature: From<P::Signature>,
    <T::Signature as Verify>::Signer:
        From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
    P: Pair,
{
    /// Creates a new [`Signer`] from a [`Pair`].
    pub fn new(signer: P) -> Self {
        let account_id =
            <T::Signature as Verify>::Signer::from(signer.public()).into_account();
        Self {
            account_id,
            nonce: None,
            signer,
        }
    }

    /// Sets the nonce to a new value. By default, the nonce will
    /// be retrieved from the node. Setting one here will override that.
    pub fn set_nonce(&mut self, nonce: T::Index) {
        self.nonce = Some(nonce);
    }

    /// Increment the nonce.
    pub fn increment_nonce(&mut self) {
        self.nonce = self.nonce.map(|nonce| nonce + 1u32.into());
    }

    /// Returns the [`Pair`] implementation used to construct this.
    pub fn signer(&self) -> &P {
        &self.signer
    }

    /// Return the account ID.
    pub fn account_id(&self) -> &T::AccountId {
        &self.account_id
    }
}

impl<T, P> Signer<T> for PairSigner<T, P>
where
    T: Config,
    T::AccountId: Into<T::Address> + Clone + 'static,
    P: Pair + 'static,
    P::Signature: Into<T::Signature> + 'static,
{
    fn nonce(&self) -> Option<T::Index> {
        self.nonce
    }

    fn account_id(&self) -> &T::AccountId {
        &self.account_id
    }

    fn address(&self) -> T::Address {
        self.account_id.clone().into()
    }

    fn sign(&self, signer_payload: &[u8]) -> T::Signature {
        self.signer.sign(signer_payload).into()
    }
}
