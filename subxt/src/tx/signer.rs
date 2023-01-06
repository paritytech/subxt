// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A library to **sub**mit e**xt**rinsics to a
//! [substrate](https://github.com/paritytech/substrate) node via RPC.

use crate::Config;

/// Signing transactions requires a [`Signer`]. This is responsible for
/// providing the "from" account that the transaction is being signed by,
/// as well as actually signing a SCALE encoded payload.
pub trait Signer<T: Config> {
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

#[cfg(feature = "substrate-extra")]
pub use pair_signer::PairSigner;

// A signer suitable for substrate based chains. This provides compatibility with Substrate
// packages like sp_keyring and such, and so relies on sp_core and sp_runtime to be included.
#[cfg(feature = "substrate-extra")]
mod pair_signer {
    use super::Signer;
    use crate::Config;
    use sp_core::Pair as PairT;
    use sp_runtime::{
        traits::{
            IdentifyAccount,
            Verify,
        },
        AccountId32 as SpAccountId32,
        MultiSignature as SpMultiSignature,
    };

    /// A [`Signer`] implementation that can be constructed from an [`sp_core::Pair`].
    #[derive(Clone, Debug)]
    pub struct PairSigner<T: Config, Pair> {
        account_id: T::AccountId,
        signer: Pair,
    }

    impl<T, Pair> PairSigner<T, Pair>
    where
        T: Config,
        T::AccountId: From<SpAccountId32>,
        Pair: PairT,
        SpMultiSignature: From<Pair::Signature> + Verify,
        <SpMultiSignature as Verify>::Signer:
            From<Pair::Public> + IdentifyAccount<AccountId = SpAccountId32>,
    {
        /// Creates a new [`Signer`] from an [`sp_core::Pair`].
        pub fn new(signer: Pair) -> Self {
            let account_id = <SpMultiSignature as Verify>::Signer::from(signer.public())
                .into_account();
            Self {
                account_id: account_id.into(),
                signer,
            }
        }

        /// Returns the [`sp_core::Pair`] implementation used to construct this.
        pub fn signer(&self) -> &Pair {
            &self.signer
        }

        /// Return the account ID.
        pub fn account_id(&self) -> &T::AccountId {
            &self.account_id
        }
    }

    impl<T, Pair> Signer<T> for PairSigner<T, Pair>
    where
        T: Config,
        Pair: PairT + 'static,
        // use SpMultiSignature as a bridge; our substrate Pair
        // must be convertible into that, which then must convert
        // into the Signature type we've configured.
        Pair::Signature: Into<SpMultiSignature> + 'static,
        SpMultiSignature: Into<T::Signature>,
    {
        fn account_id(&self) -> &T::AccountId {
            &self.account_id
        }

        fn address(&self) -> T::Address {
            self.account_id.clone().into()
        }

        fn sign(&self, signer_payload: &[u8]) -> T::Signature {
            self.signer.sign(signer_payload).into().into()
        }
    }
}
