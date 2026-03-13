// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! ECIES encryption/decryption example using sr25519 dev accounts.

use subxt_signer::sr25519;

fn main() {
    let alice = sr25519::dev::alice();
    let bob = sr25519::dev::bob();

    let plaintext = b"The quick brown fox jumps over the lazy dog";
    let ctx = b"example-ecies-v1";

    // Alice encrypts a message for Bob
    let encrypted = alice
        .encrypt(plaintext, &bob.public_key(), ctx)
        .expect("encryption failed");

    println!("Plaintext:  {} bytes", plaintext.len());
    println!("Ciphertext: {} bytes (overhead: {} bytes)", encrypted.len(), encrypted.len() - plaintext.len());

    // Bob decrypts
    let decrypted = bob.decrypt(&encrypted, ctx).expect("decryption failed");
    assert_eq!(&decrypted[..], plaintext);
    println!("Bob decrypted successfully: {:?}", core::str::from_utf8(&decrypted).unwrap());

    // Alice cannot decrypt (wrong key)
    let result = alice.decrypt(&encrypted, ctx);
    assert!(result.is_err());
    println!("Alice cannot decrypt Bob's message: {:?}", result.unwrap_err());

    // Wrong context fails
    let result = bob.decrypt(&encrypted, b"wrong-context");
    assert!(result.is_err());
    println!("Wrong context fails: {:?}", result.unwrap_err());
}
