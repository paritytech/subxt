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

pub use pair_signer::{
    PairSigner,
    SubstrateSigner,
    PolkadotSigner,
};

// A signer suitable for substrate based chains. At the moment, this relies on
// types defined in sp_core and sp_runtime.
mod pair_signer {
    use super::Signer;
    use serde::Serialize;
    use codec::Encode;
    use sp_core::Pair as PairT;
    use sp_runtime::traits::{
        IdentifyAccount,
        Verify,
    };

    /// A [`Signer`] implementation that can be constructed from an [`sp_core::Pair`].
    /// This is suitable for Substrate based chains; see [`PairSigner`] for a generic
    /// type that can be configured with the correct `AccountId`, `Address` and `Signature`
    /// types for your specific chain.
    pub type SubstrateSigner<Pair> = PairSigner<
        sp_runtime::AccountId32,
        sp_runtime::MultiAddress<sp_runtime::AccountId32, u32>,
        sp_runtime::MultiSignature,
        Pair,
    >;

    /// A [`Signer`] implementation that can be constructed from an [`sp_core::Pair`].
    /// This is suitable for signing things on Polkadot.
    pub type PolkadotSigner<Pair> = SubstrateSigner<Pair>;

    /// A [`Signer`] implementation that can be constructed from an [`sp_core::Pair`].
    #[derive(Clone, Debug)]
    pub struct PairSigner<AccountId, Address, Signature, Pair> {
        account_id: AccountId,
        signer: Pair,
        _types: std::marker::PhantomData<(Address, Signature)>
    }

    impl<AccountId, Address, Signature, Pair> PairSigner<AccountId, Address, Signature, Pair>
    where
        Pair: PairT,
        Signature: From<Pair::Signature> + Verify,
        <Signature as Verify>::Signer: From<Pair::Public> + IdentifyAccount<AccountId = AccountId>
    {
        /// Creates a new [`Signer`] from a [`Pair`].
        pub fn new(signer: Pair) -> Self {
            let account_id =
                <Signature as Verify>::Signer::from(signer.public()).into_account();
            Self { account_id, signer, _types: std::marker::PhantomData }
        }

        /// Returns the [`Pair`] implementation used to construct this.
        pub fn signer(&self) -> &Pair {
            &self.signer
        }

        /// Return the account ID.
        pub fn account_id(&self) -> &AccountId {
            &self.account_id
        }
    }

    impl<AccountId, Address, Signature, Pair> Signer for PairSigner<AccountId, Address, Signature, Pair>
    where
        AccountId: Serialize + Into<Address> + Clone + 'static,
        Pair: PairT + 'static,
        Address: Encode,
        Signature: Encode,
        Pair::Signature: Into<Signature> + 'static,
    {
        type AccountId = AccountId;
        type Address = Address;
        type Signature = Signature;

        fn account_id(&self) -> &Self::AccountId {
            &self.account_id
        }

        fn address(&self) -> Self::Address {
            self.account_id.clone().into()
        }

        fn sign(&self, signer_payload: &[u8]) -> Self::Signature {
            self.signer.sign(signer_payload).into()
        }
    }
}