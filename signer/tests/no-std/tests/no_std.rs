#[no_std]

use subxt_signer::{ecdsa, eth, sr25519};

// Run the tests by calling:
//
// ```text
// cargo test
// ```
//
// These are independent of any other package to ensure that nothing
// else enabled the same feature flag that subxt-signer needs to work ok
// (subxt seems to, for instance).

#[test]
fn sr25519_signing_works() {
    let alice = sr25519::dev::alice();

    // There's some non-determinism in the signing, so this ensures that
    // the rand stuff is configured properly to run ok in wasm.
    let signature = alice.sign(b"Hello there");
    assert!(sr25519::verify(
        &signature,
        b"Hello there",
        &alice.public_key()
    ));
}

#[test]
fn ecdsa_signing_works() {
    let alice = ecdsa::dev::alice();

    // There's some non-determinism in the signing, so this ensures that
    // the rand stuff is configured properly to run ok in wasm.
    let signature = alice.sign(b"Hello there");
    assert!(ecdsa::verify(
        &signature,
        b"Hello there",
        &alice.public_key()
    ));
}

#[test]
fn eth_signing_works() {
    let alice = eth::dev::alith();

    // There's some non-determinism in the signing, so this ensures that
    // the rand stuff is configured properly to run ok in wasm.
    let signature = alice.sign(b"Hello there");
    assert!(eth::verify(&signature, b"Hello there", &alice.public_key()));
}
