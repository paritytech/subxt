// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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
use sp_runtime::{
    traits::Hash,
    ApplyExtrinsicResult,
};

use crate::{
    error::{
        BasicError,
        HasModuleError,
    },
    extrinsic::{
        ExtrinsicParams,
        Signer,
    },
    rpc::{
        Rpc,
        RpcClient,
        RuntimeVersion,
        SystemProperties,
    },
    storage::StorageClient,
    transaction::TransactionProgress,
    updates::UpdateClient,
    Call,
    Config,
    Encoded,
    Metadata,
};
use codec::{
    Compact,
    Decode,
    Encode,
};
use derivative::Derivative;
use parking_lot::RwLock;
use std::sync::Arc;

/// ClientBuilder for constructing a Client.
#[derive(Default)]
pub struct ClientBuilder {
    url: Option<String>,
    client: Option<RpcClient>,
    metadata: Option<Metadata>,
    page_size: Option<u32>,
}

impl ClientBuilder {
    /// Creates a new ClientBuilder.
    pub fn new() -> Self {
        Self {
            url: None,
            client: None,
            metadata: None,
            page_size: None,
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

    /// Set the metadata.
    ///
    /// *Note:* Metadata will no longer be downloaded from the runtime node.
    #[cfg(feature = "integration-tests")]
    pub fn set_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Builder for [Client].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subxt::{ClientBuilder, DefaultConfig};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Build the client.
    ///     let client = ClientBuilder::new()
    ///          .set_url("wss://rpc.polkadot.io:443")
    ///          .build::<DefaultConfig>()
    ///          .await
    ///          .unwrap();
    ///     // Use the client...
    /// }
    /// ```
    pub async fn build<T: Config>(self) -> Result<Client<T>, BasicError> {
        let client = if let Some(client) = self.client {
            client
        } else {
            let url = self.url.as_deref().unwrap_or("ws://127.0.0.1:9944");
            crate::rpc::ws_client(url).await?
        };
        let rpc = Rpc::new(client);
        let (genesis_hash, runtime_version, properties) = future::join3(
            rpc.genesis_hash(),
            rpc.runtime_version(None),
            rpc.system_properties(),
        )
        .await;

        let metadata = if let Some(metadata) = self.metadata {
            metadata
        } else {
            rpc.metadata().await?
        };

        Ok(Client {
            rpc,
            genesis_hash: genesis_hash?,
            metadata: Arc::new(RwLock::new(metadata)),
            properties: properties.unwrap_or_else(|_| Default::default()),
            runtime_version: Arc::new(RwLock::new(runtime_version?)),
            iter_page_size: self.page_size.unwrap_or(10),
        })
    }
}

/// Client to interface with a substrate node.
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Client<T: Config> {
    rpc: Rpc<T>,
    genesis_hash: T::Hash,
    metadata: Arc<RwLock<Metadata>>,
    properties: SystemProperties,
    runtime_version: Arc<RwLock<RuntimeVersion>>,
    iter_page_size: u32,
}

impl<T: Config> std::fmt::Debug for Client<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("rpc", &"<Rpc>")
            .field("genesis_hash", &self.genesis_hash)
            .field("metadata", &"<Metadata>")
            .field("events_decoder", &"<EventsDecoder>")
            .field("properties", &self.properties)
            .field("runtime_version", &self.runtime_version)
            .field("iter_page_size", &self.iter_page_size)
            .finish()
    }
}

impl<T: Config> Client<T> {
    /// Returns the genesis hash.
    pub fn genesis(&self) -> &T::Hash {
        &self.genesis_hash
    }

    /// Returns the chain metadata.
    pub fn metadata(&self) -> Arc<RwLock<Metadata>> {
        Arc::clone(&self.metadata)
    }

    /// Returns the properties defined in the chain spec as a JSON object.
    ///
    /// # Note
    ///
    /// Many chains use this to define common properties such as `token_decimals` and `token_symbol`
    /// required for UIs, but this is merely a convention. It is up to the library user to
    /// deserialize the JSON into the appropriate type or otherwise extract the properties defined
    /// in the target chain's spec.
    pub fn properties(&self) -> &SystemProperties {
        &self.properties
    }

    /// Returns the rpc client.
    pub fn rpc(&self) -> &Rpc<T> {
        &self.rpc
    }

    /// Create a client for accessing runtime storage
    pub fn storage(&self) -> StorageClient<T> {
        StorageClient::new(&self.rpc, self.metadata(), self.iter_page_size)
    }

