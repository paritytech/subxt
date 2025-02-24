// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Construct and sign transactions.
//!
//! # Example
//!
//! ```rust
//! use subxt_signer::sr25519::dev;
//! use subxt_macro::subxt;
//! use subxt_core::config::PolkadotConfig;
//! use subxt_core::config::DefaultExtrinsicParamsBuilder as Params;
//! use subxt_core::tx;
//! use subxt_core::utils::H256;
//! use subxt_core::metadata;
//!
//! // If we generate types without `subxt`, we need to point to `::subxt_core`:
//! #[subxt(
//!     crate = "::subxt_core",
//!     runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale",
//! )]
//! pub mod polkadot {}
//!
//! // Gather some other information about the chain that we'll need to construct valid extrinsics:
//! let state = tx::ClientState::<PolkadotConfig> {
//!     metadata: {
//!         let metadata_bytes = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
//!         metadata::decode_from(&metadata_bytes[..]).unwrap()
//!     },
//!     genesis_hash: {
//!         let h = "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";
//!         let bytes = hex::decode(h).unwrap();
//!         H256::from_slice(&bytes)
//!     },
//!     runtime_version: tx::RuntimeVersion {
//!         spec_version: 9370,
//!         transaction_version: 20,
//!     }
//! };
//!
//! // Now we can build a balance transfer extrinsic.
//! let dest = dev::bob().public_key().into();
//! let call = polkadot::tx().balances().transfer_allow_death(dest, 10_000);
//! let params = Params::new().tip(1_000).nonce(0).build();
//!
//! // We can validate that this lines up with the given metadata:
//! tx::validate(&call, &state.metadata).unwrap();
//!
//! // We can build a signed transaction:
//! let signed_call = tx::create_signed(&call, &state, &dev::alice(), params).unwrap();
//!
//! // And log it:
//! println!("Tx: 0x{}", hex::encode(signed_call.encoded()));
//! ```

pub mod payload;
pub mod signer;

use crate::config::{Config, ExtrinsicParams, ExtrinsicParamsEncoder, Hasher};
use crate::error::{Error, ExtrinsicError, MetadataError};
use crate::metadata::Metadata;
use crate::utils::Encoded;
use alloc::borrow::{Cow, ToOwned};
use alloc::vec::Vec;
use codec::{Compact, Encode};
use payload::Payload;
use signer::Signer as SignerT;
use sp_crypto_hashing::blake2_256;

// Expose these here since we expect them in some calls below.
pub use crate::client::{ClientState, RuntimeVersion};

/// Run the validation logic against some extrinsic you'd like to submit. Returns `Ok(())`
/// if the call is valid (or if it's not possible to check since the call has no validation hash).
/// Return an error if the call was not valid or something went wrong trying to validate it (ie
/// the pallet or call in question do not exist at all).
pub fn validate<Call: Payload>(call: &Call, metadata: &Metadata) -> Result<(), Error> {
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
pub fn call_data<Call: Payload>(call: &Call, metadata: &Metadata) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    call.encode_call_data_to(metadata, &mut bytes)?;
    Ok(bytes)
}

/// Creates an unsigned transaction without submitting it. This will create a transaction
/// at a version appropriate for the metadata provided. (either V4 or V5).
///
/// Use [`create_v4_unsigned`] or [`create_v5_bare`] if you'd prefer to explicitly create
/// a V4 or V5 unsigned/bare transaction.
pub fn create_unsigned<T: Config, Call: Payload>(
    call: &Call,
    metadata: &Metadata,
) -> Result<Transaction<T>, Error> {
    // 1. Get the first supported TX version from the metadata.
    let tx_version = TransactionVersion::from_metadata(metadata)?;

    // 2. Encode extrinsic at that version
    create_unsigned_at_version(call, tx_version, metadata)
}

/// Creates a V4 "unsigned" transaction without submitting it.
pub fn create_v4_unsigned<T: Config, Call: Payload>(
    call: &Call,
    metadata: &Metadata,
) -> Result<Transaction<T>, Error> {
    create_unsigned_at_version(call, TransactionVersion::V4, metadata)
}

