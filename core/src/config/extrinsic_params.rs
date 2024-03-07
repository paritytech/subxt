// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains a trait which controls the parameters that must
//! be provided in order to successfully construct an extrinsic.
//! [`crate::config::DefaultExtrinsicParams`] provides a general-purpose
//! implementation of this that will work in many cases.

use crate::{client::ClientMetadata, Config, ExtrinsicParamsError};
use alloc::vec::Vec;

use super::refine_params::RefineParams;
/// This trait allows you to configure the "signed extra" and
/// "additional" parameters that are a part of the transaction payload
/// or the signer payload respectively.
pub trait ExtrinsicParams<T: Config>: ExtrinsicParamsEncoder + Sized + 'static {
    /// These parameters can be provided to the constructor along with
    /// some default parameters that `subxt` understands, in order to
    /// help construct your [`ExtrinsicParams`] object.
    type Params: RefineParams<T>;

    /// Construct a new instance of our [`ExtrinsicParams`].
    fn new(client: &ClientMetadata<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError>;
}

/// This trait is expected to be implemented for any [`ExtrinsicParams`], and
/// defines how to encode the "additional" and "extra" params. Both functions
/// are optional and will encode nothing by default.
pub trait ExtrinsicParamsEncoder: 'static {
    /// This is expected to SCALE encode the "signed extra" parameters
    /// to some buffer that has been provided. These are the parameters
    /// which are sent along with the transaction, as well as taken into
    /// account when signing the transaction.
    fn encode_extra_to(&self, _v: &mut Vec<u8>) {}

    /// This is expected to SCALE encode the "additional" parameters
    /// to some buffer that has been provided. These parameters are _not_
    /// sent along with the transaction, but are taken into account when
    /// signing it, meaning the client and node must agree on their values.
    fn encode_additional_to(&self, _v: &mut Vec<u8>) {}
}
