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

use futures::future::Future;
use jsonrpc_core_client::transports::ws;
use metadata::Metadata;
use parity_scale_codec::{
    Codec,
    Decode,
    Encode,
};
use runtime_primitives::traits::StaticLookup;
use substrate_primitives::{
    storage::StorageKey,
    Pair,
};
use url::Url;

pub use error::Error;
use srml::system::{
    System,
    SystemStore,
};

mod codec;
mod error;
mod metadata;
mod rpc;
pub mod srml;

/// Captures data for when an extrinsic is successfully included in a block
#[derive(Debug)]
pub struct ExtrinsicSuccess<T: System> {
    pub block: T::Hash,
    pub extrinsic: T::Hash,
    pub events: Vec<T::Event>,
}

fn connect<T: System>(url: &Url) -> impl Future<Item = rpc::Rpc<T>, Error = Error> {
    ws::connect(url.as_str())
        .expect("Url is a valid url; qed")
        .map_err(Into::into)
}

pub struct ClientBuilder<T: System> {
    _marker: std::marker::PhantomData<T>,
    url: Option<Url>,
}

impl<T: System> ClientBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
            url: None,
        }
    }

    pub fn set_url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    pub fn build(self) -> impl Future<Item = Client<T>, Error = Error> {
        let url = self.url.unwrap_or_else(|| {
            Url::parse("ws://127.0.0.1:9944").expect("Is valid url; qed")
        });
        connect::<T>(&url).and_then(|rpc| {
            rpc.metadata()
                .join(rpc.genesis_hash())
                .map(|(metadata, genesis_hash)| {
                    Client {
                        url,
                        genesis_hash,
                        metadata,
                    }
                })
        })
    }
}

pub struct Client<T: System> {
    url: Url,
    genesis_hash: T::Hash,
    metadata: Metadata,
}

