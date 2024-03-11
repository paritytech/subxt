// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Miscellaneous utility helpers.

use url::Url;

use crate::{error::RpcError, Error};

pub use subxt_core::utils::{
    strip_compact_prefix, to_hex, AccountId32, Encoded, Era, MultiAddress, MultiSignature,
    PhantomDataSendSync, Static, UncheckedExtrinsic, WrapperKeepOpaque, Yes, H160, H256, H512, KeyedVec, bits,
};

/// A URL is considered secure if it uses a secure scheme ("https" or "wss") or is referring to localhost.
///
/// Returns an error if the the string could not be parsed into a URL.
pub fn url_is_secure(url: &str) -> Result<bool, Error> {
    let url = Url::parse(url).map_err(|e| Error::Rpc(RpcError::ClientError(Box::new(e))))?;

    let secure_scheme = url.scheme() == "https" || url.scheme() == "wss";
    let is_localhost = url.host().is_some_and(|e| match e {
        url::Host::Domain(e) => e == "localhost",
        url::Host::Ipv4(e) => e.is_loopback(),
        url::Host::Ipv6(e) => e.is_loopback(),
    });

    Ok(secure_scheme || is_localhost)
}

/// Validates, that the given Url is secure ("https" or "wss" scheme) or is referring to localhost.
pub fn validate_url_is_secure(url: &str) -> Result<(), Error> {
    if !url_is_secure(url)? {
        Err(Error::Rpc(crate::error::RpcError::InsecureUrl(url.into())))
    } else {
        Ok(())
    }
}
