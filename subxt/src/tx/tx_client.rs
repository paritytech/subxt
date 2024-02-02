// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::borrow::Cow;

use crate::{
    backend::{BackendExt, BlockRef, TransactionStatus},
    client::{OfflineClientT, OnlineClientT},
    config::{Config, ExtrinsicParams, ExtrinsicParamsEncoder, Hasher},
    error::{Error, MetadataError},
    utils::{Encoded, PhantomDataSendSync},
};
use codec::{Compact, Decode, Encode};
use derivative::Derivative;
use sp_core_hashing::blake2_256;
use subxt_core::{metadata::MetadatExt, tx::TxPayload, Signer as SignerT};

use super::TxProgress;

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
        account_nonce: u64,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams,
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
        let additional_and_extra_params = <T::ExtrinsicParams as ExtrinsicParams<T>>::new(
            account_nonce,
            &self.client.base(),
            other_params,
        )?;

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
        account_nonce: u64,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams,
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
    pub async fn account_nonce(&self, account_id: &T::AccountId) -> Result<u64, Error> {
        let block_ref = self.client.backend().latest_finalized_block_ref().await?;
        crate::blocks::get_account_nonce(&self.client, account_id, block_ref.hash()).await
    }

    /// Creates a partial signed extrinsic, without submitting it.
    pub async fn create_partial_signed<Call>(
        &self,
        call: &Call,
        account_id: &T::AccountId,
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams,
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
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams,
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
        <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams: Default,
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
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams,
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
        <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams: Default,
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
        other_params: <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams,
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

    /// Calculate and return the hash of the extrinsic, based on the configured hasher.
    pub fn hash(&self) -> T::Hash {
        T::Hasher::hash_of(&self.encoded)
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
        let ext_hash = self.hash();

        // Submit and watch for transaction progress.
        let sub = self
            .client
            .backend()
            .submit_transaction(&self.encoded.0)
            .await?;

        Ok(TxProgress::new(sub, self.client.clone(), ext_hash))
    }

    /// Submits the extrinsic to the chain for block inclusion.
    ///
    /// It's usually better to call `submit_and_watch` to get an idea of the progress of the
    /// submission and whether it's eventually successful or not. This call does not guarantee
    /// success, and is just sending the transaction to the chain.
    pub async fn submit(&self) -> Result<T::Hash, Error> {
        let ext_hash = self.hash();
        let mut sub = self
            .client
            .backend()
            .submit_transaction(&self.encoded.0)
            .await?;

        // If we get a bad status or error back straight away then error, else return the hash.
        match sub.next().await {
            Some(Ok(status)) => match status {
                TransactionStatus::Validated
                | TransactionStatus::Broadcasted { .. }
                | TransactionStatus::InBestBlock { .. }
                | TransactionStatus::NoLongerInBestBlock
                | TransactionStatus::InFinalizedBlock { .. } => Ok(ext_hash),
                TransactionStatus::Error { message } => {
                    Err(Error::Other(format!("Transaction error: {message}")))
                }
                TransactionStatus::Invalid { message } => {
                    Err(Error::Other(format!("Transaction invalid: {message}")))
                }
                TransactionStatus::Dropped { message } => {
                    Err(Error::Other(format!("Transaction dropped: {message}")))
                }
            },
            Some(Err(e)) => Err(e),
            None => Err(Error::Other(
                "Transaction broadcast was unsuccessful; stream terminated early".into(),
            )),
        }
    }

    /// Validate a transaction by submitting it to the relevant Runtime API. A transaction that is
    /// valid can be added to a block, but may still end up in an error state.
    ///
    /// Returns `Ok` with a [`ValidationResult`], which is the result of attempting to dry run the extrinsic.
    pub async fn validate(&self) -> Result<ValidationResult, Error> {
        let latest_block_ref = self.client.backend().latest_finalized_block_ref().await?;
        self.validate_at(latest_block_ref).await
    }

    /// Validate a transaction by submitting it to the relevant Runtime API. A transaction that is
    /// valid can be added to a block, but may still end up in an error state.
    ///
    /// Returns `Ok` with a [`ValidationResult`], which is the result of attempting to dry run the extrinsic.
    pub async fn validate_at(
        &self,
        at: impl Into<BlockRef<T::Hash>>,
    ) -> Result<ValidationResult, Error> {
        let block_hash = at.into().hash();

        // Approach taken from https://github.com/paritytech/json-rpc-interface-spec/issues/55.
        let mut params = Vec::with_capacity(8 + self.encoded.0.len() + 8);
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
            .await?;

        ValidationResult::try_from_bytes(res)
    }

    /// This returns an estimate for what the extrinsic is expected to cost to execute, less any tips.
    /// The actual amount paid can vary from block to block based on node traffic and other factors.
    pub async fn partial_fee_estimate(&self) -> Result<u128, Error> {
        let mut params = self.encoded().to_vec();
        (self.encoded().len() as u32).encode_to(&mut params);
        let latest_block_ref = self.client.backend().latest_finalized_block_ref().await?;

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
            .await?;
        Ok(partial_fee)
    }
}

