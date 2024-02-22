// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains a trait which controls the parameters that must
//! be provided in order to successfully construct an extrinsic.
//! [`crate::config::DefaultExtrinsicParams`] provides a general-purpose
//! implementation of this that will work in many cases.

use crate::{client::OfflineClientT, Config};
use core::fmt::Debug;

use super::refine_params::RefineParams;

/// An error that can be emitted when trying to construct an instance of [`ExtrinsicParams`],
/// encode data from the instance, or match on signed extensions.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ExtrinsicParamsError {
    /// Cannot find a type id in the metadata. The context provides some additional
    /// information about the source of the error (eg the signed extension name).
    #[error("Cannot find type id '{type_id} in the metadata (context: {context})")]
    MissingTypeId {
        /// Type ID.
        type_id: u32,
        /// Some arbitrary context to help narrow the source of the error.
        context: &'static str,
    },
    /// A signed extension in use on some chain was not provided.
    #[error("The chain expects a signed extension with the name {0}, but we did not provide one")]
    UnknownSignedExtension(String),
    /// Some custom error.
    #[error("Error constructing extrinsic parameters: {0}")]
    Custom(CustomExtrinsicParamsError),
}

/// A custom error.
pub type CustomExtrinsicParamsError = Box<dyn std::error::Error + Send + Sync + 'static>;

impl From<std::convert::Infallible> for ExtrinsicParamsError {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}
impl From<CustomExtrinsicParamsError> for ExtrinsicParamsError {
    fn from(value: CustomExtrinsicParamsError) -> Self {
        ExtrinsicParamsError::Custom(value)
    }
}

/// This trait allows you to configure the "signed extra" and
/// "additional" parameters that are a part of the transaction payload
/// or the signer payload respectively.
pub trait ExtrinsicParams<T: Config>: ExtrinsicParamsEncoder + Sized + 'static {
    /// These parameters can be provided to the constructor along with
    /// some default parameters that `subxt` understands, in order to
    /// help construct your [`ExtrinsicParams`] object.
    type Params: RefineParams<T>;

    /// Construct a new instance of our [`ExtrinsicParams`].
    fn new<Client: OfflineClientT<T>>(
        client: Client,
        other_params: Self::Params,
    ) -> Result<Self, ExtrinsicParamsError>;
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