/// Creates a V5 "bare" transaction without submitting it.
pub fn create_v5_bare<T: Config, Call: Payload>(
    call: &Call,
    metadata: &Metadata,
) -> Result<Transaction<T>, Error> {
    create_unsigned_at_version(call, TransactionVersion::V5, metadata)
}

// Create a V4 "unsigned" transaction or V5 "bare" transaction.
fn create_unsigned_at_version<T: Config, Call: Payload>(
    call: &Call,
    tx_version: TransactionVersion,
    metadata: &Metadata,
) -> Result<Transaction<T>, Error> {
    // 1. Validate this call against the current node metadata if the call comes
    // with a hash allowing us to do so.
    validate(call, metadata)?;

    // 3. Encode extrinsic
    let extrinsic = {
        let mut encoded_inner = Vec::new();
        // encode the transaction version first.
        tx_version.to_u8().encode_to(&mut encoded_inner);
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
    Ok(Transaction::from_bytes(extrinsic))
}

/// Create a partial extrinsic.
///
/// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
/// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
pub fn create_partial<T: Config, Call: Payload>(
    call: &Call,
    client_state: &ClientState<T>,
    params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
) -> Result<PartialTransaction<T>, Error> {
    // 1. Validate this call against the current node metadata if the call comes
    // with a hash allowing us to do so.
    validate(call, &client_state.metadata)?;

    // 2. Work out which TX and TX extension version to target based on metadata (unless we
    // explicitly ask for a specific transaction version at a later step).
    let tx_version = TransactionVersion::from_metadata(&client_state.metadata)?;
    let tx_extensions_version = client_state
        .metadata
        .extrinsic()
        .transaction_extensions_version();

    // 3. SCALE encode call data to bytes (pallet u8, call u8, call params).
    let call_data = call_data(call, &client_state.metadata)?;

    // 4. Construct our custom additional/extra params.
    let additional_and_extra_params =
        <T::ExtrinsicParams as ExtrinsicParams<T>>::new(client_state, params)?;

    // Return these details, ready to construct a signed extrinsic from.
    Ok(PartialTransaction {
        call_data,
        additional_and_extra_params,
        recommended_tx_version: tx_version,
        tx_extensions_version,
    })
}

/// Creates a signed extrinsic without submitting it.
///
/// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
/// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
pub fn create_signed<T, Call, Signer>(
    call: &Call,
    client_state: &ClientState<T>,
    signer: &Signer,
    params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
) -> Result<Transaction<T>, Error>
where
    T: Config,
    Call: Payload,
    Signer: SignerT<T>,
{
    // 1. Validate this call against the current node metadata if the call comes
    // with a hash allowing us to do so.
    validate(call, &client_state.metadata)?;

    // 2. Gather the "additional" and "extra" params along with the encoded call data,
    //    ready to be signed.
    let mut partial_signed = create_partial(call, client_state, params)?;

    // 3. Sign and construct an extrinsic from these details.
    Ok(partial_signed.sign(signer))
}

/// This represents a partially constructed transaction that needs signing before it is ready
/// to submit. Use [`PartialTransaction::signer_payload()`] to return the payload that needs signing,
/// [`PartialTransaction::sign()`] to sign the transaction using a [`SignerT`] impl, or
/// [`PartialTransaction::sign_with_account_and_signature()`] to apply an existing signature and account ID
/// to the transaction.
pub struct PartialTransaction<T: Config> {
    call_data: Vec<u8>,
    additional_and_extra_params: T::ExtrinsicParams,
    // Based on the metadata, which version transaction should we build?
    // This is only a suggestion and can be overridden.
    recommended_tx_version: TransactionVersion,
    // What version of transaction extensions are we encoding?
    tx_extensions_version: u8,
}

impl<T: Config> PartialTransaction<T> {
    // Obtain bytes representing the signer payload and run call some function
    // with them. This can avoid an allocation in some cases when compared to
    // [`PartialExtrinsic::signer_payload()`].
    fn with_signer_payload<F, R>(&self, version: TransactionVersion, f: F) -> R
    where
        F: for<'a> FnOnce(Cow<'a, [u8]>) -> R,
    {
        let mut bytes = self.call_data.clone();
        self.additional_and_extra_params
            .encode_signer_payload_value_to(&mut bytes);
        self.additional_and_extra_params
            .encode_implicit_to(&mut bytes);

        // For V4 transactions we only hash if >256 bytes.
        // For V5 transactions we _always_ hash.
        if version == TransactionVersion::V5 || bytes.len() > 256 {
            f(Cow::Borrowed(blake2_256(&bytes).as_ref()))
        } else {
            f(Cow::Owned(bytes))
        }
    }

    /// Return the signer payload for this extrinsic. These are the bytes that must
    /// be signed in order to produce a valid signature for the extrinsic.
    ///
    /// This payload can be used with the non-versioned `sign*` methods (ie any method without
    /// `v4` or `v5` in the name). If you wish to use the versioned `sign*` methods, use
    /// [`Self::v4_signer_payload`] or [`Self::v5_signer_payload`] to construct the correct
    /// payload for that version.
    pub fn signer_payload(&self) -> Vec<u8> {
        self.with_signer_payload(self.recommended_tx_version, |bytes| bytes.to_vec())
    }

    /// Return the V4 signer payload for this extrinsic. These are the bytes that must
    /// be signed in order to produce a valid signature for the extrinsic.
    pub fn v4_signer_payload(&self) -> Vec<u8> {
        self.with_signer_payload(TransactionVersion::V4, |bytes| bytes.to_vec())
    }

    /// Return the V5 signer payload for this extrinsic. These are the bytes that must
    /// be signed in order to produce a valid signature for the extrinsic.
    pub fn v5_signer_payload(&self) -> Vec<u8> {
        self.with_signer_payload(TransactionVersion::V5, |bytes| bytes.to_vec())
    }

    /// Return the bytes representing the call data for this partially constructed
    /// extrinsic.
    pub fn call_data(&self) -> &[u8] {
        &self.call_data
    }

    /// Convert this [`PartialTransaction`] into a [`Transaction`], ready to submit.
    /// The provided `signer` is responsible for providing the "from" address for the transaction,
    /// as well as providing a signature to attach to it.
    ///
    /// This builds either a V4 or V5 transaction depending on the provided chain metadata.
    pub fn sign<Signer>(&mut self, signer: &Signer) -> Transaction<T>
    where
        Signer: SignerT<T>,
    {
        // Given our signer, we can sign the payload representing this extrinsic.
        let signature =
            self.with_signer_payload(self.recommended_tx_version, |bytes| signer.sign(&bytes));
        // Now, use the signature and "from" address to build the extrinsic.
        self.sign_with_account_and_signature(&signer.account_id(), &signature)
    }

    /// Convert this [`PartialTransaction`] into a [`Transaction`], ready to submit.
    /// An account ID, and something representing a signature that can be SCALE encoded, are both
    /// needed in order to construct it. If you have a `Signer` to hand, you can use
    /// [`PartialTransaction::sign()`] instead.
    ///
    /// This builds either a V4 or V5 transaction depending on the provided chain metadata.
    pub fn sign_with_account_and_signature(
        &mut self,
        account_id: &T::AccountId,
        signature: &T::Signature,
    ) -> Transaction<T> {
        match self.recommended_tx_version {
            TransactionVersion::V4 => {
                let address: T::Address = account_id.clone().into();
                self.to_v4_signed_with_address_and_signature(&address, signature)
            }
            TransactionVersion::V5 => {
                self.to_v5_general_with_account_and_signature(account_id, signature)
            }
        }
    }

    /// Convert this [`PartialTransaction`] into a V4 signed [`Transaction`], ready to submit.
    /// The provided `signer` is responsible for providing the "from" address for the transaction,
    /// as well as providing a signature to attach to it.
    pub fn to_v4_signed<Signer>(&self, signer: &Signer) -> Transaction<T>
    where
        Signer: SignerT<T>,
    {
        // Given our signer, we can sign the payload representing this extrinsic.
        let signature =
            self.with_signer_payload(TransactionVersion::V4, |bytes| signer.sign(&bytes));
        // Now, use the signature and "from" address to build the extrinsic.
        self.to_v4_signed_with_address_and_signature(&signer.address(), &signature)
    }

    /// Convert this [`PartialTransaction`] into a V4 signed [`Transaction`], ready to submit.
    /// The provided `address` and `signature` will be used.
    ///
    /// The signature should be derived by signing [`Self::v4_signer_payload`].
    pub fn to_v4_signed_with_address_and_signature(
        &self,
        address: &T::Address,
        signature: &T::Signature,
    ) -> Transaction<T> {
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

        // Return an extrinsic ready to be submitted.
        Transaction::from_bytes(extrinsic)
    }

    /// Convert this [`PartialTransaction`] into a V5 "general" [`Transaction`].
    ///
    /// This transaction has not been explicitly signed. Use [`Self::to_v5_general_with_signer`]
    /// or [`Self::to_v5_general_with_account_and_signature`] if you wish to provide a
    /// signature (this is usually a necessary step).
    pub fn to_v5_general(&self) -> Transaction<T> {
        let extrinsic = {
            let mut encoded_inner = Vec::new();
            // "is general" + transaction protocol version (5)
            (0b0100000 + 5u8).encode_to(&mut encoded_inner);
            // Encode versions for the transaction extensions
            self.tx_extensions_version.encode_to(&mut encoded_inner);
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
        };

        // Return an extrinsic ready to be submitted.
        Transaction::from_bytes(extrinsic)
    }

    /// Convert this [`PartialTransaction`] into a V5 "general" [`Transaction`] with a signature.
    pub fn to_v5_general_with_signer<Signer>(&mut self, signer: &Signer) -> Transaction<T>
    where
        Signer: SignerT<T>,
    {
        // Given our signer, we can sign the payload representing this extrinsic.
        let signature =
            self.with_signer_payload(TransactionVersion::V5, |bytes| signer.sign(&bytes));
        // Now, use the signature and "from" account to build the extrinsic.
        self.to_v5_general_with_account_and_signature(&signer.account_id(), &signature)
    }

    /// Convert this [`PartialTransaction`] into a V5 "general" [`Transaction`] with a signature.
    /// Prefer [`Self::to_v5_general_with_signer`] if you have a [`SignerT`] instance to use.
    ///
    /// The signature should be derived by signing [`Self::v5_signer_payload`].
    pub fn to_v5_general_with_account_and_signature(
        &mut self,
        account_id: &T::AccountId,
        signature: &T::Signature,
    ) -> Transaction<T> {
        // Inject the signature into the transaction extensions
        // before constructing it.
        self.additional_and_extra_params
            .inject_signature(account_id, signature);

        self.to_v5_general()
    }
}

