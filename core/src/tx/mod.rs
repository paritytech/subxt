// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types and functions responsible for encoding and signing transactions.

pub mod payload;
pub mod signer;

use crate::client::ClientState;
use crate::config::{Config, ExtrinsicParams, ExtrinsicParamsEncoder, Hasher};
use crate::error::{Error, MetadataError};
use crate::metadata::Metadata;
use crate::utils::Encoded;
use codec::{Compact, Encode};
use payload::TxPayload;
use signer::Signer as SignerT;
use sp_crypto_hashing::blake2_256;
use std::borrow::Cow;

/// Run the validation logic against some extrinsic you'd like to submit. Returns `Ok(())`
/// if the call is valid (or if it's not possible to check since the call has no validation hash).
/// Return an error if the call was not valid or something went wrong trying to validate it (ie
/// the pallet or call in question do not exist at all).
pub fn validate<Call: TxPayload>(metadata: &Metadata, call: &Call) -> Result<(), Error> {
    if let Some(details) = call.validation_details() {
        let expected_hash = metadata
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
pub fn call_data<Call: TxPayload>(metadata: &Metadata, call: &Call) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    call.encode_call_data_to(&metadata, &mut bytes)?;
    Ok(bytes)
}

/// Creates an unsigned extrinsic without submitting it.
pub fn create_unsigned<T: Config, Call: TxPayload>(
    metadata: &Metadata,
    call: &Call,
) -> Result<Transaction<T>, Error> {
    // 1. Validate this call against the current node metadata if the call comes
    // with a hash allowing us to do so.
    validate(metadata, call)?;

    // 2. Encode extrinsic
    let extrinsic = {
        let mut encoded_inner = Vec::new();
        // transaction protocol version (4) (is not signed, so no 1 bit at the front).
        4u8.encode_to(&mut encoded_inner);
        // encode call data after this byte.
        call.encode_call_data_to(&metadata, &mut encoded_inner)?;
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
    Ok(Transaction::from_bytes(extrinsic))
}

/// Create a partial extrinsic.
///
/// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
/// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
pub fn create_partial_signed<T: Config, Call: TxPayload>(
    metadata: &Metadata,
    client_state: &ClientState<T>,
    call: &Call,
    params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
) -> Result<PartialTransaction<T>, Error> {
    // 1. Validate this call against the current node metadata if the call comes
    // with a hash allowing us to do so.
    validate(metadata, call)?;

    // 2. SCALE encode call data to bytes (pallet u8, call u8, call params).
    let call_data = call_data(metadata, call)?;

    // 3. Construct our custom additional/extra params.
    let additional_and_extra_params =
        <T::ExtrinsicParams as ExtrinsicParams<T>>::new(client_state, params)?;

    // Return these details, ready to construct a signed extrinsic from.
    Ok(PartialTransaction {
        call_data,
        additional_and_extra_params,
    })
}

/// Creates a signed extrinsic without submitting it.
///
/// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
/// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
pub fn create_signed<T, Call, Signer>(
    metadata: &Metadata,
    client_state: &ClientState<T>,
    call: &Call,
    signer: &Signer,
    params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
) -> Result<Transaction<T>, Error>
where
    T: Config,
    Call: TxPayload,
    Signer: SignerT<T>,
{
    // 1. Validate this call against the current node metadata if the call comes
    // with a hash allowing us to do so.
    validate(metadata, call)?;

    // 2. Gather the "additional" and "extra" params along with the encoded call data,
    //    ready to be signed.
    let partial_signed = create_partial_signed(metadata, client_state, call, params)?;

    // 3. Sign and construct an extrinsic from these details.
    Ok(partial_signed.sign(signer))
}

/// This represents a partially constructed transaction that needs signing before it is ready
/// to submit. Use [`PartialTransaction::signer_payload()`] to return the payload that needs signing,
/// [`PartialTransaction::sign()`] to sign the transaction using a [`SignerT`] impl, or
/// [`PartialTransaction::sign_with_address_and_signature()`] to apply an existing signature and address
/// to the transaction.
pub struct PartialTransaction<T: Config> {
    call_data: Vec<u8>,
    additional_and_extra_params: T::ExtrinsicParams,
}

impl<T: Config> PartialTransaction<T> {
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
    pub fn sign<Signer>(&self, signer: &Signer) -> Transaction<T>
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
    ) -> Transaction<T> {
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
        Transaction::from_bytes(extrinsic)
    }
}

/// This represents a signed transaction that's ready to be submitted.
/// Use [`Transaction::encoded()`] or [`Transaction::into_encoded()`] to
/// get the bytes for it, or [`Transaction::hash()`] to get the hash.
pub struct Transaction<T> {
    encoded: Encoded,
    marker: std::marker::PhantomData<T>,
}

impl<T: Config> Transaction<T> {
    /// Create a [`Transaction`] from some already-signed and prepared
    /// extrinsic bytes,
    ///
    /// Prefer to use [`TxClient`] to create and sign extrinsics. This is simply
    /// exposed in case you want to skip this process and submit something you've
    /// already created.
    pub fn from_bytes(tx_bytes: Vec<u8>) -> Self {
        Self {
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
