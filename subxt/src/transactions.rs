//! This module exposes [`TransactionsClient`], which has methods for constructing and submitting
//! transactions. It's created by calling [`crate::client::ClientAtBlock::transactions()`], or
//! [`crate::client::ClientAtBlock::tx()`] for short.
//!
//! ```rust,no_run
//! pub use subxt::{OnlineClient, PolkadotConfig};
//!
//! let client = OnlineClient::new().await?;
//! let at_block = client.at_current_block().await?;
//!
//! let transactions = at_block.transactions();
//! ```

mod account_nonce;
mod default_params;
mod payload;
mod signer;
mod transaction_progress;
mod validation_result;

use crate::backend::{BackendExt, TransactionStatus as BackendTransactionStatus};
use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::extrinsic_params::Params;
use crate::config::{
    ClientState, Config, ExtrinsicParams, ExtrinsicParamsEncoder, HashFor, Hasher, Header,
};
use crate::error::{ExtrinsicError, TransactionStatusError};
use codec::{Compact, Encode};
use core::marker::PhantomData;
use futures::{TryFutureExt, future::try_join};
use sp_crypto_hashing::blake2_256;
use std::borrow::Cow;

pub use default_params::DefaultParams;
pub use payload::{DynamicPayload, Payload, StaticPayload, dynamic};
pub use signer::Signer;
pub use transaction_progress::{TransactionProgress, TransactionStatus};
pub use validation_result::{
    TransactionInvalid, TransactionUnknown, TransactionValid, ValidationResult,
};

/// A client for working with transactions. See [the module docs](crate::transactions) for more.
#[derive(Clone)]
pub struct TransactionsClient<'atblock, T, Client> {
    client: &'atblock Client,
    marker: PhantomData<T>,
}

impl<'atblock, T, Client> TransactionsClient<'atblock, T, Client> {
    pub(crate) fn new(client: &'atblock Client) -> Self {
        TransactionsClient {
            client,
            marker: PhantomData,
        }
    }
}

