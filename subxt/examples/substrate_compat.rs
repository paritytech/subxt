#![allow(missing_docs, unused)]

use codec::{Decode, Encode};
use polkadot_sdk::sp_core::{self, sr25519, Pair as PairT};
use polkadot_sdk::sp_runtime::{
    self,
    traits::{IdentifyAccount, Verify},
    AccountId32 as SpAccountId32, MultiSignature as SpMultiSignature,
};
use subxt::config::PolkadotExtrinsicParams;
use subxt::{Config, OnlineClient};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

pub enum PolkadotConfig {}

/// AccountId32 for substrate compatibility.
#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    Debug,
    scale_encode::EncodeAsType,
    scale_decode::DecodeAsType,
    scale_info::TypeInfo,
)]
pub struct AccountId32(subxt::config::substrate::AccountId32);

impl From<sp_runtime::AccountId32> for AccountId32 {
    fn from(acc: sp_runtime::AccountId32) -> Self {
        Self(subxt::config::substrate::AccountId32(acc.into()))
    }
}

/// MultiAddress type for substrate compatibility.
#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    Debug,
    scale_encode::EncodeAsType,
    scale_decode::DecodeAsType,
    scale_info::TypeInfo,
)]
pub struct MultiAddress<AccountId, AccountIndex>(
    subxt::config::substrate::MultiAddress<AccountId, AccountIndex>,
);

impl<AccountId, AccountIndex> From<AccountId> for MultiAddress<AccountId, AccountIndex> {
    fn from(a: AccountId) -> Self {
        Self(subxt::config::substrate::MultiAddress::Id(a))
    }
}

/// MultiAddress type for substrate compatibility.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug, scale_info::TypeInfo)]
pub struct MultiSignature(subxt::config::substrate::MultiSignature);

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

impl subxt::Config for PolkadotConfig {
    type Hash = <subxt::PolkadotConfig as Config>::Hash;
    type AccountId = AccountId32;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Signature = MultiSignature;
    type Hasher = <subxt::PolkadotConfig as Config>::Hasher;
    type Header = <subxt::PolkadotConfig as Config>::Header;
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;
    type AssetId = u32;
}

/// A [`Signer`] implementation that can be constructed from an [`sp_core::Pair`].
#[derive(Clone, Debug)]
pub struct PairSigner<T: Config, Pair> {
    account_id: T::AccountId,
    signer: Pair,
}

impl<T, Pair> PairSigner<T, Pair>
where
    T: Config,
    Pair: PairT,
    // We go via an `sp_runtime::MultiSignature`. We can probably generalise this
    // by implementing some of these traits on our built-in MultiSignature and then
    // requiring them on all T::Signatures, to avoid any go-between.
    <SpMultiSignature as Verify>::Signer: From<Pair::Public>,
    T::AccountId: From<SpAccountId32>,
{
    /// Creates a new [`Signer`] from an [`sp_core::Pair`].
    pub fn new(signer: Pair) -> Self {
        let account_id = <SpMultiSignature as Verify>::Signer::from(signer.public()).into_account();
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

impl<T, Pair> subxt::tx::Signer<T> for PairSigner<T, Pair>
where
    T: Config,
    Pair: PairT,
    Pair::Signature: Into<T::Signature>,
{
    fn account_id(&self) -> T::AccountId {
        self.account_id.clone()
    }

    fn address(&self) -> T::Address {
        self.account_id.clone().into()
    }

    fn sign(&self, signer_payload: &[u8]) -> T::Signature {
        self.signer.sign(signer_payload).into()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let signer: PairSigner<PolkadotConfig, _> = {
        let acc = sr25519::Pair::from_string("//Alice", None)?;
        PairSigner::new(acc)
    };

    let dest = subxt_signer::sr25519::dev::bob().public_key().into();

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
