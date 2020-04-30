// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

#![deny(
    bad_style,
    const_err,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates
)]
#![allow(clippy::type_complexity)]

use std::{
    convert::TryFrom,
    marker::PhantomData,
};

use codec::{
    Codec,
    Encode,
};
use futures::future;
use jsonrpsee::client::Subscription;
use sp_core::{
    storage::{
        StorageChangeSet,
        StorageKey,
    },
    Pair,
};
use sp_runtime::{
    generic::{
        SignedPayload,
        UncheckedExtrinsic,
    },
    traits::{
        IdentifyAccount,
        SignedExtension,
        Verify,
    },
    MultiSignature,
};
use sp_version::RuntimeVersion;

mod error;
mod events;
mod extrinsic;
mod frame;
mod metadata;
mod rpc;
mod runtimes;

pub use crate::{
    error::Error,
    events::{
        EventsDecoder,
        EventsError,
        RawEvent,
    },
    extrinsic::*,
    frame::*,
    metadata::{
        Metadata,
        MetadataError,
    },
    rpc::{
        BlockNumber,
        ExtrinsicSuccess,
    },
    runtimes::*,
};
use crate::{
    frame::{
        balances::Balances,
        system::{
            AccountStore,
            Phase,
            System,
            SystemEvent,
        },
    },
    rpc::{
        ChainBlock,
        Rpc,
    },
};

/// ClientBuilder for constructing a Client.
#[derive(Default)]
pub struct ClientBuilder<T: System, S = MultiSignature, E = DefaultExtra<T>> {
    _marker: std::marker::PhantomData<(T, S, E)>,
    url: Option<String>,
    client: Option<jsonrpsee::Client>,
}

impl<T: System, S, E> ClientBuilder<T, S, E> {
    /// Creates a new ClientBuilder.
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
            url: None,
            client: None,
        }
    }

    /// Sets the jsonrpsee client.
    pub fn set_client<P: Into<jsonrpsee::Client>>(mut self, client: P) -> Self {
        self.client = Some(client.into());
        self
    }

    /// Set the substrate rpc address.
    pub fn set_url<P: Into<String>>(mut self, url: P) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Creates a new Client.
    pub async fn build(self) -> Result<Client<T, S, E>, Error> {
        let client = if let Some(client) = self.client {
            client
        } else {
            let url = self
                .url
                .as_ref()
                .map(|s| &**s)
                .unwrap_or("ws://127.0.0.1:9944");
            if url.starts_with("ws://") || url.starts_with("wss://") {
                jsonrpsee::ws_client(url).await?
            } else {
                jsonrpsee::http_client(url)
            }
        };
        let rpc = Rpc::new(client).await?;

        let (metadata, genesis_hash, runtime_version) = future::join3(
            rpc.metadata(),
            rpc.genesis_hash(),
            rpc.runtime_version(None),
        )
        .await;
        Ok(Client {
            rpc,
            genesis_hash: genesis_hash?,
            metadata: metadata?,
            runtime_version: runtime_version?,
            _marker: PhantomData,
        })
    }
}

/// Client to interface with a substrate node.
pub struct Client<T: System, S = MultiSignature, E = DefaultExtra<T>> {
    rpc: Rpc<T>,
    genesis_hash: T::Hash,
    metadata: Metadata,
    runtime_version: RuntimeVersion,
    _marker: PhantomData<(fn() -> S, E)>,
}