impl<T: System> Clone for Client<T> {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            genesis_hash: self.genesis_hash.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<T: System + 'static> Client<T> {
    fn connect(&self) -> impl Future<Item = rpc::Rpc<T>, Error = Error> {
        connect(&self.url)
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn fetch<V: Decode>(
        &self,
        key: StorageKey,
    ) -> impl Future<Item = Option<V>, Error = Error> {
        self.connect().and_then(|rpc| rpc.storage::<V>(key))
    }

    pub fn fetch_or<V: Decode>(
        &self,
        key: StorageKey,
        default: V,
    ) -> impl Future<Item = V, Error = Error> {
        self.fetch(key).map(|value| value.unwrap_or(default))
    }

    pub fn fetch_or_default<V: Decode + Default>(
        &self,
        key: StorageKey,
    ) -> impl Future<Item = V, Error = Error> {
        self.fetch(key).map(|value| value.unwrap_or_default())
    }

    pub fn xt<P>(
        &self,
        signer: P,
        nonce: Option<T::Index>,
    ) -> impl Future<Item = XtBuilder<T, P>, Error = Error>
    where
        P: Pair,
        P::Public: Into<T::AccountId> + Into<<T::Lookup as StaticLookup>::Source>,
        P::Signature: Codec,
    {
        let client = self.clone();
        match nonce {
            Some(nonce) => futures::future::Either::A(futures::future::ok(nonce)),
            None => {
                futures::future::Either::B(self.account_nonce(signer.public().into()))
            }
        }
        .map(|nonce| {
            XtBuilder {
                client,
                nonce,
                signer,
            }
        })
    }
}

pub struct XtBuilder<T: System, P> {
    client: Client<T>,
    nonce: T::Index,
    signer: P,
}

impl<T: System + 'static, P> XtBuilder<T, P>
where
    P: Pair,
    P::Public: Into<<T::Lookup as StaticLookup>::Source>,
    P::Signature: Codec,
{
    pub fn metadata(&self) -> &Metadata {
        self.client.metadata()
    }

    pub fn set_nonce(&mut self, nonce: T::Index) {
        self.nonce = nonce;
    }

    pub fn submit<C: Encode + Send>(
        &mut self,
        call: C,
    ) -> impl Future<Item = T::Hash, Error = Error> {
        let signer = self.signer.clone();
        let nonce = self.nonce.clone();
        let genesis_hash = self.client.genesis_hash.clone();
        self.set_nonce(nonce + 1.into());
        self.client.connect().and_then(move |rpc| {
            rpc.create_and_submit_extrinsic(signer, call, nonce, genesis_hash)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::srml::balances::{
        Balances,
        BalancesCalls,
        BalancesStore,
    };
    use parity_scale_codec::Encode;
    use runtime_primitives::generic::Era;
    use runtime_support::StorageMap;
    use substrate_keyring::AccountKeyring;
    use substrate_primitives::{
        blake2_256,
        storage::StorageKey,
        Pair,
    };

    struct Runtime;

    impl System for Runtime {
        type Index = <node_runtime::Runtime as srml_system::Trait>::Index;
        type BlockNumber = <node_runtime::Runtime as srml_system::Trait>::BlockNumber;
        type Hash = <node_runtime::Runtime as srml_system::Trait>::Hash;
        type Hashing = <node_runtime::Runtime as srml_system::Trait>::Hashing;
        type AccountId = <node_runtime::Runtime as srml_system::Trait>::AccountId;
        type Lookup = <node_runtime::Runtime as srml_system::Trait>::Lookup;
        type Event = <node_runtime::Runtime as srml_system::Trait>::Event;

        type SignedExtra = (
            srml_system::CheckGenesis<node_runtime::Runtime>,
            srml_system::CheckEra<node_runtime::Runtime>,
            srml_system::CheckNonce<node_runtime::Runtime>,
            srml_system::CheckWeight<node_runtime::Runtime>,
            srml_balances::TakeFees<node_runtime::Runtime>,
        );
        fn extra(nonce: Self::Index) -> Self::SignedExtra {
            (
                srml_system::CheckGenesis::<node_runtime::Runtime>::new(),
                srml_system::CheckEra::<node_runtime::Runtime>::from(Era::Immortal),
                srml_system::CheckNonce::<node_runtime::Runtime>::from(nonce),
                srml_system::CheckWeight::<node_runtime::Runtime>::new(),
                srml_balances::TakeFees::<node_runtime::Runtime>::from(0),
            )
        }
    }

    impl Balances for Runtime {
        type Balance = <node_runtime::Runtime as srml_balances::Trait>::Balance;
    }

    type Index = <Runtime as System>::Index;
    type AccountId = <Runtime as System>::AccountId;
    type Address = <<Runtime as System>::Lookup as StaticLookup>::Source;
    type Balance = <Runtime as Balances>::Balance;

    fn test_setup() -> (tokio::runtime::Runtime, Client<Runtime>) {
        env_logger::try_init().ok();
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let client_future = ClientBuilder::<Runtime>::new().build();
        let client = rt.block_on(client_future).unwrap();
        (rt, client)
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_balance_transfer() {
        let (mut rt, client) = test_setup();

        let signer = AccountKeyring::Alice.pair();
        let mut xt = rt.block_on(client.xt(signer, None)).unwrap();

        let dest = AccountKeyring::Bob.pair().public();
        rt.block_on(xt.transfer(dest.into(), 10_000)).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_fetch_account_balance() {
        let (mut rt, client) = test_setup();

        let account = AccountKeyring::Alice.pair().public();
        rt.block_on(client.free_balance(account.into())).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_fetch_metadata() {
        let (_, client) = test_setup();

        let balances = client.metadata().module("Balances").unwrap();

        let dest = substrate_keyring::AccountKeyring::Bob.pair().public();
        let address: Address = dest.clone().into();
        let amount: Balance = 10_000;

        let transfer = srml_balances::Call::transfer(address.clone(), amount);
        let call = node_runtime::Call::Balances(transfer);
        let call2 = balances
            .call("transfer", (address, codec::compact(amount)))
            .unwrap();
        assert_eq!(call.encode().to_vec(), call2.0);

        let free_balance =
            <srml_balances::FreeBalance<node_runtime::Runtime>>::key_for(&dest);
        let free_balance_key = StorageKey(blake2_256(&free_balance).to_vec());
        let free_balance_key2 = balances
            .storage("FreeBalance")
            .unwrap()
            .get_map::<AccountId, Balance>()
            .unwrap()
            .key(dest.clone());
        assert_eq!(free_balance_key, free_balance_key2);

        let account_nonce =
            <srml_system::AccountNonce<node_runtime::Runtime>>::key_for(&dest);
        let account_nonce_key = StorageKey(blake2_256(&account_nonce).to_vec());
        let account_nonce_key2 = client
            .metadata()
            .module("System")
            .unwrap()
            .storage("AccountNonce")
            .unwrap()
            .get_map::<AccountId, Index>()
            .unwrap()
            .key(dest);
        assert_eq!(account_nonce_key, account_nonce_key2);
    }
}
