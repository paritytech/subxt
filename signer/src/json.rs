// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A Polkadot-JS account loader.

use base64::Engine;
use core::fmt::Display;
use crypto_secretbox::{
    aead::{Aead, KeyInit},
    Key, Nonce, XSalsa20Poly1305,
};
use serde::Deserialize;
use subxt_core::utils::AccountId32;

use crate::sr25519;

/// Error
#[derive(Debug)]
pub enum Error {
    /// The keypair has an unsupported encoding.
    UnsupportedEncoding,
    /// Base64 decoding error.
    Base64(base64::DecodeError),
    /// Wrong Scrypt parameters
    UnsupportedScryptParameters {
        /// N
        n: u32,
        /// p
        p: u32,
        /// r
        r: u32,
    },
    /// Decryption error.
    Secretbox(crypto_secretbox::Error),
    /// The decrypted keys are not valid.
    InvalidKeys,
}

impl_from!(base64::DecodeError => Error::Base64);
impl_from!(crypto_secretbox::Error => Error::Secretbox);

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::UnsupportedEncoding => write!(f, "Unsupported encoding."),
            Error::Base64(e) => write!(f, "Base64 decoding error: {e}"),
            Error::UnsupportedScryptParameters { n, p, r } => {
                write!(f, "Unsupported Scrypt parameters: N: {n}, p: {p}, r: {r}")
            }
            Error::Secretbox(e) => write!(f, "Decryption error: {e}"),
            Error::InvalidKeys => write!(f, "The decrypted keys are not valid."),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EncryptedJsonDescriptor {
    /// Descriptor for the content
    content: Vec<String>,
    /// The encoding (in current/latest versions this is always an array)
    r#type: Vec<String>,
    /// The version of encoding applied
    version: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Meta {
    genesis_hash: String,
    is_hardware: Option<bool>,
    name: String,
    tags: Option<Vec<String>>,
    when_created: u64,
}

/// https://github.com/polkadot-js/common/blob/37fa211fdb141d4f6eb32e8f377a4651ed2d9068/packages/keyring/src/types.ts#L67
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct KeyringPairJson {
    /// The encoded string
    encoded: String,
    /// The encoding used
    encoding: EncryptedJsonDescriptor,
    /// The ss58 encoded address or the hex-encoded version (the latter is for ETH-compat chains)
    address: AccountId32,
    /// The underlying metadata associated with the keypair
    meta: Meta,
}

impl KeyringPairJson {
    /// Decrypt JSON keypair.
    pub fn decrypt(self, password: &str) -> Result<sr25519::Keypair, Error> {
        // Check encoding.
        // https://github.com/polkadot-js/common/blob/37fa211fdb141d4f6eb32e8f377a4651ed2d9068/packages/keyring/src/keyring.ts#L166
        if self.encoding.version != "3"
            || !self.encoding.content.contains(&"pkcs8".to_owned())
            || !self.encoding.content.contains(&"sr25519".to_owned())
            || !self.encoding.r#type.contains(&"scrypt".to_owned())
            || !self
                .encoding
                .r#type
                .contains(&"xsalsa20-poly1305".to_owned())
        {
            return Err(Error::UnsupportedEncoding);
        }

        // Decode from Base64.
        let decoded = base64::engine::general_purpose::STANDARD.decode(self.encoded)?;
        if decoded.len() < 68 {
            return Err(Error::UnsupportedEncoding);
        }

        // Extract scrypt parameters.
        // https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/scrypt/fromU8a.ts
        let salt = &decoded[0..32];
        let n = u32::from_le_bytes(decoded[32..36].try_into().unwrap());
        let p = u32::from_le_bytes(decoded[36..40].try_into().unwrap());
        let r = u32::from_le_bytes(decoded[40..44].try_into().unwrap());

        if n != 32768 || p != 1 || r != 8 {
            return Err(Error::UnsupportedScryptParameters { n, p, r });
        }

        // Hash password.
        let scrypt_params = scrypt::Params::new(15, 8, 1, 32).unwrap();
        let mut key = Key::default();
        scrypt::scrypt(password.as_bytes(), salt, &scrypt_params, &mut key).unwrap();

        // Decrypt keys.
        // https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/json/decryptData.ts
        let cipher = XSalsa20Poly1305::new(&key);
        let nonce = Nonce::from_slice(&decoded[44..68]);
        let ciphertext = &decoded[68..];
        let plaintext = cipher.decrypt(nonce, ciphertext)?;

        // https://github.com/polkadot-js/common/blob/master/packages/keyring/src/pair/decode.ts
        if plaintext.len() != 117 {
            return Err(Error::InvalidKeys);
        }

        let header = &plaintext[0..16];
        let secret_key = &plaintext[16..80];
        let div = &plaintext[80..85];
        let public_key = &plaintext[85..117];

        if header != [48, 83, 2, 1, 1, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32]
            || div != [161, 35, 3, 33, 0]
        {
            return Err(Error::InvalidKeys);
        }

        // Generate keypair.
        let keypair = sr25519::Keypair::from_ed25519_bytes(secret_key).unwrap();

        // Ensure keys are correct.
        if keypair.public_key().0 != public_key
            || keypair.public_key().to_account_id() != self.address
        {
            return Err(Error::InvalidKeys);
        }

        Ok(keypair)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_keypair_sr25519() {
        let json = r#"
            {
              "encoded": "DumgApKCTqoCty1OZW/8WS+sgo6RdpHhCwAkA2IoDBMAgAAAAQAAAAgAAAB6IG/q24EeVf0JqWqcBd5m2tKq5BlyY84IQ8oamLn9DZe9Ouhgunr7i36J1XxUnTI801axqL/ym1gil0U8440Qvj0lFVKwGuxq38zuifgoj0B3Yru0CI6QKEvQPU5xxj4MpyxdSxP+2PnTzYao0HDH0fulaGvlAYXfqtU89xrx2/z9z7IjSwS3oDFPXRQ9kAdDebtyCVreZ9Otw9v3",
              "encoding": {
                "content": [
                  "pkcs8",
                  "sr25519"
                ],
                "type": [
                  "scrypt",
                  "xsalsa20-poly1305"
                ],
                "version": "3"
              },
              "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
              "meta": {
                "genesisHash": "",
                "name": "Alice",
                "whenCreated": 1718265838755
              }
            }
        "#;
        let pair_json: KeyringPairJson = serde_json::from_str(json).unwrap();
        pair_json.decrypt("whoisalice").unwrap();
    }
}