impl<'atblock, T: Config, Client: OfflineClientAtBlockT<T>>
    TransactionsClient<'atblock, T, Client>
{
    /// Run the validation logic against some transaction you'd like to submit. Returns `Ok(())`
    /// if the call is valid (or if it's not possible to check since the call has no validation hash).
    /// Return an error if the call was not valid or something went wrong trying to validate it (ie
    /// the pallet or call in question do not exist at all).
    pub fn validate<Call>(&self, call: &Call) -> Result<(), ExtrinsicError>
    where
        Call: Payload,
    {
        let Some(details) = call.validation_details() else {
            return Ok(());
        };

        let pallet_name = details.pallet_name;
        let call_name = details.call_name;

        let expected_hash = self
            .client
            .metadata_ref()
            .pallet_by_name(pallet_name)
            .ok_or_else(|| ExtrinsicError::PalletNameNotFound(pallet_name.to_string()))?
            .call_hash(call_name)
            .ok_or_else(|| ExtrinsicError::CallNameNotFound {
                pallet_name: pallet_name.to_string(),
                call_name: call_name.to_string(),
            })?;

        if details.hash != expected_hash {
            Err(ExtrinsicError::IncompatibleCodegen)
        } else {
            Ok(())
        }
    }

    /// Create a [`SubmittableTransaction`] from some already-signed and prepared
    /// transaction bytes, and some client (anything implementing [`OfflineClientAtBlockT`]
    /// or [`OnlineClientAtBlockT`]).
    pub fn from_bytes(
        client: &'atblock Client,
        tx_bytes: Vec<u8>,
    ) -> SubmittableTransaction<'atblock, T, Client> {
        SubmittableTransaction {
            client,
            encoded: tx_bytes,
            marker: PhantomData,
        }
    }

    /// Return the SCALE encoded bytes representing the call data of the transaction.
    pub fn call_data<Call>(&self, call: &Call) -> Result<Vec<u8>, ExtrinsicError>
    where
        Call: Payload,
    {
        let mut bytes = Vec::new();
        let metadata = self.client.metadata_ref();
        call.encode_call_data_to(metadata, &mut bytes)?;
        Ok(bytes)
    }

    /// Creates an unsigned transaction without submitting it. Depending on the metadata, we might end
    /// up constructing either a v4 or v5 transaction. See [`Self::create_v4_unsigned`] or
    /// [`Self::create_v5_unsigned`] if you'd like to explicitly create an unsigned transaction of a certain version.
    pub fn create_unsigned<Call>(
        &self,
        call: &Call,
    ) -> Result<SubmittableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        let tx = match self.default_transaction_version()? {
            SupportedTransactionVersion::V4 => self.create_v4_unsigned(call),
            SupportedTransactionVersion::V5 => self.create_v5_unsigned(call),
        }?;

        Ok(tx)
    }

    /// Creates a V4 "unsigned" transaction without submitting it.
    pub fn create_v4_unsigned<Call>(
        &self,
        call: &Call,
    ) -> Result<SubmittableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.create_unsigned_at_version(call, SupportedTransactionVersion::V4)
    }

    /// Creates a V5 "bare" transaction without submitting it.
    pub fn create_v5_unsigned<Call>(
        &self,
        call: &Call,
    ) -> Result<SubmittableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.create_unsigned_at_version(call, SupportedTransactionVersion::V5)
    }

    /// Create a signable transaction. Depending on the metadata, we might end up constructing either a v4 or
    /// v5 transaction. Use [`Self::create_v4_signable_offline`] or [`Self::create_v5_signable_offline`] if you'd
    /// like to manually use a specific version.
    ///
    /// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
    /// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
    pub fn create_signable_offline<Call>(
        &self,
        call: &Call,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<SignableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        match self.default_transaction_version()? {
            SupportedTransactionVersion::V4 => self.create_v4_signable_offline(call, params),
            SupportedTransactionVersion::V5 => self.create_v5_signable_offline(call, params),
        }
    }

    /// Create a v4 partial transaction, ready to sign.
    ///
    /// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
    /// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
    ///
    /// Prefer [`Self::create_signable_offline()`] if you don't know which version to create; this will pick the
    /// most suitable one for the given chain.
    pub fn create_v4_signable_offline<Call>(
        &self,
        call: &Call,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<SignableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.create_signable_at_version(call, params, SupportedTransactionVersion::V4)
    }

    /// Create a v5 partial transaction, ready to sign.
    ///
    /// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
    /// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
    ///
    /// Prefer [`Self::create_signable_offline()`] if you don't know which version to create; this will pick the
    /// most suitable one for the given chain.
    pub fn create_v5_signable_offline<Call>(
        &self,
        call: &Call,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<SignableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.create_signable_at_version(call, params, SupportedTransactionVersion::V5)
    }

    /// Returns the suggested transaction versions to build for a given chain, or an error
    /// if Subxt doesn't support any version expected by the chain.
    ///
    /// When using methods like [`Self::create_signable_offline`] and [`Self::create_unsigned`],
    /// this will be used internally to decide which transaction version to construct.
    pub fn default_transaction_version(
        &self,
    ) -> Result<SupportedTransactionVersion, ExtrinsicError> {
        let metadata = self.client.metadata_ref();
        let versions = metadata.extrinsic().supported_versions();

        if versions.contains(&4) {
            Ok(SupportedTransactionVersion::V4)
        } else if versions.contains(&5) {
            Ok(SupportedTransactionVersion::V5)
        } else {
            Err(ExtrinsicError::UnsupportedVersion)
        }
    }

    // Create a V4 "unsigned" transaction or V5 "bare" transaction.
    fn create_unsigned_at_version<Call: Payload>(
        &self,
        call: &Call,
        tx_version: SupportedTransactionVersion,
    ) -> Result<SubmittableTransaction<'atblock, T, Client>, ExtrinsicError> {
        let metadata = self.client.metadata_ref();

        // 1. Validate this call against the current node metadata if the call comes
        // with a hash allowing us to do so.
        self.validate(call)?;

        // 2. Encode extrinsic
        let extrinsic = {
            let mut encoded_inner = Vec::new();
            // encode the transaction version first.
            (tx_version as u8).encode_to(&mut encoded_inner);
            // encode call data after this byte.
            call.encode_call_data_to(metadata, &mut encoded_inner)?;
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
        Ok(SubmittableTransaction {
            client: self.client,
            encoded: extrinsic,
            marker: PhantomData,
        })
    }

    // Create a V4 "signed" or a V5 "general" transaction.
    fn create_signable_at_version<Call>(
        &self,
        call: &Call,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
        tx_version: SupportedTransactionVersion,
    ) -> Result<SignableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        // 1. Validate this call against the current node metadata if the call comes
        // with a hash allowing us to do so.
        self.validate(call)?;

        // 2. Work out which TX extension version to target based on metadata.
        let tx_extension_version = match tx_version {
            SupportedTransactionVersion::V4 => None,
            SupportedTransactionVersion::V5 => {
                let v = self
                    .client
                    .metadata_ref()
                    .extrinsic()
                    .transaction_extension_version_to_use_for_encoding();
                Some(v)
            }
        };

        // 3. SCALE encode call data to bytes (pallet u8, call u8, call params).
        let call_data = self.call_data(call)?;

        // 4. Construct our custom additional/extra params.
        let client_state = ClientState {
            genesis_hash: self
                .client
                .genesis_hash()
                .ok_or(ExtrinsicError::GenesisHashNotProvided)?,
            spec_version: self.client.spec_version(),
            transaction_version: self.client.transaction_version(),
            metadata: self.client.metadata(),
        };
        let additional_and_extra_params =
            <T::ExtrinsicParams as ExtrinsicParams<T>>::new(&client_state, params)?;

        // Return these details, ready to construct a signed extrinsic from.
        Ok(SignableTransaction {
            client: self.client,
            call_data,
            additional_and_extra_params,
            tx_extension_version,
        })
    }
}

