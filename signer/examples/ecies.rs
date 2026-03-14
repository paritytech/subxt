// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! ECIES encryption/decryption example using sr25519 dev accounts.

use subxt_signer::sr25519;

fn main() {
    let alice = sr25519::dev::alice();
    let bob = sr25519::dev::bob();
    let alice_vk = alice.viewing_key();
    let bob_vk = bob.viewing_key();

    let plaintext = b"The quick brown fox jumps over the lazy dog";
    let ctx = b"example-ecies";

    // Alice encrypts a message for Bob
    let encrypted = alice
        .encrypt(plaintext, bob_vk.ivk_public(), ctx, alice_vk.ovk())
        .expect("encryption failed");

    println!("Plaintext:  {} bytes", plaintext.len());
    println!("Ciphertext: {} bytes (overhead: {} bytes)", encrypted.len(), encrypted.len() - plaintext.len());

    // Bob decrypts with his viewing key
    let decrypted = bob_vk.decrypt_incoming(&encrypted, ctx).expect("decryption failed");
    assert_eq!(&decrypted[..], plaintext);
    println!("Bob decrypted: {:?}", core::str::from_utf8(&decrypted).unwrap());

    // Alice re-reads her outgoing message
    let outgoing = alice_vk
        .decrypt_outgoing(&encrypted, bob_vk.ivk_public(), ctx)
        .expect("outgoing decryption failed");
    assert_eq!(&outgoing[..], plaintext);
    println!("Alice re-read outgoing: {:?}", core::str::from_utf8(&outgoing).unwrap());

    // Wrong context fails
    let result = bob_vk.decrypt_incoming(&encrypted, b"wrong-context");
    assert!(result.is_err());
    println!("Wrong context fails: {:?}", result.unwrap_err());
}
