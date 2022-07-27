// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::TxPayload;
use crate::{
    client::{
        OfflineClientT,
        OnlineClientT,
    },
    error::Error,
    tx::{
        ExtrinsicParams,
        Signer,
        TxProgress,
    },
    utils::{
        Encoded,
        PhantomDataSendSync,
    },
    Config,
};
use codec::{
    Compact,
    Encode,
};
use derivative::Derivative;
use sp_runtime::{
    traits::Hash,
    ApplyExtrinsicResult,
};

/// A client for working with transactions.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct TxClient<T: Config, Client> {
    client: Client,
    _marker: PhantomDataSendSync<T>,
}

impl<T: Config, Client> TxClient<T, Client> {
    /// Create a new [`TxClient`]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: PhantomDataSendSync::new(),
        }
    }
}

impl<T: Config, C: OfflineClientT<T>> TxClient<T, C> {
    /// Run the validation logic against some extrinsic you'd like to submit. Returns `Ok(())`
    /// if the call is valid (or if it's not possible to check since the call has no validation hash).
    /// Return an error if the call was not valid or something went wrong trying to validate it (ie
    /// the pallet or call in question do not exist at all).
    pub fn validate<Call>(&self, call: &Call) -> Result<(), Error>
    where
        Call: TxPayload,
    {
        if let Some(actual_hash) = call.validation_hash() {
            let metadata = self.client.metadata();
            let expected_hash =
                metadata.call_hash(call.pallet_name(), call.call_name())?;
            if actual_hash != expected_hash {
                return Err(crate::metadata::MetadataError::IncompatibleMetadata.into())
            }
        }
        Ok(())
    }

    /// Return the SCALE encoded bytes representing the call data of the transaction.
    pub fn call_data<Call>(&self, call: &Call) -> Result<Vec<u8>, Error>
    where
        Call: TxPayload,
    {
        let metadata = self.client.metadata();
        let mut bytes = Vec::new();
        call.encode_call_data(&metadata, &mut bytes)?;
        Ok(bytes)
    }

    /// Creates a raw signed extrinsic, without submitting it.
    pub async fn create_signed_with_nonce<Call>(
        &self,
        call: &Call,
        signer: &(dyn Signer<T> + Send + Sync),
        account_nonce: T::Index,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<SignedSubmittableExtrinsic<T, C>, Error>
    where
        Call: TxPayload,
    {
        // 1. Validate this call against the current node metadata if the call comes
        // with a hash allowing us to do so.
        self.validate(call)?;

        // 2. SCALE encode call data to bytes (pallet u8, call u8, call params).
        let call_data = Encoded(self.call_data(call)?);

        // 3. Construct our custom additional/extra params.
        let additional_and_extra_params = {
            // Obtain spec version and transaction version from the runtime version of the client.
            let runtime = self.client.runtime_version();
            <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::new(
                runtime.spec_version,
                runtime.transaction_version,
                account_nonce,
                self.client.genesis_hash(),
                other_params,
            )
        };

        tracing::debug!(
            "tx additional_and_extra_params: {:?}",
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

        tracing::debug!("tx signature: {}", hex::encode(signature.encode()));

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
            client: self.client.clone(),
            encoded: Encoded(extrinsic),
            marker: std::marker::PhantomData,
        })
    }
}

impl<T: Config, C: OnlineClientT<T>> TxClient<T, C> {
    /// Creates a raw signed extrinsic, without submitting it.
    pub async fn create_signed<Call>(
        &self,
        call: &Call,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<SignedSubmittableExtrinsic<T, C>, Error>
    where
        Call: TxPayload,
    {
        // Get nonce from the node.
        let account_nonce = if let Some(nonce) = signer.nonce() {
            nonce
        } else {
            self.client
                .rpc()
                .system_account_next_index(signer.account_id())
                .await?
        };

        self.create_signed_with_nonce(call, signer, account_nonce, other_params)
            .await
    }

    /// Creates and signs an extrinsic and submits it to the chain. Passes default parameters
    /// to construct the "signed extra" and "additional" payloads needed by the extrinsic.
    ///
    /// Returns a [`TxProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch_default<Call>(
        &self,
        call: &Call,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<TxProgress<T, C>, Error>
    where
        Call: TxPayload,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams: Default,
    {
        self.sign_and_submit_then_watch(call, signer, Default::default())
            .await
    }

    /// Creates and signs an extrinsic and submits it to the chain.
    ///
    /// Returns a [`TxProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch<Call>(
        &self,
        call: &Call,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<TxProgress<T, C>, Error>
    where
        Call: TxPayload,
    {
        self.create_signed(call, signer, other_params)
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
    pub async fn sign_and_submit_default<Call>(
        &self,
        call: &Call,
        signer: &(dyn Signer<T> + Send + Sync),
    ) -> Result<T::Hash, Error>
    where
        Call: TxPayload,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams: Default,
    {
        self.sign_and_submit(call, signer, Default::default()).await
    }

    /// Creates and signs an extrinsic and submits to the chain for block inclusion.
    ///
    /// Returns `Ok` with the extrinsic hash if it is valid extrinsic.
    ///
    /// # Note
    ///
    /// Success does not mean the extrinsic has been included in the block, just that it is valid
    /// and has been included in the transaction pool.
    pub async fn sign_and_submit<Call>(
        &self,
        call: &Call,
        signer: &(dyn Signer<T> + Send + Sync),
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<T::Hash, Error>
    where
        Call: TxPayload,
    {
        self.create_signed(call, signer, other_params)
            .await?
            .submit()
            .await
    }
}

/// This represents an extrinsic that has been signed and is ready to submit.
pub struct SignedSubmittableExtrinsic<T, C> {
    client: C,
    encoded: Encoded,
    marker: std::marker::PhantomData<T>,
}

impl<T, C> SignedSubmittableExtrinsic<T, C>
where
    T: Config,
    C: OnlineClientT<T>,
{
    /// Submits the extrinsic to the chain.
    ///
    /// Returns a [`TxProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn submit_and_watch(&self) -> Result<TxProgress<T, C>, Error> {
        // Get a hash of the extrinsic (we'll need this later).
        let ext_hash = T::Hashing::hash_of(&self.encoded);

        // Submit and watch for transaction progress.
        let sub = self.client.rpc().watch_extrinsic(&self.encoded).await?;

        Ok(TxProgress::new(sub, self.client.clone(), ext_hash))
    }

    /// Submits the extrinsic to the chain for block inclusion.
    ///
    /// Returns `Ok` with the extrinsic hash if it is valid extrinsic.
    ///
    /// # Note
    ///
    /// Success does not mean the extrinsic has been included in the block, just that it is valid
    /// and has been included in the transaction pool.
    pub async fn submit(&self) -> Result<T::Hash, Error> {
        self.client.rpc().submit_extrinsic(&self.encoded).await
    }

    /// Submits the extrinsic to the dry_run RPC, to test if it would succeed.
    ///
    /// Returns `Ok` with an [`ApplyExtrinsicResult`], which is the result of applying of an extrinsic.
    pub async fn dry_run(
        &self,
        at: Option<T::Hash>,
    ) -> Result<ApplyExtrinsicResult, Error> {
        self.client.rpc().dry_run(self.encoded(), at).await
    }

    /// Returns the SCALE encoded extrinsic bytes.
    pub fn encoded(&self) -> &[u8] {
        &self.encoded.0
    }
}
