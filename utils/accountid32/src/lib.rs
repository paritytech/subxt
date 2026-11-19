// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The "default" Substrate/Polkadot AccountId. This is used in codegen, as well as signing related bits.
//! This doesn't contain much functionality itself, but is easy to convert to/from an `sp_core::AccountId32`
//! for instance, to gain functionality without forcing a dependency on Substrate crates here.

#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use thiserror::Error as DeriveError;

/// A 32-byte cryptographic identifier. This is a simplified version of Substrate's
/// `sp_core::crypto::AccountId32`. To obtain more functionality, convert this into
/// that type.
#[derive(
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    Debug,
    scale_encode::EncodeAsType,
    scale_decode::DecodeAsType,
    scale_info::TypeInfo,
)]
pub struct AccountId32(pub [u8; 32]);

impl AsRef<[u8]> for AccountId32 {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsRef<[u8; 32]> for AccountId32 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<[u8; 32]> for AccountId32 {
    fn from(x: [u8; 32]) -> Self {
        AccountId32(x)
    }
}

impl AccountId32 {
    // Display and serde retain the default Substrate prefix.
    fn ss58(&self) -> String {
        self.to_ss58check_with_prefix(42)
            .expect("42 is a valid SS58 prefix")
    }

    /// Encode this account as an SS58 address with the given network prefix.
    ///
    /// Prefixes `0..=63` occupy one byte and `64..=16383` occupy two bytes.
    /// Prefixes `46` and `47` are reserved and are rejected, as are values above
    /// `16383`. This does not change the prefix used by [`Display`](core::fmt::Display)
    /// or serde, which remains `42`.
    ///
    /// ```
    /// use subxt_utils_accountid32::AccountId32;
    ///
    /// let account = AccountId32([1; 32]);
    /// let address = account.to_ss58check_with_prefix(0).unwrap();
    /// assert_eq!(address.parse::<AccountId32>().unwrap(), account);
    /// ```
    pub fn to_ss58check_with_prefix(&self, prefix: u16) -> Result<String, InvalidSs58Prefix> {
        if prefix > 16383 || matches!(prefix, 46 | 47) {
            return Err(InvalidSs58Prefix);
        }
        let mut v = if prefix <= 63 {
            vec![prefix as u8]
        } else {
            vec![
                0b0100_0000 | ((prefix & 0x00fc) >> 2) as u8,
                (prefix >> 8) as u8 | ((prefix & 0x0003) << 6) as u8,
            ]
        };
        // then push the account ID bytes.
        v.extend(self.0);
        // then push a 2 byte checksum of what we have so far.
        let r = ss58hash(&v);
        v.extend(&r[0..2]);
        // then encode to base58.
        use base58::ToBase58;
        Ok(v.to_base58())
    }

