mod payload;
mod signer;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::{ClientState, Config};
use crate::error::ExtrinsicError;
use codec::Compact;
use core::marker::PhantomData;

pub use payload::Payload;
pub use signer::Signer;

/// A client for working with transactions.
#[derive(Clone)]
pub struct Transactions<'atblock, T, Client> {
    client: &'atblock Client,
    marker: PhantomData<T>,
}

impl<'atblock, T, Client> Transactions<'atblock, T, Client> {
    pub(crate) fn new(client: &'atblock Client) -> Self {
        Transactions {
            client,
            marker: PhantomData,
        }
    }
}

impl<'atblock, T: Config, C: OfflineClientAtBlockT<T>> Transactions<'atblock, T, C> {
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
    /// [`Self::create_v5_bare`] if you'd like to explicitly create an unsigned transaction of a certain version.
    pub fn create_unsigned<Call>(
        &self,
        call: &Call,
    ) -> Result<SubmittableTransaction<T, C>, ExtrinsicError>
    where
        Call: Payload,
    {
        let tx = match self.default_transaction_version()? {
            TransactionVersion::V4 => self.create_v4_unsigned(call),
            TransactionVersion::V5 => self.create_v5_bare(call),
        }?;

        Ok(SubmittableTransaction {
            client: self.client.clone(),
            inner: tx,
        })
    }

    /// Creates a V4 "unsigned" transaction without submitting it.
    pub fn create_v4_unsigned<Call>(&self, call: &Call) -> Result<Transaction<T>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.create_unsigned_at_version(call, 4)
    }

    /// Creates a V5 "bare" transaction without submitting it.
    pub fn create_v5_bare<Call>(&self, call: &Call) -> Result<Transaction<T>, ExtrinsicError>
    where
        Call: Payload,
    {
        self.create_unsigned_at_version(call, 5)
    }

    /// Create a partial transaction. Depending on the metadata, we might end up constructing either a v4 or
    /// v5 transaction. See [`subxt_core::tx`] if you'd like to manually pick the version to construct
    ///
    /// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
    /// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
    pub fn create_partial_offline<Call>(
        &self,
        call: &Call,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<PartialTransaction<T, C>, ExtrinsicError>
    where
        Call: Payload,
    {
        let metadata = self.client.metadata();
        let client_state = ClientState {
            genesis_hash: self.client.genesis_hash(),
            spec_version: self.client.spec_version(),
            transaction_version: self.client.transaction_version(),
            metadata: self.client.metadata(),
        };

        let tx = match self.default_transaction_version()? {
            TransactionVersion::V4 => {
                PartialTransactionInner::V4(self.create_v4_signed(call, &client_state, params)?)
            }
            TransactionVersion::V5 => {
                PartialTransactionInner::V5(self.create_v5_general(call, &client_state, params)?)
            }
        };

        Ok(PartialTransaction {
            client: self.client.clone(),
            inner: tx,
        })
    }

    /// Create a v4 partial transaction, ready to sign.
    ///
    /// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
    /// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
    ///
    /// Prefer [`Self::create_partial_offline()`] if you don't know which version to create; this will pick the
    /// most suitable one for the given chain.
    pub fn create_v4_partial_offline<Call>(
        &self,
        call: &Call,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<PartialTransaction<T, C>, ExtrinsicError>
    where
        Call: Payload,
    {
        let tx = PartialTransactionInner::V4(subxt_core::tx::create_v4_signed(
            call,
            &self.client.client_state(),
            params,
        )?);

        Ok(PartialTransaction {
            client: self.client.clone(),
            inner: tx,
        })
    }

    /// Create a v5 partial transaction, ready to sign.
    ///
    /// Note: if not provided, the default account nonce will be set to 0 and the default mortality will be _immortal_.
    /// This is because this method runs offline, and so is unable to fetch the data needed for more appropriate values.
    ///
    /// Prefer [`Self::create_partial_offline()`] if you don't know which version to create; this will pick the
    /// most suitable one for the given chain.
    pub fn create_v5_partial_offline<Call>(
        &self,
        call: &Call,
        params: <T::ExtrinsicParams as ExtrinsicParams<T>>::Params,
    ) -> Result<PartialTransaction<T, C>, ExtrinsicError>
    where
        Call: Payload,
    {
        let tx = PartialTransactionInner::V5(subxt_core::tx::create_v5_general(
            call,
            &self.client.client_state(),
            params,
        )?);

        Ok(PartialTransaction {
            client: self.client.clone(),
            inner: tx,
        })
    }

    // Create a V4 "unsigned" transaction or V5 "bare" transaction.
    fn create_unsigned_at_version<Call: Payload>(
        &self,
        call: &Call,
        tx_version: u8,
    ) -> Result<Transaction<T>, ExtrinsicError> {
        let metadata = self.client.metadata_ref();

        // 1. Validate this call against the current node metadata if the call comes
        // with a hash allowing us to do so.
        self.validate(call)?;

        // 2. Encode extrinsic
        let extrinsic = {
            let mut encoded_inner = Vec::new();
            // encode the transaction version first.
            tx_version.encode_to(&mut encoded_inner);
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

    /// Returns the suggested transaction versions to build for a given chain, or an error
    /// if Subxt doesn't support any version expected by the chain.
    ///
    /// If the result is [`TransactionVersion::V4`], use the `v4` methods in this module. If it's
    /// [`TransactionVersion::V5`], use the `v5` ones.
    pub fn default_transaction_version(&self) -> Result<TransactionVersion, ExtrinsicError> {
        let metadata = self.client.metadata_ref();
        let versions = metadata.extrinsic().supported_versions();

        if versions.contains(&4) {
            Ok(TransactionVersion::V4)
        } else if versions.contains(&5) {
            Ok(TransactionVersion::V5)
        } else {
            Err(ExtrinsicError::UnsupportedVersion)
        }
    }
}

/// The transaction versions supported by Subxt.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TransactionVersion {
    /// v4 transactions (signed and unsigned transactions)
    V4,
    /// v5 transactions (bare and general transactions)
    V5,
}
