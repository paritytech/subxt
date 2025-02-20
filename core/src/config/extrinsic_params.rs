// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains a trait which controls the parameters that must
//! be provided in order to successfully construct an extrinsic.
//! [`crate::config::DefaultExtrinsicParams`] provides a general-purpose
//! implementation of this that will work in many cases.

use crate::{client::ClientState, error::ExtrinsicParamsError, Config};
use alloc::vec::Vec;

/// This trait allows you to configure the "signed extra" and
/// "additional" parameters that are a part of the transaction payload
/// or the signer payload respectively.
pub trait ExtrinsicParams<T: Config>: ExtrinsicParamsEncoder + Sized + Send + 'static {
    /// These parameters can be provided to the constructor along with
    /// some default parameters that `subxt` understands, in order to
    /// help construct your [`ExtrinsicParams`] object.
    type Params;

    /// Construct a new instance of our [`ExtrinsicParams`].
    fn new(client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError>;

    // The following methods allow our parameters to be updated with details from
    // the chain or user that are captured at a later stage:

    /// Set the account nonce.
    fn inject_account_nonce(&mut self, _nonce: u64) {}
    /// Set the current block.
    fn inject_block(&mut self, _number: u64, _hash: T::Hash) {}
    /// Set the signature.
    fn inject_signature(&mut self, _account_id: T::AccountId, _signature: T::Signature) {}
}

/// This trait is expected to be implemented for any [`ExtrinsicParams`], and
/// defines how to encode the "additional" and "extra" params. Both functions
/// are optional and will encode nothing by default.
pub trait ExtrinsicParamsEncoder: 'static {
    /// This is expected to SCALE encode the transaction extension data to some
    /// buffer that has been provided. This data is attached to the transaction
    /// and also (by default) attached to the signer payload which is signed to
    /// provide a signature for the transaction.
    /// 
    /// If [`ExtrinsicParamsEncoder::encode_for_signer_payloed_to`] is implemented,
    /// then that will be used instead when generating a signer payload. Useful for
    /// eg the `VerifySignature` extension, which is send with the transaction but
    /// is not a part of the signer payload.
    fn encode_value_to(&self, _v: &mut Vec<u8>) {}

    /// See [`ExtrinsicParamsEncoder::encode_to`]. This defaults to calling that
    /// method, but if implemented will dictate what is encoded to the signer payload.
    fn encode_signer_payload_to(&self, v: &mut Vec<u8>) {
        self.encode_value_to(v);
    }

    /// This is expected to SCALE encode the "implicit" (formally "additional") 
    /// parameters to some buffer that has been provided. These parameters are 
    /// _not_ sent along with the transaction, but are taken into account when
    /// signing it, meaning the client and node must agree on their values.
    fn encode_implicit_to(&self, _v: &mut Vec<u8>) {}
}
