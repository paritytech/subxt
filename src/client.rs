// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use codec::{
    Decode,
};
use futures::future;
use jsonrpsee_http_client::HttpClientBuilder;
use jsonrpsee_types::Subscription;
use jsonrpsee_ws_client::WsClientBuilder;
use sp_core::{
    storage::{
        StorageChangeSet,
        StorageKey,
    },
    Bytes,
};
pub use sp_runtime::traits::SignedExtension;
pub use sp_version::RuntimeVersion;
use std::{
    marker::PhantomData,
    sync::Arc,
};

use crate::{Error, events::EventsDecoder, extrinsic::{
    self,
    PairSigner,
    Signer,
    SignedExtra,
    UncheckedExtrinsic,
}, rpc::{
    ChainBlock,
    Rpc,
    RpcClient,
    SystemProperties,
    ExtrinsicSuccess,
}, subscription::{
    EventStorageSubscription,
    EventSubscription,
    FinalizedEventStorageSubscription,
}, BlockNumber, Metadata, ReadProof, Runtime, Call, Encoded};

/// ClientBuilder for constructing a Client.
#[derive(Default)]
pub struct ClientBuilder {
    url: Option<String>,
    client: Option<RpcClient>,
    page_size: Option<u32>,
    skip_type_sizes_check: bool,
    accept_weak_inclusion: bool,
}

impl ClientBuilder {
    /// Creates a new ClientBuilder.
    pub fn new() -> Self {
        Self {
            url: None,
            client: None,
            page_size: None,
            skip_type_sizes_check: false,
            accept_weak_inclusion: false,
        }
    }

    /// Sets the jsonrpsee client.
    pub fn set_client<C: Into<RpcClient>>(mut self, client: C) -> Self {
        self.client = Some(client.into());
        self
    }

    /// Set the substrate rpc address.
    pub fn set_url<P: Into<String>>(mut self, url: P) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the page size.
    pub fn set_page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// Only check that transactions are InBlock on submit.
    pub fn accept_weak_inclusion(mut self) -> Self {
        self.accept_weak_inclusion = true;
        self
    }

    /// Creates a new Client.
    pub async fn build<T: Runtime>(self) -> Result<Client<T>, Error> {
        let client = if let Some(client) = self.client {
            client
        } else {
            let url = self.url.as_deref().unwrap_or("ws://127.0.0.1:9944");
            if url.starts_with("ws://") || url.starts_with("wss://") {
                let client = WsClientBuilder::default()
                    .max_notifs_per_subscription(4096)
                    .build(url)
                    .await?;
                RpcClient::WebSocket(Arc::new(client))
            } else {
                let client = HttpClientBuilder::default().build(&url)?;
                RpcClient::Http(Arc::new(client))
            }
        };
        let mut rpc = Rpc::new(client);
        if self.accept_weak_inclusion {
            rpc.accept_weak_inclusion();
        }
        let (metadata, genesis_hash, runtime_version, properties) = future::join4(
            rpc.metadata(),
            rpc.genesis_hash(),
            rpc.runtime_version(None),
            rpc.system_properties(),
        )
            .await;
        let metadata = metadata?;

        let events_decoder =
            EventsDecoder::new(metadata.clone());

        Ok(Client {
            rpc,
            genesis_hash: genesis_hash?,
            metadata,
            events_decoder,
            properties: properties.unwrap_or_else(|_| Default::default()),
            runtime_version: runtime_version?,
            _marker: PhantomData,
            page_size: self.page_size.unwrap_or(10),
        })
    }
}

/// Client to interface with a substrate node.
pub struct Client<T: Runtime> {
    rpc: Rpc<T>,
    genesis_hash: T::Hash,
    metadata: Metadata,
    events_decoder: EventsDecoder<T>,
    properties: SystemProperties,
    runtime_version: RuntimeVersion,
    _marker: PhantomData<(fn() -> T::Signature, T::Extra)>,
    page_size: u32,
}

impl<T: Runtime> Clone for Client<T> {
    fn clone(&self) -> Self {
        Self {
            rpc: self.rpc.clone(),
            genesis_hash: self.genesis_hash,
            metadata: self.metadata.clone(),
            events_decoder: self.events_decoder.clone(),
            properties: self.properties.clone(),
            runtime_version: self.runtime_version.clone(),
            _marker: PhantomData,
            page_size: self.page_size,
        }
    }
}

impl<T: Runtime> Client<T> {
    /// Returns the genesis hash.
    pub fn genesis(&self) -> &T::Hash {
        &self.genesis_hash
    }

    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns the system properties
    pub fn properties(&self) -> &SystemProperties {
        &self.properties
    }

