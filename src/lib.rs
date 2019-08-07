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
};
use substrate_primitives::{
    storage::StorageKey,
    Pair,
};
use url::Url;

pub use error::Error;

mod error;
mod metadata;
mod rpc;

pub use rpc::RpcTypes;

/// Captures data for when an extrinsic is successfully included in a block
#[derive(Debug)]
pub struct ExtrinsicSuccess<T: RpcTypes> {
    pub block: T::Hash,
    pub extrinsic: T::Hash,
    pub events: Vec<T::Event>,
}

fn connect<T: RpcTypes>(
    url: &Url,
) -> impl Future<Item = rpc::Rpc<T>, Error = error::Error> {
    ws::connect(url.as_str())
        .expect("Url is a valid url; qed")
        .map_err(Into::into)
}

pub struct ClientBuilder<T: RpcTypes> {
    _marker: std::marker::PhantomData<T>,
    url: Option<Url>,
}

impl<T: RpcTypes> ClientBuilder<T> {
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

    pub fn build(self) -> impl Future<Item = Client<T>, Error = error::Error> {
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

#[derive(Clone)]
pub struct Client<T: RpcTypes> {
    url: Url,
    genesis_hash: T::Hash,
    metadata: metadata::Metadata,
}

impl<T: RpcTypes> Client<T> {
    fn connect(&self) -> impl Future<Item = rpc::Rpc<T>, Error = error::Error> {
        connect(&self.url)
    }

    pub fn metadata(&self) -> &metadata::Metadata {
        &self.metadata
    }

    pub fn fetch<V: Decode>(
        &self,
        key: StorageKey,
    ) -> impl Future<Item = Option<V>, Error = error::Error> {
        self.connect().and_then(|rpc| rpc.storage::<V>(key))
    }

    pub fn fetch_or<V: Decode>(
        &self,
        key: StorageKey,
        default: V,
    ) -> impl Future<Item = V, Error = error::Error> {
        self.fetch(key).map(|value| value.unwrap_or(default))
    }

    pub fn fetch_or_default<V: Decode + Default>(
        &self,
        key: StorageKey,
    ) -> impl Future<Item = V, Error = error::Error> {
        self.fetch(key).map(|value| value.unwrap_or_default())
    }

    pub fn xt<P, E>(
        &self,
        signer: P,
        extra: E,
    ) -> impl Future<Item = XtBuilder<T, P, E>, Error = error::Error>
    where
        P: Pair,
        P::Public: Into<T::AccountId>,
        P::Signature: Codec,
        E: Fn(T::Index) -> T::SignedExtension,
    {
        let account_id: T::AccountId = signer.public().into();
        let account_nonce_key = self
            .metadata
            .module("System")
            .expect("srml_system is present")
            .storage("AccountNonce")
            .expect("srml_system has account nonce")
            .map()
            .expect("account nonce is a map")
            .key(&account_id);
        let client = (*self).clone();
        self.fetch_or_default(account_nonce_key).map(|nonce| {
            XtBuilder {
                client,
                nonce,
                signer,
                extra,
            }
        })
    }
}

pub struct XtBuilder<T: RpcTypes, P, E> {
    client: Client<T>,
    nonce: T::Index,
    signer: P,
    extra: E,
}

impl<T: RpcTypes, P, E> XtBuilder<T, P, E>
where
    P: Pair,
    P::Public: Into<T::AccountId>,
    P::Signature: Codec,
    E: Fn(T::Index) -> T::SignedExtension,
{
    pub fn submit<C: Codec + Send>(
        &self,
        call: C,
    ) -> impl Future<Item = ExtrinsicSuccess<T>, Error = error::Error> {
        let signer = self.signer.clone();
        let nonce = self.nonce.clone();
        let extra = (self.extra)(nonce.clone());
        let genesis_hash = self.client.genesis_hash.clone();
        self.client.connect().and_then(move |rpc| {
            rpc.create_and_submit_extrinsic(signer, call, extra, nonce, genesis_hash)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parity_codec::Encode;
    use runtime_primitives::generic::Era;
    use runtime_support::StorageMap;
    use substrate_primitives::{
        blake2_256,
        storage::StorageKey,
        Pair,
    };

    #[derive(Clone)]
    struct Runtime;
    impl super::RpcTypes for Runtime {
        type AccountId = <node_runtime::Runtime as srml_system::Trait>::AccountId;
        type BlockNumber = <node_runtime::Runtime as srml_system::Trait>::BlockNumber;
        type Event = <node_runtime::Runtime as srml_system::Trait>::Event;
        type Hash = <node_runtime::Runtime as srml_system::Trait>::Hash;
        type Hashing = <node_runtime::Runtime as srml_system::Trait>::Hashing;
        type Index = <node_runtime::Runtime as srml_system::Trait>::Index;
        type SignedExtension = node_runtime::SignedExtra;
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_balance_transfer() {
        env_logger::try_init().ok();
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let client = rt
            .block_on(ClientBuilder::<Runtime>::new().build())
            .unwrap();

        let signer = substrate_keyring::AccountKeyring::Alice.pair();
        let extra = |nonce| {
            (
                srml_system::CheckGenesis::<node_runtime::Runtime>::new(),
                srml_system::CheckEra::<node_runtime::Runtime>::from(Era::Immortal),
                srml_system::CheckNonce::<node_runtime::Runtime>::from(nonce),
                srml_system::CheckWeight::<node_runtime::Runtime>::new(),
                srml_balances::TakeFees::<node_runtime::Runtime>::from(0),
            )
        };
        let xt = rt.block_on(client.xt(signer, extra)).unwrap();

        let dest = substrate_keyring::AccountKeyring::Bob.pair().public();
        let transfer =
            srml_balances::Call::transfer::<node_runtime::Runtime>(dest.into(), 10_000);
        let call = client.metadata().module("Balances").unwrap().call(transfer);
        rt.block_on(xt.submit(call)).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_fetch_account_balance() {
        env_logger::try_init().ok();
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let client = rt
            .block_on(ClientBuilder::<Runtime>::new().build())
            .unwrap();

        let account: <Runtime as RpcTypes>::AccountId =
            substrate_keyring::AccountKeyring::Alice
                .pair()
                .public()
                .into();
        let key = client
            .metadata()
            .module("Balances")
            .unwrap()
            .storage("FreeBalance")
            .unwrap()
            .map()
            .unwrap()
            .key(&account);
        type Balance = <node_runtime::Runtime as srml_balances::Trait>::Balance;
        rt.block_on(client.fetch::<Balance>(key)).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_fetch_metadata() {
        env_logger::try_init().ok();
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let client = rt
            .block_on(ClientBuilder::<Runtime>::new().build())
            .unwrap();

        let balances = client.metadata().module("Balances").unwrap();

        let dest = substrate_keyring::AccountKeyring::Bob.pair().public();
        let transfer = srml_balances::Call::transfer(dest.clone().into(), 10_000);
        let call = node_runtime::Call::Balances(transfer.clone())
            .encode()
            .to_vec();
        let call2 = balances.call(transfer);
        assert_eq!(call, call2);

        let free_balance =
            <srml_balances::FreeBalance<node_runtime::Runtime>>::key_for(&dest);
        let free_balance_key = StorageKey(blake2_256(&free_balance).to_vec());
        let free_balance_key2 = balances
            .storage("FreeBalance")
            .unwrap()
            .map()
            .unwrap()
            .key(&dest);
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
            .map()
            .unwrap()
            .key(&dest);
        assert_eq!(account_nonce_key, account_nonce_key2);
    }
}