impl ValidationResult {
    #[allow(clippy::get_first)]
    fn try_from_bytes(bytes: Vec<u8>) -> Result<ValidationResult, crate::Error> {
        // TaggedTransactionQueue_validate_transaction returns this:
        // https://github.com/paritytech/substrate/blob/0cdf7029017b70b7c83c21a4dc0aa1020e7914f6/primitives/runtime/src/transaction_validity.rs#L210
        // We copy some of the inner types and put the three states (valid, invalid, unknown) into one enum,
        // because from our perspective, the call was successful regardless.
        if bytes.get(0) == Some(&0) {
            // ok: valid. Decode but, for now we discard most of the information
            let res = TransactionValid::decode(&mut &bytes[1..])?;
            Ok(ValidationResult::Valid(res))
        } else if bytes.get(0) == Some(&1) && bytes.get(1) == Some(&0) {
            // error: invalid
            let res = TransactionInvalid::decode(&mut &bytes[2..])?;
            Ok(ValidationResult::Invalid(res))
        } else if bytes.get(0) == Some(&1) && bytes.get(1) == Some(&1) {
            // error: unknown
            let res = TransactionUnknown::decode(&mut &bytes[2..])?;
            Ok(ValidationResult::Unknown(res))
        } else {
            // unable to decode the bytes; they aren't what we expect.
            Err(crate::Error::Unknown(bytes))
        }
    }
}

/// The result of performing [`SubmittableExtrinsic::validate()`].
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationResult {
    /// The transaction is valid
    Valid(TransactionValid),
    /// The transaction is invalid
    Invalid(TransactionInvalid),
    /// Unable to validate the transaction
    Unknown(TransactionUnknown),
}

impl ValidationResult {
    /// Is the transaction valid.
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid(_))
    }
}

/// Transaction is valid; here is some more information about it.
#[derive(Decode, Clone, Debug, PartialEq)]
pub struct TransactionValid {
    /// Priority of the transaction.
    ///
    /// Priority determines the ordering of two transactions that have all
    /// their dependencies (required tags) satisfied.
    pub priority: u64,
    /// Transaction dependencies
    ///
    /// A non-empty list signifies that some other transactions which provide
    /// given tags are required to be included before that one.
    pub requires: Vec<Vec<u8>>,
    /// Provided tags
    ///
    /// A list of tags this transaction provides. Successfully importing the transaction
    /// will enable other transactions that depend on (require) those tags to be included as well.
    /// Provided and required tags allow Substrate to build a dependency graph of transactions
    /// and import them in the right (linear) order.
    pub provides: Vec<Vec<u8>>,
    /// Transaction longevity
    ///
    /// Longevity describes minimum number of blocks the validity is correct.
    /// After this period transaction should be removed from the pool or revalidated.
    pub longevity: u64,
    /// A flag indicating if the transaction should be propagated to other peers.
    ///
    /// By setting `false` here the transaction will still be considered for
    /// including in blocks that are authored on the current node, but will
    /// never be sent to other peers.
    pub propagate: bool,
}

/// The runtime was unable to validate the transaction.
#[derive(Decode, Clone, Debug, PartialEq)]
pub enum TransactionUnknown {
    /// Could not lookup some information that is required to validate the transaction.
    CannotLookup,
    /// No validator found for the given unsigned transaction.
    NoUnsignedValidator,
    /// Any other custom unknown validity that is not covered by this enum.
    Custom(u8),
}