impl<'atblock, T: Config, Client: OnlineClientAtBlockT<T>> TransactionsClient<'atblock, T, Client> {
    /// Get the account nonce for a given account ID.
    pub async fn account_nonce(&self, account_id: &T::AccountId) -> Result<u64, ExtrinsicError> {
        account_nonce::get_account_nonce(self.client, account_id)
            .await
            .map_err(|e| ExtrinsicError::AccountNonceError {
                block_hash: self.client.block_hash().into(),
                account_id: account_id.clone().encode().into(),
                reason: e,
            })
    }

    /// Creates a signable transaction. This can then be signed and submitted.
    pub async fn create_signable<Call>(
        &self,
        call: &Call,
        account_id: &T::AccountId,
        mut params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<SignableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.inject_account_nonce_and_block(account_id, &mut params)
            .await?;
        self.create_signable_offline(call, params)
    }

    /// Creates a signable V4 transaction, without submitting it. This can then be signed and submitted.
    ///
    /// Prefer [`Self::create_signable()`] if you don't know which version to create; this will pick the
    /// most suitable one for the given chain.
    pub async fn create_v4_signable<Call>(
        &self,
        call: &Call,
        account_id: &T::AccountId,
        mut params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<SignableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.inject_account_nonce_and_block(account_id, &mut params)
            .await?;
        self.create_v4_signable_offline(call, params)
    }

    /// Creates a signable V5 transaction, without submitting it. This can then be signed and submitted.
    ///
    /// Prefer [`Self::create_signable()`] if you don't know which version to create; this will pick the
    /// most suitable one for the given chain.
    pub async fn create_v5_signable<Call>(
        &self,
        call: &Call,
        account_id: &T::AccountId,
        mut params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<SignableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.inject_account_nonce_and_block(account_id, &mut params)
            .await?;
        self.create_v5_signable_offline(call, params)
    }

    /// Creates a signed transaction, without submitting it.
    pub async fn create_signed<Call, S>(
        &mut self,
        call: &Call,
        signer: &S,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<SubmittableTransaction<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
        S: Signer<T>,
    {
        let mut signable = self
            .create_signable(call, &signer.account_id(), params)
            .await?;

        Ok(signable.sign(signer))
    }

    /// Creates and signs an transaction and submits it to the chain. Passes default parameters
    /// to construct the "signed extra" and "additional" payloads needed by the transaction.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch_default<Call, S>(
        &mut self,
        call: &Call,
        signer: &S,
    ) -> Result<TransactionProgress<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
        S: Signer<T>,
        <T::ExtrinsicParams as ExtrinsicParams<T>>::Params: DefaultParams,
    {
        self.sign_and_submit_then_watch(call, signer, DefaultParams::default_params())
            .await
    }