/// This represents a signed transaction that's ready to be submitted.
/// Use [`Transaction::encoded()`] or [`Transaction::into_encoded()`] to
/// get the bytes for it, or [`Transaction::hash()`] to get the hash.
pub struct Transaction<T> {
    encoded: Encoded,
    marker: core::marker::PhantomData<T>,
}

impl<T: Config> Transaction<T> {
    /// Create a [`Transaction`] from some already-signed and prepared
    /// extrinsic bytes,
    pub fn from_bytes(tx_bytes: Vec<u8>) -> Self {
        Self {
            encoded: Encoded(tx_bytes),
            marker: core::marker::PhantomData,
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

    /// Consumes this [`Transaction`] and returns the SCALE encoded
    /// extrinsic bytes.
    pub fn into_encoded(self) -> Vec<u8> {
        self.encoded.0
    }
}

/// This represents the transaction versions supported by Subxt.
#[derive(PartialEq, Copy, Clone, Debug)]
enum TransactionVersion {
    V4,
    V5,
}

impl TransactionVersion {
    fn from_metadata(metadata: &Metadata) -> Result<Self, Error> {
        metadata
            .extrinsic()
            .supported_versions()
            .iter()
            .filter_map(|n| TransactionVersion::from_u8(*n).ok())
            .next()
            .ok_or(Error::Extrinsic(ExtrinsicError::UnsupportedVersion))
    }

    fn from_u8(n: u8) -> Result<Self, Error> {
        if n == 4 {
            Ok(TransactionVersion::V4)
        } else if n == 5 {
            Ok(TransactionVersion::V5)
        } else {
            Err(Error::Extrinsic(ExtrinsicError::UnsupportedVersion))
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            TransactionVersion::V4 => 4,
            TransactionVersion::V5 => 5,
        }
    }
}
