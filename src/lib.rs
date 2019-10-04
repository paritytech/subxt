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

//! A library to **sub**mit e**xt**rinsics to a
//! [substrate](https://github.com/paritytech/substrate) node via RPC.

#![deny(missing_docs)]
#![deny(warnings)]

use futures::future::{
    self,
    Either,
    Future,
    IntoFuture,
};
use jsonrpc_core_client::transports::ws;
use metadata::Metadata;
use parity_scale_codec::{
    Codec,
    Decode,
};
use runtime_primitives::generic::UncheckedExtrinsic;
use sr_version::RuntimeVersion;
use std::{
    convert::TryFrom,
    marker::PhantomData,
};
use substrate_primitives::{
    storage::{
        StorageChangeSet,
        StorageKey,
    },
    Pair,
};
use url::Url;

use crate::{
    codec::Encoded,
    events::EventsDecoder,
    extrinsic::{
        DefaultExtra,
        SignedExtra,
    },
    metadata::MetadataError,
    rpc::{
        BlockNumber,
        ChainBlock,
        MapStream,
        Rpc,
    },
    srml::{
        balances::Balances,
        system::{
            System,
            SystemEvent,
            SystemStore,
        },
        ModuleCalls,
    },
};

mod codec;
mod error;
mod events;
mod extrinsic;
mod metadata;
mod rpc;
mod runtimes;
mod srml;

pub use error::Error;
pub use events::RawEvent;
pub use rpc::ExtrinsicSuccess;
pub use runtimes::*;
pub use srml::*;

fn connect<T: System>(url: &Url) -> impl Future<Item = Rpc<T>, Error = Error> {
    ws::connect(url).map_err(Into::into)
}

/// ClientBuilder for constructing a Client.
pub struct ClientBuilder<T: System> {
    _marker: std::marker::PhantomData<T>,
    url: Option<Url>,
}

impl<T: System> ClientBuilder<T> {
    /// Creates a new ClientBuilder.
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
            url: None,
        }
    }

    /// Set the substrate rpc address.
    pub fn set_url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    /// Creates a new Client.
    pub fn build(self) -> impl Future<Item = Client<T>, Error = Error> {
        let url = self.url.unwrap_or_else(|| {
            Url::parse("ws://127.0.0.1:9944").expect("Is valid url; qed")
        });
        connect::<T>(&url).and_then(|rpc| {
            rpc.metadata()
                .join3(rpc.genesis_hash(), rpc.runtime_version(None))
                .map(|(metadata, genesis_hash, runtime_version)| {
                    Client {
                        url,
                        genesis_hash,
                        metadata,
                        runtime_version,
                    }
                })
        })
    }
}

/// Client to interface with a substrate node.
pub struct Client<T: System> {
    url: Url,
    genesis_hash: T::Hash,
    metadata: Metadata,
    runtime_version: RuntimeVersion,
}

impl<T: System> Clone for Client<T> {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            genesis_hash: self.genesis_hash.clone(),
            metadata: self.metadata.clone(),
            runtime_version: self.runtime_version.clone(),
        }
    }
}

impl<T: System + Balances + 'static> Client<T> {
    fn connect(&self) -> impl Future<Item = Rpc<T>, Error = Error> {
        connect(&self.url)
    }

    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Fetch a StorageKey.
    pub fn fetch<V: Decode>(
        &self,
        key: StorageKey,
    ) -> impl Future<Item = Option<V>, Error = Error> {
        self.connect().and_then(|rpc| rpc.storage::<V>(key))
    }

    /// Fetch a StorageKey or return the default.
    pub fn fetch_or<V: Decode>(
        &self,
        key: StorageKey,
        default: V,
    ) -> impl Future<Item = V, Error = Error> {
        self.fetch(key).map(|value| value.unwrap_or(default))
    }

    /// Fetch a StorageKey or return the default.
    pub fn fetch_or_default<V: Decode + Default>(
        &self,
        key: StorageKey,
    ) -> impl Future<Item = V, Error = Error> {
        self.fetch(key).map(|value| value.unwrap_or_default())
    }

    /// Get a block hash. By default returns the latest block hash
    pub fn block_hash(
        &self,
        hash: Option<BlockNumber<T>>,
    ) -> impl Future<Item = Option<T::Hash>, Error = Error> {
        self.connect()
            .and_then(|rpc| rpc.block_hash(hash.map(|h| h)))
    }

    /// Get a block
    pub fn block<H>(
        &self,
        hash: Option<H>,
    ) -> impl Future<Item = Option<ChainBlock<T>>, Error = Error>
    where
        H: Into<T::Hash> + 'static,
    {
        self.connect()
            .and_then(|rpc| rpc.block(hash.map(|h| h.into())))
    }

    /// Subscribe to events.
    pub fn subscribe_events(
        &self,
    ) -> impl Future<Item = MapStream<StorageChangeSet<T::Hash>>, Error = Error> {
        self.connect().and_then(|rpc| rpc.subscribe_events())
    }

    /// Subscribe to new blocks.
    pub fn subscribe_blocks(
        &self,
    ) -> impl Future<Item = MapStream<T::Header>, Error = Error> {
        self.connect().and_then(|rpc| rpc.subscribe_blocks())
    }

    /// Subscribe to finalized blocks.
    pub fn subscribe_finalized_blocks(
        &self,
    ) -> impl Future<Item = MapStream<T::Header>, Error = Error> {
        self.connect()
            .and_then(|rpc| rpc.subscribe_finalized_blocks())
    }

    /// Create a transaction builder for a private key.
    pub fn xt<P>(
        &self,
        signer: P,
        nonce: Option<T::Index>,
    ) -> impl Future<Item = XtBuilder<T, P>, Error = Error>
    where
        P: Pair,
        P::Public: Into<T::AccountId> + Into<T::Address>,
        P::Signature: Codec,
    {
        let client = self.clone();
        match nonce {
            Some(nonce) => Either::A(future::ok(nonce)),
            None => Either::B(self.account_nonce(signer.public().into())),
        }
        .map(|nonce| {
            let genesis_hash = client.genesis_hash.clone();
            let runtime_version = client.runtime_version.clone();
            XtBuilder {
                client,
                nonce,
                runtime_version,
                genesis_hash,
                signer,
                call: None,
                marker: PhantomData,
            }
        })
    }
}

