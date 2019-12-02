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
#![allow(clippy::type_complexity)]

use std::{
    convert::TryFrom,
    marker::PhantomData,
};

use futures::future::{
    self,
    Either,
    Future,
    IntoFuture,
};
use jsonrpc_core_client::transports::ws;
use parity_scale_codec::{
    Codec,
    Decode,
    Encode,
};
use sp_runtime::{
    generic::UncheckedExtrinsic,
    traits::{
        IdentifyAccount,
        Verify,
    },
    MultiSignature,
};
use sp_version::RuntimeVersion;
use sp_core::{
    storage::{
        StorageChangeSet,
        StorageKey,
    },
    Pair,
};
use url::Url;

mod codec;
mod error;
mod events;
mod extrinsic;
mod frame;
mod metadata;
mod rpc;
mod runtimes;

use self::{
    codec::Encoded,
    events::EventsDecoder,
    extrinsic::{
        DefaultExtra,
        SignedExtra,
    },
    frame::{
        balances::Balances,
        system::{
            System,
            SystemEvent,
            SystemStore,
        },
    },
    metadata::Metadata,
    rpc::{
        BlockNumber,
        ChainBlock,
        MapStream,
        Rpc,
    },
};
pub use self::{
    error::Error,
    events::RawEvent,
    frame::*,
    rpc::ExtrinsicSuccess,
    runtimes::*,
};

fn connect<T: System>(url: &Url) -> impl Future<Item = Rpc<T>, Error = Error> {
    ws::connect(url).map_err(Into::into)
}

/// ClientBuilder for constructing a Client.
#[derive(Default)]
pub struct ClientBuilder<T: System, S = MultiSignature> {
    _marker: std::marker::PhantomData<(T, S)>,
    url: Option<Url>,
}

impl<T: System, S> ClientBuilder<T, S> {
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
    pub fn build(self) -> impl Future<Item = Client<T, S>, Error = Error> {
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
                        _marker: PhantomData,
                    }
                })
        })
    }
}

/// Client to interface with a substrate node.
pub struct Client<T: System, S = MultiSignature> {
    url: Url,
    genesis_hash: T::Hash,
    metadata: Metadata,
    runtime_version: RuntimeVersion,
    _marker: PhantomData<fn() -> S>,
}

impl<T: System, S> Clone for Client<T, S> {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            genesis_hash: self.genesis_hash,
            metadata: self.metadata.clone(),
            runtime_version: self.runtime_version.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: System + Balances + 'static, S: 'static> Client<T, S> {
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
    ) -> impl Future<Item = XtBuilder<T, P, S>, Error = Error>
    where
        P: Pair,
        P::Signature: Codec,
        S: Verify,
        S::Signer: From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
    {
        let client = self.clone();
        let account_id = S::Signer::from(signer.public()).into_account();
        match nonce {
            Some(nonce) => Either::A(future::ok(nonce)),
            None => Either::B(self.account_nonce(account_id)),
        }
        .map(|nonce| {
            let genesis_hash = client.genesis_hash;
            let runtime_version = client.runtime_version.clone();
            XtBuilder {
                client,
                nonce,
                runtime_version,
                genesis_hash,
                signer,
            }
        })
    }
}

/// Transaction builder.
pub struct XtBuilder<T: System, P, S> {
    client: Client<T, S>,
    nonce: T::Index,
    runtime_version: RuntimeVersion,
    genesis_hash: T::Hash,
    signer: P,
}

impl<T: System + Balances + 'static, P, S: 'static> XtBuilder<T, P, S>
where
    P: Pair,
{
    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        self.client.metadata()
    }

    /// Returns the nonce.
    pub fn nonce(&self) -> T::Index {
        self.nonce
    }

    /// Sets the nonce to a new value.
    pub fn set_nonce(&mut self, nonce: T::Index) -> &mut XtBuilder<T, P, S> {
        self.nonce = nonce;
        self
    }

    /// Increment the nonce
    pub fn increment_nonce(&mut self) -> &mut XtBuilder<T, P, S> {
        self.set_nonce(self.nonce() + 1.into());
        self
    }
}