    /// Returns the rpc client.
    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc.client
    }

    /// Fetch the value under an unhashed storage key
    pub async fn fetch_unhashed<V: Decode>(
        &self,
        key: StorageKey,
        hash: Option<T::Hash>,
    ) -> Result<Option<V>, Error> {
        if let Some(data) = self.rpc.storage(&key, hash).await? {
            Ok(Some(Decode::decode(&mut &data.0[..])?))
        } else {
            Ok(None)
        }
    }

    /// Query historical storage entries
    pub async fn query_storage(
        &self,
        keys: Vec<StorageKey>,
        from: T::Hash,
        to: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<T::Hash>>, Error> {
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
        block_number: Option<BlockNumber>,
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

    /// Get proof of storage entries at a specific block's state.
    pub async fn read_proof<H>(
        &self,
        keys: Vec<StorageKey>,
        hash: Option<H>,
    ) -> Result<ReadProof<T::Hash>, Error>
        where
            H: Into<T::Hash> + 'static,
    {
        let proof = self.rpc.read_proof(keys, hash.map(|h| h.into())).await?;
        Ok(proof)
    }

    /// Subscribe to events.
    ///
    /// *WARNING* these may not be included in the finalized chain, use
    /// `subscribe_finalized_events` to ensure events are finalized.
    pub async fn subscribe_events(&self) -> Result<EventStorageSubscription<T>, Error> {
        let events = self.rpc.subscribe_events().await?;
        Ok(events)
    }

    /// Subscribe to finalized events.
    pub async fn subscribe_finalized_events(
        &self,
    ) -> Result<EventStorageSubscription<T>, Error> {
        let events = self.rpc.subscribe_finalized_events().await?;
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

    // /// Encodes a call.
    // pub fn encode<C: Call<T>>(&self, call: C) -> Result<Encoded, Error> {
    //     Ok(self
    //         .metadata()
    //         .module_with_calls(C::MODULE)
    //         .and_then(|module| module.call(C::FUNCTION, call))?)
    // }

    /// Creates an unsigned extrinsic.
    // pub fn create_unsigned<C: Call<T> + Send + Sync>(
    //     &self,
    //     call: C,
    // ) -> Result<UncheckedExtrinsic<T>, Error> {
    //     let call = self.encode(call)?;
    //     Ok(extrinsic::create_unsigned::<T>(call))
    // }

    /// Creates a signed extrinsic.
    pub async fn create_signed<C: Call + Send + Sync>(
        &self,
        call: C,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<UncheckedExtrinsic<T>, Error>
        where
            <<T::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
            Send + Sync,
    {
        let account_nonce = if let Some(nonce) = signer.nonce() {
            nonce
        } else {
            todo!("fetch nonce if not supplied")
            // self.account(signer.account_id(), None).await?.nonce
        };
        let call = self.encode(call)?;
        let signed = extrinsic::create_signed(
            &self.runtime_version,
            self.genesis_hash,
            account_nonce,
            call,
            signer,
        )
            .await?;
        Ok(signed)
    }

    /// Encodes a call.
    pub fn encode<C: Call>(&self, call: C) -> Result<Encoded, Error> {
        Ok(self
            .metadata()
            .pallet(C::PALLET)
            .and_then(|pallet| pallet.encode_call(call))?)
    }

    /// Returns the events decoder.
    pub fn events_decoder(&self) -> &EventsDecoder<T> {
        &self.events_decoder
    }

    /// Create and submit an extrinsic and return corresponding Hash if successful
    pub async fn submit_extrinsic(
        &self,
        extrinsic: UncheckedExtrinsic<T>,
    ) -> Result<T::Hash, Error> {
        self.rpc.submit_extrinsic(extrinsic).await
    }

    /// Create and submit an extrinsic and return corresponding Event if successful
    pub async fn submit_and_watch_extrinsic(
        &self,
        extrinsic: UncheckedExtrinsic<T>,
    ) -> Result<ExtrinsicSuccess<T>, Error> {
        self.rpc
            .submit_and_watch_extrinsic(extrinsic, &self.events_decoder)
            .await
    }

    /// Submits a transaction to the chain.
    pub async fn submit<C: Call + Send + Sync>(
        &self,
        call: C,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<T::Hash, Error>
        where
            <<T::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
            Send + Sync,
    {
        let extrinsic = self.create_signed(call, signer).await?;
        self.submit_extrinsic(extrinsic).await
    }

    /// Submits transaction to the chain and watch for events.
    // pub async fn watch<C: Call<T> + Send + Sync>(
    //     &self,
    //     call: C,
    //     signer: &(dyn Signer<T> + Send + Sync),
    // ) -> Result<ExtrinsicSuccess<T>, Error>
    //     where
    //         <<T::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
    //         Send + Sync,
    // {
    //     let extrinsic = self.create_signed(call, signer).await?;
    //     self.submit_and_watch_extrinsic(extrinsic).await
    // }

    /// Insert a key into the keystore.
    pub async fn insert_key(
        &self,
        key_type: String,
        suri: String,
        public: Bytes,
    ) -> Result<(), Error> {
        self.rpc.insert_key(key_type, suri, public).await
    }

    /// Generate new session keys and returns the corresponding public keys.
    pub async fn rotate_keys(&self) -> Result<Bytes, Error> {
        self.rpc.rotate_keys().await
    }

    /// Checks if the keystore has private keys for the given session public keys.
    ///
    /// `session_keys` is the SCALE encoded session keys object from the runtime.
    ///
    /// Returns `true` iff all private keys could be found.
    pub async fn has_session_keys(&self, session_keys: Bytes) -> Result<bool, Error> {
        self.rpc.has_session_keys(session_keys).await
    }

    /// Checks if the keystore has private keys for the given public key and key type.
    ///
    /// Returns `true` if a private key could be found.
    pub async fn has_key(
        &self,
        public_key: Bytes,
        key_type: String,
    ) -> Result<bool, Error> {
        self.rpc.has_key(public_key, key_type).await
    }
}