/// The transaction is invalid.
#[derive(Decode, Clone, Debug, PartialEq)]
pub enum TransactionInvalid {
    /// The call of the transaction is not expected.
    Call,
    /// General error to do with the inability to pay some fees (e.g. account balance too low).
    Payment,
    /// General error to do with the transaction not yet being valid (e.g. nonce too high).
    Future,
    /// General error to do with the transaction being outdated (e.g. nonce too low).
    Stale,
    /// General error to do with the transaction's proofs (e.g. signature).
    ///
    /// # Possible causes
    ///
    /// When using a signed extension that provides additional data for signing, it is required
    /// that the signing and the verifying side use the same additional data. Additional
    /// data will only be used to generate the signature, but will not be part of the transaction
    /// itself. As the verifying side does not know which additional data was used while signing
    /// it will only be able to assume a bad signature and cannot express a more meaningful error.
    BadProof,
    /// The transaction birth block is ancient.
    ///
    /// # Possible causes
    ///
    /// For `FRAME`-based runtimes this would be caused by `current block number
    /// - Era::birth block number > BlockHashCount`. (e.g. in Polkadot `BlockHashCount` = 2400, so
    ///   a
    /// transaction with birth block number 1337 would be valid up until block number 1337 + 2400,
    /// after which point the transaction would be considered to have an ancient birth block.)
    AncientBirthBlock,
    /// The transaction would exhaust the resources of current block.
    ///
    /// The transaction might be valid, but there are not enough resources
    /// left in the current block.
    ExhaustsResources,
    /// Any other custom invalid validity that is not covered by this enum.
    Custom(u8),
    /// An extrinsic with a Mandatory dispatch resulted in Error. This is indicative of either a
    /// malicious validator or a buggy `provide_inherent`. In any case, it can result in
    /// dangerously overweight blocks and therefore if found, invalidates the block.
    BadMandatory,
    /// An extrinsic with a mandatory dispatch tried to be validated.
    /// This is invalid; only inherent extrinsics are allowed to have mandatory dispatches.
    MandatoryValidation,
    /// The sending address is disabled or known to be invalid.
    BadSigner,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transaction_validity_decoding_empty_bytes() {
        // No panic should occur decoding empty bytes.
        let decoded = ValidationResult::try_from_bytes(vec![]);
        assert!(decoded.is_err())
    }

    #[test]
    fn transaction_validity_decoding_is_ok() {
        use sp_runtime::transaction_validity as sp;
        use sp_runtime::transaction_validity::TransactionValidity as T;

        let pairs = vec![
            (
                T::Ok(sp::ValidTransaction {
                    ..Default::default()
                }),
                ValidationResult::Valid(TransactionValid {
                    // By default, tx is immortal
                    longevity: u64::MAX,
                    // Default is true
                    propagate: true,
                    priority: 0,
                    provides: vec![],
                    requires: vec![],
                }),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::BadProof,
                )),
                ValidationResult::Invalid(TransactionInvalid::BadProof),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::Call,
                )),
                ValidationResult::Invalid(TransactionInvalid::Call),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::Payment,
                )),
                ValidationResult::Invalid(TransactionInvalid::Payment),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::Future,
                )),
                ValidationResult::Invalid(TransactionInvalid::Future),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::Stale,
                )),
                ValidationResult::Invalid(TransactionInvalid::Stale),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::AncientBirthBlock,
                )),
                ValidationResult::Invalid(TransactionInvalid::AncientBirthBlock),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::ExhaustsResources,
                )),
                ValidationResult::Invalid(TransactionInvalid::ExhaustsResources),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::BadMandatory,
                )),
                ValidationResult::Invalid(TransactionInvalid::BadMandatory),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::MandatoryValidation,
                )),
                ValidationResult::Invalid(TransactionInvalid::MandatoryValidation),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::BadSigner,
                )),
                ValidationResult::Invalid(TransactionInvalid::BadSigner),
            ),
            (
                T::Err(sp::TransactionValidityError::Invalid(
                    sp::InvalidTransaction::Custom(123),
                )),
                ValidationResult::Invalid(TransactionInvalid::Custom(123)),
            ),
            (
                T::Err(sp::TransactionValidityError::Unknown(
                    sp::UnknownTransaction::CannotLookup,
                )),
                ValidationResult::Unknown(TransactionUnknown::CannotLookup),
            ),
            (
                T::Err(sp::TransactionValidityError::Unknown(
                    sp::UnknownTransaction::NoUnsignedValidator,
                )),
                ValidationResult::Unknown(TransactionUnknown::NoUnsignedValidator),
            ),
            (
                T::Err(sp::TransactionValidityError::Unknown(
                    sp::UnknownTransaction::Custom(123),
                )),
                ValidationResult::Unknown(TransactionUnknown::Custom(123)),
            ),
        ];

        for (sp, validation_result) in pairs {
            let encoded = sp.encode();
            let decoded = ValidationResult::try_from_bytes(encoded).expect("should decode OK");
            assert_eq!(decoded, validation_result);
        }
    }
}