impl<T: System + Balances + Send + Sync + 'static, P, S: 'static> XtBuilder<T, P, S>
where
    P: Pair,
    S: Verify + Codec + From<P::Signature>,
    S::Signer: From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
    T::Address: From<T::AccountId>,
{
    /// Creates and signs an Extrinsic for the supplied `Call`
    pub fn create_and_sign<C>(
        &self,
        call: Call<C>,
    ) -> Result<
        UncheckedExtrinsic<
            T::Address,
            Encoded,
            S,
            <DefaultExtra<T> as SignedExtra<T>>::Extra,
        >,
        Error,
    >
    where
        C: parity_scale_codec::Encode,
    {
        let signer = self.signer.clone();
        let account_nonce = self.nonce;
        let version = self.runtime_version.spec_version;
        let genesis_hash = self.genesis_hash;
        let call = self
            .metadata()
            .module_with_calls(&call.module)
            .and_then(|module| module.call(&call.function, call.args))?;

        log::info!(
            "Creating Extrinsic with genesis hash {:?} and account nonce {:?}",
            genesis_hash,
            account_nonce
        );

        let extra = extrinsic::DefaultExtra::new(version, account_nonce, genesis_hash);
        let xt = extrinsic::create_and_sign::<_, _, _, S, _>(signer, call, extra)?;
        Ok(xt)
    }

    /// Submits a transaction to the chain.
    pub fn submit<C: Encode>(
        &self,
        call: Call<C>,
    ) -> impl Future<Item = T::Hash, Error = Error> {
        let cli = self.client.connect();
        self.create_and_sign(call)
            .into_future()
            .map_err(Into::into)
            .and_then(move |extrinsic| {
                cli.and_then(move |rpc| rpc.submit_extrinsic(extrinsic))
            })
    }

    /// Submits transaction to the chain and watch for events.
    pub fn submit_and_watch<C: Encode>(
        &self,
        call: Call<C>,
    ) -> impl Future<Item = ExtrinsicSuccess<T>, Error = Error> {
        let cli = self.client.connect();
        let metadata = self.client.metadata().clone();
        let decoder = EventsDecoder::try_from(metadata)
            .into_future()
            .map_err(Into::into);

        self.create_and_sign(call)
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
    use futures::stream::Stream;
    use parity_scale_codec::Encode;
    use frame_support::StorageMap;
    use sp_keyring::AccountKeyring;
    use sp_core::storage::StorageKey;

    use super::*;
    use crate::{
        frame::balances::{
            Balances,
            BalancesStore,
        },
        DefaultNodeRuntime as Runtime,
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

        let dest = AccountKeyring::Bob.to_account_id();
        let transfer =
            xt.submit(balances::transfer::<Runtime>(dest.clone().into(), 10_000));
        rt.block_on(transfer).unwrap();

        // check that nonce is handled correctly
        let transfer = xt
            .increment_nonce()
            .submit(balances::transfer::<Runtime>(dest.clone().into(), 10_000));

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

        let put_code = xt.submit_and_watch(contracts::put_code(500_000, wasm));

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

        let account = AccountKeyring::Alice.to_account_id();
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

        let balances = client.metadata().module_with_calls("Balances").unwrap();
        let dest = sp_keyring::AccountKeyring::Bob.to_account_id();
        let address: Address = dest.clone().into();
        let amount: Balance = 10_000;

        let transfer = pallet_balances::Call::transfer(address.clone(), amount);
        let call = node_runtime::Call::Balances(transfer);
        let subxt_transfer = crate::frame::balances::transfer::<Runtime>(address, amount);
        let call2 = balances.call("transfer", subxt_transfer.args).unwrap();
        assert_eq!(call.encode().to_vec(), call2.0);

        let free_balance =
            <pallet_balances::FreeBalance<node_runtime::Runtime>>::hashed_key_for(&dest);
        let free_balance_key = StorageKey(free_balance);
        let free_balance_key2 = client
            .metadata()
            .module("Balances")
            .unwrap()
            .storage("FreeBalance")
            .unwrap()
            .get_map::<AccountId, Balance>()
            .unwrap()
            .key(dest.clone());
        assert_eq!(free_balance_key, free_balance_key2);

        let account_nonce =
            <frame_system::AccountNonce<node_runtime::Runtime>>::hashed_key_for(&dest);
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
