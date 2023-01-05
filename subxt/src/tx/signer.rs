// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A library to **sub**mit e**xt**rinsics to a
//! [substrate](https://github.com/paritytech/substrate) node via RPC.

use codec::Encode;
use serde::Serialize;

/// Signing transactions requires a [`Signer`]. This is responsible for
/// providing the "from" account that the transaction is being signed by,
/// as well as actually signing a SCALE encoded payload.
pub trait Signer {
    /// This is used to obtain the next available nonce from the node RPC. As such,
    /// we need to be able to serialize it appropriately for this call.
    type AccountId: Serialize;

    /// The "from" address is encoded into extrinsics.
    type Address: Encode;

    /// This is the result of signing an extrinsic payload, and is encoded into extrinsics.
    type Signature: Encode;

    /// Return the "from" account ID.
    fn account_id(&self) -> &Self::AccountId;

    /// Return the "from" address.
    fn address(&self) -> Self::Address;

    /// Takes a signer payload for an extrinsic, and returns a signature based on it.
    ///
    /// Some signers may fail, for instance because the hardware on which the keys are located has
    /// refused the operation.
    fn sign(&self, signer_payload: &[u8]) -> Self::Signature;
}

#[cfg(feature = "substrate-extra")]
pub use pair_signer::PairSigner;

// A signer suitable for substrate based chains. At the moment, this relies on
// types defined in sp_core and sp_runtime.
#[cfg(feature = "substrate-extra")]
mod pair_signer {
    use super::Signer;
    use crate::Config;
    use sp_core::Pair as PairT;
    use sp_runtime::traits::{
        IdentifyAccount,
        Verify,
    };
    use sp_runtime::{
        MultiSignature as SpMultiSignature,
        AccountId32 as SpAccountId32,
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
        <SpMultiSignature as Verify>::Signer: From<Pair::Public> + IdentifyAccount<AccountId = SpAccountId32>
    {
        /// Creates a new [`Signer`] from a [`Pair`].
        pub fn new(signer: Pair) -> Self {
            let account_id =
                <SpMultiSignature as Verify>::Signer::from(signer.public()).into_account();
            Self { account_id: account_id.into(), signer }
        }

        /// Returns the [`Pair`] implementation used to construct this.
        pub fn signer(&self) -> &Pair {
            &self.signer
        }

        /// Return the account ID.
        pub fn account_id(&self) -> &T::AccountId {
            &self.account_id
        }
    }

    impl<T, Pair> Signer for PairSigner<T, Pair>
    where
        T: Config,
        Pair: PairT + 'static,
        // use SpMultiSignature as a bridge; our substrate Pair
        // must be convertible into that, which then must convert
        // into the Signature type we've configured.
        Pair::Signature: Into<SpMultiSignature> + 'static,
        SpMultiSignature: Into<T::Signature>,
    {
        type AccountId = T::AccountId;
        type Address = T::Address;
        type Signature = T::Signature;

        fn account_id(&self) -> &Self::AccountId {
            &self.account_id
        }

        fn address(&self) -> Self::Address {
            self.account_id.clone().into()
        }

        fn sign(&self, signer_payload: &[u8]) -> Self::Signature {
            self.signer.sign(signer_payload).into().into()
        }
    }

    // Add these impls to allow our own AccountId32 and MultiSignature to be
    // used in Config, and this PairSigner to still work.
    impl From<SpAccountId32> for crate::utils::account_id::AccountId32 {
        fn from(value: SpAccountId32) -> Self {
            Self(value.into())
        }
    }
    impl From<SpMultiSignature> for crate::utils::multi_signature::MultiSignature {
        fn from(value: SpMultiSignature) -> Self {
            match value {
                SpMultiSignature::Ed25519(s) => Self::Ed25519(s.0),
                SpMultiSignature::Sr25519(s) => Self::Sr25519(s.0),
                SpMultiSignature::Ecdsa(s) => Self::Ecdsa(s.0),
            }
        }
    }
}