// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;
use zeroize::Zeroize;

/// This is taken from `substrate-bip39` so that we can keep dependencies in line, and
/// is the same logic that sp-core uses to go from mnemonic entropy to seed. Returns
/// `None` if invalid length.
pub fn seed_from_entropy(entropy: &[u8], password: &str) -> Option<[u8; 64]> {
    if entropy.len() < 16 || entropy.len() > 32 || entropy.len() % 4 != 0 {
        return None;
    }

    let mut salt = String::with_capacity(8 + password.len());
    salt.push_str("mnemonic");
    salt.push_str(password);

    let mut seed = [0u8; 64];

    pbkdf2::<Hmac<Sha512>>(entropy, salt.as_bytes(), 2048, &mut seed).ok()?;

    salt.zeroize();

    Some(seed)
}