/// The extrinsic builder is ready to finalize construction.
pub enum Valid {}
/// The extrinsic builder is not ready to finalize construction.
pub enum Invalid {}

/// Transaction builder.
pub struct XtBuilder<T: System, P, V = Invalid> {
    client: Client<T>,
    nonce: T::Index,
    runtime_version: RuntimeVersion,
    genesis_hash: T::Hash,
    signer: P,
    call: Option<Result<Encoded, MetadataError>>,
    marker: PhantomData<fn() -> V>,
}

impl<T: System + Balances + 'static, P, V> XtBuilder<T, P, V>
where
    P: Pair,
{
    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        self.client.metadata()
    }

    /// Returns the nonce.
    pub fn nonce(&self) -> T::Index {
        self.nonce.clone()
    }

    /// Sets the nonce to a new value.
    pub fn set_nonce(&mut self, nonce: T::Index) -> &mut XtBuilder<T, P, V> {
        self.nonce = nonce;
        self
    }

    /// Increment the nonce
    pub fn increment_nonce(&mut self) -> &mut XtBuilder<T, P, V> {
        self.set_nonce(self.nonce() + 1.into());
        self
    }

    /// Sets the module call to a new value
    pub fn set_call<F>(&self, module: &'static str, f: F) -> XtBuilder<T, P, Valid>
    where
        F: FnOnce(ModuleCalls<T, P>) -> Result<Encoded, MetadataError>,
    {
        let call = self
            .metadata()
            .module(module)
            .and_then(|module| f(ModuleCalls::new(module)))
            .map_err(Into::into);

        XtBuilder {
            client: self.client.clone(),
            nonce: self.nonce.clone(),
            runtime_version: self.runtime_version.clone(),
            genesis_hash: self.genesis_hash.clone(),
            signer: self.signer.clone(),
            call: Some(call),
            marker: PhantomData,
        }
    }
}

