// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains a trait which controls the parameters that must
//! be provided in order to successfully construct an extrinsic.
//! [`crate::config::DefaultExtrinsicParams`] provides a general-purpose
//! implementation of this that will work in many cases.

use crate::{client::OfflineClientT, Config};
use core::fmt::Debug;

/// An error that can be emitted when trying to construct
/// an instance of [`ExtrinsicParams`].
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ExtrinsicParamsError {
    /// A signed extension was encountered that we don't know about.
    #[error("Error constructing extrinsic parameters: Unknown signed extension '{0}'")]
    UnknownSignedExtension(String),
    /// Cannot find the type id of a signed extension in the metadata.
    #[error("Cannot find extension's '{0}' type id '{1} in the metadata")]
    MissingTypeId(String, u32),
    /// User provided a different signed extension than the one expected.
    #[error("Provided a different signed extension for '{0}', the metadata expect '{1}'")]
    ExpectedAnotherExtension(String, String),
    /// The inner type of a signed extension is not present in the metadata.
    #[error("The inner type of the signed extension '{0}' is not present in the metadata")]
    MissingInnerSignedExtension(String),
    /// The inner type of the signed extension is not named.
    #[error("The signed extension's '{0}' type id '{1}' does not have a name in the metadata")]
    ExpectedNamedTypeId(String, u32),
    /// Some custom error.s
    #[error("Error constructing extrinsic parameters: {0}")]
    Custom(CustomError),
}

/// A custom error.
type CustomError = Box<dyn std::error::Error + Send + Sync + 'static>;

impl From<std::convert::Infallible> for ExtrinsicParamsError {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}

/// This trait allows you to configure the "signed extra" and
/// "additional" parameters that are a part of the transaction payload
/// or the signer payload respectively.
pub trait ExtrinsicParams<T: Config>: ExtrinsicParamsEncoder + Sized + 'static {
    /// These parameters can be provided to the constructor along with
    /// some default parameters that `subxt` understands, in order to
    /// help construct your [`ExtrinsicParams`] object.
    type OtherParams;

    /// The type of error returned from [`ExtrinsicParams::new()`].
    type Error: Into<ExtrinsicParamsError>;

    /// Construct a new instance of our [`ExtrinsicParams`]
    fn new<Client: OfflineClientT<T>>(
        nonce: u64,
        client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error>;
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
