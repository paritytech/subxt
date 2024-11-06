// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! `AccountId20` is a representation of Ethereum address derived from hashing the public key.

use alloc::format;
use alloc::string::String;
use codec::{Decode, Encode};
use keccak_hash::keccak;
use serde::{Deserialize, Serialize};
use thiserror::Error as DeriveError;

#[derive(
    Copy,
    Clone,
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
/// Ethereum-compatible `AccountId`.
pub struct AccountId20(pub [u8; 20]);

impl AsRef<[u8]> for AccountId20 {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsRef<[u8; 20]> for AccountId20 {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl From<[u8; 20]> for AccountId20 {
    fn from(x: [u8; 20]) -> Self {
        AccountId20(x)
    }
}

impl AccountId20 {
    /// Convert to a public key hash
    pub fn checksum(&self) -> String {
        let hex_address = hex::encode(self.0);
        let hash = keccak(hex_address.as_bytes());

        let mut checksum_address = String::with_capacity(42);
        checksum_address.push_str("0x");

        for (i, ch) in hex_address.chars().enumerate() {
            // Get the corresponding nibble from the hash
            let nibble = hash[i / 2] >> (if i % 2 == 0 { 4 } else { 0 }) & 0xf;

            if nibble >= 8 {
                checksum_address.push(ch.to_ascii_uppercase());
            } else {
                checksum_address.push(ch);
            }
        }

        checksum_address
    }
}

/// An error obtained from trying to interpret a hex encoded string into an AccountId20
#[derive(Clone, Copy, Eq, PartialEq, Debug, DeriveError)]
#[allow(missing_docs)]
pub enum FromChecksumError {
    #[error("Length is bad")]
    BadLength,
    #[error("Invalid checksum")]
    InvalidChecksum,
    #[error("Invalid checksum prefix byte.")]
    InvalidPrefix,
}

impl Serialize for AccountId20 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.checksum())
    }
}

impl<'de> Deserialize<'de> for AccountId20 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<AccountId20>()
            .map_err(|e| serde::de::Error::custom(format!("{e:?}")))
    }
}

impl core::fmt::Display for AccountId20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.checksum())
    }
}

impl core::str::FromStr for AccountId20 {
    type Err = FromChecksumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 42 {
            return Err(FromChecksumError::BadLength);
        }
        if !s.starts_with("0x") {
            return Err(FromChecksumError::InvalidPrefix);
        }
        hex::decode(&s.as_bytes()[2..])
            .map_err(|_| FromChecksumError::InvalidChecksum)?
            .try_into()
            .map(AccountId20)
            .map_err(|_| FromChecksumError::BadLength)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialisation() {
        let key_hashes = vec![
            "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac",
            "0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0",
            "0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc",
            "0x773539d4Ac0e786233D90A233654ccEE26a613D9",
            "0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB",
            "0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d",
        ];

        for key_hash in key_hashes {
            let parsed: AccountId20 = key_hash.parse().expect("Failed to parse");

            let encoded = parsed.checksum();

            // `encoded` should be equal to the initial key_hash
            assert_eq!(encoded, key_hash);
        }
    }
}