impl<T: System + Balances + Send + Sync + 'static, P> XtBuilder<T, P, Valid>
where
    P: Pair,
    P::Public: Into<T::Address>,
    P::Signature: Codec,
{
    /// Creates and signs an Extrinsic for the supplied `Call`
    pub fn create_and_sign(
        &self,
    ) -> Result<
        UncheckedExtrinsic<
            T::Address,
            Encoded,
            P::Signature,
            <DefaultExtra<T> as SignedExtra<T>>::Extra,
        >,
        Error,
    >
    where
        P: Pair,
        P::Public: Into<T::Address>,
        P::Signature: Codec,
    {
        let signer = self.signer.clone();
        let account_nonce = self.nonce.clone();
        let version = self.runtime_version.spec_version;
        let genesis_hash = self.genesis_hash.clone();
        let call = self
            .call
            .clone()
            .expect("A Valid extrinisic builder has a call; qed")?;

        log::info!(
            "Creating Extrinsic with genesis hash {:?} and account nonce {:?}",
            genesis_hash,
            account_nonce
        );

        let extra = extrinsic::DefaultExtra::new(version, account_nonce, genesis_hash);
        let xt = extrinsic::create_and_sign(signer, call, extra)?;
        Ok(xt)
    }

    /// Submits a transaction to the chain.
    pub fn submit(&self) -> impl Future<Item = T::Hash, Error = Error> {
        let cli = self.client.connect();
        self.create_and_sign()
            .into_future()
            .map_err(Into::into)
            .and_then(move |extrinsic| {
                cli.and_then(move |rpc| rpc.submit_extrinsic(extrinsic))
            })
    }

    /// Submits transaction to the chain and watch for events.
    pub fn submit_and_watch(
        &self,
    ) -> impl Future<Item = ExtrinsicSuccess<T>, Error = Error> {
        let cli = self.client.connect();
        let metadata = self.client.metadata().clone();
        let decoder = EventsDecoder::try_from(metadata)
            .into_future()
            .map_err(Into::into);

        self.create_and_sign()
            .into_future()
            .map_err(Into::into)
            .join(decoder)
            .and_then(move |(extrinsic, decoder)| {
                cli.and_then(move |rpc| {
                    rpc.submit_and_watch_extrinsic(extrinsic, decoder)
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::srml::{
        balances::{
            Balances,
            BalancesStore,
            BalancesXt,
        },
        contracts::ContractsXt,
    };
    use futures::stream::Stream;
    use node_runtime::Runtime;
    use parity_scale_codec::Encode;
    use runtime_support::StorageMap;
    use substrate_keyring::AccountKeyring;
    use substrate_primitives::{
        storage::StorageKey,
        Pair,
    };

    type Index = <Runtime as System>::Index;
    type AccountId = <Runtime as System>::AccountId;
    type Address = <Runtime as System>::Address;
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
    fn test_tx_transfer_balance() {
        let (mut rt, client) = test_setup();

        let signer = AccountKeyring::Alice.pair();
        let mut xt = rt.block_on(client.xt(signer, None)).unwrap();

        let dest = AccountKeyring::Bob.pair().public();
        let transfer = xt
            .balances(|calls| calls.transfer(dest.clone().into(), 10_000))
            .submit();
        rt.block_on(transfer).unwrap();

        // check that nonce is handled correctly
        let transfer = xt
            .increment_nonce()
            .balances(|calls| calls.transfer(dest.clone().into(), 10_000))
            .submit();
        rt.block_on(transfer).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn test_tx_contract_put_code() {
        let (mut rt, client) = test_setup();

        let signer = AccountKeyring::Alice.pair();
        let xt = rt.block_on(client.xt(signer, None)).unwrap();

        const CONTRACT: &str = r#"
(module
    (func (export "call"))
    (func (export "deploy"))
)
"#;
        let wasm = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

        let put_code = xt
            .contracts(|call| call.put_code(500_000, wasm))
            .submit_and_watch();

        let success = rt
            .block_on(put_code)
            .expect("Extrinsic should be included in a block");

        let code_hash =
            success.find_event::<<Runtime as System>::Hash>("Contracts", "CodeStored");

        assert!(
            code_hash.is_some(),
            "Contracts CodeStored event should be present"
        );
        assert!(
            code_hash.unwrap().is_ok(),
            "CodeStored Hash should decode successfully"
        );
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn test_getting_hash() {
        let (mut rt, client) = test_setup();
        rt.block_on(client.block_hash(None)).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn test_getting_block() {
        let (mut rt, client) = test_setup();
        rt.block_on(client.block_hash(None).and_then(move |h| client.block(h)))
            .unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn test_state_read_free_balance() {
        let (mut rt, client) = test_setup();

        let account = AccountKeyring::Alice.pair().public();
        rt.block_on(client.free_balance(account.into())).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn test_chain_subscribe_blocks() {
        let (mut rt, client) = test_setup();

        let stream = rt.block_on(client.subscribe_blocks()).unwrap();
        let (_header, _) = rt
            .block_on(stream.into_future().map_err(|(e, _)| e))
            .unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn test_chain_subscribe_finalized_blocks() {
        let (mut rt, client) = test_setup();

        let stream = rt.block_on(client.subscribe_finalized_blocks()).unwrap();
        let (_header, _) = rt
            .block_on(stream.into_future().map_err(|(e, _)| e))
            .unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn test_chain_read_metadata() {
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
            <srml_balances::FreeBalance<node_runtime::Runtime>>::hashed_key_for(&dest);
        let free_balance_key = StorageKey(free_balance);
        let free_balance_key2 = balances
            .storage("FreeBalance")
            .unwrap()
            .get_map::<AccountId, Balance>()
            .unwrap()
            .key(dest.clone());
        assert_eq!(free_balance_key, free_balance_key2);

        let account_nonce =
            <srml_system::AccountNonce<node_runtime::Runtime>>::hashed_key_for(&dest);
        let account_nonce_key = StorageKey(account_nonce);
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