    /// Creates and signs an transaction and submits it to the chain.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn sign_and_submit_then_watch<Call, S>(
        &mut self,
        call: &Call,
        signer: &S,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<TransactionProgress<'atblock, T, Client>, ExtrinsicError>
    where
        Call: Payload,
        S: Signer<T>,
    {
        self.create_signed(call, signer, params)
            .await?
            .submit_and_watch()
            .await
    }

    /// Creates and signs an transaction and submits to the chain for block inclusion. Passes
    /// default parameters to construct the "signed extra" and "additional" payloads needed
    /// by the transaction.
    ///
    /// Returns `Ok` with the transaction hash if it is valid transaction.
    ///
    /// # Note
    ///
    /// Success does not mean the transaction has been included in the block, just that it is valid
    /// and has been included in the transaction pool.
    pub async fn sign_and_submit_default<Call, S>(
        &mut self,
        call: &Call,
        signer: &S,
    ) -> Result<HashFor<T>, ExtrinsicError>
    where
        Call: Payload,
        S: Signer<T>,
        <T::ExtrinsicParams as ExtrinsicParams<T>>::Params: DefaultParams,
    {
        self.sign_and_submit(call, signer, DefaultParams::default_params())
            .await
    }

    /// Creates and signs an transaction and submits to the chain for block inclusion.
    ///
    /// Returns `Ok` with the transaction hash if it is valid transaction.
    ///
    /// # Note
    ///
    /// Success does not mean the transaction has been included in the block, just that it is valid
    /// and has been included in the transaction pool.
    pub async fn sign_and_submit<Call, S>(
        &mut self,
        call: &Call,
        signer: &S,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<HashFor<T>, ExtrinsicError>
    where
        Call: Payload,
        S: Signer<T>,
    {
        self.create_signed(call, signer, params)
            .await?
            .submit()
            .await
    }

    /// Fetch the latest block header and account nonce from the backend and use them to refine [`ExtrinsicParams::Params`].
    async fn inject_account_nonce_and_block(
        &self,
        account_id: &T::AccountId,
        params: &mut <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<(), ExtrinsicError> {
        let block_ref = self
            .client
            .backend()
            .latest_finalized_block_ref()
            .await
            .map_err(ExtrinsicError::CannotGetLatestFinalizedBlock)?;

        let (block_header, account_nonce) = try_join(
            self.client
                .backend()
                .block_header(block_ref.hash())
                .map_err(ExtrinsicError::CannotGetLatestFinalizedBlock),
            self.account_nonce(account_id),
        )
        .await?;

        let block_header = block_header.ok_or_else(|| ExtrinsicError::CannotFindBlockHeader {
            block_hash: block_ref.hash().into(),
        })?;

        params.inject_account_nonce(account_nonce);
        params.inject_block(block_header.number(), block_ref.hash());

        Ok(())
    }
}

/// The transaction versions supported by Subxt.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum SupportedTransactionVersion {
    /// v4 transactions (signed and unsigned transactions)
    V4 = 4u8,
    /// v5 transactions (bare and general transactions)
    V5 = 5u8,
}

/// This is a transaction that requires signing before it can be submitted.
pub struct SignableTransaction<'atblock, T: Config, Client> {
    client: &'atblock Client,
    call_data: Vec<u8>,
    additional_and_extra_params: <T as Config>::ExtrinsicParams,
    // For V4 transactions this doesn't exist, and for V5 it does.
    tx_extension_version: Option<u8>,
}

