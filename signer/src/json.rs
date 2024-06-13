use base64::Engine;
use core::fmt::Display;
use crypto_secretbox::{
    aead::{Aead, KeyInit},
    Key, Nonce, XSalsa20Poly1305,
};
use serde::Deserialize;
use subxt_core::utils::AccountId32;

use crate::sr25519;

#[derive(Debug)]
pub enum Error {
    Sr25519(sr25519::Error),
}

impl_from!(sr25519::Error => Error::Sr25519);

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Sr25519(_) => todo!(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EncryptedJsonDescriptor {
    /** Descriptor for the content */
    content: Vec<String>,
    /** The encoding (in current/latest versions this is always an array) */
    r#type: Vec<String>,
    /** The version of encoding applied */
    version: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meta {
    genesis_hash: String,
    is_hardware: Option<bool>,
    name: String,
    tags: Option<Vec<String>>,
    when_created: u64,
}

/// Defined here: https://github.com/polkadot-js/common/blob/37fa211fdb141d4f6eb32e8f377a4651ed2d9068/packages/keyring/src/types.ts#L67
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyringPairJson {
    /** The encoded string */
    encoded: String,
    /** The encoding used */
    encoding: EncryptedJsonDescriptor,
    /** The ss58 encoded address or the hex-encoded version (the latter is for ETH-compat chains) */
    address: AccountId32,
    /** The underlying metadata associated with the keypair */
    meta: Meta,
}

impl KeyringPairJson {
    fn get_keypair_sr25519(self, password: &str) -> Result<sr25519::Keypair, Error> {
        // Check compatibility.
        if self.encoding.version != "3" {}
        // Decode from Base64.
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(self.encoded)
            .unwrap();
        // Extract scrypt parameters.
        // https://datatracker.ietf.org/doc/html/rfc7914#section-7
        let salt = &decoded[0..32];
        let _n = u32::from_be_bytes(decoded[32..36].try_into().unwrap());
        let _p = u32::from_be_bytes(decoded[36..40].try_into().unwrap());
        let _r = u32::from_be_bytes(decoded[40..44].try_into().unwrap());
        // Hash password.
        let scrypt_params = scrypt::Params::new(15, 8, 1, 64).unwrap();
        let mut key = Key::default();
        scrypt::scrypt(password.as_bytes(), salt, &scrypt_params, &mut key).unwrap();
        // Decrypt private key.
        let nonce = Nonce::from_slice(&decoded[44..68]);
        let ciphertext = &decoded[68..];
        let cipher = XSalsa20Poly1305::new(&key);
        let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();
        // Get pair.
        crate::sr25519::Keypair::from_secret_key(plaintext.try_into().unwrap())
            .map_err(|e| Error::Sr25519(e))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn check_from_phrase_matches() {
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

        let pair = pair_json.get_keypair_sr25519("whoisalice").unwrap();
    }
}
