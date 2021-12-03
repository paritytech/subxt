// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of subxt.
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
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

pub use sp_runtime::traits::SignedExtension;
pub use sp_version::RuntimeVersion;
use sp_core::storage::StorageKey;
use sp_runtime::traits::Hash;
use futures::future;

use crate::{
    events::EventsDecoder,
    extrinsic::{
        self,
        SignedExtra,
        Signer,
        UncheckedExtrinsic,
    },
    rpc::{
        ExtrinsicSuccess,
        Rpc,
        RpcClient,
        SystemProperties, TransactionStatus
    },
    error::{
        Error,
        TransactionError
    },
    subscription::SystemEvents,
    storage::StorageClient,
    AccountData,
    Call,
    Config,
    ExtrinsicExtraData,
    Metadata, Phase,
};
use jsonrpsee::types::{
    Error as RpcError,
    Subscription as RpcSubscription,
};
use std::sync::Arc;

/// ClientBuilder for constructing a Client.
#[derive(Default)]
pub struct ClientBuilder {
    url: Option<String>,
    client: Option<RpcClient>,
    page_size: Option<u32>,
    accept_weak_inclusion: bool,
}

impl ClientBuilder {
    /// Creates a new ClientBuilder.
    pub fn new() -> Self {
        Self {
            url: None,
            client: None,
            page_size: None,
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
    pub async fn build<T: Config>(self) -> Result<Client<T>, Error> {
        let client = if let Some(client) = self.client {
            client
        } else {
            let url = self.url.as_deref().unwrap_or("ws://127.0.0.1:9944");
            RpcClient::try_from_url(url).await?
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

        let events_decoder = EventsDecoder::new(metadata.clone());

        Ok(Client(Arc::new(ClientInner{
            rpc,
            genesis_hash: genesis_hash?,
            metadata,
            events_decoder,
            properties: properties.unwrap_or_else(|_| Default::default()),
            runtime_version: runtime_version?,
            iter_page_size: self.page_size.unwrap_or(10),
        })))
    }
}

/// Client to interface with a substrate node.
#[derive(Clone)]
pub struct Client<T: Config>(Arc<ClientInner<T>>);

#[derive(Clone)]
struct ClientInner<T: Config> {
    rpc: Rpc<T>,
    genesis_hash: T::Hash,
    metadata: Metadata,
    events_decoder: EventsDecoder<T>,
    properties: SystemProperties,
    runtime_version: RuntimeVersion,
    iter_page_size: u32,
}

impl<T: Config> Client<T> {
    /// Returns the genesis hash.
    pub fn genesis(&self) -> &T::Hash {
        &self.0.genesis_hash
    }

    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.0.metadata
    }

    /// Returns the system properties
    pub fn properties(&self) -> &SystemProperties {
        &self.0.properties
    }

    /// Returns the rpc client.
    pub fn rpc(&self) -> &Rpc<T> {
        &self.0.rpc
    }

    /// Create a client for accessing runtime storage
    pub fn storage(&self) -> StorageClient<T> {
        StorageClient::new(&self.0.rpc, &self.0.metadata, self.0.iter_page_size)
    }

    /// Convert the client to a runtime api wrapper for custom runtime access.
    ///
    /// The `subxt` proc macro will provide methods to submit extrinsics and read storage specific
    /// to the target runtime.
    pub fn to_runtime_api<R: From<Self>>(self) -> R {
        self.into()
    }

    /// Returns the events decoder.
    pub fn events_decoder(&self) -> &EventsDecoder<T> {
        &self.0.events_decoder
    }
}

/// A constructed call ready to be signed and submitted.
pub struct SubmittableExtrinsic<T: Config, C> {
    client: Arc<Client<T>>,
    call: C,
}

impl<T, C> SubmittableExtrinsic<T, C>
where
    T: Config + ExtrinsicExtraData<T>,
    C: Call + Send + Sync,
{
    /// Create a new [`SubmittableExtrinsic`].
    pub fn new(client: Arc<Client<T>>, call: C) -> Self {
        Self { client, call }
    }

    /// Creates and signs an extrinsic and submits it to the chain.
    ///
    /// Returns when the extrinsic has successfully been included in the block, together with any
    /// events which were triggered by the extrinsic.
    pub async fn sign_and_submit_then_watch(
        self,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<ExtrinsicSuccess<T>, Error>
    where
        <<<T as ExtrinsicExtraData<T>>::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync + 'static
    {
        let extrinsic = self.create_signed(signer, Default::default()).await?;
        self.client
            .rpc()
            .submit_and_watch_extrinsic(extrinsic, self.client.events_decoder())
            .await
    }

    /// TODO document
    pub async fn sign_and_submit_then_watch_new(
        self,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<TransactionProgress<T>, Error>
    where
        <<<T as ExtrinsicExtraData<T>>::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync + 'static
    {
        // Sign the call data to create our extrinsic.
        let extrinsic = self.create_signed(signer, Default::default()).await?;
        // Get a hash of the extrinsic (we'll need this later).
        let ext_hash = T::Hashing::hash_of(&extrinsic);
        // Submit and watch for transaction progress.
        let sub = self.client.rpc().watch_extrinsic(extrinsic).await?;

        Ok(TransactionProgress::new(sub, self.client, ext_hash))
    }

    /// Creates and signs an extrinsic and submits to the chain for block inclusion.
    ///
    /// Returns `Ok` with the extrinsic hash if it is valid extrinsic.
    ///
    /// # Note
    ///
    /// Success does not mean the extrinsic has been included in the block, just that it is valid
    /// and has been included in the transaction pool.
    pub async fn sign_and_submit(
        self,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<T::Hash, Error>
    where
        <<<T as ExtrinsicExtraData<T>>::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync + 'static
    {
        let extrinsic = self.create_signed(signer, Default::default()).await?;
        self.client.rpc().submit_extrinsic(extrinsic).await
    }

    /// Creates a signed extrinsic.
    pub async fn create_signed(
        &self,
        signer: &(dyn Signer<T> + Send + Sync),
        additional_params: <T::Extra as SignedExtra<T>>::Parameters,
    ) -> Result<UncheckedExtrinsic<T>, Error>
    where
        <<<T as ExtrinsicExtraData<T>>::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync + 'static
    {
        let account_nonce = if let Some(nonce) = signer.nonce() {
            nonce
        } else {
            let account_storage_entry =
                <<T as ExtrinsicExtraData<T>>::AccountData as AccountData<T>>::storage_entry(signer.account_id().clone());
            let account_data = self
                .client
                .storage()
                .fetch_or_default(&account_storage_entry, None)
                .await?;
            <<T as ExtrinsicExtraData<T>>::AccountData as AccountData<T>>::nonce(
                &account_data,
            )
        };
        let call = self
            .client
            .metadata()
            .pallet(C::PALLET)
            .and_then(|pallet| pallet.encode_call(&self.call))?;

        let signed = extrinsic::create_signed(
            &self.client.0.runtime_version,
            self.client.0.genesis_hash,
            account_nonce,
            call,
            signer,
            additional_params,
        )
        .await?;
        Ok(signed)
    }
}

pub struct TransactionProgress<T: Config> {
    sub: RpcSubscription<TransactionStatus<T::Hash, T::Hash>>,
    ext_hash: T::Hash,
    client: Arc<Client<T>>,
}

impl <T: Config> TransactionProgress<T> {
    pub (crate) fn new(sub: RpcSubscription<TransactionStatus<T::Hash, T::Hash>>, client: Arc<Client<T>>, ext_hash: T::Hash) -> Self {
        Self { sub, client, ext_hash }
    }

    /// Return the next transaction status when it's emitted.
    pub async fn next(&mut self) -> Result<Option<TransactionProgressStatus<T>>, Error> {
        let res = self.sub.next().await?;
        Ok(res.map(|status| {
            match status {
                TransactionStatus::Future => TransactionProgressStatus::Future,
                TransactionStatus::Ready => TransactionProgressStatus::Ready,
                TransactionStatus::Broadcast(peers) => TransactionProgressStatus::Broadcast(peers),
                TransactionStatus::InBlock(hash) => TransactionProgressStatus::InBlock(TransactionInBlock {
                    block_hash: hash,
                    ext_hash: self.ext_hash,
                    client: Arc::clone(&self.client)
                }),
                TransactionStatus::Retracted(hash) => TransactionProgressStatus::Retracted(hash),
                TransactionStatus::FinalityTimeout(hash) => TransactionProgressStatus::FinalityTimeout(hash),
                TransactionStatus::Finalized(hash) => TransactionProgressStatus::Finalized(TransactionInBlock {
                    block_hash: hash,
                    ext_hash: self.ext_hash,
                    client: Arc::clone(&self.client)
                }),
                TransactionStatus::Usurped(hash) => TransactionProgressStatus::Usurped(hash),
                TransactionStatus::Dropped => TransactionProgressStatus::Dropped,
                TransactionStatus::Invalid => TransactionProgressStatus::Invalid,
            }
        }))
    }

    /// Wait for the transaction to be in a block (but not necessarily finalized), and return
    /// an [`TransactionInBlock`] instance when this happens, or an error if there was a problem
    /// waiting for finalization.
    pub async fn wait_for_in_block(mut self) -> Result<TransactionInBlock<T>, Error> {
        while let Some(status) = self.next().await? {
            match status {
                // Finalized or otherwise in a block! Return.
                TransactionProgressStatus::InBlock(s)
                | TransactionProgressStatus::Finalized(s) => return Ok(s),
                // Error scenarios; return the error.
                TransactionProgressStatus::FinalityTimeout(_) => return Err(TransactionError::FinalitySubscriptionTimeout.into()),
                TransactionProgressStatus::Invalid => return Err(TransactionError::Invalid.into()),
                TransactionProgressStatus::Usurped(_) => return Err(TransactionError::Usurped.into()),
                TransactionProgressStatus::Dropped => return Err(TransactionError::Dropped.into()),
                // Ignore and wait for next status event:
                _ => continue
            }
        }
        Err(RpcError::Custom("RPC subscription dropped".into()).into())
    }

    /// Wait for the transaction to be finalized, and return a [`TransactionInBlock`]
    /// instance when it is, or an error if there was a problem waiting for finalization.
    pub async fn wait_for_finalized(mut self) -> Result<TransactionInBlock<T>, Error> {
        while let Some(status) = self.next().await? {
            match status {
                // Finalized! Return.
                TransactionProgressStatus::Finalized(s) => return Ok(s),
                // Error scenarios; return the error.
                TransactionProgressStatus::FinalityTimeout(_) => return Err(TransactionError::FinalitySubscriptionTimeout.into()),
                TransactionProgressStatus::Invalid => return Err(TransactionError::Invalid.into()),
                TransactionProgressStatus::Usurped(_) => return Err(TransactionError::Usurped.into()),
                TransactionProgressStatus::Dropped => return Err(TransactionError::Dropped.into()),
                // Ignore and wait for next status event:
                _ => continue
            }
        }
        Err(RpcError::Custom("RPC subscription dropped".into()).into())
    }
}

/// Possible transaction status events returned from our [`crate::Client`].
#[derive(Clone)]
pub enum TransactionProgressStatus<T: Config> {
    Future,
    Ready,
    Broadcast(Vec<String>),
    InBlock(TransactionInBlock<T>),
    Retracted(T::Hash),
    FinalityTimeout(T::Hash),
    Finalized(TransactionInBlock<T>),
    Usurped(T::Hash),
    Dropped,
    Invalid
}

impl <T: Config> TransactionProgressStatus<T> {
    /// A convenience method to return the `Finalized` details. Returns
    /// [`None`] if the enum variant is not [`TransactionProgressStatus::Finalized`].
    pub fn as_finalized(&self) -> Option<&TransactionInBlock<T>> {
        match self {
            Self::Finalized(val) => Some(val),
            _ => None
        }
    }

    /// A convenience method to return the `InBlock` details. Returns
    /// [`None`] if the enum variant is not [`TransactionProgressStatus::Finalized`].
    pub fn as_in_block(&self) -> Option<&TransactionInBlock<T>> {
        match self {
            Self::InBlock(val) => Some(val),
            _ => None
        }
    }
}

/// This struct represents a transaction that has made it into a block.
#[derive(Clone)]
pub struct TransactionInBlock<T: Config> {
    block_hash: T::Hash,
    ext_hash: T::Hash,
    client: Arc<Client<T>>
}

impl <T: Config> TransactionInBlock<T> {
    /// Return the hash of the block that the transaction has made it into.
    pub fn block_hash(&self) -> T::Hash {
        self.block_hash
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> T::Hash {
        self.ext_hash
    }

    /// Fetch the events associated with this transaction.
    pub async fn events(&self) -> Result<impl Iterator<Item=crate::RawEvent>, Error> {
        let block = self.client
            .rpc()
            .block(Some(self.block_hash))
            .await?
            .ok_or(Error::Transaction(TransactionError::BlockHashNotFound))?;

        let extrinsic_idx = block.block.extrinsics
            .iter()
            .position(|ext| {
                let hash = T::Hashing::hash_of(ext);
                hash == self.ext_hash
            })
            // TODO [jsdw] is it possible to actually hit an error here?
            .ok_or(Error::Transaction(TransactionError::BlockHashNotFound))?;

        let raw_events = self.client
            .rpc()
            .storage(&StorageKey::from(SystemEvents::new()), Some(self.block_hash))
            .await?
            .map(|s| s.0)
            .unwrap_or(Vec::new());

        let event_iter = self.client
            .events_decoder()
            .decode_events(&mut &*raw_events)?
            .into_iter()
            .filter(|(phase, _raw)| {
                phase == &Phase::ApplyExtrinsic(extrinsic_idx as u32)
            })
            .map(|(_phase, raw)| raw);

        Ok(event_iter)
    }
}