impl<'atblock, T: Config, Client: OfflineClientAtBlockT<T>>
    SignableTransaction<'atblock, T, Client>
{
    /// Return the bytes representing the call data for this partially constructed
    /// transaction.
    pub fn call_data(&self) -> &[u8] {
        &self.call_data
    }

    /// Return the signer payload for this transaction. These are the bytes that must
    /// be signed in order to produce a valid signature for the transaction.
    pub fn signer_payload(&self) -> Vec<u8> {
        self.with_signer_payload(|bytes| bytes.to_vec())
    }

    /// Convert this [`SignableTransaction`] into a [`SubmittableTransaction`], ready to submit.
    /// The provided `signer` is responsible for providing the "from" address for the transaction,
    /// as well as providing a signature to attach to it.
    pub fn sign<S: Signer<T>>(
        &mut self,
        signer: &S,
    ) -> SubmittableTransaction<'atblock, T, Client> {
        // Given our signer, we can sign the payload representing this extrinsic.
        let signature = signer.sign(&self.signer_payload());
        // Now, use the signature and "from" account to build the extrinsic.
        self.sign_with_account_and_signature(&signer.account_id(), &signature)
    }

    /// Convert this [`PartialTransaction`] into a [`SubmittableTransaction`], ready to submit.
    /// An address, and something representing a signature that can be SCALE encoded, are both
    /// needed in order to construct it. If you have a `Signer` to hand, you can use
    /// [`PartialTransaction::sign()`] instead.
    pub fn sign_with_account_and_signature(
        &mut self,
        account_id: &T::AccountId,
        signature: &T::Signature,
    ) -> SubmittableTransaction<'atblock, T, Client> {
        let encoded = if let Some(tx_extensions_version) = self.tx_extension_version {
            let mut encoded_inner = Vec::new();
            // Pass account and signature to extensions to be added.
            self.additional_and_extra_params
                .inject_signature(account_id, signature);
            // "is general" + transaction protocol version (5)
            (0b01000000 + 5u8).encode_to(&mut encoded_inner);
            // Encode versions for the transaction extensions
            (tx_extensions_version as u8).encode_to(&mut encoded_inner);
            // Encode the actual transaction extensions values
            self.additional_and_extra_params
                .encode_value_to(&mut encoded_inner);
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
        } else {
            let mut encoded_inner = Vec::new();
            // "is signed" + transaction protocol version (4)
            (0b10000000 + 4u8).encode_to(&mut encoded_inner);
            // from address for signature
            let address: T::Address = account_id.clone().into();
            address.encode_to(&mut encoded_inner);
            // the signature
            signature.encode_to(&mut encoded_inner);
            // attach custom extra params
            self.additional_and_extra_params
                .encode_value_to(&mut encoded_inner);
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

        SubmittableTransaction {
            client: self.client,
            encoded,
            marker: PhantomData,
        }
    }

    // Obtain bytes representing the signer payload and run call some function
    // with them. This can avoid an allocation in some cases.
    fn with_signer_payload<F, R>(&self, f: F) -> R
    where
        F: for<'a> FnOnce(Cow<'a, [u8]>) -> R,
    {
        let mut bytes = self.call_data.clone();
        self.additional_and_extra_params
            .encode_signer_payload_value_to(&mut bytes);
        self.additional_and_extra_params
            .encode_implicit_to(&mut bytes);

        // For V5 transactions we _always_ blake2 hash. For V4 we only
        // hash if more than 256 bytes in the payload.
        if self.is_v5() || bytes.len() > 256 {
            f(Cow::Borrowed(&blake2_256(&bytes)))
        } else {
            f(Cow::Owned(bytes))
        }
    }

    // Are we working with a V5 transaction? This is handled a bit differently.
    fn is_v5(&self) -> bool {
        self.tx_extension_version.is_some()
    }
}

/// This is a transaction that is ready to submit.
pub struct SubmittableTransaction<'atblock, T, Client> {
    client: &'atblock Client,
    encoded: Vec<u8>,
    marker: PhantomData<T>,
}

impl<'atblock, T, Client> SubmittableTransaction<'atblock, T, Client>
where
    T: Config,
    Client: OfflineClientAtBlockT<T>,
{
    /// Calculate and return the hash of the transaction, based on the configured hasher.
    pub fn hash(&self) -> HashFor<T> {
        self.client.hasher().hash(&self.encoded)
    }

    /// Returns the SCALE encoded transaction bytes.
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// Consumes [`SubmittableTransaction`] and returns the SCALE encoded
    /// transaction bytes.
    pub fn into_encoded(self) -> Vec<u8> {
        self.encoded.clone()
    }
}

