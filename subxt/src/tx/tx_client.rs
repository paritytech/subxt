// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::borrow::Cow;

use codec::{Compact, Encode};
use derivative::Derivative;
use sp_core_hashing::blake2_256;

use crate::{
    client::{OfflineClientT, OnlineClientT},
    config::{Config, ExtrinsicParams, Hasher},
    error::{Error, MetadataError},
    tx::{Signer as SignerT, TxPayload, TxProgress},
    utils::{Encoded, PhantomDataSendSync},
};

// This is returned from an API below, so expose it here.
pub use crate::rpc::types::DryRunResult;

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
        if let Some(details) = call.validation_details() {
            let expected_hash = self
                .client
                .metadata()
                .pallet_by_name_err(details.pallet_name)?
                .call_hash(details.call_name)
                .ok_or_else(|| MetadataError::CallNameNotFound(details.call_name.to_owned()))?;

            if details.hash != expected_hash {
                return Err(MetadataError::IncompatibleCodegen.into());
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
        call.encode_call_data_to(&metadata, &mut bytes)?;
        Ok(bytes)
    }

    /// Creates an unsigned extrinsic without submitting it.
    pub fn create_unsigned<Call>(&self, call: &Call) -> Result<SubmittableExtrinsic<T, C>, Error>
    where
        Call: TxPayload,
    {
        // 1. Validate this call against the current node metadata if the call comes
        // with a hash allowing us to do so.
        self.validate(call)?;

        // 2. Encode extrinsic
        let extrinsic = {
            let mut encoded_inner = Vec::new();
            // transaction protocol version (4) (is not signed, so no 1 bit at the front).
            4u8.encode_to(&mut encoded_inner);
            // encode call data after this byte.
            call.encode_call_data_to(&self.client.metadata(), &mut encoded_inner)?;
            // now, prefix byte length:
            let len = Compact(
                u32::try_from(encoded_inner.len()).expect("extrinsic size expected to be <4GB"),
            );
            let mut encoded = Vec::new();
            len.encode_to(&mut encoded);
            encoded.extend(encoded_inner);
            encoded
        };

        // Wrap in Encoded to ensure that any more "encode" calls leave it in the right state.
        Ok(SubmittableExtrinsic::from_bytes(
            self.client.clone(),
            extrinsic,
        ))
    }

    /// Create a partial extrinsic.
    pub fn create_partial_signed_with_nonce<Call>(
        &self,
        call: &Call,
        account_nonce: T::Index,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<PartialExtrinsic<T, C>, Error>
    where
        Call: TxPayload,
    {
        // 1. Validate this call against the current node metadata if the call comes
        // with a hash allowing us to do so.
        self.validate(call)?;

        // 2. SCALE encode call data to bytes (pallet u8, call u8, call params).
        let call_data = self.call_data(call)?;

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

        // Return these details, ready to construct a signed extrinsic from.
        Ok(PartialExtrinsic {
            client: self.client.clone(),
            call_data,
            additional_and_extra_params,
        })
    }

    /// Creates a signed extrinsic without submitting it.
    pub fn create_signed_with_nonce<Call, Signer>(
        &self,
        call: &Call,
        signer: &Signer,
        account_nonce: T::Index,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<SubmittableExtrinsic<T, C>, Error>
    where
        Call: TxPayload,
        Signer: SignerT<T>,
    {
        // 1. Validate this call against the current node metadata if the call comes
        // with a hash allowing us to do so.
        self.validate(call)?;

        // 2. Gather the "additional" and "extra" params along with the encoded call data,
        //    ready to be signed.
        let partial_signed =
            self.create_partial_signed_with_nonce(call, account_nonce, other_params)?;

        // 3. Sign and construct an extrinsic from these details.
        Ok(partial_signed.sign(signer))
    }
}

impl<T, C> TxClient<T, C>
where
    T: Config,
    C: OnlineClientT<T>,
{
    /// Get the account nonce for a given account ID.
    pub async fn account_nonce(&self, account_id: &T::AccountId) -> Result<T::Index, Error> {
        self.client
            .rpc()
            .state_call(
                "AccountNonceApi_account_nonce",
                Some(&account_id.encode()),
                None,
            )
            .await
    }

    /// Creates a partial signed extrinsic, without submitting it.
    pub async fn create_partial_signed<Call>(
        &self,
        call: &Call,
        account_id: &T::AccountId,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<PartialExtrinsic<T, C>, Error>
    where
        Call: TxPayload,
    {
        let account_nonce = self.account_nonce(account_id).await?;
        self.create_partial_signed_with_nonce(call, account_nonce, other_params)
    }

    /// Creates a signed extrinsic, without submitting it.
    pub async fn create_signed<Call, Signer>(
        &self,
        call: &Call,
        signer: &Signer,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<SubmittableExtrinsic<T, C>, Error>
    where
        Call: TxPayload,
        Signer: SignerT<T>,
    {
        let account_nonce = self.account_nonce(&signer.account_id()).await?;
        self.create_signed_with_nonce(call, signer, account_nonce, other_params)
    }

    /// Creates and signs an extrinsic and submits it to the chain. Passes default parameters
    /// to construct the "signed extra" and "additional" payloads needed by the extrinsic.
    ///
    /// Returns a [`TxProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch_default<Call, Signer>(
        &self,
        call: &Call,
        signer: &Signer,
    ) -> Result<TxProgress<T, C>, Error>
    where
        Call: TxPayload,
        Signer: SignerT<T>,
        <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams: Default,
    {
        self.sign_and_submit_then_watch(call, signer, Default::default())
            .await
    }

    /// Creates and signs an extrinsic and submits it to the chain.
    ///
    /// Returns a [`TxProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch<Call, Signer>(
        &self,
        call: &Call,
        signer: &Signer,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<TxProgress<T, C>, Error>
    where
        Call: TxPayload,
        Signer: SignerT<T>,
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
    pub async fn sign_and_submit_default<Call, Signer>(
        &self,
        call: &Call,
        signer: &Signer,
    ) -> Result<T::Hash, Error>
    where
        Call: TxPayload,
        Signer: SignerT<T>,
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
    pub async fn sign_and_submit<Call, Signer>(
        &self,
        call: &Call,
        signer: &Signer,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams,
    ) -> Result<T::Hash, Error>
    where
        Call: TxPayload,
        Signer: SignerT<T>,
    {
        self.create_signed(call, signer, other_params)
            .await?
            .submit()
            .await
    }
}

/// This payload contains the information needed to produce an extrinsic.
pub struct PartialExtrinsic<T: Config, C> {
    client: C,
    call_data: Vec<u8>,
    additional_and_extra_params: T::ExtrinsicParams,
}

impl<T, C> PartialExtrinsic<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    // Obtain bytes representing the signer payload and run call some function
    // with them. This can avoid an allocation in some cases when compared to
    // [`PartialExtrinsic::signer_payload()`].
    fn with_signer_payload<F, R>(&self, f: F) -> R
    where
        F: for<'a> FnOnce(Cow<'a, [u8]>) -> R,
    {
        let mut bytes = self.call_data.clone();
        self.additional_and_extra_params.encode_extra_to(&mut bytes);
        self.additional_and_extra_params
            .encode_additional_to(&mut bytes);
        if bytes.len() > 256 {
            f(Cow::Borrowed(blake2_256(&bytes).as_ref()))
        } else {
            f(Cow::Owned(bytes))
        }
    }

    /// Return the signer payload for this extrinsic. These are the bytes that must
    /// be signed in order to produce a valid signature for the extrinsic.
    pub fn signer_payload(&self) -> Vec<u8> {
        self.with_signer_payload(|bytes| bytes.to_vec())
    }

    /// Return the bytes representing the call data for this partially constructed
    /// extrinsic.
    pub fn call_data(&self) -> &[u8] {
        &self.call_data
    }

    /// Convert this [`PartialExtrinsic`] into a [`SubmittableExtrinsic`], ready to submit.
    /// The provided `signer` is responsible for providing the "from" address for the transaction,
    /// as well as providing a signature to attach to it.
    pub fn sign<Signer>(&self, signer: &Signer) -> SubmittableExtrinsic<T, C>
    where
        Signer: SignerT<T>,
    {
        // Given our signer, we can sign the payload representing this extrinsic.
        let signature = self.with_signer_payload(|bytes| signer.sign(&bytes));
        // Now, use the signature and "from" address to build the extrinsic.
        self.sign_with_address_and_signature(&signer.address(), &signature)
    }

    /// Convert this [`PartialExtrinsic`] into a [`SubmittableExtrinsic`], ready to submit.
    /// An address, and something representing a signature that can be SCALE encoded, are both
    /// needed in order to construct it. If you have a `Signer` to hand, you can use
    /// [`PartialExtrinsic::sign()`] instead.
    pub fn sign_with_address_and_signature(
        &self,
        address: &T::Address,
        signature: &T::Signature,
    ) -> SubmittableExtrinsic<T, C> {
        // Encode the extrinsic (into the format expected by protocol version 4)
        let extrinsic = {
            let mut encoded_inner = Vec::new();
            // "is signed" + transaction protocol version (4)
            (0b10000000 + 4u8).encode_to(&mut encoded_inner);
            // from address for signature
            address.encode_to(&mut encoded_inner);
            // the signature
            signature.encode_to(&mut encoded_inner);
            // attach custom extra params
            self.additional_and_extra_params
                .encode_extra_to(&mut encoded_inner);
            // and now, call data (remembering that it's been encoded already and just needs appending)
            encoded_inner.extend(&self.call_data);
            // now, prefix byte length:
            let len = Compact(
                u32::try_from(encoded_inner.len()).expect("extrinsic size expected to be <4GB"),
            );
            let mut encoded = Vec::new();
            len.encode_to(&mut encoded);
            encoded.extend(encoded_inner);
            encoded
        };

        // Return an extrinsic ready to be submitted.
        SubmittableExtrinsic::from_bytes(self.client.clone(), extrinsic)
    }
}

/// This represents an extrinsic that has been signed and is ready to submit.
pub struct SubmittableExtrinsic<T, C> {
    client: C,
    encoded: Encoded,
    marker: std::marker::PhantomData<T>,
}

impl<T, C> SubmittableExtrinsic<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    /// Create a [`SubmittableExtrinsic`] from some already-signed and prepared
    /// extrinsic bytes, and some client (anything implementing [`OfflineClientT`]
    /// or [`OnlineClientT`]).
    ///
    /// Prefer to use [`TxClient`] to create and sign extrinsics. This is simply
    /// exposed in case you want to skip this process and submit something you've
    /// already created.
    pub fn from_bytes(client: C, tx_bytes: Vec<u8>) -> Self {
        Self {
            client,
            encoded: Encoded(tx_bytes),
            marker: std::marker::PhantomData,
        }
    }

    /// Returns the SCALE encoded extrinsic bytes.
    pub fn encoded(&self) -> &[u8] {
        &self.encoded.0
    }

    /// Consumes [`SubmittableExtrinsic`] and returns the SCALE encoded
    /// extrinsic bytes.
    pub fn into_encoded(self) -> Vec<u8> {
        self.encoded.0
    }
}

