// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A Polkadot-JS account loader.

use base64::Engine;
use crypto_secretbox::{
    Key, Nonce, XSalsa20Poly1305,
    aead::{Aead, KeyInit},
};
use serde::Deserialize;

use thiserror::Error as DeriveError;

use crate::sr25519;

/// Given a JSON keypair as exported from Polkadot-JS, this returns an [`sr25519::Keypair`]
pub fn decrypt_json(json: &str, password: &str) -> Result<sr25519::Keypair, Error> {
    let pair_json: KeyringPairJson = serde_json::from_str(json)?;
    Ok(pair_json.decrypt(password)?)
}

/// Error
#[derive(Debug, DeriveError)]
pub enum Error {
    /// Error decoding JSON.
    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// The keypair has an unsupported encoding.
    #[error("Unsupported encoding.")]
    UnsupportedEncoding,
    /// Base64 decoding error.
    #[error("Base64 decoding error: {0}")]
    Base64(#[from] base64::DecodeError),
    /// Wrong Scrypt parameters
    #[error("Unsupported Scrypt parameters: N: {n}, p: {p}, r: {r}")]
    UnsupportedScryptParameters {
        /// N
        n: u32,
        /// p
        p: u32,
        /// r
        r: u32,
    },
    /// Decryption error.
    #[error("Decryption error: {0}")]
    Secretbox(#[from] crypto_secretbox::Error),
    /// sr25519 keypair error.
    #[error(transparent)]
    Sr25519(#[from] sr25519::Error),
    /// The decrypted keys are not valid.
    #[error("The decrypted keys are not valid.")]
    InvalidKeys,
}

#[derive(Deserialize)]
struct EncryptionMetadata {
    /// Descriptor for the content
    content: Vec<String>,
    /// The encoding (in current/latest versions this is always an array)
    r#type: Vec<String>,
    /// The version of encoding applied
    version: String,
}

/// https://github.com/polkadot-js/common/blob/37fa211fdb141d4f6eb32e8f377a4651ed2d9068/packages/keyring/src/types.ts#L67
#[derive(Deserialize)]
struct KeyringPairJson {
    /// The encoded string
    encoded: String,
    /// The encoding used
    encoding: EncryptionMetadata,
    /// The ss58 encoded address or the hex-encoded version (the latter is for ETH-compat chains)
    address: AccountId32,
}

// Re-export this type which is used above.
pub use subxt_utils_accountid32::AccountId32;

// This can be removed once split_array is stabilized.
fn slice_to_u32(slice: &[u8]) -> u32 {
    u32::from_le_bytes(slice.try_into().expect("Slice should be 4 bytes."))
}

impl KeyringPairJson {
    /// Decrypt JSON keypair.
    fn decrypt(self, password: &str) -> Result<sr25519::Keypair, Error> {
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
        let params: [u8; 68] = decoded[..68]
            .try_into()
            .map_err(|_| Error::UnsupportedEncoding)?;

        // Extract scrypt parameters.
        // https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/scrypt/fromU8a.ts
        let salt = &params[0..32];
        let n = slice_to_u32(&params[32..36]);
        let p = slice_to_u32(&params[36..40]);
        let r = slice_to_u32(&params[40..44]);

        // FIXME At this moment we assume these to be fixed params, this is not a great idea
        // since we lose flexibility and updates for greater security. However we need some
        // protection against carefully-crafted params that can eat up CPU since these are user
        // inputs. So we need to get very clever here, but atm we only allow the defaults
        // and if no match, bail out.
        if n != 32768 || p != 1 || r != 8 {
            return Err(Error::UnsupportedScryptParameters { n, p, r });
        }

        // Hash password.
        let scrypt_params =
            scrypt::Params::new(15, 8, 1, 32).expect("Provided parameters should be valid.");
        let mut key = Key::default();
        scrypt::scrypt(password.as_bytes(), salt, &scrypt_params, &mut key)
            .expect("Key should be 32 bytes.");

        // Decrypt keys.
        // https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/json/decryptData.ts
        let cipher = XSalsa20Poly1305::new(&key);
        let nonce = Nonce::from_slice(&params[44..68]);
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
        let keypair = sr25519::Keypair::from_ed25519_bytes(secret_key)?;

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
        decrypt_json(json, "whoisalice").unwrap();
    }
}