impl<'atblock, T: Config, Client: OnlineClientAtBlockT<T>>
    SubmittableTransaction<'atblock, T, Client>
{
    /// Submits the transaction to the chain.
    ///
    /// Returns a [`TransactionProgress`], which can be used to track the status of the transaction
    /// and obtain details about it, once it has made it into a block.
    pub async fn submit_and_watch(
        &self,
    ) -> Result<TransactionProgress<'atblock, T, Client>, ExtrinsicError> {
        // Get a hash of the transaction (we'll need this later).
        let ext_hash = self.hash();

        // Submit and watch for transaction progress.
        let sub = self
            .client
            .backend()
            .submit_transaction(self.encoded())
            .await
            .map_err(ExtrinsicError::ErrorSubmittingTransaction)?;

        Ok(TransactionProgress::new(sub, self.client, ext_hash))
    }

    /// Submits the transaction to the chain for block inclusion.
    ///
    /// It's usually better to call `submit_and_watch` to get an idea of the progress of the
    /// submission and whether it's eventually successful or not. This call does not guarantee
    /// success, and is just sending the transaction to the chain.
    pub async fn submit(&self) -> Result<HashFor<T>, ExtrinsicError> {
        let ext_hash = self.hash();
        let mut sub = self
            .client
            .backend()
            .submit_transaction(self.encoded())
            .await
            .map_err(ExtrinsicError::ErrorSubmittingTransaction)?;

        // If we get a bad status or error back straight away then error, else return the hash.
        match sub.next().await {
            Some(Ok(status)) => match status {
                BackendTransactionStatus::Validated
                | BackendTransactionStatus::Broadcasted
                | BackendTransactionStatus::InBestBlock { .. }
                | BackendTransactionStatus::NoLongerInBestBlock
                | BackendTransactionStatus::InFinalizedBlock { .. } => Ok(ext_hash),
                BackendTransactionStatus::Error { message } => Err(
                    ExtrinsicError::TransactionStatusError(TransactionStatusError::Error(message)),
                ),
                BackendTransactionStatus::Invalid { message } => {
                    Err(ExtrinsicError::TransactionStatusError(
                        TransactionStatusError::Invalid(message),
                    ))
                }
                BackendTransactionStatus::Dropped { message } => {
                    Err(ExtrinsicError::TransactionStatusError(
                        TransactionStatusError::Dropped(message),
                    ))
                }
            },
            Some(Err(e)) => Err(ExtrinsicError::TransactionStatusStreamError(e)),
            None => Err(ExtrinsicError::UnexpectedEndOfTransactionStatusStream),
        }
    }

    /// Validate a transaction by submitting it to the relevant Runtime API. A transaction that is
    /// valid can be added to a block, but may still end up in an error state.
    ///
    /// Returns `Ok` with a [`ValidationResult`], which is the result of attempting to dry run the transaction.
    pub async fn validate(&self) -> Result<ValidationResult, ExtrinsicError> {
        let block_hash = self.client.block_hash();

        // Approach taken from https://github.com/paritytech/json-rpc-interface-spec/issues/55.
        let mut params = Vec::with_capacity(8 + self.encoded().len() + 8);
        2u8.encode_to(&mut params);
        params.extend(self.encoded().iter());
        block_hash.encode_to(&mut params);

        let res: Vec<u8> = self
            .client
            .backend()
            .call(
                "TaggedTransactionQueue_validate_transaction",
                Some(&params),
                block_hash,
            )
            .await
            .map_err(ExtrinsicError::CannotGetValidationInfo)?;

        ValidationResult::try_from_bytes(res)
    }

    /// This returns an estimate for what the transaction is expected to cost to execute, less any tips.
    /// The actual amount paid can vary from block to block based on node traffic and other factors.
    pub async fn partial_fee_estimate(&self) -> Result<u128, ExtrinsicError> {
        let mut params = self.encoded().to_vec();
        (self.encoded().len() as u32).encode_to(&mut params);
        let latest_block_ref = self
            .client
            .backend()
            .latest_finalized_block_ref()
            .await
            .map_err(ExtrinsicError::CannotGetLatestFinalizedBlock)?;

        // destructuring RuntimeDispatchInfo, see type information <https://paritytech.github.io/substrate/master/pallet_transaction_payment_rpc_runtime_api/struct.RuntimeDispatchInfo.html>
        // data layout: {weight_ref_time: Compact<u64>, weight_proof_size: Compact<u64>, class: u8, partial_fee: u128}
        let (_, _, _, partial_fee) = self
            .client
            .backend()
            .call_decoding::<(Compact<u64>, Compact<u64>, u8, u128)>(
                "TransactionPaymentApi_query_info",
                Some(&params),
                latest_block_ref.hash(),
            )
            .await
            .map_err(ExtrinsicError::CannotGetFeeInfo)?;

        Ok(partial_fee)
    }
}
