//! This example demonstrates how to use to add a layer for full functionality for types in substrate.
//!
//! Similar functionality was provided by the `substrate-compat` feature in the original `subxt` crate.
//! which is now removed.

#![allow(missing_docs)]

use std::ops::Deref;

use polkadot_sdk::{sp_core, sp_runtime};
use sp_core::Pair as _;
use subxt::{Config, OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

/// AccountId32 for substrate compatibility.
#[derive(Clone, Debug)]
pub struct AccountId32(subxt::config::substrate::AccountId32);

/// MultiAddress type for substrate compatibility.
#[derive(Clone, Debug)]
pub struct MultiAddress<AccountId, AccountIndex>(
    subxt::config::substrate::MultiAddress<AccountId, AccountIndex>,
);

/// A [`Signer`] implementation that can be constructed from an [`sp_core::Pair`].
#[derive(Clone, Debug)]
pub struct PairSigner<Pair> {
    account_id: AccountId32,
    signer: Pair,
}

/// MultiSignature type for Substrate compatibility.
#[derive(Clone, Debug)]
pub struct MultiSignature(subxt::config::substrate::MultiSignature);

mod account_impls {
    use super::*;

    impl From<sp_runtime::AccountId32> for AccountId32 {
        fn from(value: sp_runtime::AccountId32) -> Self {
            let bytes: [u8; 32] = value.into();
            let subxt_acc: subxt::config::substrate::AccountId32 = bytes.into();
            subxt_acc.into()
        }
    }
    impl From<sp_core::sr25519::Public> for AccountId32 {
        fn from(value: sp_core::sr25519::Public) -> Self {
            let acc: sp_runtime::AccountId32 = value.into();
            acc.into()
        }
    }
    impl From<sp_core::ed25519::Public> for AccountId32 {
        fn from(value: sp_core::ed25519::Public) -> Self {
            let acc: sp_runtime::AccountId32 = value.into();
            acc.into()
        }
    }
    impl From<subxt::config::substrate::AccountId32> for AccountId32 {
        fn from(value: subxt::config::substrate::AccountId32) -> Self {
            Self(value)
        }
    }

    impl Deref for AccountId32 {
        type Target = subxt::config::substrate::AccountId32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

mod multi_addr_impls {
    use super::*;

    impl<N> From<sp_runtime::AccountId32> for MultiAddress<AccountId32, N> {
        fn from(value: sp_runtime::AccountId32) -> Self {
            value.into()
        }
    }

    impl<N> From<subxt::config::substrate::MultiAddress<AccountId32, N>>
        for MultiAddress<AccountId32, N>
    {
        fn from(value: subxt::config::substrate::MultiAddress<AccountId32, N>) -> Self {
            Self(value)
        }
    }

    impl<Id, N> From<sp_runtime::MultiAddress<Id, N>> for MultiAddress<AccountId32, N>
    where
        Id: Into<AccountId32>,
    {
        fn from(value: sp_runtime::MultiAddress<Id, N>) -> Self {
            let inner = match value {
                sp_runtime::MultiAddress::Id(v) => {
                    subxt::config::substrate::MultiAddress::Id(v.into())
                }
                sp_runtime::MultiAddress::Index(v) => {
                    subxt::config::substrate::MultiAddress::Index(v)
                }
                sp_runtime::MultiAddress::Raw(v) => subxt::config::substrate::MultiAddress::Raw(v),
                sp_runtime::MultiAddress::Address32(v) => {
                    subxt::config::substrate::MultiAddress::Address32(v)
                }
                sp_runtime::MultiAddress::Address20(v) => {
                    subxt::config::substrate::MultiAddress::Address20(v)
                }
            };
            inner.into()
        }
    }

    impl<Id, N> Deref for MultiAddress<Id, N> {
        type Target = subxt::config::substrate::MultiAddress<Id, N>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

mod multi_signature_impls {
    use super::*;

    impl From<sp_runtime::MultiSignature> for MultiSignature {
        fn from(value: sp_runtime::MultiSignature) -> Self {
            let inner = match value {
                sp_runtime::MultiSignature::Ed25519(s) => {
                    subxt::config::substrate::MultiSignature::Ed25519(s.0)
                }
                sp_runtime::MultiSignature::Sr25519(s) => {
                    subxt::config::substrate::MultiSignature::Sr25519(s.0)
                }
                sp_runtime::MultiSignature::Ecdsa(s) => {
                    subxt::config::substrate::MultiSignature::Ecdsa(s.0)
                }
            };
            Self(inner)
        }
    }

    impl From<subxt::config::substrate::MultiSignature> for MultiSignature {
        fn from(value: subxt::config::substrate::MultiSignature) -> Self {
            Self(value)
        }
    }

    impl From<sp_core::ed25519::Signature> for MultiSignature {
        fn from(value: sp_core::ed25519::Signature) -> Self {
            let sig: sp_runtime::MultiSignature = value.into();
            sig.into()
        }
    }

    impl From<sp_core::sr25519::Signature> for MultiSignature {
        fn from(value: sp_core::sr25519::Signature) -> Self {
            let sig: sp_runtime::MultiSignature = value.into();
            sig.into()
        }
    }

    impl From<sp_core::ecdsa::Signature> for MultiSignature {
        fn from(value: sp_core::ecdsa::Signature) -> Self {
            let sig: sp_runtime::MultiSignature = value.into();
            sig.into()
        }
    }

    impl Deref for MultiSignature {
        type Target = subxt::config::substrate::MultiSignature;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

mod pair_signer {
    use super::*;
    use polkadot_sdk::sp_core::Pair as PairT;
    use polkadot_sdk::sp_runtime::{
        traits::{IdentifyAccount, Verify},
        AccountId32 as SpAccountId32, MultiSignature as SpMultiSignature,
    };
    use subxt::tx::Signer;

    impl<Pair> PairSigner<Pair>
    where
        Pair: PairT,
        <SpMultiSignature as Verify>::Signer: From<Pair::Public>,
    {
        /// Creates a new [`Signer`] from an [`sp_core::Pair`].
        pub fn new(signer: Pair) -> Self {
            let account_id =
                <SpMultiSignature as Verify>::Signer::from(signer.public()).into_account();
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
        pub fn account_id(&self) -> &subxt::config::substrate::AccountId32 {
            &self.account_id.0
        }
    }

    impl<Pair> Signer<PolkadotConfig> for PairSigner<Pair>
    where
        Pair: PairT,
        Pair::Signature: Into<<PolkadotConfig as Config>::Signature>,
    {
        fn account_id(&self) -> <PolkadotConfig as Config>::AccountId {
            self.account_id.0.clone()
        }

        fn address(&self) -> <PolkadotConfig as Config>::Address {
            self.account_id.0.clone().into()
        }

        fn sign(&self, signer_payload: &[u8]) -> <PolkadotConfig as Config>::Signature {
            self.signer.sign(signer_payload).into()
        }
    }
}

mod other_impls {
    use super::*;
    use polkadot_sdk::sp_runtime;

    impl<T: sp_runtime::traits::Header> Header for T
    where
        <T as sp_runtime::traits::Header>::Number: Into<u64>,
    {
        type Number = T::Number;
        type Hasher = T::Hashing;

        fn number(&self) -> Self::Number {
            *self.number()
        }
    }

    impl<T: sp_runtime::traits::Hash> Hasher for T {
        type Output = T::Output;

        fn hash(s: &[u8]) -> Self::Output {
            <T as sp_runtime::traits::Hash>::hash(s)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let signer: PairSigner<_> = {
        let acc = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
        PairSigner::new(acc)
    };

    let alice = signer.account_id().clone().into();

    // Build a balance transfer extrinsic.
    let balance_transfer_tx = polkadot::tx()
        .balances()
        .transfer_allow_death(alice, 10_000);

    /*

    This doesn't compile...

    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Find a Transfer event and print it.
    let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }

    */

    Ok(())
}