    /// Create a wrapper for performing runtime updates on this client.
    ///
    /// # Note
    ///
    /// The update client is intended to be used in the background for
    /// performing runtime updates, while the API is still in use.
    /// Without performing runtime updates the submitted extrinsics may fail.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use subxt::{ClientBuilder, DefaultConfig};
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// #    let client = ClientBuilder::new()
    /// #         .set_url("wss://rpc.polkadot.io:443")
    /// #         .build::<DefaultConfig>()
    /// #         .await
    /// #         .unwrap();
    /// #
    /// let update_client = client.updates();
    /// // Spawn a new background task to handle runtime updates.
    /// tokio::spawn(async move {
    ///     let result = update_client.perform_runtime_updates().await;
    ///     println!("Runtime update finished with result={:?}", result);
    /// });
    /// # }
    /// ```
    pub fn updates(&self) -> UpdateClient<T> {
        UpdateClient::new(
            self.rpc.clone(),
            self.metadata(),
            self.runtime_version.clone(),
        )
    }

    /// Convert the client to a runtime api wrapper for custom runtime access.
    ///
    /// The `subxt` proc macro will provide methods to submit extrinsics and read storage specific
    /// to the target runtime.
    pub fn to_runtime_api<R: From<Self>>(self) -> R {
        self.into()
    }

    /// Returns the client's Runtime Version.
    pub fn runtime_version(&self) -> Arc<RwLock<RuntimeVersion>> {
        Arc::clone(&self.runtime_version)
    }
}

/// A constructed call ready to be signed and submitted.
pub struct SubmittableExtrinsic<'client, T: Config, X, C, E: Decode, Evs: Decode> {
    client: &'client Client<T>,
    call: C,
    marker: std::marker::PhantomData<(X, E, Evs)>,
}

impl<'client, T, X, C, E, Evs> SubmittableExtrinsic<'client, T, X, C, E, Evs>
where
    T: Config,
    X: ExtrinsicParams<T>,
    C: Call + Send + Sync,
    E: Decode + HasModuleError,
    Evs: Decode,
{
    /// Create a new [`SubmittableExtrinsic`].
    pub fn new(client: &'client Client<T>, call: C) -> Self {
        Self {
            client,
            call,
            marker: Default::default(),
        }
    }

    /// Creates and signs an extrinsic and submits it to the chain. Passes default parameters
    /// to construct the "signed extra" and "additional" payloads needed by the extrinsic.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch_default(
        &self,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<TransactionProgress<'client, T, E, Evs>, BasicError>
    where
        X::OtherParams: Default,
    {
        self.sign_and_submit_then_watch(signer, Default::default())
            .await
    }

    /// Creates and signs an extrinsic and submits it to the chain.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch(
        &self,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: X::OtherParams,
    ) -> Result<TransactionProgress<'client, T, E, Evs>, BasicError> {
        self.create_signed(signer, other_params)
            .await?
            .submit_and_watch()
            .await
    }

    /// Creates and signs an extrinsic and submits to the chain for block inclusion. Passes
    /// default parameters to construct the "signed extra" and "additional" payloads needed
    /// by the extrinsic.
    ///
    /// Returns `Ok` with the extrinsic hash if it is valid extrinsic.
    ///
    /// # Note
    ///
    /// Success does not mean the extrinsic has been included in the block, just that it is valid
    /// and has been included in the transaction pool.
    pub async fn sign_and_submit_default(
        &self,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<T::Hash, BasicError>
    where
        X::OtherParams: Default,
    {
        self.sign_and_submit(signer, Default::default()).await
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
        &self,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: X::OtherParams,
    ) -> Result<T::Hash, BasicError> {
        self.create_signed(signer, other_params)
            .await?
            .submit()
            .await
    }

    /// Return the SCALE encoded bytes representing the call data of the transaction.
    pub fn call_data(&self) -> Result<Vec<u8>, BasicError> {
        let mut bytes = Vec::new();
        let locked_metadata = self.client.metadata();
        let metadata = locked_metadata.read();
        let pallet = metadata.pallet(C::PALLET)?;
        bytes.push(pallet.index());
        bytes.push(pallet.call_index::<C>()?);
        self.call.encode_to(&mut bytes);
        Ok(bytes)
    }

    /// Creates a returns a raw signed extrinsic, without submitting it.
    pub async fn create_signed(
        &self,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: X::OtherParams,
    ) -> Result<SignedSubmittableExtrinsic<'client, T, X, E, Evs>, BasicError> {
        // 1. Get nonce
        let account_nonce = if let Some(nonce) = signer.nonce() {
            nonce
        } else {
            self.client
                .rpc()
                .system_account_next_index(signer.account_id())
                .await?
        };

        // 2. SCALE encode call data to bytes (pallet u8, call u8, call params).
        let call_data = Encoded(self.call_data()?);

        // 3. Construct our custom additional/extra params.
        let additional_and_extra_params = {
            // Obtain spec version and transaction version from the runtime version of the client.
            let locked_runtime = self.client.runtime_version();
            let runtime = locked_runtime.read();
            X::new(
                runtime.spec_version,
                runtime.transaction_version,
                account_nonce,
                self.client.genesis_hash,
                other_params,
            )
        };

        tracing::debug!(
            "additional_and_extra_params: {:?}",
            additional_and_extra_params
        );

        // 4. Construct signature. This is compatible with the Encode impl
        //    for SignedPayload (which is this payload of bytes that we'd like)
        //    to sign. See:
        //    https://github.com/paritytech/substrate/blob/9a6d706d8db00abb6ba183839ec98ecd9924b1f8/primitives/runtime/src/generic/unchecked_extrinsic.rs#L215)
        let signature = {
            let mut bytes = Vec::new();
            call_data.encode_to(&mut bytes);
            additional_and_extra_params.encode_extra_to(&mut bytes);
            additional_and_extra_params.encode_additional_to(&mut bytes);
            if bytes.len() > 256 {
                signer.sign(&sp_core::blake2_256(&bytes))
            } else {
                signer.sign(&bytes)
            }
        };

        tracing::info!("xt signature: {}", hex::encode(signature.encode()));

        // 5. Encode extrinsic, now that we have the parts we need. This is compatible
        //    with the Encode impl for UncheckedExtrinsic (protocol version 4).
        let extrinsic = {
            let mut encoded_inner = Vec::new();
            // "is signed" + transaction protocol version (4)
            (0b10000000 + 4u8).encode_to(&mut encoded_inner);
            // from address for signature
            signer.address().encode_to(&mut encoded_inner);
            // the signature bytes
            signature.encode_to(&mut encoded_inner);
            // attach custom extra params
            additional_and_extra_params.encode_extra_to(&mut encoded_inner);
            // and now, call data
            call_data.encode_to(&mut encoded_inner);
            // now, prefix byte length:
            let len = Compact(
                u32::try_from(encoded_inner.len())
                    .expect("extrinsic size expected to be <4GB"),
            );
            let mut encoded = Vec::new();
            len.encode_to(&mut encoded);
            encoded.extend(encoded_inner);
            encoded
        };

        // Wrap in Encoded to ensure that any more "encode" calls leave it in the right state.
        // maybe we can just return the raw bytes..
        Ok(SignedSubmittableExtrinsic {
            client: self.client,
            encoded: Encoded(extrinsic),
            marker: self.marker,
        })
    }
}

