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

use futures::future;
pub use sp_runtime::traits::SignedExtension;
pub use sp_version::RuntimeVersion;

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
        SystemProperties,
    },
    storage::StorageClient,
    AccountData,
    Call,
    Config,
    Error,
    ExtrinsicExtraData,
    Metadata,
};

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

        Ok(Client {
            rpc,
            genesis_hash: genesis_hash?,
            metadata,
            events_decoder,
            properties: properties.unwrap_or_else(|_| Default::default()),
            runtime_version: runtime_version?,
            iter_page_size: self.page_size.unwrap_or(10),
        })
    }
}

/// Client to interface with a substrate node.
pub struct Client<T: Config> {
    rpc: Rpc<T>,
    genesis_hash: T::Hash,
    metadata: Metadata,
    events_decoder: EventsDecoder<T>,
    properties: SystemProperties,
    runtime_version: RuntimeVersion,
    // _marker: PhantomData<(fn() -> T::Signature, T::Extra)>,
    iter_page_size: u32,
}

impl<T: Config> Clone for Client<T> {
    fn clone(&self) -> Self {
        Self {
            rpc: self.rpc.clone(),
            genesis_hash: self.genesis_hash,
            metadata: self.metadata.clone(),
            events_decoder: self.events_decoder.clone(),
            properties: self.properties.clone(),
            runtime_version: self.runtime_version.clone(),
            iter_page_size: self.iter_page_size,
        }
    }
}

impl<T: Config> Client<T> {
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
    pub fn rpc(&self) -> &Rpc<T> {
        &self.rpc
    }

    /// Create a client for accessing runtime storage
    pub fn storage(&self) -> StorageClient<T> {
        StorageClient::new(&self.rpc, &self.metadata, self.iter_page_size)
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
        &self.events_decoder
    }
}

/// A constructed call ready to be signed and submitted.
pub struct SubmittableExtrinsic<'a, T: Config, C> {
    client: &'a Client<T>,
    call: C,
}

impl<'a, T, C> SubmittableExtrinsic<'a, T, C>
where
    T: Config + ExtrinsicExtraData<T>,
    C: Call + Send + Sync,
{
    /// Create a new [`SubmittableExtrinsic`].
    pub fn new(client: &'a Client<T>, call: C) -> Self {
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
            &self.client.runtime_version,
            self.client.genesis_hash,
            account_nonce,
            call,
            signer,
            additional_params,
        )
        .await?;
        Ok(signed)
    }
}