    // This isn't strictly needed, but to give our AccountId32 a little more usefulness, we also
    // implement the logic needed to decode an AccountId32 from an SS58 encoded string. This is exposed
    // via a `FromStr` impl.
    fn from_ss58(s: &str) -> Result<Self, FromSs58Error> {
        const CHECKSUM_LEN: usize = 2;
        let body_len = 32;

        use base58::FromBase58;
        let data = s.from_base58().map_err(|_| FromSs58Error::BadBase58)?;
        if data.len() < 2 {
            return Err(FromSs58Error::BadLength);
        }
        let prefix_len = match data[0] {
            0..=63 => 1,
            64..=127 => 2,
            _ => return Err(FromSs58Error::InvalidPrefix),
        };
        if data.len() != prefix_len + body_len + CHECKSUM_LEN {
            return Err(FromSs58Error::BadLength);
        }
        let hash = ss58hash(&data[0..body_len + prefix_len]);
        let checksum = &hash[0..CHECKSUM_LEN];
        if data[body_len + prefix_len..body_len + prefix_len + CHECKSUM_LEN] != *checksum {
            // Invalid checksum.
            return Err(FromSs58Error::InvalidChecksum);
        }

        let result = data[prefix_len..body_len + prefix_len]
            .try_into()
            .map_err(|_| FromSs58Error::BadLength)?;
        Ok(AccountId32(result))
    }
}

/// An invalid SS58 network prefix: greater than `16383`, or reserved (`46` or `47`).
#[derive(Clone, Copy, Eq, PartialEq, Debug, DeriveError)]
#[error("Invalid SS58 network prefix: expected 0..=16383, excluding 46 and 47")]
pub struct InvalidSs58Prefix;

/// An error obtained from trying to interpret an SS58 encoded string into an AccountId32
#[derive(Clone, Copy, Eq, PartialEq, Debug, DeriveError)]
#[allow(missing_docs)]
pub enum FromSs58Error {
    #[error("Base 58 requirement is violated")]
    BadBase58,
    #[error("Length is bad")]
    BadLength,
    #[error("Invalid checksum")]
    InvalidChecksum,
    #[error("Invalid SS58 prefix byte.")]
    InvalidPrefix,
}

// We do this just to get a checksum to help verify the validity of the address in ss58
fn ss58hash(data: &[u8]) -> Vec<u8> {
    use blake2::{Blake2b512, Digest};
    const PREFIX: &[u8] = b"SS58PRE";
    let mut ctx = Blake2b512::new();
    ctx.update(PREFIX);
    ctx.update(data);
    ctx.finalize().to_vec()
}

impl Serialize for AccountId32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.ss58())
    }
}

impl<'de> Deserialize<'de> for AccountId32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        AccountId32::from_ss58(&String::deserialize(deserializer)?)
            .map_err(|e| serde::de::Error::custom(format!("{e:?}")))
    }
}

impl core::fmt::Display for AccountId32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.ss58())
    }
}

impl core::str::FromStr for AccountId32 {
    type Err = FromSs58Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AccountId32::from_ss58(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sp_core::{self, crypto::Ss58Codec};
    use sp_keyring::sr25519::Keyring;

    #[test]
    fn ss58_prefixes_match_substrate_and_round_trip() {
        for keyring in [Keyring::Alice, Keyring::Bob, Keyring::Charlie] {
            let substrate_account = keyring.to_account_id();
            let account = AccountId32(substrate_account.clone().into());
            for prefix in [0, 2, 42, 63, 64, 65, 255, 256, 1000, 16383] {
                let address = account.to_ss58check_with_prefix(prefix).unwrap();
                assert_eq!(
                    address,
                    substrate_account.to_ss58check_with_version(prefix.into()),
                    "prefix {prefix}"
                );
                assert_eq!(address.parse::<AccountId32>().unwrap(), account);
                let (decoded, format) =
                    sp_core::crypto::AccountId32::from_ss58check_with_version(&address).unwrap();
                assert_eq!(decoded, substrate_account);
                assert_eq!(u16::from(format), prefix);
            }
            assert_eq!(
                format!("{account}"),
                account.to_ss58check_with_prefix(42).unwrap()
            );
        }
    }

    #[test]
    fn ss58_rejects_reserved_and_out_of_range_prefixes() {
        let account = AccountId32([0; 32]);
        for prefix in [46, 47, 16384, u16::MAX] {
            assert_eq!(
                account.to_ss58check_with_prefix(prefix),
                Err(InvalidSs58Prefix)
            );
        }
    }

    #[test]
    fn ss58_is_compatible_with_substrate_impl() {
        let keyrings = vec![Keyring::Alice, Keyring::Bob, Keyring::Charlie];

        for keyring in keyrings {
            let substrate_account = keyring.to_account_id();
            let local_account = AccountId32(substrate_account.clone().into());

            // Both should encode to ss58 the same way:
            let substrate_ss58 = substrate_account.to_ss58check();
            assert_eq!(substrate_ss58, local_account.ss58());

            // Both should decode from ss58 back to the same:
            assert_eq!(
                sp_core::crypto::AccountId32::from_ss58check(&substrate_ss58).unwrap(),
                substrate_account
            );
            assert_eq!(
                AccountId32::from_ss58(&substrate_ss58).unwrap(),
                local_account
            );
        }
    }
}