pub struct SignedSubmittableExtrinsic<'client, T: Config, X, E: Decode, Evs: Decode> {
    client: &'client Client<T>,
    encoded: Encoded,
    marker: std::marker::PhantomData<(X, E, Evs)>,
}

impl<'client, T, X, E, Evs> SignedSubmittableExtrinsic<'client, T, X, E, Evs>
where
    T: Config,
    X: ExtrinsicParams<T>,
    E: Decode + HasModuleError,
    Evs: Decode,
{
    /// Submits the extrinsic to the chain.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn submit_and_watch(
        &self,
    ) -> Result<TransactionProgress<'client, T, E, Evs>, BasicError> {
        // Get a hash of the extrinsic (we'll need this later).
        let ext_hash = T::Hashing::hash_of(&self.encoded);

        // Submit and watch for transaction progress.
        let sub = self.client.rpc().watch_extrinsic(&self.encoded).await?;

        Ok(TransactionProgress::new(sub, self.client, ext_hash))
    }

    /// Submits the extrinsic to the chain for block inclusion.
    ///
    /// Returns `Ok` with the extrinsic hash if it is valid extrinsic.
    ///
    /// # Note
    ///
    /// Success does not mean the extrinsic has been included in the block, just that it is valid
    /// and has been included in the transaction pool.
    pub async fn submit(&self) -> Result<T::Hash, BasicError> {
        self.client.rpc().submit_extrinsic(&self.encoded).await
    }

    /// Submits the extrinsic to the dry_run RPC, to test if it would succeed.
    ///
    /// Returns `Ok` with an [`ApplyExtrinsicResult`], which is the result of applying of an extrinsic.
    pub async fn dry_run(
        &self,
        at: Option<T::Hash>,
    ) -> Result<ApplyExtrinsicResult, BasicError> {
        self.client.rpc().dry_run(self.encoded(), at).await
    }

    /// Returns the SCALE encoded extrinsic bytes.
    pub fn encoded(&self) -> &[u8] {
        &self.encoded.0
    }
}
