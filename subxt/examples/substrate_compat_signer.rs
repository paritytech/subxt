//! This example demonstrates how to use to add a custom signer implementation to `subxt`
//! by using the signer implementation from polkadot-sdk.
//!
//! Similar functionality was provided by the `substrate-compat` feature in the original `subxt` crate.
//! which is now removed.

#![allow(missing_docs, unused)]

use sp_core::{Pair as _, sr25519};
use subxt::config::substrate::MultiAddress;
use subxt::{Config, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

/// A concrete PairSigner implementation which relies on `sr25519::Pair` for signing
/// and that PolkadotConfig is the runtime configuration.
mod pair_signer {
    use super::*;
    use sp_runtime::{
        MultiSignature as SpMultiSignature,
        traits::{IdentifyAccount, Verify},
    };
    use subxt::{
        config::substrate::{AccountId32, MultiSignature},
        tx::Signer,
    };

    /// A [`Signer`] implementation for [`sp_core::sr25519::Pair`].
    #[derive(Clone)]
    pub struct PairSigner {
        account_id: <PolkadotConfig as Config>::AccountId,
        signer: sr25519::Pair,
    }

    impl PairSigner {
        /// Creates a new [`Signer`] from an [`sp_core::sr25519::Pair`].
        pub fn new(signer: sr25519::Pair) -> Self {
            let account_id =
                <SpMultiSignature as Verify>::Signer::from(signer.public()).into_account();
            Self {
                // Convert `sp_core::AccountId32` to `subxt::config::substrate::AccountId32`.
                //
                // This is necessary because we use `subxt::config::substrate::AccountId32` and no
                // From/Into impls are provided between `sp_core::AccountId32` because `polkadot-sdk` isn't a direct
                // dependency in subxt.
                //
                // This can also be done by provided a wrapper type around `subxt::config::substrate::AccountId32` to implement
                // such conversions but that also most likely requires a custom `Config` with a separate `AccountId` type to work
                // properly without additional hacks.
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

        fn sign(&self, signer_payload: &[u8]) -> <PolkadotConfig as Config>::Signature {
            let signature = self.signer.sign(signer_payload);
            MultiSignature::Sr25519(signature.0)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let signer = {
        let acc = sr25519::Pair::from_string("//Alice", None)?;
        pair_signer::PairSigner::new(acc)
    };

    let dest = {
        let acc = sr25519::Pair::from_string("//Bob", None)?;
        MultiAddress::Address32(acc.public().0)
    };

    // Build a balance transfer extrinsic.
    let balance_transfer_tx = polkadot::tx()
        .balances()
        .transfer_allow_death(dest, 100_000);

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