impl<T, C> SubmittableExtrinsic<T, C>
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
        let ext_hash = T::Hasher::hash_of(&self.encoded);

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
    /// Returns `Ok` with a [`DryRunResult`], which is the result of attempting to dry run the extrinsic.
    pub async fn dry_run(&self, at: Option<T::Hash>) -> Result<DryRunResult, Error> {
        let dry_run_bytes = self.client.rpc().dry_run(self.encoded(), at).await?;
        dry_run_bytes.into_dry_run_result(&self.client.metadata())
    }

    /// This returns an estimate for what the extrinsic is expected to cost to execute, less any tips.
    /// The actual amount paid can vary from block to block based on node traffic and other factors.
    pub async fn partial_fee_estimate(&self) -> Result<u128, Error> {
        let mut params = self.encoded().to_vec();
        (self.encoded().len() as u32).encode_to(&mut params);
        // destructuring RuntimeDispatchInfo, see type information <https://paritytech.github.io/substrate/master/pallet_transaction_payment_rpc_runtime_api/struct.RuntimeDispatchInfo.html>
        // data layout: {weight_ref_time: Compact<u64>, weight_proof_size: Compact<u64>, class: u8, partial_fee: u128}
        let (_, _, _, partial_fee) = self
            .client
            .rpc()
            .state_call::<(Compact<u64>, Compact<u64>, u8, u128)>(
                "TransactionPaymentApi_query_info",
                Some(&params),
                None,
            )
            .await?;
        Ok(partial_fee)
    }
}