impl<T: System, S, E> Clone for Client<T, S, E> {
    fn clone(&self) -> Self {
        Self {
            rpc: self.rpc.clone(),
            genesis_hash: self.genesis_hash,
            metadata: self.metadata.clone(),
            runtime_version: self.runtime_version.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: System, S, E> Client<T, S, E> {
    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Fetch a StorageKey.
    pub async fn fetch<F: Store<T>>(
        &self,
        store: F,
        hash: Option<T::Hash>,
    ) -> Result<Option<F::Returns>, Error> {
        let key = store.key(&self.metadata)?;
        let value = self.rpc.storage::<F::Returns>(key, hash).await?;
        if let Some(v) = value {
            Ok(Some(v))
        } else {
            Ok(store.default(&self.metadata)?)
        }
    }

    /// Query historical storage entries
    pub async fn query_storage(
        &self,
        keys: Vec<StorageKey>,
        from: T::Hash,
        to: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<<T as System>::Hash>>, Error> {
        self.rpc.query_storage(keys, from, to).await
    }

    /// Get a header
    pub async fn header<H>(&self, hash: Option<H>) -> Result<Option<T::Header>, Error>
    where
        H: Into<T::Hash> + 'static,
    {
        let header = self.rpc.header(hash.map(|h| h.into())).await?;
        Ok(header)
    }

    /// Get a block hash. By default returns the latest block hash
    pub async fn block_hash(
        &self,
        block_number: Option<BlockNumber<T>>,
    ) -> Result<Option<T::Hash>, Error> {
        let hash = self.rpc.block_hash(block_number).await?;
        Ok(hash)
    }

    /// Get a block hash of the latest finalized block
    pub async fn finalized_head(&self) -> Result<T::Hash, Error> {
        let head = self.rpc.finalized_head().await?;
        Ok(head)
    }

    /// Get a block
    pub async fn block<H>(&self, hash: Option<H>) -> Result<Option<ChainBlock<T>>, Error>
    where
        H: Into<T::Hash> + 'static,
    {
        let block = self.rpc.block(hash.map(|h| h.into())).await?;
        Ok(block)
    }

    /// Create and submit an extrinsic and return corresponding Hash if successful
    pub async fn submit_extrinsic<X: Encode>(
        &self,
        extrinsic: X,
    ) -> Result<T::Hash, Error> {
        let xt_hash = self.rpc.submit_extrinsic(extrinsic).await?;
        Ok(xt_hash)
    }

    /// Create and submit an extrinsic and return corresponding Event if successful
    pub async fn submit_and_watch_extrinsic<X: Encode + 'static>(
        self,
        extrinsic: X,
        decoder: EventsDecoder<T>,
    ) -> Result<ExtrinsicSuccess<T>, Error> {
        let success = self
            .rpc
            .submit_and_watch_extrinsic(extrinsic, decoder)
            .await?;
        Ok(success)
    }

    /// Subscribe to events.
    pub async fn subscribe_events(
        &self,
    ) -> Result<Subscription<StorageChangeSet<T::Hash>>, Error> {
        let events = self.rpc.subscribe_events().await?;
        Ok(events)
    }

    /// Subscribe to new blocks.
    pub async fn subscribe_blocks(&self) -> Result<Subscription<T::Header>, Error> {
        let headers = self.rpc.subscribe_blocks().await?;
        Ok(headers)
    }

    /// Subscribe to finalized blocks.
    pub async fn subscribe_finalized_blocks(
        &self,
    ) -> Result<Subscription<T::Header>, Error> {
        let headers = self.rpc.subscribe_finalized_blocks().await?;
        Ok(headers)
    }
}

impl<T, S, E> Client<T, S, E>
where
    T: System + Balances + Send + Sync,
    S: 'static,
    E: SignedExtra<T> + SignedExtension + 'static,
{
    /// Creates raw payload to be signed for the supplied `Call` without private key
    pub async fn create_raw_payload<C: Call<T>>(
        &self,
        account_id: &<T as System>::AccountId,
        call: C,
    ) -> Result<SignedPayload<Encoded, <E as SignedExtra<T>>::Extra>, Error> {
        let account_nonce = self
            .fetch(AccountStore(account_id), None)
            .await?
            .unwrap()
            .nonce;
        let version = self.runtime_version.spec_version;
        let genesis_hash = self.genesis_hash;
        let call = self
            .metadata()
            .module_with_calls(C::MODULE)
            .and_then(|module| module.call(C::FUNCTION, call))?;
        let extra: E = E::new(version, account_nonce, genesis_hash);
        let raw_payload = SignedPayload::new(call, extra.extra())?;
        Ok(raw_payload)
    }

    /// Create a transaction builder for a private key.
    pub async fn xt<P>(
        &self,
        signer: P,
        nonce: Option<T::Index>,
    ) -> Result<XtBuilder<T, P, S, E>, Error>
    where
        P: Pair,
        P::Signature: Codec,
        S: Verify,
        S::Signer: From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
    {
        let account_id = S::Signer::from(signer.public()).into_account();
        let nonce = match nonce {
            Some(nonce) => nonce,
            None => {
                self.fetch(AccountStore(&account_id), None)
                    .await?
                    .unwrap()
                    .nonce
            }
        };

        let genesis_hash = self.genesis_hash;
        let runtime_version = self.runtime_version.clone();
        Ok(XtBuilder {
            client: self.clone(),
            nonce,
            runtime_version,
            genesis_hash,
            signer,
        })
    }
}

/// Transaction builder.
#[derive(Clone)]
pub struct XtBuilder<T: System, P, S, E> {
    client: Client<T, S, E>,
    nonce: T::Index,
    runtime_version: RuntimeVersion,
    genesis_hash: T::Hash,
    signer: P,
}

impl<T: System, P, S, E> XtBuilder<T, P, S, E> {
    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        self.client.metadata()
    }

    /// Returns the nonce.
    pub fn nonce(&self) -> T::Index {
        self.nonce
    }

    /// Sets the nonce to a new value.
    pub fn set_nonce(&mut self, nonce: T::Index) -> &mut XtBuilder<T, P, S, E> {
        self.nonce = nonce;
        self
    }

    /// Increment the nonce
    pub fn increment_nonce(&mut self) -> &mut XtBuilder<T, P, S, E> {
        self.set_nonce(self.nonce() + 1.into());
        self
    }
}

impl<T: System + Send + Sync, P, S: 'static, E> XtBuilder<T, P, S, E>
where
    P: Pair,
    S: Verify + Codec + From<P::Signature>,
    S::Signer: From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
    T::Address: From<T::AccountId>,
    E: SignedExtra<T> + SignedExtension + 'static,
{
    /// Creates and signs an Extrinsic for the supplied `Call`
    pub fn create_and_sign<C: Call<T>>(
        &self,
        call: C,
    ) -> Result<
        UncheckedExtrinsic<T::Address, Encoded, S, <E as SignedExtra<T>>::Extra>,
        Error,
    > {
        let signer = self.signer.clone();
        let account_nonce = self.nonce;
        let version = self.runtime_version.spec_version;
        let genesis_hash = self.genesis_hash;
        let call = self
            .metadata()
            .module_with_calls(C::MODULE)
            .and_then(|module| module.call(C::FUNCTION, call))?;

        log::info!(
            "Creating Extrinsic with genesis hash {:?} and account nonce {:?}",
            genesis_hash,
            account_nonce
        );

        let extra = E::new(version, account_nonce, genesis_hash);
        let xt = extrinsic::create_and_sign::<_, _, _, S, _>(signer, call, extra)?;
        Ok(xt)
    }

    /// Submits a transaction to the chain.
    pub async fn submit<C: Call<T>>(&self, call: C) -> Result<T::Hash, Error> {
        let extrinsic = self.create_and_sign(call)?;
        let xt_hash = self.client.submit_extrinsic(extrinsic).await?;
        Ok(xt_hash)
    }

    /// Submits transaction to the chain and watch for events.
    pub fn watch(self) -> EventsSubscriber<T, P, S, E> {
        let metadata = self.client.metadata().clone();
        let decoder = EventsDecoder::try_from(metadata).map_err(Into::into);
        EventsSubscriber {
            client: self.client.clone(),
            builder: self,
            decoder,
        }
    }
}

/// Submits an extrinsic and subscribes to the triggered events
pub struct EventsSubscriber<T: System, P, S, E> {
    client: Client<T, S, E>,
    builder: XtBuilder<T, P, S, E>,
    decoder: Result<EventsDecoder<T>, EventsError>,
}

impl<T: System, P, S, E> EventsSubscriber<T, P, S, E> {
    /// Access the events decoder for registering custom type sizes
    pub fn events_decoder<
        F: FnOnce(&mut EventsDecoder<T>) -> Result<usize, EventsError>,
    >(
        self,
        f: F,
    ) -> Self {
        let mut this = self;
        if let Ok(ref mut decoder) = this.decoder {
            if let Err(err) = f(decoder) {
                this.decoder = Err(err)
            }
        }
        this
    }
}

impl<T: System + Send + Sync, P, S: 'static, E> EventsSubscriber<T, P, S, E>
where
    P: Pair,
    S: Verify + Codec + From<P::Signature>,
    S::Signer: From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
    T::Address: From<T::AccountId>,
    E: SignedExtra<T> + SignedExtension + 'static,
{
    /// Submits transaction to the chain and watch for events.
    pub async fn submit<C: Call<T>>(self, call: C) -> Result<ExtrinsicSuccess<T>, Error> {
        let mut decoder = self.decoder?;
        C::events_decoder(&mut decoder)?;
        let extrinsic = self.builder.create_and_sign(call)?;
        let xt_success = self
            .client
            .submit_and_watch_extrinsic(extrinsic, decoder)
            .await?;
        Ok(xt_success)
    }
}

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use sp_keyring::{
        AccountKeyring,
        Ed25519Keyring,
    };

    use super::*;

    pub(crate) async fn test_client() -> Client<crate::DefaultNodeRuntime> {
        ClientBuilder::new()
            .build()
            .await
            .expect("Error creating client")
    }

    #[async_std::test]
    #[ignore] // requires locally running substrate node
    async fn test_tx_transfer_balance() {
        env_logger::try_init().ok();
        let signer = AccountKeyring::Alice.pair();
        let dest = AccountKeyring::Bob.to_account_id().into();

        let client = test_client().await;
        let mut xt = client.xt(signer, None).await.unwrap();
        let _ = xt
            .submit(balances::TransferCall {
                to: &dest,
                amount: 10_000,
            })
            .await
            .unwrap();

        // check that nonce is handled correctly
        xt.increment_nonce()
            .submit(balances::TransferCall {
                to: &dest,
                amount: 10_000,
            })
            .await
            .unwrap();
    }

    #[async_std::test]
    #[ignore] // requires locally running substrate node
    async fn test_getting_hash() {
        let client = test_client().await;
        client.block_hash(None).await.unwrap();
    }

    #[async_std::test]
    #[ignore] // requires locally running substrate node
    async fn test_getting_block() {
        let client = test_client().await;
        let block_hash = client.block_hash(None).await.unwrap();
        client.block(block_hash).await.unwrap();
    }

    #[async_std::test]
    #[ignore] // requires locally running substrate node
    async fn test_state_total_issuance() {
        let client = test_client().await;
        client
            .fetch(balances::TotalIssuance(Default::default()), None)
            .await
            .unwrap()
            .unwrap();
    }

    #[async_std::test]
    #[ignore] // requires locally running substrate node
    async fn test_state_read_free_balance() {
        let client = test_client().await;
        let account = AccountKeyring::Alice.to_account_id();
        client
            .fetch(AccountStore(&account), None)
            .await
            .unwrap()
            .unwrap();
    }

    #[async_std::test]
    #[ignore] // requires locally running substrate node
    async fn test_chain_subscribe_blocks() {
        let client = test_client().await;
        let mut blocks = client.subscribe_blocks().await.unwrap();
        blocks.next().await;
    }

    #[async_std::test]
    #[ignore] // requires locally running substrate node
    async fn test_chain_subscribe_finalized_blocks() {
        let client = test_client().await;
        let mut blocks = client.subscribe_finalized_blocks().await.unwrap();
        blocks.next().await;
    }

    #[async_std::test]
    #[ignore] // requires locally running substrate node
    async fn test_create_raw_payload() {
        let signer_pair = Ed25519Keyring::Alice.pair();
        let signer_account_id = Ed25519Keyring::Alice.to_account_id();
        let dest = AccountKeyring::Bob.to_account_id().into();

        let client = test_client().await;

        // create raw payload with AccoundId and sign it
        let raw_payload = client
            .create_raw_payload(
                &signer_account_id,
                balances::TransferCall {
                    to: &dest,
                    amount: 10_000,
                },
            )
            .await
            .unwrap();
        let raw_signature = signer_pair.sign(raw_payload.encode().as_slice());
        let raw_multisig = MultiSignature::from(raw_signature);

        // create signature with Xtbuilder
        let xt = client.xt(signer_pair.clone(), None).await.unwrap();
        let xt_multi_sig = xt
            .create_and_sign(balances::TransferCall {
                to: &dest,
                amount: 10_000,
            })
            .unwrap()
            .signature
            .unwrap()
            .1;

        // compare signatures
        assert_eq!(raw_multisig, xt_multi_sig);
    }
}
