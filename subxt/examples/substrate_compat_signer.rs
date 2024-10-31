//! This example demonstrates how to use to add a layer for full functionality for types in substrate.
//!
//! Similar functionality was provided by the `substrate-compat` feature in the original `subxt` crate.
//! which is now removed.

#![allow(missing_docs)]

use polkadot_sdk::sp_core::{sr25519, Pair as _};
use subxt::{Config, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

/// A concrete PairSigner implementation which relies on `sr25519::Pair` for signing
/// and that PolkadotConfig is the runtime configuration.
mod pair_signer {
    use super::*;
    use polkadot_sdk::sp_runtime::{
        traits::{IdentifyAccount, Verify},
        MultiSignature as SpMultiSignature,
    };
    use subxt::{
        config::substrate::{AccountId32, MultiSignature},
        tx::Signer,
    };

    /// A [`Signer`] implementation for `polkadot_sdk::sp_core::sr25519::Pair`.
    #[derive(Clone)]
    pub struct PairSigner {
        account_id: <PolkadotConfig as Config>::AccountId,
        signer: sr25519::Pair,
    }

    impl PairSigner {
        /// Creates a new [`Signer`] from an [`sp_core::Pair`].
        pub fn new(signer: sr25519::Pair) -> Self {
            let account_id =
                <SpMultiSignature as Verify>::Signer::from(signer.public()).into_account();
            Self {
                // Convert `sp_core::AccountId32` to `subxt::config::substrate::AccountId32`.
                account_id: AccountId32(account_id.into()),
                signer,
            }
        }

        /// Returns the [`sp_core::sr25519::Pair`] implementation used to construct this.
        pub fn signer(&self) -> &sr25519::Pair {
            &self.signer
        }

        /// Return the account ID.
        pub fn account_id(&self) -> &AccountId32 {
            &self.account_id
        }
    }

    impl Signer<PolkadotConfig> for PairSigner {
        fn account_id(&self) -> <PolkadotConfig as Config>::AccountId {
            self.account_id.clone()
        }

        fn address(&self) -> <PolkadotConfig as Config>::Address {
            self.account_id.clone().into()
        }

        fn sign(&self, signer_payload: &[u8]) -> <PolkadotConfig as Config>::Signature {
            let signature = self.signer.sign(signer_payload);
            MultiSignature::Sr25519(signature.0)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let signer = {
        let acc = sr25519::Pair::from_string("//Alice", None).unwrap();
        pair_signer::PairSigner::new(acc)
    };

    let alice = signer.account_id().clone().into();

    // Build a balance transfer extrinsic.
    let balance_transfer_tx = polkadot::tx()
        .balances()
        .transfer_allow_death(alice, 10_000);

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

    Ok(())
}
