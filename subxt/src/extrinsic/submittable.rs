// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use sp_runtime::{
    traits::Hash,
    ApplyExtrinsicResult,
};

use crate::{
    client::{
        OfflineClient,
        OnlineClient,
    },
    error::{
        BasicError,
        HasModuleError,
    },
    extrinsic::{
        ExtrinsicParams,
        Signer,
    },
    metadata::{
        EncodeWithMetadata,
    },
    transaction::TransactionProgress,
    Config,
    Encoded,
};
use codec::{
    Compact,
    Decode,
    Encode,
};

/// A constructed call ready to be signed and submitted.
pub struct SubmittableExtrinsic<C, Err = (), Evs = ()> {
    call: C,
    marker: std::marker::PhantomData<(Err, Evs)>,
}

impl<C, Err, Evs> SubmittableExtrinsic<C, Err, Evs>
where
    C: EncodeWithMetadata,
    Evs: Decode,
    Err: Decode + HasModuleError,
{
    /// Create a new [`SubmittableExtrinsic`], given some call data that can be SCALE
    /// encoded with the help of our [`Metadata`].
    pub fn new(call: C) -> Self {
        SubmittableExtrinsic {
            call,
            marker: Default::default(),
        }
    }

    /// Creates and signs an extrinsic and submits it to the chain. Passes default parameters
    /// to construct the "signed extra" and "additional" payloads needed by the extrinsic.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch_default<Client, T>(
        &self,
        client: Client,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<TransactionProgress<T, Err, Evs>, BasicError>
    where
        Client: Into<OnlineClient<T>>,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams: Default,
        T: Config,
    {
        self.sign_and_submit_then_watch(client, signer, Default::default())
            .await
    }

    /// Creates and signs an extrinsic and submits it to the chain.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch<Client, T>(
        &self,
        client: Client,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<TransactionProgress<T, Err, Evs>, BasicError>
    where
        Client: Into<OnlineClient<T>>,
        T: Config,
    {
        self.create_signed(client, signer, other_params)
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
    pub async fn sign_and_submit_default<Client, T>(
        &self,
        client: Client,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<T::Hash, BasicError>
    where
        Client: Into<OnlineClient<T>>,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams: Default,
        T: Config,
    {
        self.sign_and_submit(client, signer, Default::default()).await
    }

    /// Creates and signs an extrinsic and submits to the chain for block inclusion.
    ///
    /// Returns `Ok` with the extrinsic hash if it is valid extrinsic.
    ///
    /// # Note
    ///
    /// Success does not mean the extrinsic has been included in the block, just that it is valid
    /// and has been included in the transaction pool.
    pub async fn sign_and_submit<Client, T>(
        &self,
        client: Client,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<T::Hash, BasicError>
    where
        Client: Into<OnlineClient<T>>,
        T: Config,
    {
        self.create_signed(client, signer, other_params)
            .await?
            .submit()
            .await
    }

    /// Return the SCALE encoded bytes representing the call data of the transaction.
    pub fn call_data<Client, T>(&self, client: Client) -> Result<Vec<u8>, BasicError>
    where
        Client: Into<OfflineClient<T>>,
        T: Config,
    {
        let client = client.into();
        let metadata = client.metadata();
        let bytes = self.call.encode_with_metadata(metadata)?;
        Ok(bytes)
    }

    /// Creates a returns a raw signed extrinsic, without submitting it.
    pub async fn create_signed<Client, T>(
        &self,
        client: Client,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<SignedSubmittableExtrinsic<T, Err, Evs>, BasicError>
    where
        Client: Into<OnlineClient<T>>,
        T: Config,
    {
        let client: OnlineClient<T> = client.into();

        // 1. Get nonce
        let account_nonce = if let Some(nonce) = signer.nonce() {
            nonce
        } else {
            client
                .rpc()
                .system_account_next_index(signer.account_id())
                .await?
        };

        // 2. SCALE encode call data to bytes (pallet u8, call u8, call params).
        let call_data = Encoded(self.call_data(client.offline())?);

        // 3. Construct our custom additional/extra params.
        let additional_and_extra_params = {
            // Obtain spec version and transaction version from the runtime version of the client.
            let runtime = client.runtime_version();
            <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::new(
                runtime.spec_version,
                runtime.transaction_version,
                account_nonce,
                client.genesis_hash(),
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
            client,
            encoded: Encoded(extrinsic),
            marker: self.marker,
        })
    }
}

/// This represents an extrinsic that has been signed and is ready to submit.
pub struct SignedSubmittableExtrinsic<T: Config, Err: Decode, Evs: Decode> {
    client: OnlineClient<T>,
    encoded: Encoded,
    marker: std::marker::PhantomData<(Err, Evs)>,
}

impl<'client, T, Err, Evs> SignedSubmittableExtrinsic<T, Err, Evs>
where
    T: Config,
    Err: Decode + HasModuleError,
    Evs: Decode,
{
    /// Submits the extrinsic to the chain.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn submit_and_watch(
        &self,
    ) -> Result<TransactionProgress<T, Err, Evs>, BasicError> {
        // Get a hash of the extrinsic (we'll need this later).
        let ext_hash = T::Hashing::hash_of(&self.encoded);

        // Submit and watch for transaction progress.
        let sub = self.client.rpc().watch_extrinsic(&self.encoded).await?;

        Ok(TransactionProgress::new(sub, self.client.clone(), ext_hash))
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
